# OCOS-Chain: Ledger Subsystem (`/ledger`)

**Stateful | Auditable | Modular | Blockchain-Native Accounting**

---

## Overview

The `/ledger` directory is the backbone of the OCOS-Chain blockchain’s persistent state, recording all blocks, transactions, account balances, state transitions, and historical proofs.  
It is designed for maximum auditability, modular integration, and high-throughput deterministic execution.

---

## Main Modules and Files

| File / Folder      | Purpose & Function                                                        |
|--------------------|---------------------------------------------------------------------------|
| `mod.rs`           | Ledger root module; integrates all ledger components                      |
| `block.rs`         | Block structure, header, and block validation logic                       |
| `transaction.rs`   | Transactions, signed transactions, and core tx processing                 |
| `state.rs`         | State management: balances, nonces, smart contract code & storage roots   |
| `block_store.rs`   | Persistent storage and indexed access to all blocks                       |
| `tx_pool.rs`       | Mempool for unconfirmed/pending transactions                              |
| `executor.rs`      | Deterministic block & transaction execution engine                        |
| `receipt.rs`       | Transaction receipts & event logs for contract/audit tracing              |
| `snapshot.rs`      | State snapshot/restore for backup, fast sync, audit, and chain forks      |
| `history.rs`       | Ordered block and transaction history; supports explorer & rewind         |
| `audit.rs`         | Ledger auditing, compliance hooks, trace logs                             |
| `tests.rs`         | Unit/integration tests for all ledger modules                             |

---

## Key Features

- **Stateful accounting:** Tracks every block, tx, account, and contract state
- **Auditable by design:** All operations log receipts, proofs, and history for compliance
- **Modular:** Each module is decoupled for easy upgrades and integrations
- **Fast sync & backup:** Native snapshot/restore for rapid onboarding and disaster recovery
- **Explorer ready:** API-friendly interfaces for block/tx queries, event logs, and chain analysis

---

## Directory Structure

```
ledger/
│
├── mod.rs
├── block.rs
├── transaction.rs
├── state.rs
├── block_store.rs
├── tx_pool.rs
├── executor.rs
├── receipt.rs
├── snapshot.rs
├── history.rs
├── audit.rs
└── tests.rs
```

---

## Usage Scenarios

- Block and transaction storage/retrieval
- On-chain account balance, nonce, contract storage management
- Audit, compliance, and event tracing
- Fast node bootstrap & fork handling via snapshots
- DApp and DAO explorer support (transaction/event search, state proofs, etc.)

---

## Maintainers & Contact

- [OCOS Foundation](https://ocos.io)
- [Ocoshy Nakomoto (Protocol Architect)](https://github.com/Ocoshy)

---

## License

This module is part of the OCOS-Chain open-source project.  
See [LICENSE](../LICENSE) for more details.
