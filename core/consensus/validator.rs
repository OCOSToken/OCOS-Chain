//! OCOS-Chain: Validator Management Module
//!
//! Provides structures and functions for validator identity, stake management,
//! slashing, jail status, and quantum-safe key storage. Designed for secure,
//! auditable and flexible consensus operation.

use std::collections::HashMap;

/// Validator status (Active, Jailed, Retired)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidatorStatus {
    Active,
    Jailed,
    Retired,
}

/// Validator main struct
#[derive(Debug, Clone)]
pub struct Validator {
    pub address: String,
    pub stake: u64,
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>, // For demonstration; in real, secure enclave/HSM!
    pub status: ValidatorStatus,
    pub jailed_since: Option<u64>,
    pub metadata: Option<HashMap<String, String>>, // e.g., node info, geo, governance info
}

impl Validator {
    /// Create a new validator
    pub fn new(address: String, stake: u64, public_key: Vec<u8>, private_key: Vec<u8>) -> Self {
        Validator {
            address,
            stake,
            public_key,
            private_key,
            status: ValidatorStatus::Active,
            jailed_since: None,
            metadata: None,
        }
    }

    /// Slash the validator (reduce stake & set jail if needed)
    pub fn slash(&mut self, amount: u64, block_height: u64) {
        if amount >= self.stake {
            self.stake = 0;
            self.status = ValidatorStatus::Jailed;
            self.jailed_since = Some(block_height);
        } else {
            self.stake -= amount;
        }
    }

    /// Release (unjail) validator after penalty or governance decision
    pub fn unjail(&mut self) {
        if self.status == ValidatorStatus::Jailed {
            self.status = ValidatorStatus::Active;
            self.jailed_since = None;
        }
    }

    /// Retire validator from active set (voluntary or forced)
    pub fn retire(&mut self) {
        self.status = ValidatorStatus::Retired;
    }

    /// Add or update validator metadata
    pub fn set_metadata(&mut self, key: String, value: String) {
        if self.metadata.is_none() {
            self.metadata = Some(HashMap::new());
        }
        self.metadata.as_mut().unwrap().insert(key, value);
    }
}

/// Efficient set of validators, indexed by address
#[derive(Debug, Clone)]
pub struct ValidatorSet {
    pub validators: HashMap<String, Validator>,
}

impl ValidatorSet {
    pub fn new(validators: Vec<Validator>) -> Self {
        let map = validators.into_iter().map(|v| (v.address.clone(), v)).collect();
        ValidatorSet { validators: map }
    }

    /// Get validator by address (immutable)
    pub fn get_by_address(&self, address: &str) -> Option<&Validator> {
        self.validators.get(address)
    }

    /// Get validator by address (mutable)
    pub fn get_by_address_mut(&mut self, address: &str) -> Option<&mut Validator> {
        self.validators.get_mut(address)
    }

    /// Select validator by stake (PoS): weighted random or deterministic (simple version: round robin)
    pub fn select_by_stake(&self, height: u64) -> Option<&Validator> {
        // For demo: simple round robin
        let keys: Vec<&String> = self.validators.keys().collect();
        if keys.is_empty() {
            return None;
        }
        let idx = (height as usize) % keys.len();
        self.validators.get(keys[idx])
    }

    /// Select validator by round robin (PoA)
    pub fn select_by_round_robin(&self, height: u64) -> Option<&Validator> {
        self.select_by_stake(height)
    }

    /// Load all validators (mock/demo)
    pub fn load_all() -> Vec<Validator> {
        // Placeholder: In real system, load from persistent storage or state
        vec![]
    }
}
