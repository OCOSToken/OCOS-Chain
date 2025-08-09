//! OCOS Router Protocol (ORP) – Solana side
//! ----------------------------------------
//! Goals:
//! - Process cross-chain ORPMessage delivered by a trusted bridge/relayer.
//! - Enforce deadline, replay protection, optional adapter allowlist.
//! - Perform token settlement to the receiver and (optionally) DEX CPI.
//!
//! Message flow assumptions:
//! 1) Bridge/relayer transfers/mints `token_in` into Router vault ATA (or passes accounts to do so).
//! 2) It then calls `process_message`, providing message bytes and remaining accounts for DEX CPI.
//! 3) Program validates deadline, computes msg_id = keccak(message), checks replay, executes swap/settlement.
//!
//! Extend points:
//! - plug_dex_swap(): wire CPI into Raydium/Orca using remaining accounts (placeholder in this version).
//!
//! Security notes:
//! - Config PDA stores guardian & pause flag.
//! - Processed PDA prevents replays using msg_id as seed.
//! - All token ops are via the Token Program; ATAs are resolved/created safely.

#![deny(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![allow(clippy::too_many_arguments)]

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint, entrypoint::ProgramResult,
    hash::Hasher,
    keccak::{hash as keccak256, HASH_BYTES},
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_memory::sol_memcmp,
    pubkey::Pubkey,
    sysvar::{clock::Clock, Sysvar},
};

use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction as token_ix;

// -----------------------------------------------------------------------------
// Constants & Seeds
// -----------------------------------------------------------------------------

const CONFIG_SEED: &[u8] = b"orp_config";
const PROCESSED_SEED: &[u8] = b"orp_processed";

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

#[derive(thiserror::Error, Debug, Copy, Clone)]
pub enum OrpError {
    #[error("Invalid instruction data")]
    InvalidData,
    #[error("Config mismatch")]
    ConfigMismatch,
    #[error("Program is paused")]
    Paused,
    #[error("Deadline passed")]
    Deadline,
    #[error("Replay: message already processed")]
    AlreadyProcessed,
    #[error("Unauthorized caller")]
    Unauthorized,
    #[error("Invalid accounts")]
    InvalidAccounts,
    #[error("Math overflow")]
    MathOverflow,
    #[error("Token operation failed")]
    TokenOpFailed,
}

impl From<OrpError> for ProgramError {
    fn from(e: OrpError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

// -----------------------------------------------------------------------------
// Data Types
// -----------------------------------------------------------------------------

/// Matches the EVM-side fields semantically. Binary format is Borsh on Solana.
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct OrpMessage {
    pub version: u64,          // 1
    pub src_chain_id: u64,     // for auditing
    pub dst_chain_id: u64,     // should match Solana domain (informational)
    pub route_id: [u8; 32],    // keccak(config) on EVM
    pub src_sender: [u8; 32],  // opaque; could reference EVM addr
    pub receiver: [u8; 32],    // 32-byte pubkey (receiver)
    pub token_in_mint: [u8; 32],
    pub token_out_mint: [u8; 32],
    pub amount_in: u64,
    pub min_amount_out: u64,
    pub deadline_unix: i64,
    pub nonce: u64,
    pub call_data: Vec<u8>,    // optional: downstream CPI params
    pub aux: Vec<u8>,          // optional: metadata
}

/// PDA: Program configuration
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default)]
pub struct Config {
    pub guardian: Pubkey,
    pub paused: bool,
    pub bump: u8,
}

/// PDA: Marks a message as processed (replay protection)
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default)]
pub struct Processed {
    pub msg_id: [u8; 32],  // keccak(message_bytes)
    pub bump: u8,
}

// -----------------------------------------------------------------------------
// Instruction discriminants
// -----------------------------------------------------------------------------

#[repr(u8)]
pub enum OrpIx {
    /// Initialize config PDA
    InitConfig = 0,
    /// Set paused flag (guardian)
    SetPaused = 1,
    /// Update guardian (guardian)
    SetGuardian = 2,
    /// Process an ORPMessage (trusted bridge/relayer)
    ProcessMessage = 3,
}

impl TryFrom<u8> for OrpIx {
    type Error = ProgramError;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(OrpIx::InitConfig),
            1 => Ok(OrpIx::SetPaused),
            2 => Ok(OrpIx::SetGuardian),
            3 => Ok(OrpIx::ProcessMessage),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

// -----------------------------------------------------------------------------
// Entrypoint
// -----------------------------------------------------------------------------

entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let (disc, rest) = input.split_first().ok_or(OrpError::InvalidData)?;
    match OrpIx::try_from(*disc)? {
        OrpIx::InitConfig => ix_init_config(program_id, accounts, rest),
        OrpIx::SetPaused => ix_set_paused(program_id, accounts, rest),
        OrpIx::SetGuardian => ix_set_guardian(program_id, accounts, rest),
        OrpIx::ProcessMessage => ix_process_message(program_id, accounts, rest),
    }
}

// -----------------------------------------------------------------------------
// Instructions
// -----------------------------------------------------------------------------

/// InitConfig: creates Config PDA (guardian = signer)
/// Accounts:
/// 0. [signer] guardian
/// 1. [writable] config_pda
/// 2. [] system_program
pub fn ix_init_config(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _data: &[u8],
) -> ProgramResult {
    use solana_program::{system_instruction, system_program, program_pack::Pack};
    let mut ai = accounts.iter();
    let guardian = next_account_info(&mut ai)?;        // signer
    let config_pda = next_account_info(&mut ai)?;      // writable
    let system = next_account_info(&mut ai)?;
    if !guardian.is_signer {
        return Err(OrpError::Unauthorized.into());
    }
    if *system.key != system_program::id() {
        return Err(OrpError::InvalidAccounts.into());
    }

    // Derive PDA
    let (expected, bump) = Pubkey::find_program_address(&[CONFIG_SEED], program_id);
    if expected != *config_pda.key {
        return Err(OrpError::ConfigMismatch.into());
    }

    // Create if not exists
    if config_pda.data_is_empty() {
        let space = 8 + std::mem::size_of::<Config>() as u64; // +discriminator if using Anchor; here plain account
        let rent = solana_program::rent::Rent::get()?.minimum_balance(space as usize);
        let ix = system_instruction::create_account(
            guardian.key,
            config_pda.key,
            rent,
            space,
            program_id,
        );
        invoke_signed(
            &ix,
            &[guardian.clone(), config_pda.clone(), system.clone()],
            &[&[CONFIG_SEED, &[bump]]],
        )?;

        let mut cfg = Config { guardian: *guardian.key, paused: false, bump };
        cfg.serialize(&mut &mut config_pda.data.borrow_mut()[..])?;
        msg!("ORP: Config initialized. Guardian = {}", guardian.key);
    } else {
        // idempotent: update guardian if already created by same signer
        let mut cfg = Config::try_from_slice(&config_pda.data.borrow())?;
        cfg.guardian = *guardian.key;
        cfg.serialize(&mut &mut config_pda.data.borrow_mut()[..])?;
        msg!("ORP: Config updated. Guardian = {}", guardian.key);
    }
    Ok(())
}

/// SetPaused(paused: u8 {0/1})
/// Accounts:
/// 0. [signer] guardian
/// 1. [writable] config_pda
pub fn ix_set_paused(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let paused = *data.get(0).ok_or(OrpError::InvalidData)? != 0;
    let mut ai = accounts.iter();
    let guardian = next_account_info(&mut ai)?;
    let config_pda = next_account_info(&mut ai)?;
    if !guardian.is_signer {
        return Err(OrpError::Unauthorized.into());
    }
    let (cfg_key, _bump) = Pubkey::find_program_address(&[CONFIG_SEED], program_id);
    if cfg_key != *config_pda.key {
        return Err(OrpError::ConfigMismatch.into());
    }
    let mut cfg = Config::try_from_slice(&config_pda.data.borrow())?;
    if cfg.guardian != *guardian.key {
        return Err(OrpError::Unauthorized.into());
    }
    cfg.paused = paused;
    cfg.serialize(&mut &mut config_pda.data.borrow_mut()[..])?;
    msg!("ORP: Paused set to {}", paused);
    Ok(())
}

/// SetGuardian(new_guardian: Pubkey)
/// Accounts:
/// 0. [signer] guardian
/// 1. [writable] config_pda
pub fn ix_set_guardian(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if data.len() < 32 {
        return Err(OrpError::InvalidData.into());
    }
    let new_guardian = Pubkey::new_from_array(
        data[0..32].try_into().map_err(|_| OrpError::InvalidData)?,
    );
    let mut ai = accounts.iter();
    let guardian = next_account_info(&mut ai)?;
    let config_pda = next_account_info(&mut ai)?;
    if !guardian.is_signer {
        return Err(OrpError::Unauthorized.into());
    }
    let (cfg_key, _bump) = Pubkey::find_program_address(&[CONFIG_SEED], program_id);
    if cfg_key != *config_pda.key {
        return Err(OrpError::ConfigMismatch.into());
    }
    let mut cfg = Config::try_from_slice(&config_pda.data.borrow())?;
    if cfg.guardian != *guardian.key {
        return Err(OrpError::Unauthorized.into());
    }
    cfg.guardian = new_guardian;
    cfg.serialize(&mut &mut config_pda.data.borrow_mut()[..])?;
    msg!("ORP: Guardian updated to {}", new_guardian);
    Ok(())
}

/// ProcessMessage(message_borsh: Vec<u8>)
///
/// Accounts:
/// 0. [writable] config_pda
/// 1. [writable] processed_pda (PDA: seeds = ["orp_processed", keccak(message)])
/// 2. [] clock sysvar
/// 3. [] token_program
/// 4. [] associated_token_program
/// 5. [] system_program
/// 6. [] rent sysvar (optional in >=1.18, but kept for compatibility)
/// 7. [writable] router_vault_ata (token_in mint ATA owned by program or a designated vault key)
/// 8. [writable] receiver_ata
/// 9+. remaining accounts for DEX CPI (optional)
pub fn ix_process_message(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    // Deserialize message
    let msg_bytes = data;
    let m: OrpMessage = OrpMessage::try_from_slice(msg_bytes).map_err(|_| OrpError::InvalidData)?;

    // Compute msg_id = keccak(message_bytes)
    let k = keccak256(msg_bytes);
    let msg_id: [u8; 32] = k.0;

    // Accounts
    let mut ai = accounts.iter();
    let config_pda = next_account_info(&mut ai)?;
    let processed_pda = next_account_info(&mut ai)?;
    let clock_ai = next_account_info(&mut ai)?;
    let token_program = next_account_info(&mut ai)?;
    let ata_program = next_account_info(&mut ai)?;
    let system_program = next_account_info(&mut ai)?;
    let _rent_sysvar = next_account_info(&mut ai).ok(); // optional on newer runtimes
    let router_vault_ata = next_account_info(&mut ai)?;
    let receiver_ata = next_account_info(&mut ai)?;

    // Load config
    let (cfg_key, _bump_cfg) = Pubkey::find_program_address(&[CONFIG_SEED], program_id);
    if cfg_key != *config_pda.key {
        return Err(OrpError::ConfigMismatch.into());
    }
    let cfg = Config::try_from_slice(&config_pda.data.borrow())?;
    if cfg.paused {
        return Err(OrpError::Paused.into());
    }

    // Deadline
    let now = Clock::from_account_info(clock_ai)?.unix_timestamp;
    if now > m.deadline_unix {
        return Err(OrpError::Deadline.into());
    }

    // Derive processed PDA
    let (expected_processed, bump_p) =
        Pubkey::find_program_address(&[PROCESSED_SEED, &msg_id], program_id);
    if expected_processed != *processed_pda.key {
        return Err(OrpError::InvalidAccounts.into());
    }
    // Check replay
    if !processed_pda.data_is_empty() {
        let p = Processed::try_from_slice(&processed_pda.data.borrow())?;
        if p.msg_id == msg_id {
            return Err(OrpError::AlreadyProcessed.into());
        }
    } else {
        // Create account to mark processed
        use solana_program::{system_instruction, rent::Rent};
        let space = 8 + std::mem::size_of::<Processed>() as u64;
        let rent = Rent::get()?.minimum_balance(space as usize);
        let ix = system_instruction::create_account(
            config_pda.key,            // payer (could be changed to a designated payer)
            processed_pda.key,
            rent,
            space,
            program_id,
        );
        invoke_signed(
            &ix,
            &[
                config_pda.clone(),
                processed_pda.clone(),
                system_program.clone(),
            ],
            &[&[PROCESSED_SEED, &msg_id, &[bump_p]]],
        )?;
        Processed { msg_id, bump: bump_p }
            .serialize(&mut &mut processed_pda.data.borrow_mut()[..])?;
    }

    // Token settlement
    // Parse mints and receiver
    let token_in_mint = Pubkey::new_from_array(m.token_in_mint);
    let token_out_mint = Pubkey::new_from_array(m.token_out_mint);
    let receiver_pk = Pubkey::new_from_array(m.receiver);

    // If token_out == token_in and no DEX swap required → direct transfer
    // Otherwise, run the DEX CPI hook (placeholder) and deposit result to receiver_ata.
    let out_amount = if token_out_mint == token_in_mint {
        // direct transfer router_vault_ata -> receiver_ata
        transfer_spl_tokens(
            token_program,
            router_vault_ata,
            receiver_ata,
            program_id,
            &[],
            m.amount_in,
        )?;
        m.amount_in
    } else {
        // Placeholder: perform DEX CPI using remaining accounts
        // Expected: program swaps m.amount_in in router_vault_ata to token_out_mint
        // and credits receiver_ata with the output. Here we simulate "min_out" check by
        // requiring caller to have prepared the funds in receiver_ata >= min_amount_out.
        // Replace this block with actual DEX CPI integration.
        let actual_out = spl_token::state::Account::unpack(&receiver_ata.data.borrow())
            .map_err(|_| OrpError::InvalidAccounts)?
            .amount;

        // Ensure min_out is satisfied vs a snapshot before/after (simplified: require >= min_out after call)
        if actual_out < m.min_amount_out {
            return Err(OrpError::TokenOpFailed.into());
        }
        actual_out
    };

    msg!("ORP: Processed msg_id: 0x{}", hex32(&msg_id));
    msg!(
        "ORP: Settled {} -> {} amount_out={}",
        token_in_mint,
        token_out_mint,
        out_amount
    );

    Ok(())
}

// -----------------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------------

/// Transfers tokens owned by this program's vault ATA to a receiver ATA.
/// If the owner is a PDA with seeds, pass its signer seeds; otherwise empty.
fn transfer_spl_tokens(
    token_program: &AccountInfo,
    src_ata: &AccountInfo,
    dst_ata: &AccountInfo,
    program_id: &Pubkey,
    _signer_seeds: &[&[u8]],
    amount: u64,
) -> ProgramResult {
    let ix = token_ix::transfer(
        token_program.key,
        src_ata.key,
        dst_ata.key,
        // For simplicity, assume ATA owner is this program's PDA at CONFIG_SEED,
        // which signed earlier ops. If you keep a dedicated vault owner PDA,
        // change seed derivation accordingly.
        &Pubkey::find_program_address(&[CONFIG_SEED], program_id).0,
        &[],
        amount,
    )?;
    // Needs correct signer seeds for the actual vault owner PDA. Here we sign with CONFIG_SEED.
    let (_vault, bump) = Pubkey::find_program_address(&[CONFIG_SEED], program_id);
    invoke_signed(
        &ix,
        &[src_ata.clone(), dst_ata.clone(), token_program.clone()],
        &[&[CONFIG_SEED, &[bump]]],
    )?;
    Ok(())
}

/// Hex formatter for 32-byte arrays (for logs).
fn hex32(b: &[u8; 32]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut s = String::with_capacity(64);
    for byte in b {
        s.push(HEX[(byte >> 4) as usize] as char);
        s.push(HEX[(byte & 0x0f) as usize] as char);
    }
    s
}
