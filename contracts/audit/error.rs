//! OCOS-Chain: Audit & Compliance Error Types
//!
//! Defines all error codes and messages for audit, tracing, compliance,
//! storage, proof, and analytics systems.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditError {
    TraceNotFound,
    LogNotFound,
    EventNotFound,
    ComplianceViolation(String),
    InvalidProof,
    HashMismatch,
    Unauthorized,
    AccessDenied,
    StorageError(String),
    DecodeError(String),
    NotImplemented,
    Timeout,
    RateLimited,
    InvalidParameter,
    ForensicMismatch(String),
    MetricsUnavailable,
    Custom(String),
}

impl fmt::Display for AuditError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuditError::TraceNotFound => write!(f, "Audit trace not found"),
            AuditError::LogNotFound => write!(f, "Audit log not found"),
            AuditError::EventNotFound => write!(f, "Audit event not found"),
            AuditError::ComplianceViolation(reason) => write!(f, "Compliance violation: {}", reason),
            AuditError::InvalidProof => write!(f, "Invalid or unverifiable proof"),
            AuditError::HashMismatch => write!(f, "Hash commitment mismatch"),
            AuditError::Unauthorized => write!(f, "Unauthorized operation"),
            AuditError::AccessDenied => write!(f, "Access denied to audit data"),
            AuditError::StorageError(msg) => write!(f, "Storage error: {}", msg),
            AuditError::DecodeError(msg) => write!(f, "Decode error: {}", msg),
            AuditError::NotImplemented => write!(f, "Feature not implemented"),
            AuditError::Timeout => write!(f, "Audit operation timed out"),
            AuditError::RateLimited => write!(f, "Rate limited: too many audit queries"),
            AuditError::InvalidParameter => write!(f, "Invalid audit parameter"),
            AuditError::ForensicMismatch(msg) => write!(f, "Forensic mismatch: {}", msg),
            AuditError::MetricsUnavailable => write!(f, "Metrics unavailable"),
            AuditError::Custom(msg) => write!(f, "Audit error: {}", msg),
        }
    }
}

impl std::error::Error for AuditError {}
