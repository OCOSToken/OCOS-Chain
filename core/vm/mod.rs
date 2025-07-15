//! OCOS-Chain Virtual Machine (VM) Module
//!
//! This module provides the foundation for the OCOS runtime execution environment,
//! supporting smart contract processing, DAO logic execution, and custom governance operations.
//!
//! Features:
//! - WebAssembly (WASM) contract execution
//! - Gas metering and trap handling
//! - System call (syscall) interface for state and event access
//! - Context-aware, sandboxed runtime

pub mod engine;
pub mod wasm;
pub mod context;
pub mod memory;
pub mod gas;
pub mod syscall;
pub mod result;
pub mod registry;

#[cfg(test)]
pub mod tests;

// Re-exports for global access
pub use engine::VmEngine;
pub use wasm::WasmExecutor;
pub use context::VmContext;
pub use memory::VmMemory;
pub use gas::{GasMeter, GasLimit, GasError};
pub use syscall::SyscallHandler;
pub use result::{VmResult, VmError, ExitReason};
pub use registry::VmRegistry;
