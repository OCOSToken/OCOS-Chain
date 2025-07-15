//! OCOS-Chain: Liquidity/DEX Common Types & Identifiers
//!
//! Centralizes all shared types, identifiers, and enums for the liquidity protocol.

pub type PoolId = u64;
pub type TokenId = u64;
pub type LPTokenId = u64;
pub type Amount = u128;
pub type Price = u128;
pub type Address = [u8; 20];
pub type OracleId = u64;
pub type BridgeId = u64;
pub type ChainId = u32;
pub type Proof = Vec<u8>;
pub type Timestamp = u64;
pub type ProposalId = u64;

/// Status of a liquidity governance proposal
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProposalStatus {
    Pending,
    Approved,
    Rejected,
    Executed,
    Failed,
    Closed,
}

/// Governance proposal kinds for liquidity/DEX
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GovernanceProposalKind {
    ConfigUpdate { key: String },
    LaunchPool { token_a: TokenId, token_b: TokenId },
    UpdateFarming { pool_id: PoolId, new_reward_per_block: Amount },
    Upgrade { description: String },
    TreasuryAction { description: String },
    Custom { name: String, data: Vec<u8> },
}

/// Example event structure for liquidity operations (for logs/indexing)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiquidityEventKind {
    PoolCreated { pool_id: PoolId },
    LiquidityAdded { pool_id: PoolId, amount_a: Amount, amount_b: Amount },
    LiquidityRemoved { pool_id: PoolId, amount_a: Amount, amount_b: Amount },
    TokenSwapped { pool_id: PoolId, amount_in: Amount, amount_out: Amount },
    RewardsHarvested { pool_id: PoolId, user: Address, amount: Amount },
    OraclePriceUpdated { oracle_id: OracleId, new_price: Price },
    // ...extendable for all DEX/farming/bridge events
}
