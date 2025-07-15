//! OCOS-Chain: Ledger State Snapshot Module
//!
//! Enables creation, storage, loading and verification of blockchain state snapshots.
//! Used for fast sync, backup/restore, DAO audit, and chain data export.

use crate::ledger::state::State;
use std::collections::HashMap;

/// Snapshot struct: Immutable copy of the ledger state at a given block height
#[derive(Debug, Clone)]
pub struct Snapshot {
    pub block_height: u64,                     // Snapshot alınan blokun nömrəsi
    pub state_root: Vec<u8>,                   // Merkle root və ya state root hash
    pub accounts: HashMap<Vec<u8>, AccountSnapshot>, // Bütün account və balanslar
    pub metadata: Option<HashMap<String, String>>,   // Əlavə audit və info üçün
}

#[derive(Debug, Clone)]
pub struct AccountSnapshot {
    pub balance: u128,
    pub nonce: u64,
    pub code_hash: Option<Vec<u8>>,
    pub storage_root: Option<Vec<u8>>,
}

impl Snapshot {
    /// State-dən snapshot yarat (blok nömrəsi və root hash daxilində)
    pub fn from_state(height: u64, state_root: Vec<u8>, state: &State) -> Self {
        let mut accounts = HashMap::new();
        for (addr, acc) in &state.accounts {
            accounts.insert(addr.clone(), AccountSnapshot {
                balance: acc.balance,
                nonce: acc.nonce,
                code_hash: acc.code_hash.clone(),
                storage_root: acc.storage_root.clone(),
            });
        }
        Snapshot {
            block_height: height,
            state_root,
            accounts,
            metadata: None,
        }
    }

    /// Snapshot-ı yüklə və state-ə bərpa et
    pub fn restore_state(&self) -> State {
        let mut state = State::new();
        for (addr, acc_snap) in &self.accounts {
            state.update_account(addr.clone(), crate::ledger::state::AccountState {
                balance: acc_snap.balance,
                nonce: acc_snap.nonce,
                code_hash: acc_snap.code_hash.clone(),
                storage_root: acc_snap.storage_root.clone(),
            });
        }
        state
    }
}
