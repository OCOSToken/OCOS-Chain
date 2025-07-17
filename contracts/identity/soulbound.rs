//! OCOS-Chain: Soulbound Token (SBT) & Non-transferable Credential Module
//!
//! Implements minting, revocation, metadata, and verification of soulbound tokens (SBTs)
//! for identity, proof, DAO membership, or KYC. Fully on-chain and non-transferable.

use crate::identity::types::{SBTId, Address, SBTMetadata};
use crate::identity::error::IdentityError;
use std::collections::HashMap;

/// Main registry for SBTs (Soulbound Tokens)
#[derive(Default)]
pub struct SBTRegistry {
    /// sbt_id → owner address
    pub owners: HashMap<SBTId, Address>,
    /// sbt_id → metadata
    pub metadata: HashMap<SBTId, SBTMetadata>,
    /// owner address → set of sbt_ids
    pub user_tokens: HashMap<Address, Vec<SBTId>>,
    /// sbt_id → revoked status
    pub revoked: HashMap<SBTId, bool>,
}

impl SBTRegistry {
    /// Mint a new SBT to a user (non-transferable)
    pub fn mint(
        &mut self,
        sbt_id: SBTId,
        owner: Address,
        metadata: SBTMetadata,
    ) -> Result<(), IdentityError> {
        if self.owners.contains_key(&sbt_id) {
            return Err(IdentityError::AlreadyExists);
        }
        self.owners.insert(sbt_id, owner);
        self.metadata.insert(sbt_id, metadata);
        self.user_tokens.entry(owner).or_default().push(sbt_id);
        Ok(())
    }

    /// Revoke a SBT (e.g., DAO, KYC, credential loss)
    pub fn revoke(
        &mut self,
        sbt_id: SBTId,
        by: Address,
    ) -> Result<(), IdentityError> {
        let owner = self.owners.get(&sbt_id).ok_or(IdentityError::NotFound)?;
        // Optionally, only the issuer or DAO can revoke
        if &by != owner {
            return Err(IdentityError::Unauthorized);
        }
        self.revoked.insert(sbt_id, true);
        Ok(())
    }

    /// Query metadata of an SBT
    pub fn get_metadata(&self, sbt_id: SBTId) -> Option<&SBTMetadata> {
        self.metadata.get(&sbt_id)
    }

    /// Check if a SBT is revoked
    pub fn is_revoked(&self, sbt_id: SBTId) -> bool {
        *self.revoked.get(&sbt_id).unwrap_or(&false)
    }

    /// List all SBTs owned by a user
    pub fn tokens_of(&self, owner: Address) -> Vec<SBTId> {
        self.user_tokens.get(&owner).cloned().unwrap_or_default()
    }
}
