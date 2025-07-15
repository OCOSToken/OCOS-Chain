//! OCOS-Chain Consensus Layer
//!
//! This module implements the core consensus logic, combining
//! quantum-resistant signature verification, hybrid PoS/PoA authority,
//! validator management, governance hooks, and slashing mechanisms.
//!
//! Each submodule is designed for high modularity, security, and auditability.
//!
//! All consensus operations are exposed via unified public interfaces
//! for integration with ledger, networking, and smart contract layers.

pub mod consensus_engine;
pub mod validator;
pub mod quantum_sig;
pub mod block;
pub mod slashing;
pub mod governance;

// Public re-exports for external use
pub use consensus_engine::{ConsensusEngine, ConsensusMode};
pub use validator::Validator;
pub use quantum_sig::QuantumSignature;
pub use block::{BlockHeader, Block};
pub use slashing::SlashingManager;
pub use governance::GovernanceHook;

/// Global consensus error type
#[derive(Debug, thiserror::Error)]
pub enum ConsensusError {
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Validator slashed")]
    ValidatorSlashed,
    #[error("Unauthorized consensus operation")]
    Unauthorized,
    #[error("Governance update rejected")]
    GovernanceRejected,
    #[error("Unknown consensus error")]
    Unknown,
}

/// Consensus event types for audit/logging
#[derive(Debug, Clone)]
pub enum ConsensusEvent {
    BlockProposed { proposer: String, height: u64 },
    BlockFinalized { hash: String, height: u64 },
    ValidatorSlashed { validator: String, amount: u64 },
    GovernanceUpdate { key: String, value: String },
}

/// Consensus initialization entrypoint
pub fn initialize_consensus(mode: ConsensusMode) -> ConsensusEngine {
    let validators = Validator::load_all();
    ConsensusEngine::new(validators, mode)
}
