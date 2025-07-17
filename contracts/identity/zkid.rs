//! OCOS-Chain: Zero-Knowledge Identity (zkID) Module
//!
//! Implements zero-knowledge proof-based identities, verifiable ZK attestations,
//! privacy-preserving credentials, and DAO/personhood proofs.

use crate::identity::types::{IdentityId, Address};
use crate::identity::error::IdentityError;
use std::collections::HashMap;

/// ZK identity record
#[derive(Debug, Clone)]
pub struct ZKID {
    pub identity_id: IdentityId,
    pub address: Address,
    pub zk_proof: Vec<u8>,      // ZKP for personhood, age, KYC, etc.
    pub fields_hash: Vec<u8>,   // Hash of personal fields (optional)
    pub verified: bool,
    pub attested_by: Vec<Address>, // DAO or third-party validators
}

/// Registry for all zkID identities
#[derive(Default)]
pub struct ZKIDRegistry {
    // identity_id → ZKID
    pub zkids: HashMap<IdentityId, ZKID>,
}

impl ZKIDRegistry {
    /// Register a new ZK identity with proof and hash
    pub fn register(
        &mut self,
        identity_id: IdentityId,
        address: Address,
        zk_proof: Vec<u8>,
        fields_hash: Vec<u8>,
    ) -> Result<(), IdentityError> {
        if self.zkids.contains_key(&identity_id) {
            return Err(IdentityError::IdentityExists);
        }
        let zkid = ZKID {
            identity_id,
            address,
            zk_proof,
            fields_hash,
            verified: false,
            attested_by: vec![],
        };
        self.zkids.insert(identity_id, zkid);
        Ok(())
    }

    /// DAO or provider attests a zkID after verifying the ZKP off-chain
    pub fn attest(
        &mut self,
        identity_id: IdentityId,
        attestor: Address,
    ) -> Result<(), IdentityError> {
        let zkid = self.zkids.get_mut(&identity_id).ok_or(IdentityError::NotFound)?;
        if !zkid.attested_by.contains(&attestor) {
            zkid.attested_by.push(attestor);
        }
        zkid.verified = true;
        Ok(())
    }

    /// Check if zkID is verified (by DAO/provider)
    pub fn is_verified(&self, identity_id: IdentityId) -> bool {
        self.zkids.get(&identity_id).map(|z| z.verified).unwrap_or(false)
    }

    /// Query zk proof and hash (for smart contract verification)
    pub fn get_zkid(&self, identity_id: IdentityId) -> Option<&ZKID> {
        self.zkids.get(&identity_id)
    }
}
