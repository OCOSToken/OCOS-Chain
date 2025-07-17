# OCOS-Chain: REST API Module

**Modular | Auditable | Secure | Enterprise-Ready**

---

## Overview

The `/api/rest` module is the central gateway for all OCOS-Chain data, transaction, and governance access via HTTP/JSON.  
It provides a production-grade, fully auditable, and extensible REST API for explorers, wallets, dApps, DAO dashboards, exchanges, analytics, and external services.

---

## Features

- **Unified endpoint router:** Blocks, transactions, accounts, governance, identity, liquidity, DAO, analytics
- **OpenAPI/Swagger documentation:** Interactive API explorer and codegen
- **Token/JWT-based authentication:** Secure, role-based access control for public and protected endpoints
- **Comprehensive error mapping:** All errors mapped to HTTP codes and JSON schema
- **Rate limiting, CORS, logging, gzip/compression, and monitoring:** Production security and performance best practices
- **API versioning:** Backward-compatible upgrades (v1, v2, etc.)
- **Automated integration and endpoint tests:** CI-ready for every release

---

## Directory Structure

```
api/rest/
│
├── mod.rs          # API entry point, mounts all routes, middleware, and exports
├── routes.rs       # Main HTTP router: all endpoints (ledger, identity, DAO, etc.)
├── handlers/       # Business logic for each API namespace:
│    ├── ledger.rs      # Block, transaction, and state endpoints
│    ├── governance.rs  # DAO, proposals, voting, council endpoints
│    ├── identity.rs    # On-chain profiles, KYC, soulbound, reputation endpoints
│    ├── liquidity.rs   # Pools, swaps, staking, oracles, bridge endpoints
│    ├── dao.rs         # DAO metadata, members, permissions
│    ├── metrics.rs     # Performance and analytics endpoints
├── types.rs        # Shared API models (pagination, error, request/response)
├── auth.rs         # Auth/token/session middleware
├── error.rs        # API error mapping, HTTP codes, and JSON responses
├── version.rs      # API version, build, and healthcheck endpoints
├── docs.rs         # Swagger/OpenAPI doc serving
├── middleware.rs   # CORS, logging, compression, rate-limiting, tracing
├── tests.rs        # Full integration and endpoint coverage tests
```

---

## Security & Audit

- **JWT & API key auth:** Full support for bearer tokens and permissioned routes
- **Rate limiting:** Abuse protection at all endpoints
- **Comprehensive request/response logging:** Forensics, trace, and security monitoring
- **CI/CD tested:** Every change validated by automated endpoint tests

---

## Usage

- Designed for easy integration with web/mobile UIs, exchanges, oracles, analytics dashboards, and bot clients
- All endpoints documented and discoverable via `/docs` (Swagger UI)
- Supports both public and admin/permissioned API access

---

## License

This module is part of the OCOS-Chain project. See [LICENSE](../../LICENSE) for terms.
