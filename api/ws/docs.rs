//! OCOS-Chain: WebSocket API Documentation Generator
//!
//! Provides real-time, machine-readable and human-readable documentation
//! for all WebSocket topics, event schemas, subscription flows, and protocol extensions.

use crate::ws::types::{WsTopic, WsMessage};
use serde_json::json;

/// Returns OpenAPI-style JSON documentation for all available WebSocket channels and events.
pub fn api_docs_json() -> serde_json::Value {
    json!({
        "protocol": "OCOS-Chain WebSocket",
        "version": "1.0.0",
        "topics": [
            {
                "name": "Blocks",
                "description": "New blocks, finalized blocks, and chain state updates.",
                "subscribe": "Subscribe to real-time new block notifications.",
                "messages": [
                    "NewBlock", "NewTransaction", "ChainStateUpdate"
                ]
            },
            {
                "name": "Governance",
                "description": "DAO proposals, voting, execution, council actions.",
                "subscribe": "Subscribe to governance proposal and voting events.",
                "messages": [
                    "GovernanceProposalCreated", "GovernanceVoteCast",
                    "GovernanceProposalExecuted", "CouncilEvent", "GovernanceConfigUpdate"
                ]
            },
            {
                "name": "Dao",
                "description": "DAO creation, membership, metadata, and custom DAO actions.",
                "subscribe": "Subscribe to DAO-level organization events.",
                "messages": [
                    "DaoCreated", "DaoMetadataUpdated", "DaoMemberJoined",
                    "DaoMemberLeft", "DaoCustomEvent"
                ]
            },
            {
                "name": "Liquidity",
                "description": "Pool creation, swaps, staking/farming, oracle, and bridge events.",
                "subscribe": "Subscribe to all DeFi/AMM events.",
                "messages": [
                    "PoolCreated", "LiquidityChanged", "SwapExecuted", "StakingEvent",
                    "RewardHarvested", "OraclePriceUpdated", "BridgeEvent"
                ]
            },
            {
                "name": "NFT",
                "description": "NFT minting, transfer, listing, sale, auction, and collection updates.",
                "subscribe": "Subscribe to NFT and collection marketplace events.",
                "messages": [
                    "NftMinted", "NftTransferred", "NftBurned", "NftListed", "NftSold",
                    "NftAuctionEvent", "CollectionEvent"
                ]
            },
            {
                "name": "Identity",
                "description": "Identity registration, reputation, attestation, soulbound, KYC.",
                "subscribe": "Subscribe to identity and reputation events.",
                "messages": [
                    "IdentityRegistered", "ReputationUpdated", "AttestationEvent", "SoulboundEvent",
                    "IdentityCustomEvent"
                ]
            },
            {
                "name": "Analytics",
                "description": "Real-time analytics, protocol KPIs, performance and health events.",
                "subscribe": "Subscribe to protocol and DAO analytics streams.",
                "messages": [
                    "AnalyticsMetric", "HealthStatus", "DaoKpi", "AnalyticsCustomEvent"
                ]
            }
        ],
        "protocol_ops": [
            { "op": "Subscribe", "description": "Subscribe to a topic (channel)." },
            { "op": "Unsubscribe", "description": "Unsubscribe from a topic." },
            { "op": "Ping/Pong", "description": "Heartbeat for connection keepalive." },
            { "op": "Ack", "description": "Acknowledge a successful subscription." },
            { "op": "Error", "description": "Protocol or application error message." },
            { "op": "Handshake", "description": "Connection handshake with version negotiation." },
            { "op": "Close", "description": "Graceful disconnect from server." },
            { "op": "Custom", "description": "Extensible custom protocol operation." }
        ],
        "schema_url": "/ws/schema", // Path to full JSON schema (if available)
        "documentation_url": "https://ocos.io/docs/ws-api",
        "sample_code": "See /ws/docs or GitHub for sample client integration."
    })
}

/// Returns human-readable markdown documentation for explorers or devs.
pub fn api_docs_markdown() -> String {
    let md = r#"
# OCOS-Chain WebSocket API Documentation

**Real-time event channels for OCOS-Chain core, governance, DeFi, NFT, identity, analytics, and more.**

---

## Available Topics & Events

| Topic        | Description                                       | Example Messages                      |
|--------------|---------------------------------------------------|---------------------------------------|
| Blocks       | Block events, chain state                         | NewBlock, NewTransaction              |
| Governance   | DAO proposals, voting, council, exec              | GovernanceProposalCreated, GovernanceVoteCast |
| Dao          | DAO membership, metadata, org events              | DaoCreated, DaoMemberJoined           |
| Liquidity    | Pools, swaps, rewards, oracle, bridge             | PoolCreated, SwapExecuted, OraclePriceUpdated |
| NFT          | Minting, transfer, auction, collection            | NftMinted, NftSold, CollectionEvent   |
| Identity     | Registration, KYC, soulbound, reputation          | IdentityRegistered, ReputationUpdated |
| Analytics    | Metrics, KPIs, health                             | AnalyticsMetric, HealthStatus         |

## Protocol Operations

- `Subscribe` / `Unsubscribe` — Add/remove a channel subscription
- `Ping` / `Pong` — Connection keepalive
- `Ack` — Successful operation confirmation
- `Error` — Protocol/application error message
- `Handshake` — Connection/version negotiation
- `Close` — Graceful disconnect
- `Custom` — User-defined operations

## Schema & Samples

- [JSON schema](https://ocos.io/ws/schema) — Full event type definitions
- [API reference](https://ocos.io/docs/ws-api)
- See GitHub for client code samples

---
    "#;
    md.to_string()
}
