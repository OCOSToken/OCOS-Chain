# OCOS-Chain: GraphQL API Module

**Modular | Real-Time | Analytics-Ready | Enterprise-Grade**

---

## Overview

The `/api/graphql` directory provides a complete, modular, and extensible GraphQL API for OCOS-Chain.  
It exposes real-time, queryable access to all core blockchain, DAO, DeFi, identity, governance, analytics, and ledger data.  
Designed for high-throughput applications, explorers, dashboards, and integrations with external Web3 infrastructure.

---

## Features

- **Full protocol coverage:** Blocks, transactions, ledger, pools, swaps, DAO proposals, governance, identity, oracles, and more.
- **Real-time subscriptions:** Push chain, DeFi, and governance events to clients instantly.
- **Powerful filtering, pagination, and query flexibility:** Built for analytics, explorers, and advanced dashboards.
- **Enterprise-grade security:** Auth, rate limiting, middleware, directives, and extension hooks.
- **OpenAPI and Apollo-compatible:** Supports federated schemas, introspection, tracing, and analytics.

---

## Directory Structure

```
api/graphql/
│
├── mod.rs            # Main API entry, schema build and public exports
├── schema.rs         # Root GraphQL schema: Query, Mutation, Subscription
├── context.rs        # Per-request context: auth, state, logger
├── types.rs          # Shared types, enums, and GraphQL objects
├── resolvers/        # Modular resolvers (ledger, governance, liquidity, identity, dao, analytics)
│   ├── ledger.rs
│   ├── governance.rs
│   ├── identity.rs
│   ├── liquidity.rs
│   ├── dao.rs
│   └── analytics.rs
├── directives.rs     # Custom field directives (auth, rateLimit, audit, deprecated)
├── middleware.rs     # Middleware: logging, tracing, error, rate limit, auth
├── extensions.rs     # Schema-level extensions: ApolloTracing, Introspection, OTel
├── subscriptions.rs  # Real-time chain, DeFi, and DAO event streaming
├── error.rs          # API error mapping and formatting
├── tests.rs          # Automated integration/unit tests for all API modules
```

---

## Security & Best Practices

- **Authentication:** Token/JWT/Session-based, per-query and per-field
- **Rate limiting and audit logging:** Modular, configurable
- **Error handling:** Uniform, developer-friendly, with unique codes for every error
- **Composability:** Easy to add new modules, types, extensions, or middleware

---

## Integration

- Used by explorers, wallets, analytics dashboards, and bot integrations
- Designed for high-throughput queries and federated/enterprise deployments
- Connects seamlessly to OCOS-Chain core, ledger, and off-chain data sources

---

## License

This module is part of the OCOS-Chain project. See [LICENSE](../../LICENSE) for terms.
