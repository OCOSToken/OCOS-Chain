//! OCOS-Chain: On-chain KYC/KYB & Compliance Module
//!
//! Provides on-chain user verification (KYC), entity/business verification (KYB),
//! status management, DAO/provider attestation, and privacy-preserving checks.

use crate::identity::types::{IdentityId, Address};
use crate::identity::error::IdentityError;
use std::collections::{HashMap, HashSet};

/// KYC/KYB verification status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KYCStatus {
    Pending,
    Approved,
    Rejected,
    Revoked,
    Expired,
}

/// Main registry for on-chain KYC/KYB
#[derive(Default)]
pub struct KYCRegistry {
    // identity_id → status
    pub status: HashMap<IdentityId, KYCStatus>,
    // identity_id → verifying provider/DAO
    pub attestors: HashMap<IdentityId, HashSet<Address>>,
    // identity_id → verification timestamp
    pub verified_at: HashMap<IdentityId, u64>,
    // Optionally: hash of KYC docs or ZK proof
    pub zk_hashes: HashMap<IdentityId, Vec<u8>>,
}

impl KYCRegistry {
    /// Submit a KYC/KYB verification request (pending status)
    pub fn request_verification(
        &mut self,
        identity_id: IdentityId,
    ) -> Result<(), IdentityError> {
        if self.status.contains_key(&identity_id) {
            return Err(IdentityError::AlreadyExists);
        }
        self.status.insert(identity_id, KYCStatus::Pending);
        Ok(())
    }

    /// Approve verification (by DAO or provider)
    pub fn approve(
        &mut self,
        identity_id: IdentityId,
        attestor: Address,
        now: u64,
    ) -> Result<(), IdentityError> {
        let status = self.status.get_mut(&identity_id).ok_or(IdentityError::NotFound)?;
        *status = KYCStatus::Approved;
        self.attestors.entry(identity_id).or_default().insert(attestor);
        self.verified_at.insert(identity_id, now);
        Ok(())
    }

    /// Revoke KYC/KYB approval
    pub fn revoke(
        &mut self,
        identity_id: IdentityId,
        by: Address,
    ) -> Result<(), IdentityError> {
        let status = self.status.get_mut(&identity_id).ok_or(IdentityError::NotFound)?;
        if *status != KYCStatus::Approved {
            return Err(IdentityError::InvalidOperation);
        }
        *status = KYCStatus::Revoked;
        if let Some(attestors) = self.attestors.get_mut(&identity_id) {
            attestors.remove(&by);
        }
        Ok(())
    }

    /// Add zero-knowledge hash for privacy (optional)
    pub fn add_zk_hash(
        &mut self,
        identity_id: IdentityId,
        hash: Vec<u8>,
    ) -> Result<(), IdentityError> {
        self.zk_hashes.insert(identity_id, hash);
        Ok(())
    }

    /// Query KYC/KYB status
    pub fn get_status(&self, identity_id: IdentityId) -> Option<&KYCStatus> {
        self.status.get(&identity_id)
    }

    /// Query KYC/KYB attestors
    pub fn get_attestors(&self, identity_id: IdentityId) -> Option<&HashSet<Address>> {
        self.attestors.get(&identity_id)
    }

    /// Query verification timestamp
    pub fn get_verified_at(&self, identity_id: IdentityId) -> Option<&u64> {
        self.verified_at.get(&identity_id)
    }

    /// Query ZK hash (if used)
    pub fn get_zk_hash(&self, identity_id: IdentityId) -> Option<&Vec<u8>> {
        self.zk_hashes.get(&identity_id)
    }
}
