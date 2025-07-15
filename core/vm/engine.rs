//! OCOS-Chain: Virtual Machine Execution Engine
//!
//! This is the core engine responsible for executing smart contracts and DAO-level logic
//! in a sandboxed and gas-limited environment.
//!
//! It delegates execution to specific interpreters (e.g. WASM), and manages pre/post conditions.

use crate::vm::{
    context::VmContext,
    wasm::WasmExecutor,
    gas::{GasMeter, GasLimit, GasError},
    result::{VmResult, VmError, ExitReason},
    syscall::SyscallHandler,
};

/// The trait that all virtual machine backends must implement.
pub trait VmEngine {
    /// Executes a smart contract or DAO program.
    fn execute(&mut self, ctx: VmContext, bytecode: &[u8]) -> VmResult;
}

/// Default implementation for WebAssembly contracts
pub struct DefaultVmEngine;

impl VmEngine for DefaultVmEngine {
    fn execute(&mut self, ctx: VmContext, bytecode: &[u8]) -> VmResult {
        let mut gas_meter = GasMeter::new(ctx.gas_limit);
        let mut syscall = SyscallHandler::default();

        // WASM execution
        let exec = WasmExecutor::new(&mut gas_meter, &mut syscall);
        let result = exec.run(ctx, bytecode)?;

        Ok(result)
    }
}
