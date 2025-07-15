//! OCOS-Chain: Governance Error Types
//!
//! Defines all error codes and categories for governance and DAO operations,
//! to support strict audit, debugging, and user feedback.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GovernanceError {
    Unauthorized,
    AlreadyVoted,
    QuorumNotMet,
    ProposalExists,
    ProposalNotFound,
    ProposalNotApproved,
    ProposalExpired,
    ProposalActive,
    ProposalClosed,
    ExecutionNotSupported,
    InvalidParameter,
    InvalidPayload,
    EngineNotFound,
    DelegationCycle,
    InvalidDelegation,
    CouncilFull,
    NotCouncilMember,
    VoteNotAllowed,
    StorageError(String),
    Other(String),
}

impl fmt::Display for GovernanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GovernanceError::Unauthorized => write!(f, "Unauthorized action"),
            GovernanceError::AlreadyVoted => write!(f, "Voter has already voted on this proposal"),
            GovernanceError::QuorumNotMet => write!(f, "Quorum not met for this proposal"),
            GovernanceError::ProposalExists => write!(f, "Proposal with this ID already exists"),
            GovernanceError::ProposalNotFound => write!(f, "Proposal not found"),
            GovernanceError::ProposalNotApproved => write!(f, "Proposal has not been approved"),
            GovernanceError::ProposalExpired => write!(f, "Proposal voting period has expired"),
            GovernanceError::ProposalActive => write!(f, "Proposal is still active"),
            GovernanceError::ProposalClosed => write!(f, "Proposal voting is closed"),
            GovernanceError::ExecutionNotSupported => write!(f, "Execution for this proposal type is not supported"),
            GovernanceError::InvalidParameter => write!(f, "Invalid governance parameter"),
            GovernanceError::InvalidPayload => write!(f, "Invalid payload data"),
            GovernanceError::EngineNotFound => write!(f, "Governance engine not found"),
            GovernanceError::DelegationCycle => write!(f, "Delegation would create a voting cycle"),
            GovernanceError::InvalidDelegation => write!(f, "Delegation is invalid or not permitted"),
            GovernanceError::CouncilFull => write!(f, "Council is at maximum size"),
            GovernanceError::NotCouncilMember => write!(f, "Not a current council member"),
            GovernanceError::VoteNotAllowed => write!(f, "Voting not allowed in this context"),
            GovernanceError::StorageError(msg) => write!(f, "Storage error: {}", msg),
            GovernanceError::Other(msg) => write!(f, "Other governance error: {}", msg),
        }
    }
}

impl std::error::Error for GovernanceError {}
