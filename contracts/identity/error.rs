//! OCOS-Chain: Identity Protocol Error Types
//!
//! Defines all error codes and messages for identity, profile, DID, SBT, KYC/KYB,
//! recovery, group, governance and attestation operations.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentityError {
    Unauthorized,
    NotFound,
    AlreadyExists,
    InvalidOperation,
    InvalidParameter,
    DIDAlreadyExists,
    DIDNotFound,
    SBTAlreadyExists,
    SBTNotFound,
    KYCRejected,
    KYCExpired,
    GroupNotFound,
    NotGroupAdmin,
    RecoveryNotEnabled,
    ApprovalThresholdNotMet,
    ProposalExists,
    ProposalNotFound,
    ProposalClosed,
    AlreadyVoted,
    StorageError(String),
    Other(String),
}

impl fmt::Display for IdentityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdentityError::Unauthorized => write!(f, "Unauthorized operation"),
            IdentityError::NotFound => write!(f, "Identity or resource not found"),
            IdentityError::AlreadyExists => write!(f, "Resource already exists"),
            IdentityError::InvalidOperation => write!(f, "Invalid operation"),
            IdentityError::InvalidParameter => write!(f, "Invalid parameter"),
            IdentityError::DIDAlreadyExists => write!(f, "DID already exists"),
            IdentityError::DIDNotFound => write!(f, "DID not found"),
            IdentityError::SBTAlreadyExists => write!(f, "SBT already exists"),
            IdentityError::SBTNotFound => write!(f, "SBT not found"),
            IdentityError::KYCRejected => write!(f, "KYC/KYB request rejected"),
            IdentityError::KYCExpired => write!(f, "KYC/KYB status expired"),
            IdentityError::GroupNotFound => write!(f, "Group not found"),
            IdentityError::NotGroupAdmin => write!(f, "Not a group admin"),
            IdentityError::RecoveryNotEnabled => write!(f, "Recovery is not enabled for this identity"),
            IdentityError::ApprovalThresholdNotMet => write!(f, "Guardian approval threshold not met"),
            IdentityError::ProposalExists => write!(f, "Governance proposal already exists"),
            IdentityError::ProposalNotFound => write!(f, "Governance proposal not found"),
            IdentityError::ProposalClosed => write!(f, "Governance proposal is closed"),
            IdentityError::AlreadyVoted => write!(f, "Already voted on this proposal"),
            IdentityError::StorageError(msg) => write!(f, "Storage error: {}", msg),
            IdentityError::Other(msg) => write!(f, "Other identity error: {}", msg),
        }
    }
}

impl std::error::Error for IdentityError {}
