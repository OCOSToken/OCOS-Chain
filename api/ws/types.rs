//! OCOS-Chain: WebSocket Shared Types
//!
//! Shared enums, structs, topics, message schemas, and client/session types for all WS handlers.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// --- WebSocket Topics (subscribe/unsubscribe channels) ---
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WsTopic {
    Blocks,
    Transactions,
    ChainState,
    Governance,
    Dao,
    Liquidity,
    Nft,
    Identity,
    Analytics,
    Custom(String),
}

// --- WebSocket Requests from clients ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsRequest {
    Subscribe { topic: WsTopic },
    Unsubscribe { topic: WsTopic },
    Ping,
    Pong,
    Custom { payload: serde_json::Value },
}

// --- WebSocket Responses/Push Messages ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsResponse {
    Ack { topic: WsTopic },
    Pong,
    Error(String),
    // You may add further push messages here.
}

// --- Main Message Types for Streaming (used in handlers) ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsMessage {
    // Blocks/Txs
    NewBlock { block: BlockInfo },
    NewTransaction { tx: TxInfo },
    ChainStateUpdate { state: serde_json::Value },

    // Governance/DAO
    GovernanceProposalCreated { proposal: ProposalInfo },
    GovernanceVoteCast { vote: VoteInfo },
    GovernanceProposalExecuted { proposal_id: u64, status: GovernanceStatus },
    CouncilEvent { event: serde_json::Value },
    GovernanceConfigUpdate { config: serde_json::Value },
    DaoCreated { dao: DaoInfo },
    DaoMetadataUpdated { dao: DaoInfo },
    DaoMemberJoined { dao_id: u64, member: MemberInfo },
    DaoMemberLeft { dao_id: u64, member: MemberInfo },
    DaoCustomEvent { dao_id: u64, action: serde_json::Value },

    // Liquidity/DeFi
    PoolCreated { pool: PoolInfo },
    LiquidityChanged { pool_id: u64, delta: i128 },
    SwapExecuted { swap: SwapInfo },
    StakingEvent { staking: StakingInfo },
    RewardHarvested { reward: RewardInfo },
    OraclePriceUpdated { oracle: OraclePriceInfo },
    BridgeEvent { bridge: BridgeEvent },

    // NFT
    NftMinted { nft: NftInfo },
    NftTransferred { transfer: NftTransfer },
    NftBurned { nft: NftInfo },
    NftListed { listing: NftListing },
    NftSold { sale: NftSale },
    NftAuctionEvent { auction: NftAuction },
    CollectionEvent { collection: CollectionInfo },

    // Identity
    IdentityRegistered { identity: IdentityInfo },
    ReputationUpdated { event: ReputationEvent },
    AttestationEvent { event: AttestationEvent },
    SoulboundEvent { event: SoulboundEvent },
    IdentityCustomEvent { payload: serde_json::Value },

    // Analytics
    AnalyticsMetric { metric: AnalyticsMetric },
    HealthStatus { status: HealthStatus },
    DaoKpi { kpi: DaoKpi },
    AnalyticsCustomEvent { payload: serde_json::Value },

    // Generic/custom
    Custom { payload: serde_json::Value },
}

// --- Example type definitions for reference ---

pub type ClientId = u64;

// Block & Tx
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockInfo {
    pub number: u64,
    pub hash: String,
    pub timestamp: u64,
    pub tx_count: u32,
    pub producer: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxInfo {
    pub hash: String,
    pub from: String,
    pub to: String,
    pub value: u128,
    pub status: String,
    pub block: Option<u64>,
}

// Governance & DAO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalInfo {
    pub id: u64,
    pub proposer: String,
    pub kind: String,
    pub description: String,
    pub status: GovernanceStatus,
    pub created_at: u64,
    pub voting_end: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteInfo {
    pub proposal_id: u64,
    pub voter: String,
    pub amount: u128,
    pub option: String,
    pub timestamp: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GovernanceStatus {
    Pending,
    Approved,
    Rejected,
    Executed,
    Expired,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaoInfo {
    pub id: u64,
    pub name: String,
    pub metadata: serde_json::Value,
    pub created_at: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberInfo {
    pub address: String,
    pub role: String,
    pub joined_at: u64,
}

// Liquidity/DeFi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolInfo {
    pub id: u64,
    pub token_a: String,
    pub token_b: String,
    pub total_liquidity: u128,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapInfo {
    pub pool_id: u64,
    pub user: String,
    pub amount_in: u128,
    pub amount_out: u128,
    pub token_in: String,
    pub token_out: String,
    pub price: f64,
    pub timestamp: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakingInfo {
    pub user: String,
    pub pool_id: u64,
    pub amount_staked: u128,
    pub timestamp: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardInfo {
    pub user: String,
    pub pool_id: u64,
    pub reward: u128,
    pub timestamp: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OraclePriceInfo {
    pub oracle_id: u64,
    pub token: String,
    pub price: u128,
    pub updated_at: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeEvent {
    pub bridge_id: u64,
    pub from_chain: String,
    pub to_chain: String,
    pub amount: u128,
    pub status: String,
    pub timestamp: u64,
}

// NFT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftInfo {
    pub id: u64,
    pub owner: String,
    pub uri: String,
    pub collection: Option<u64>,
    pub attributes: serde_json::Value,
    pub minted_at: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftTransfer {
    pub nft_id: u64,
    pub from: String,
    pub to: String,
    pub timestamp: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftSale {
    pub nft_id: u64,
    pub buyer: String,
    pub seller: String,
    pub price: u128,
    pub timestamp: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftListing {
    pub nft_id: u64,
    pub seller: String,
    pub price: u128,
    pub listed_at: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftAuction {
    pub auction_id: u64,
    pub nft_id: u64,
    pub status: String,
    pub current_bid: Option<u128>,
    pub end_time: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionInfo {
    pub id: u64,
    pub name: String,
    pub owner: String,
    pub metadata: serde_json::Value,
    pub created_at: u64,
}

// Identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityInfo {
    pub id: u64,
    pub address: String,
    pub did: String,
    pub display_name: Option<String>,
    pub profile: serde_json::Value,
    pub created_at: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationEvent {
    pub identity_id: u64,
    pub score: i32,
    pub badge: Option<String>,
    pub reason: Option<String>,
    pub updated_at: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationEvent {
    pub identity_id: u64,
    pub attestor: String,
    pub attestation_type: String,
    pub status: String,
    pub metadata: serde_json::Value,
    pub issued_at: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulboundEvent {
    pub identity_id: u64,
    pub token_id: u64,
    pub attributes: serde_json::Value,
    pub issued_at: u64,
}

// Analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsMetric {
    pub name: String,
    pub value: f64,
    pub unit: Option<String>,
    pub tags: Option<Vec<String>>,
    pub timestamp: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub component: String,
    pub status: String, // e.g., "healthy", "degraded", "critical"
    pub details: Option<serde_json::Value>,
    pub checked_at: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaoKpi {
    pub dao_id: u64,
    pub metric: String,
    pub value: f64,
    pub updated_at: u64,
}
