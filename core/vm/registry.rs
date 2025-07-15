//! OCOS-Chain: Virtual Machine Registry
//!
//! Allows multiple VM backends (WASM, DAO-VM, EVM, etc.) to be registered and dispatched
//! dynamically based on runtime input or contract type.

use std::collections::HashMap;
use crate::vm::{VmEngine, VmContext, VmResult};

/// A string identifier for VM types (e.g., "wasm", "evm", "dao")
pub type VmType = String;

/// Registry to hold and manage VM engine instances
#[derive(Default)]
pub struct VmRegistry {
    engines: HashMap<VmType, Box<dyn VmEngine>>,
}

impl VmRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        VmRegistry {
            engines: HashMap::new(),
        }
    }

    /// Register a VM engine by name (e.g. "wasm", "evm")
    pub fn register(&mut self, name: &str, engine: Box<dyn VmEngine>) {
        self.engines.insert(name.to_lowercase(), engine);
    }

    /// Check if a VM type is registered
    pub fn exists(&self, name: &str) -> bool {
        self.engines.contains_key(&name.to_lowercase())
    }

    /// Execute using the appropriate engine
    pub fn execute(&mut self, name: &str, ctx: VmContext, bytecode: &[u8]) -> VmResult {
        let key = name.to_lowercase();
        if let Some(engine) = self.engines.get_mut(&key) {
            engine.execute(ctx, bytecode)
        } else {
            Err(crate::vm::result::VmError::InternalError(format!(
                "VM engine '{}' not found",
                name
            )))
        }
    }
}
