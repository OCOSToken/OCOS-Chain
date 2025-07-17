//! OCOS-Chain: Identity Registry Module
//!
//! Unified registry for on-chain identities, DIDs, profiles, SBTs, reputation, attestations,
//! and recovery logic. Enables search, resolve, and update for universal Web3 identity.

use crate::identity::types::{IdentityId, Address, DID, SBTId};
use crate::identity::profile::IdentityProfile;
use crate::identity::did::DIDRegistry;
use crate::identity::soulbound::SBTRegistry;
use crate::identity::reputation::ReputationRegistry;
use crate::identity::attestation::AttestationRegistry;
use crate::identity::recovery::RecoveryRegistry;
use crate::identity::error::IdentityError;
use std::collections::HashMap;

#[derive(Default)]
pub struct IdentityRegistry {
    // identity_id → Address
    pub identities: HashMap<IdentityId, Address>,
    // address → IdentityProfile
    pub profiles: HashMap<Address, IdentityProfile>,
    // DID registry
    pub dids: DIDRegistry,
    // SBT registry
    pub sbts: SBTRegistry,
    // Reputation
    pub reputation: ReputationRegistry,
    // Attestation registry
    pub attestations: AttestationRegistry,
    // Recovery registry
    pub recovery: RecoveryRegistry,
}

impl IdentityRegistry {
    /// Register a new identity/profile
    pub fn register_identity(
        &mut self,
        identity_id: IdentityId,
        address: Address,
        profile: IdentityProfile,
    ) -> Result<(), IdentityError> {
        if self.identities.contains_key(&identity_id) {
            return Err(IdentityError::IdentityExists);
        }
        self.identities.insert(identity_id, address);
        self.profiles.insert(address, profile);
        Ok(())
    }

    /// Resolve profile by address
    pub fn resolve_profile(&self, address: Address) -> Option<&IdentityProfile> {
        self.profiles.get(&address)
    }

    /// Resolve DID by id
    pub fn resolve_did(&self, did: &DID) -> Option<&crate::identity::types::DIDDocument> {
        self.dids.resolve(did)
    }

    /// Query SBTs by owner
    pub fn soulbounds_of(&self, owner: Address) -> Vec<SBTId> {
        self.sbts.tokens_of(owner)
    }

    /// Query reputation by address
    pub fn reputation_of(&self, address: Address) -> u64 {
        self.reputation.get(address)
    }

    /// Query attestations for an address
    pub fn attestations_of(&self, address: Address) -> Vec<String> {
        self.attestations.list(address)
    }

    /// Query recovery config for an address
    pub fn recovery_config(&self, address: Address) -> Option<&crate::identity::recovery::RecoveryConfig> {
        self.recovery.get(address)
    }
}
