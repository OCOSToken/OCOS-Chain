//! OCOS-Chain: Audit & Tracing Types
//!
//! Defines all base types, enums, and identifiers for the audit, trace, compliance, and metrics systems.

pub type TraceId = u64;
pub type AuditRecordId = u64;
pub type Address = [u8; 20];
pub type Timestamp = u64;

/// Action performed in the system (for trace and logs)
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum ActionType {
    Transfer,
    Mint,
    Burn,
    Swap,
    GovernanceProposal,
    GovernanceVote,
    StorageWrite,
    StorageRead,
    ContractCall,
    DaoMembership,
    TreasuryWithdraw,
    ComplianceCheck,
    Custom(u8), // For protocol-specific extensions
}

/// Level of audit trace/log (for filtering and analytics)
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum TraceLevel {
    Info,
    Warn,
    Error,
    Security,
    Compliance,
    Forensic,
    Custom(u8),
}

/// Log severity
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Critical,
    Custom(u8),
}

/// Core audit log entry
#[derive(Debug, Clone)]
pub struct AuditLogEntry {
    pub timestamp: Timestamp,
    pub trace_id: TraceId,
    pub actor: Address,
    pub action_type: ActionType,
    pub level: LogLevel,
    pub details: String,
}

/// Audit event kind for event normalization
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditEventKind {
    BlockCommitted,
    TransactionProcessed,
    ContractDeployed,
    TokenSwapped,
    GovernanceProposal,
    GovernanceVote,
    DaoMembership,
    ComplianceCheck,
    SecurityAlert,
    Custom(String),
}

/// Structured event for audit API
#[derive(Debug, Clone)]
pub struct AuditEvent {
    pub timestamp: Timestamp,
    pub trace_id: Option<TraceId>,
    pub actor: Option<Address>,
    pub kind: AuditEventKind,
    pub details: String,
}

/// Compliance policies supported in audit
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompliancePolicy {
    KycRequired,
    DaoMemberOnly,
    MinVotingPower(u64),
    CustomRule(String),
}

/// Compliance check result
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComplianceResult {
    Passed,
    Rejected(String),
}
