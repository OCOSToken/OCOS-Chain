# OCOS-Chain: Governance Smart Contract Module

**Modular | Extensible | On-Chain Voting | DAO-Grade Security**

---

## Overview

The `contracts/governance` module provides a full-featured, auditable, and extensible governance system for OCOS-Chain. It implements council-based, weighted, referendum, and delegated voting; proposal lifecycle management; parameter/configuration control; execution logic; and robust event and storage infrastructure—entirely at the smart contract layer.

---

## Features

- **Council governance:** Multisig committees, rotating members, and emergency execution
- **Weighted voting:** Stake, reputation, or token-based voting power and thresholds
- **Referenda:** DAO-wide binary, approval, and quadratic voting for major changes
- **Delegation:** Liquid democracy and proxy voting mechanics
- **Proposal lifecycle:** Creation, voting period, status, approval/rejection, execution, and audit log
- **Configurable parameters:** On-chain adjustable voting/quorum/threshold rules
- **Event-driven:** Full on-chain event emission for proposals, votes, and config changes
- **Audit & upgrade ready:** Every action is logged; registry supports safe governance engine upgrades

---

## Directory Structure

```
contracts/governance/
│
├── mod.rs           # Main entry point, exports all governance modules
├── council.rs       # Council-based voting logic
├── weighted_vote.rs # Token, stake, or reputation-based voting
├── referendum.rs    # Referendum and plebiscite logic
├── delegation.rs    # Vote delegation (liquid democracy)
├── proposal.rs      # Proposal definition and lifecycle
├── config.rs        # Governance parameters and settings
├── execution.rs     # Proposal execution logic
├── registry.rs      # Dynamic registration of governance engines
├── error.rs         # Error definitions
├── types.rs         # Common enums and types
├── events.rs        # Event emission for all actions
├── storage.rs       # Persistent proposal, voting, and council data
├── tests.rs         # Governance contract unit & integration tests
```

---

## Security & Audit

- **Role-based and threshold access control:** Every on-chain action is validated by voting outcomes and rules
- **Double-vote, reentrancy, and replay protection:** Every critical operation is atomic and idempotent
- **Event log:** Every proposal, vote, execution, delegation, and parameter change is emitted as an event
- **Self-amendment:** The governance engine can be safely upgraded on-chain via registry and proposal mechanism

---

## Integration

- Tightly integrates with `/core/consensus` for block validation hooks
- Uses `/crypto` for signature verification and voting power
- Reads and writes to `/ledger` for DAO and proposal state

---

## License

This module is part of the OCOS-Chain project. See [LICENSE](../../LICENSE) for terms.
