//! OCOS-Chain: WebSocket Handler – Liquidity/DeFi Events
//!
//! Streams real-time pool, swap, staking, farming, oracle, and bridge events over WebSocket.

use crate::ws::types::{
    WsTopic, WsMessage, PoolInfo, SwapInfo, StakingInfo, RewardInfo, OraclePriceInfo, BridgeEvent,
};
use crate::ws::router::WsRouter;

/// Handles streaming of all DeFi and liquidity-related events.
pub struct LiquidityHandler;

impl LiquidityHandler {
    /// Broadcast when a new liquidity pool is created
    pub fn on_pool_created(router: &WsRouter, pool: PoolInfo) {
        let msg = WsMessage::PoolCreated { pool: pool.clone() };
        router.broadcast(&WsTopic::Liquidity, &msg);
    }

    /// Broadcast when liquidity is added or removed
    pub fn on_liquidity_changed(router: &WsRouter, pool_id: u64, delta: i128) {
        let msg = WsMessage::LiquidityChanged { pool_id, delta };
        router.broadcast(&WsTopic::Liquidity, &msg);
    }

    /// Broadcast swap events (token swap in a pool)
    pub fn on_swap(router: &WsRouter, swap: SwapInfo) {
        let msg = WsMessage::SwapExecuted { swap: swap.clone() };
        router.broadcast(&WsTopic::Liquidity, &msg);
    }

    /// Broadcast staking or farming activity
    pub fn on_stake_event(router: &WsRouter, staking: StakingInfo) {
        let msg = WsMessage::StakingEvent { staking: staking.clone() };
        router.broadcast(&WsTopic::Liquidity, &msg);
    }

    /// Broadcast farming reward harvests
    pub fn on_reward_harvested(router: &WsRouter, reward: RewardInfo) {
        let msg = WsMessage::RewardHarvested { reward: reward.clone() };
        router.broadcast(&WsTopic::Liquidity, &msg);
    }

    /// Broadcast oracle price updates for DeFi protocols
    pub fn on_oracle_update(router: &WsRouter, oracle: OraclePriceInfo) {
        let msg = WsMessage::OraclePriceUpdated { oracle: oracle.clone() };
        router.broadcast(&WsTopic::Liquidity, &msg);
    }

    /// Broadcast cross-chain bridge events
    pub fn on_bridge_event(router: &WsRouter, bridge: BridgeEvent) {
        let msg = WsMessage::BridgeEvent { bridge: bridge.clone() };
        router.broadcast(&WsTopic::Liquidity, &msg);
    }
}

// -- Example message types for reference --

/*
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsMessage {
    PoolCreated { pool: PoolInfo },
    LiquidityChanged { pool_id: u64, delta: i128 },
    SwapExecuted { swap: SwapInfo },
    StakingEvent { staking: StakingInfo },
    RewardHarvested { reward: RewardInfo },
    OraclePriceUpdated { oracle: OraclePriceInfo },
    BridgeEvent { bridge: BridgeEvent },
    // ... more
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolInfo {
    pub id: u64,
    pub token_a: String,
    pub token_b: String,
    pub total_liquidity: u128,
    // ... etc
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
*/
