//! OCOS-Chain: VM Integration & Unit Tests
//!
//! Tests core functionality of the OCOS virtual machine, including gas accounting,
//! context handling, syscall usage, and execution traps.

use crate::vm::{
    context::VmContext,
    result::{VmError, ExitReason},
    syscall::SyscallHandler,
    engine::{VmEngine, DefaultVmEngine},
};

fn dummy_context() -> VmContext {
    VmContext::new(
        [0u8; 20], // caller
        [1u8; 20], // callee
        10_000,    // gas limit
        42,        // block number
        1_650_000_000, // timestamp
        vec![0x01, 0x02], // input
        1, // chain_id
    )
}

#[test]
fn test_vm_successful_execution() {
    let ctx = dummy_context();
    let mut engine = DefaultVmEngine;

    let bytecode = vec![0x00, 0x00]; // dummy wasm or logic
    let result = engine.execute(ctx, &bytecode);

    assert!(result.is_ok());
}

#[test]
fn test_vm_gas_limit_enforced() {
    let mut ctx = dummy_context();
    ctx.gas_limit = 1; // very low gas

    let mut engine = DefaultVmEngine;
    let bytecode = vec![0xFF]; // simulate heavy logic

    let result = engine.execute(ctx, &bytecode);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_exit_reason(), ExitReason::OutOfGas);
}

#[test]
fn test_syscall_storage_write_and_read() {
    let mut syscall = SyscallHandler::default();

    let key = b"key1";
    let value = b"value123";

    syscall.write_storage(key, value);
    let fetched = syscall.read_storage(key).unwrap();

    assert_eq!(fetched, value);
}

#[test]
fn test_vm_trap_handling() {
    let mut ctx = dummy_context();
    ctx.input = vec![0xFF, 0xFF]; // simulate trap input

    let mut engine = DefaultVmEngine;
    let result = engine.execute(ctx, &ctx.input);

    if let Err(err) = result {
        assert_eq!(err.to_exit_reason(), ExitReason::Trap);
    } else {
        panic!("Expected trap, but got success");
    }
}
