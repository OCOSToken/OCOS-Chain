//! OCOS-Chain: Virtual Machine Execution Context
//!
//! The `VmContext` structure defines the full runtime environment passed to the VM engine.
//! It encapsulates block metadata, caller identity, gas budget, contract input, and execution parameters.
//!
//! This context is deterministic and audit-friendly, designed to support on-chain governance,
//! validator execution, contract runtime, and DAO logic.

/// Address type used in OCOS-Chain
pub type Address = [u8; 20];

/// Execution context for a VM invocation
#[derive(Debug, Clone)]
pub struct VmContext {
    /// Sender of the transaction or function call
    pub caller: Address,

    /// Target contract or logic handler
    pub callee: Address,

    /// Gas limit assigned for this execution
    pub gas_limit: u64,

    /// Block number at time of execution
    pub block_number: u64,

    /// Block timestamp (UNIX time)
    pub timestamp: u64,

    /// Execution input (usually encoded function call)
    pub input: Vec<u8>,

    /// Chain ID (multi-chain support)
    pub chain_id: u64,

    /// Optional DAO-specific metadata (e.g. proposal ID, level)
    pub dao_context: Option<DaoContext>,
}

/// Extra metadata used in DAO-level executions
#[derive(Debug, Clone)]
pub struct DaoContext {
    /// Proposal ID (if executing a governance proposal)
    pub proposal_id: Option<u64>,

    /// DAO level or tier
    pub level: u8,

    /// Voting weight or stake snapshot
    pub voting_power: Option<u128>,
}

impl VmContext {
    /// Create a new basic context
    pub fn new(
        caller: Address,
        callee: Address,
        gas_limit: u64,
        block_number: u64,
        timestamp: u64,
        input: Vec<u8>,
        chain_id: u64,
    ) -> Self {
        VmContext {
            caller,
            callee,
            gas_limit,
            block_number,
            timestamp,
            input,
            chain_id,
            dao_context: None,
        }
    }

    /// Attach DAO-specific metadata
    pub fn with_dao_context(mut self, dao: DaoContext) -> Self {
        self.dao_context = Some(dao);
        self
    }
}
