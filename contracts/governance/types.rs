//! OCOS-Chain: Governance & DAO Common Types
//!
//! Defines all core types, enums, and identifiers used in governance contracts.

pub type ProposalId = u64;
pub type VoterId = u64;
pub type CouncilMemberId = u64;

/// Status of a governance proposal
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProposalStatus {
    Pending,
    Approved,
    Rejected,
    Executed,
    Expired,
    Failed,
}

/// The kind/type of governance proposal
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProposalKind {
    ConfigUpdate { key: String },
    Upgrade { description: String },
    Treasury { action: String },
    CouncilMemberAdd { member_id: CouncilMemberId },
    CouncilMemberRemove { member_id: CouncilMemberId },
    Custom { name: String, data: Vec<u8> },
    // Extendable for DAO-specific actions
}

/// Voting options (Yes/No/Abstain/Custom)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VoteOption {
    Yes,
    No,
    Abstain,
    Custom(String),
}

/// Information about a delegated vote
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegationInfo {
    pub from: VoterId,
    pub to: VoterId,
    pub weight: u128,
}

/// Event log types for governance actions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GovernanceAction {
    ProposalCreated { proposal_id: ProposalId },
    ProposalPassed { proposal_id: ProposalId },
    ProposalFailed { proposal_id: ProposalId },
    VoteCast { voter_id: VoterId, proposal_id: ProposalId, option: VoteOption },
    DelegationSet { from: VoterId, to: VoterId, weight: u128 },
    // ...and more as needed
}
