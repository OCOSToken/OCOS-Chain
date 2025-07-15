# OCOS-Chain: Virtual Machine (VM) Subsystem

**Modular | Deterministic | Gas-Metered | DAO-Aware Execution Engine**

---

## Overview

The `/vm` module provides the execution environment for all OCOS-Chain smart contracts, DAO logic, and governance proposals. Designed as a modular, gas-limited, and secure runtime, it supports WebAssembly (WASM) execution and is extensible to other virtual machines (e.g., MoveVM, EVM) via a dynamic registry system.

---

## Features

- **WASM contract execution** in a sandboxed and gas-controlled environment
- **Gas metering** and trap detection for all operations
- **System calls (syscalls)** for contract-state interaction, logging, and event dispatch
- **Execution context** encapsulating block, caller, DAO metadata, and more
- **Result handling** with fine-grained error types and exit reasons
- **Multi-VM support** via registry for future engines

---

## Directory Structure

```
vm/
│
├── mod.rs         # Main module that re-exports all VM components
├── engine.rs      # Core VM executor and runtime interface
├── wasm.rs        # WebAssembly contract support
├── context.rs     # Execution context: caller, input, block, dao
├── memory.rs      # Safe linear memory for sandboxed execution
├── gas.rs         # Gas tracking and out-of-gas enforcement
├── syscall.rs     # Host functions (log, call, get_balance, storage, etc.)
├── result.rs      # VMResult, ExitReason, and VmError definitions
├── registry.rs    # Register and dispatch VM engines by type
├── tests.rs       # Unit and integration tests
```

---

## Use Cases

- Smart contract execution in governance and finance modules
- DAO-level on-chain proposals and secure vote application
- NFT logic execution, asset minting, and bridge verification
- Secure programmable logic within staking and treasury rules

---

## Security Considerations

- **Gas metering** is enforced to prevent DoS and infinite loops
- **Syscalls** are restricted to whitelisted host operations
- **Traps** result in safe reverts without corrupting chain state
- **Memory** is isolated and cleared on context switch

---

## Future Extensions

- Support for EVM or MoveVM with unified VMEngine interface
- zkVM integration for zero-knowledge proof-based contracts
- DAO upgrade paths to register new execution engines on-chain

---

## Authors & Governance

- OCOS Foundation — [https://ocos.io](https://ocos.io)
- Protocol Lead: Ocoshy Nakomoto — [https://github.com/Ocoshy](https://github.com/Ocoshy)

---

## License

Part of the OCOS-Chain project. See [LICENSE](../LICENSE) for usage terms.
