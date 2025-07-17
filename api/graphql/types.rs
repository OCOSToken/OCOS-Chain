//! OCOS-Chain: GraphQL API Shared Types & Enums
//!
//! Centralizes all input/output types, enums, and identifiers for the GraphQL schema.

use async_graphql::{SimpleObject, InputObject, Enum, ID};

// ----- Core Identifiers -----
pub type Address = String;
pub type PoolId = u64;
pub type TokenId = u64;
pub type ProposalId = u64;
pub type DaoId = u64;
pub type OracleId = u64;

// ----- Pagination -----
#[derive(InputObject, Clone)]
pub struct Pagination {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

// ----- Ledger/Chain Types -----
#[derive(SimpleObject, Clone)]
pub struct Block {
    pub number: u64,
    pub hash: String,
    pub parent_hash: String,
    pub timestamp: u64,
    pub tx_count: usize,
    pub proposer: Address,
}

#[derive(SimpleObject, Clone)]
pub struct Transaction {
    pub hash: String,
    pub block_number: u64,
    pub from: Address,
    pub to: Address,
    pub amount: u128,
    pub timestamp: u64,
    pub status: String,
}

#[derive(SimpleObject, Clone)]
pub struct Balance {
    pub address: Address,
    pub amount: u128,
    pub token: String,
}

// ----- Governance Types -----
#[derive(SimpleObject, Clone)]
pub struct GovernanceProposal {
    pub id: ProposalId,
    pub proposer: Address,
    pub kind: String,
    pub description: String,
    pub status: ProposalStatus,
    pub created_at: u64,
    pub voting_end: u64,
    pub yes_votes: u128,
    pub no_votes: u128,
    pub executed: bool,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ProposalStatus {
    Pending,
    Approved,
    Rejected,
    Executed,
    Failed,
    Closed,
}

#[derive(InputObject, Clone)]
pub struct VoteInput {
    pub voter: Address,
    pub proposal_id: ProposalId,
    pub approve: bool,
    pub amount: u128,
}

#[derive(SimpleObject, Clone)]
pub struct VoteReceipt {
    pub voter: Address,
    pub proposal_id: ProposalId,
    pub approve: bool,
    pub amount: u128,
    pub timestamp: u64,
}

// ----- Identity Types -----
#[derive(SimpleObject, Clone)]
pub struct IdentityProfile {
    pub address: Address,
    pub did: String,
    pub username: Option<String>,
    pub email: Option<String>,
    pub kyc_status: KYCStatus,
    pub reputation: Option<Reputation>,
}

#[derive(SimpleObject, Clone)]
pub struct Reputation {
    pub score: u32,
    pub badges: Vec<String>,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum KYCStatus {
    Verified,
    Pending,
    Unverified,
}

// ----- Soulbound Tokens -----
#[derive(SimpleObject, Clone)]
pub struct SoulboundToken {
    pub id: u64,
    pub owner: Address,
    pub uri: String,
    pub level: u8,
    pub attributes: Vec<String>,
}

#[derive(InputObject, Clone)]
pub struct RegisterDidInput {
    pub address: Address,
    pub did: String,
}

#[derive(InputObject, Clone)]
pub struct UpdateProfileInput {
    pub address: Address,
    pub did: String,
    pub username: Option<String>,
    pub email: Option<String>,
}

#[derive(InputObject, Clone)]
pub struct MintSoulboundInput {
    pub address: Address,
    pub uri: String,
    pub level: u8,
    pub attributes: Vec<String>,
}

#[derive(InputObject, Clone)]
pub struct SetKycStatusInput {
    pub address: Address,
    pub status: KYCStatus,
}

// ----- DeFi/Liquidity Types -----
#[derive(SimpleObject, Clone)]
pub struct Pool {
    pub id: PoolId,
    pub token_a: Token,
    pub token_b: Token,
    pub reserve_a: u128,
    pub reserve_b: u128,
    pub total_lp_supply: u128,
    pub fee_basis_points: u16,
}

#[derive(SimpleObject, Clone)]
pub struct Token {
    pub id: TokenId,
    pub symbol: String,
    pub decimals: u8,
}

#[derive(InputObject, Clone)]
pub struct SwapInput {
    pub pool_id: PoolId,
    pub from_token: TokenId,
    pub to_token: TokenId,
    pub amount_in: u128,
    pub min_out: u128,
    pub recipient: Address,
}

#[derive(SimpleObject, Clone)]
pub struct SwapReceipt {
    pub pool_id: PoolId,
    pub from_token: TokenId,
    pub to_token: TokenId,
    pub amount_in: u128,
    pub amount_out: u128,
    pub fee_paid: u128,
    pub block_number: u64,
    pub tx_hash: String,
}

#[derive(InputObject, Clone)]
pub struct StakeInput {
    pub address: Address,
    pub pool_id: PoolId,
    pub amount: u128,
}

#[derive(InputObject, Clone)]
pub struct UnstakeInput {
    pub address: Address,
    pub pool_id: PoolId,
    pub amount: u128,
}

#[derive(SimpleObject, Clone)]
pub struct RewardInfo {
    pub address: Address,
    pub pool_id: PoolId,
    pub staked: u128,
    pub pending_rewards: u128,
    pub last_reward_block: u64,
}

#[derive(SimpleObject, Clone)]
pub struct OraclePrice {
    pub oracle_id: OracleId,
    pub token_id: TokenId,
    pub price: u128,
    pub last_updated: u64,
    pub source: String,
}

#[derive(SimpleObject, Clone)]
pub struct BridgeTransfer {
    pub bridge_id: u64,
    pub token_id: TokenId,
    pub amount: u128,
    pub from: Address,
    pub to_chain: u32,
    pub to_address: Address,
    pub locked: bool,
    pub timestamp: u64,
    pub proof: Option<Vec<u8>>,
}

// ----- DAO Types -----
#[derive(SimpleObject, Clone)]
pub struct Dao {
    pub id: DaoId,
    pub name: String,
    pub description: Option<String>,
    pub creator: Address,
    pub created_at: u64,
    pub member_count: u64,
    pub roles: Vec<DaoRole>,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum DaoRole {
    Admin,
    Council,
    Member,
}

#[derive(SimpleObject, Clone)]
pub struct DaoMember {
    pub address: Address,
    pub dao_id: DaoId,
    pub role: DaoRole,
    pub joined_at: u64,
    pub reputation: u32,
}

#[derive(InputObject, Clone)]
pub struct DaoMembershipInput {
    pub dao_id: DaoId,
    pub address: Address,
    pub action: String, // "join", "leave", etc.
}

#[derive(SimpleObject, Clone)]
pub struct DaoPermission {
    pub role: DaoRole,
    pub action: String,
    pub allowed: bool,
}

#[derive(SimpleObject, Clone)]
pub struct DaoActivity {
    pub dao_id: DaoId,
    pub actor: Address,
    pub activity: String,
    pub timestamp: u64,
    pub reference_id: Option<u64>,
}

// ----- Analytics Types -----
#[derive(SimpleObject, Clone)]
pub struct ChainStats {
    pub block_height: u64,
    pub tx_count: u64,
    pub node_count: u32,
    pub avg_block_time: f64,
    pub gas_used: u128,
    pub chain_id: u64,
}

#[derive(SimpleObject, Clone)]
pub struct DaoStats {
    pub dao_id: DaoId,
    pub members: u64,
    pub proposals_created: u64,
    pub proposals_passed: u64,
    pub votes_cast: u64,
    pub period: AnalyticsPeriod,
}

#[derive(SimpleObject, Clone)]
pub struct LiquidityStats {
    pub pool_id: PoolId,
    pub tvl: u128,
    pub swap_volume: u128,
    pub swap_count: u64,
    pub apy: f64,
    pub fees_earned: u128,
    pub stakers: u64,
    pub period: AnalyticsPeriod,
}

#[derive(SimpleObject, Clone)]
pub struct WalletStats {
    pub address: Address,
    pub balance: u128,
    pub farming_rewards: u128,
    pub proposals_voted: u64,
    pub swaps_made: u64,
    pub dao_memberships: u64,
    pub period: AnalyticsPeriod,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum AnalyticsPeriod {
    Day,
    Week,
    Month,
    All,
}
