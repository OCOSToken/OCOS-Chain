# OCOS-Chain: Liquidity, AMM & DeFi Module

**Pools | AMM | Staking | Farming | Oracles | Bridge | On-chain Governance**

---

## Overview

The `contracts/liquidity` module implements the entire DeFi and DEX protocol stack for OCOS-Chain.  
It provides modular, auditable, and extensible infrastructure for liquidity pools, AMMs, staking, farming, oracles, cross-chain bridging, rewards, and governance—all fully on-chain.

---

## Features

- **Liquidity pools:** Token pair pools with LP token issuance, deposit/withdraw logic
- **AMM swaps:** Automated Market Maker (Uniswap-like) token swaps, price discovery, and invariant checks
- **Staking & farming:** Real-time rewards for liquidity providers; block-by-block farming campaigns
- **Oracle feeds:** Secure, updatable price and data feeds for on-chain protocols
- **Bridge:** Cross-chain lock, mint, burn, and unlock for asset bridging with oracle verification
- **Configurable protocol parameters:** All fees, limits, rewards, and intervals are DAO-controlled
- **Governance:** Fully on-chain proposal, voting, and execution for liquidity and DEX changes
- **Events & audit:** Every action emits an event, fully traceable and indexable for analytics

---

## Directory Structure

```
contracts/liquidity/
│
├── mod.rs         # Entry point, exports all liquidity modules
├── pool.rs        # Core pool structure & logic
├── amm.rs         # AMM swap and price discovery logic
├── staking.rs     # Staking logic (if separated)
├── farming.rs     # Farming and reward campaign logic
├── rewards.rs     # Reward distribution engine
├── oracle.rs      # Oracle feed registry and update
├── bridge.rs      # Cross-chain asset bridge
├── config.rs      # Protocol parameters & governance integration
├── governance.rs  # Liquidity/DEX-specific DAO module
├── fee.rs         # Fee calculation and accounting
├── router.rs      # Swap/trade/farming router logic
├── types.rs       # Shared types, IDs, and enums
├── events.rs      # All on-chain event structures
├── error.rs       # Error codes & messages
├── storage.rs     # Persistent storage for all modules
├── tests.rs       # Unit & integration tests
```

---

## Security & Audit

- **Slippage, fee, and invariant checks** for every pool and swap
- **Strict role- and DAO-based parameter updates**
- **Event-driven monitoring:** All state changes emit chain events
- **On-chain proposal and upgrade for protocol evolution**

---

## Integration

- Works with `/core/consensus` for block validation and parameter updates
- Uses `/crypto` for asset/account, reward, and oracle signature verification
- Integrates with `/ledger` for on-chain balances and event recording

---

## License

This module is part of the OCOS-Chain project. See [LICENSE](../../LICENSE) for usage terms.
