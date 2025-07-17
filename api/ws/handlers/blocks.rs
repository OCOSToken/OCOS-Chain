//! OCOS-Chain: WebSocket Handler – Blocks & Transactions
//!
//! Handles real-time broadcasting of new blocks, transactions, and chain state updates.

use crate::ws::types::{WsTopic, WsMessage, BlockInfo, TxInfo};
use crate::ws::router::WsRouter;

/// Main handler for block and transaction event streaming.
pub struct BlocksHandler;

impl BlocksHandler {
    /// Called by the node or block import logic when a new block is finalized.
    pub fn on_new_block(router: &WsRouter, block: BlockInfo) {
        let msg = WsMessage::NewBlock { block: block.clone() };
        router.broadcast(&WsTopic::Blocks, &msg);
    }

    /// Called by the mempool or consensus when a new transaction is observed.
    pub fn on_new_tx(router: &WsRouter, tx: TxInfo) {
        let msg = WsMessage::NewTransaction { tx: tx.clone() };
        router.broadcast(&WsTopic::Blocks, &msg);
        router.broadcast(&WsTopic::Transactions, &msg);
    }

    /// (Optional) Broadcast chain state update (e.g., staking/validators/metrics)
    pub fn on_chain_state(router: &WsRouter, state_update: serde_json::Value) {
        let msg = WsMessage::ChainStateUpdate { state: state_update };
        router.broadcast(&WsTopic::ChainState, &msg);
    }
}

// -- Example message types for reference --

/*
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsMessage {
    NewBlock { block: BlockInfo },
    NewTransaction { tx: TxInfo },
    ChainStateUpdate { state: serde_json::Value },
    // ... other event variants
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockInfo {
    pub number: u64,
    pub hash: String,
    pub timestamp: u64,
    pub tx_count: u32,
    pub producer: String,
    // ... additional block metadata
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxInfo {
    pub hash: String,
    pub from: String,
    pub to: String,
    pub value: u128,
    pub status: String,
    pub block: Option<u64>,
    // ... additional tx metadata
}
*/
