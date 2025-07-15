//! OCOS-Chain DAO: Error Types Module
//!
//! Defines all standard error types for DAO contract logic,
//! enabling deterministic, auditable, and consistent error handling.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DaoError {
    // Proposal & Voting
    ProposalExists,
    ProposalNotFound,
    ProposalNotExecutable,
    VoteAlreadyCast,
    VoteWindowClosed,
    QuorumNotMet,
    NotEligibleToVote,
    InsufficientVotingPower,

    // Membership
    MemberExists,
    MemberNotFound,
    Unauthorized,
    InactiveMember,
    StakeTooLow,

    // Execution
    AlreadyExecuted,
    ExecutionFailed,
    NotApproved,

    // Treasury
    InsufficientFunds,
    TreasuryLocked,

    // Config
    InvalidConfig,
    EmergencyModeActive,

    // Storage
    StorageError(String),

    // Generic
    Unknown(String),
}

impl std::fmt::Display for DaoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use DaoError::*;
        let msg = match self {
            ProposalExists         => "Proposal already exists.",
            ProposalNotFound       => "Proposal not found.",
            ProposalNotExecutable  => "Proposal not in executable state.",
            VoteAlreadyCast        => "Vote already cast.",
            VoteWindowClosed       => "Voting window closed.",
            QuorumNotMet           => "Quorum not met.",
            NotEligibleToVote      => "Not eligible to vote.",
            InsufficientVotingPower=> "Insufficient voting power.",
            MemberExists           => "Member already exists.",
            MemberNotFound         => "Member not found.",
            Unauthorized           => "Not authorized.",
            InactiveMember         => "Member is not active.",
            StakeTooLow            => "Stake below minimum.",
            AlreadyExecuted        => "Proposal already executed.",
            ExecutionFailed        => "Proposal execution failed.",
            NotApproved            => "Proposal not approved.",
            InsufficientFunds      => "Insufficient treasury funds.",
            TreasuryLocked         => "Treasury is locked.",
            InvalidConfig          => "Invalid configuration.",
            EmergencyModeActive    => "DAO is in emergency mode.",
            StorageError(e)        => &e,
            Unknown(e)             => &e,
        };
        write!(f, "DAO Error: {}", msg)
    }
}

impl std::error::Error for DaoError {}
