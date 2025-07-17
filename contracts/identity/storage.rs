//! OCOS-Chain: Identity Storage Module
//!
//! Persistent storage for identities, profiles, DIDs, SBTs, KYC/KYB, recovery, reputation, groups, attestations, and governance.

use std::collections::HashMap;
use crate::identity::types::{
    IdentityId, Address, DID, SBTId, GroupId, ProposalId,
    IdentityProfile, DIDDocument, SBTMetadata, Attestation, ReputationEvent,
};
use crate::identity::did::DIDRegistry;
use crate::identity::soulbound::SBTRegistry;
use crate::identity::kyc::KYCRegistry;
use crate::identity::reputation::ReputationRegistry;
use crate::identity::recovery::RecoveryRegistry;
use crate::identity::group::GroupRegistry;
use crate::identity::attestation::AttestationRegistry;
use crate::identity::governance::{IdentityGovernanceProposal, IdentityGovernance};

#[derive(Default)]
pub struct IdentityStorage {
    // identity_id → profile
    pub profiles: HashMap<IdentityId, IdentityProfile>,
    // did → document
    pub dids: DIDRegistry,
    // sbt_id → metadata
    pub sbts: SBTRegistry,
    // KYC/KYB registry
    pub kyc: KYCRegistry,
    // reputation registry
    pub reputation: ReputationRegistry,
    // recovery configs
    pub recovery: RecoveryRegistry,
    // group registry
    pub groups: GroupRegistry,
    // attestation registry
    pub attestations: AttestationRegistry,
    // governance proposals
    pub governance_proposals: HashMap<ProposalId, IdentityGovernanceProposal>,
    // governance engine
    pub governance: IdentityGovernance,
}

impl IdentityStorage {
    // --- Profile Logic ---
    pub fn set_profile(&mut self, identity_id: IdentityId, profile: IdentityProfile) {
        self.profiles.insert(identity_id, profile);
    }
    pub fn get_profile(&self, identity_id: IdentityId) -> Option<&IdentityProfile> {
        self.profiles.get(&identity_id)
    }

    // --- DID Logic ---
    pub fn resolve_did(&self, did: &DID) -> Option<&DIDDocument> {
        self.dids.resolve(did)
    }

    // --- SBT Logic ---
    pub fn sbts_of(&self, owner: Address) -> Vec<SBTId> {
        self.sbts.tokens_of(owner)
    }

    // --- KYC/KYB Logic ---
    pub fn kyc_status(&self, identity_id: IdentityId) -> Option<&crate::identity::kyc::KYCStatus> {
        self.kyc.get_status(identity_id)
    }

    // --- Reputation Logic ---
    pub fn reputation_of(&self, address: Address) -> u64 {
        self.reputation.get(address)
    }

    // --- Recovery Logic ---
    pub fn recovery_config(&self, identity_id: IdentityId) -> Option<&crate::identity::recovery::RecoveryConfig> {
        self.recovery.get(identity_id)
    }

    // --- Group Logic ---
    pub fn group_members(&self, group_id: GroupId) -> Option<&std::collections::HashSet<Address>> {
        self.groups.members_of(group_id)
    }

    // --- Attestation Logic ---
    pub fn attestation_list(&self, address: Address) -> Vec<String> {
        self.attestations.list(address)
    }

    // --- Governance Logic ---
    pub fn add_governance_proposal(&mut self, proposal: IdentityGovernanceProposal) {
        self.governance_proposals.insert(proposal.proposal_id, proposal);
    }
    pub fn get_governance_proposal(&self, proposal_id: ProposalId) -> Option<&IdentityGovernanceProposal> {
        self.governance_proposals.get(&proposal_id)
    }
}
