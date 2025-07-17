//! OCOS-Chain: Identity Recovery & Social Backup Module
//!
//! Provides secure recovery for identities and wallets via guardians, DAO voting,
//! social recovery, and multi-factor proofs. Fully on-chain and auditable.

use crate::identity::types::{Address, IdentityId};
use crate::identity::error::IdentityError;
use std::collections::{HashMap, HashSet};

/// Configuration for recovery of a specific identity
#[derive(Debug, Clone)]
pub struct RecoveryConfig {
    pub owner: Address,
    pub guardians: HashSet<Address>, // Trusted friends, DAO, or multisig
    pub threshold: u8,               // Minimum guardians for approval
    pub enabled: bool,
}

#[derive(Default)]
pub struct RecoveryRegistry {
    // identity_id → RecoveryConfig
    pub configs: HashMap<IdentityId, RecoveryConfig>,
    // identity_id → in-progress recovery approvals
    pub approvals: HashMap<IdentityId, HashSet<Address>>,
}

impl RecoveryRegistry {
    /// Set up recovery config for an identity
    pub fn set_recovery(
        &mut self,
        identity_id: IdentityId,
        owner: Address,
        guardians: HashSet<Address>,
        threshold: u8,
    ) -> Result<(), IdentityError> {
        if guardians.len() < threshold as usize {
            return Err(IdentityError::InvalidParameter);
        }
        self.configs.insert(identity_id, RecoveryConfig {
            owner,
            guardians,
            threshold,
            enabled: true,
        });
        Ok(())
    }

    /// Start recovery request for an identity
    pub fn request_recovery(
        &mut self,
        identity_id: IdentityId,
        by: Address,
    ) -> Result<(), IdentityError> {
        let config = self.configs.get(&identity_id).ok_or(IdentityError::NotFound)?;
        if !config.enabled {
            return Err(IdentityError::InvalidOperation);
        }
        if !config.guardians.contains(&by) {
            return Err(IdentityError::Unauthorized);
        }
        self.approvals.entry(identity_id).or_default().insert(by);
        Ok(())
    }

    /// Approve recovery by a guardian
    pub fn approve_recovery(
        &mut self,
        identity_id: IdentityId,
        guardian: Address,
    ) -> Result<(), IdentityError> {
        let config = self.configs.get(&identity_id).ok_or(IdentityError::NotFound)?;
        if !config.guardians.contains(&guardian) {
            return Err(IdentityError::Unauthorized);
        }
        self.approvals.entry(identity_id).or_default().insert(guardian);
        Ok(())
    }

    /// Complete recovery if threshold is reached (returns new owner address)
    pub fn complete_recovery(
        &mut self,
        identity_id: IdentityId,
        new_owner: Address,
    ) -> Result<(), IdentityError> {
        let config = self.configs.get(&identity_id).ok_or(IdentityError::NotFound)?;
        let approvals = self.approvals.get(&identity_id).ok_or(IdentityError::Unauthorized)?;
        if approvals.len() < config.threshold as usize {
            return Err(IdentityError::InvalidOperation);
        }
        // Update owner, clear approvals
        self.configs.get_mut(&identity_id).unwrap().owner = new_owner;
        self.approvals.remove(&identity_id);
        Ok(())
    }

    /// Query recovery config
    pub fn get(&self, identity_id: IdentityId) -> Option<&RecoveryConfig> {
        self.configs.get(&identity_id)
    }
}
