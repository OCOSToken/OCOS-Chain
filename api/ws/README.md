# OCOS-Chain: WebSocket API Subsystem

**Real-Time | Modular | Topic-Based | Audit-Ready | Enterprise-Grade**

---

## Overview

The `/ws` module implements a **real-time, event-driven WebSocket API** for OCOS-Chain.  
It allows clients, dashboards, explorers, and bots to subscribe to core protocol, governance, DeFi, NFT, DAO, and analytics events and receive instant updates.  
Designed for extensibility, high performance, and maximum developer experience.

---

## Key Features

- **Topic-based subscriptions:** Clients subscribe to specific channels (blocks, governance, liquidity, NFT, etc.)
- **High-throughput event streaming:** New blocks, transactions, votes, pool swaps, NFT actions, and more in real time
- **Modular handlers:** Each topic/channel has its own handler, schema, and logic
- **Extensible protocol:** Custom operations, middleware, and protocol extensions are supported
- **Authentication and access control:** JWT, API key, signature-based login, session/token management
- **Enterprise security:** Pluggable middleware for logging, CORS, rate-limiting, replay, analytics, and tracing
- **Developer tooling:** Auto-generated docs, OpenAPI/GraphQL schema, sample code

---

## Directory Structure

```
ws/
│
├── mod.rs                # Main WebSocket entry point and dispatcher
├── router.rs             # Channel/router for topic-based subscriptions and dispatch
├── handlers/
│   ├── blocks.rs         # Real-time block, transaction and chain state events
│   ├── governance.rs     # Governance, proposals, votes, execution status events
│   ├── dao.rs            # DAO state, membership, and decision events
│   ├── liquidity.rs      # Pools, swaps, farming, reward events
│   ├── nft.rs            # NFT mint, transfer, listing, sale, auction live events
│   ├── identity.rs       # Identity, KYC, soulbound, reputation change events
│   ├── analytics.rs      # Chain-wide stats, DAO metrics, performance monitoring
├── types.rs              # Shared message/request/response types (serde, enum)
├── auth.rs               # WebSocket token/session authentication, permissions
├── protocol.rs           # Standard WS message protocol (ping/pong, subscribe, unsubscribe, error)
├── middleware.rs         # CORS, rate-limit, logging, replay/reconnect, compression
├── extensions.rs         # Optional: message replay, event filters, backfill support
├── docs.rs               # WebSocket API protocol documentation and usage
├── tests.rs              # Integration and protocol compliance tests


---

## Protocol & Subscription Model

- **Subscribe/Unsubscribe:** Clients can join/leave topics (e.g. `"blocks"`, `"nft"`, `"governance"`)
- **Real-time push:** All events are streamed instantly to all subscribed sessions
- **Heartbeat (Ping/Pong):** Automatic keep-alive for long-lived connections
- **Acknowledgement/Error:** Server returns `Ack` or `Error` for each request

**Supported Topics:**  
- Blocks & Transactions  
- Governance (DAO, proposals, voting)  
- DAO metadata and membership  
- Liquidity/DeFi (pools, swaps, staking, bridge, oracle)  
- NFT (mint, transfer, sale, collection, auction)  
- Identity & Reputation  
- Analytics & Protocol Metrics

---

## Security & Middleware

- **Authentication:** JWT, API Key, blockchain signature, or custom method
- **Rate limiting:** Per-client and per-topic
- **Logging and tracing:** Full audit trail for all message flows
- **Replay and recovery:** Late clients can catch up on missed events
- **CORS and cross-origin support:** Configurable for multi-tenant dApps

---

## Documentation & Developer Experience

- **Auto-generated docs:** See [`handlers/docs.rs`](./handlers/docs.rs) for OpenAPI/Markdown endpoints
- **Event schemas and code samples:** All message types in [`handlers/types.rs`](./handlers/types.rs)
- **Production-ready tests:** [`handlers/tests.rs`](./handlers/tests.rs) for router, handler, and middleware QA
- **Plug-and-play integrations:** Compatible with React/Vue dashboards, bots, and mobile apps

---

## Integration

- **Node/core event bus:** Seamless with async event producers (blocks, governance, DeFi, NFT)
- **Explorer/Wallet:** Real-time push for wallet UIs, block explorers, DAO dashboards
- **Analytics:** Streams protocol and DAO metrics for monitoring, research, compliance

---

## License

This subsystem is part of the OCOS-Chain project. See [LICENSE](../LICENSE) for usage terms.
