//! OCOS-Chain: State Management Module
//!
//! Manages blockchain state: account balances, storage, state proofs, and delta updates.
//! Designed for secure, auditable, and deterministic ledger state operations.

use std::collections::HashMap;

/// State struct: Main mapping of account addresses to their state
#[derive(Debug, Clone)]
pub struct State {
    pub accounts: HashMap<Vec<u8>, AccountState>, // address → state
    pub storage: HashMap<Vec<u8>, Vec<u8>>,       // generic key-value storage (optional)
}

impl State {
    /// Create a new, empty state
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            storage: HashMap::new(),
        }
    }

    /// Get account state for an address
    pub fn get_account(&self, address: &[u8]) -> Option<&AccountState> {
        self.accounts.get(address)
    }

    /// Mutably get account state for updating
    pub fn get_account_mut(&mut self, address: &[u8]) -> Option<&mut AccountState> {
        self.accounts.get_mut(address)
    }

    /// Update or insert an account state
    pub fn update_account(&mut self, address: Vec<u8>, account: AccountState) {
        self.accounts.insert(address, account);
    }

    /// Delete an account (used in dust sweeping, account removal)
    pub fn remove_account(&mut self, address: &[u8]) {
        self.accounts.remove(address);
    }

    /// Update key-value storage
    pub fn set_storage(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.storage.insert(key, value);
    }

    /// Get from key-value storage
    pub fn get_storage(&self, key: &[u8]) -> Option<&Vec<u8>> {
        self.storage.get(key)
    }
}

/// Individual account state: balance, nonce, and custom fields
#[derive(Debug, Clone)]
pub struct AccountState {
    pub balance: u128,
    pub nonce: u64,
    pub code_hash: Option<Vec<u8>>, // Smart contract code hash (for contracts)
    pub storage_root: Option<Vec<u8>>, // Merkle root of contract storage (for contracts)
    // Additional extensible fields (permissions, flags, etc)
}

impl AccountState {
    /// Create a new standard account
    pub fn new(balance: u128) -> Self {
        Self {
            balance,
            nonce: 0,
            code_hash: None,
            storage_root: None,
        }
    }
}

/// State update struct: changes for a block/tx (delta)
#[derive(Debug, Clone)]
pub struct StateUpdate {
    pub account_deltas: Vec<(Vec<u8>, AccountDelta)>, // address, changes
    pub storage_deltas: Vec<(Vec<u8>, Vec<u8>)>,      // key, value
}

#[derive(Debug, Clone)]
pub enum AccountDelta {
    BalanceChange(i128),
    NonceInc(u64),
    CodeChange(Vec<u8>),
    StorageRootChange(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct StorageProof {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
    pub merkle_proof: Vec<Vec<u8>>, // Merkle path for proof
}
