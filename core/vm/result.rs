//! OCOS-Chain: VM Result & Error Handling
//!
//! Defines the standard result type, exit reasons, and error categories
//! used during smart contract and DAO execution within the OCOS virtual machine.

/// Execution result type
pub type VmResult = Result<Vec<u8>, VmError>;

/// Exit reason for execution outcome
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExitReason {
    /// Normal successful exit
    Success,

    /// Explicit contract call to revert state
    Revert,

    /// Execution halted due to trap or panic
    Trap,

    /// Execution terminated due to out-of-gas
    OutOfGas,

    /// VM-level timeout or watchdog triggered
    Timeout,
}

/// VM error type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VmError {
    /// Execution returned a custom error code
    Reverted(String),

    /// Invalid memory access
    MemoryViolation,

    /// Out of gas during execution
    GasDepleted,

    /// Unknown system or runtime error
    InternalError(String),

    /// Trap occurred during WASM or logic execution
    Trap(String),

    /// DAO or governance-specific error
    Governance(String),
}

impl VmError {
    /// Map error to an exit reason
    pub fn to_exit_reason(&self) -> ExitReason {
        match self {
            VmError::Reverted(_) => ExitReason::Revert,
            VmError::MemoryViolation => ExitReason::Trap,
            VmError::GasDepleted => ExitReason::OutOfGas,
            VmError::InternalError(_) => ExitReason::Trap,
            VmError::Trap(_) => ExitReason::Trap,
            VmError::Governance(_) => ExitReason::Revert,
        }
    }
}
