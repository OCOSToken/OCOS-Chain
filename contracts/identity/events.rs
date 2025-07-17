//! OCOS-Chain: Identity Protocol Events Module
//!
//! Defines all on-chain events for identity, DID, KYC, SBT, profile, recovery,
//! reputation, attestation, group and identity-governance operations.

use crate::identity::types::{IdentityId, Address, DID, SBTId, GroupId, ProposalId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentityEvent {
    ProfileCreated {
        identity_id: IdentityId,
        address: Address,
    },
    ProfileUpdated {
        identity_id: IdentityId,
        field: String,
        value: String,
    },
    DIDCreated {
        did: DID,
        controller: Address,
    },
    DIDAttested {
        did: DID,
        attestor: Address,
    },
    KYCRequested {
        identity_id: IdentityId,
    },
    KYCApproved {
        identity_id: IdentityId,
        attestor: Address,
    },
    KYCRevoked {
        identity_id: IdentityId,
        by: Address,
    },
    SBTMinted {
        sbt_id: SBTId,
        owner: Address,
    },
    SBTRevoked {
        sbt_id: SBTId,
        by: Address,
    },
    ReputationChanged {
        address: Address,
        new_score: u64,
        delta: i64,
        reason: String,
    },
    AttestationAdded {
        identity_id: IdentityId,
        attestor: Address,
        data: String,
    },
    GroupCreated {
        group_id: GroupId,
        creator: Address,
    },
    GroupMemberAdded {
        group_id: GroupId,
        member: Address,
    },
    GroupMemberRemoved {
        group_id: GroupId,
        member: Address,
    },
    RecoveryRequested {
        identity_id: IdentityId,
        by: Address,
    },
    RecoveryApproved {
        identity_id: IdentityId,
        guardian: Address,
    },
    RecoveryCompleted {
        identity_id: IdentityId,
        new_owner: Address,
    },
    GovernanceProposalCreated {
        proposal_id: ProposalId,
        creator: Address,
        kind: String,
    },
    GovernanceVoted {
        proposal_id: ProposalId,
        voter: Address,
        approve: bool,
    },
    GovernanceExecuted {
        proposal_id: ProposalId,
        status: String,
    },
    // Extendable for new identity/governance events
}
