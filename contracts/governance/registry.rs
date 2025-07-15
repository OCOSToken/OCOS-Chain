//! OCOS-Chain: Governance Registry Module
//!
//! Supports dynamic registration and upgrade of governance engines (council, referendum, weighted, etc.)
//! and dispatches proposal execution to the correct module at runtime.

use std::collections::HashMap;
use crate::contracts::governance::types::{ProposalId, ProposalKind};
use crate::contracts::governance::error::GovernanceError;
use crate::contracts::governance::execution::ExecutionEngine;

/// Trait for any governance engine (council, weighted, referendum, etc.)
pub trait GovernanceEngine {
    fn execute(&mut self, proposal_id: ProposalId, kind: ProposalKind, payload: Option<Vec<u8>>, now: u64)
        -> Result<(), GovernanceError>;
}

/// Registry for governance engines (plugins/modules)
pub struct GovernanceRegistry {
    engines: HashMap<String, Box<dyn GovernanceEngine>>,
}

impl GovernanceRegistry {
    pub fn new() -> Self {
        GovernanceRegistry {
            engines: HashMap::new(),
        }
    }

    /// Register a new governance engine by name (e.g. "council", "referendum")
    pub fn register_engine(&mut self, name: &str, engine: Box<dyn GovernanceEngine>) {
        self.engines.insert(name.to_lowercase(), engine);
    }

    /// Upgrade (replace) a governance engine at runtime
    pub fn upgrade_engine(&mut self, name: &str, new_engine: Box<dyn GovernanceEngine>) {
        self.engines.insert(name.to_lowercase(), new_engine);
    }

    /// Dispatch proposal execution to the registered engine by type
    pub fn dispatch(
        &mut self,
        engine_name: &str,
        proposal_id: ProposalId,
        kind: ProposalKind,
        payload: Option<Vec<u8>>,
        now: u64,
    ) -> Result<(), GovernanceError> {
        let key = engine_name.to_lowercase();
        if let Some(engine) = self.engines.get_mut(&key) {
            engine.execute(proposal_id, kind, payload, now)
        } else {
            Err(GovernanceError::EngineNotFound)
        }
    }

    /// Check if a governance engine is registered
    pub fn exists(&self, name: &str) -> bool {
        self.engines.contains_key(&name.to_lowercase())
    }
}
