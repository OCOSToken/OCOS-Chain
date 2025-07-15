//! OCOS-Chain: Network Message Types & Envelope
//!
//! Defines the standard network message types and payloads used for
//! peer-to-peer communication in OCOS-Chain.
//!
//! Includes block propagation, transaction relay, sync requests, ping/pong,
//! gossip broadcast, and snapshot/state sharing.

use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Envelope that wraps any message sent over the network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMessage {
    pub id: String,
    pub timestamp: u64,
    pub message_type: MessageType,
    pub payload: GossipPayload,
}

/// Types of messages supported by OCOS network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Block,
    Transaction,
    Ping,
    Pong,
    Gossip,
    SyncRequest,
    SyncResponse,
    Snapshot,
    Custom(String),
}

/// Core payload format for each message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GossipPayload {
    Block(Vec<u8>),            // Serialized block data
    Transaction(Vec<u8>),      // Serialized transaction
    Ping,                      // Keep-alive / latency check
    Pong,                      // Ping response
    SyncRequest(u64),          // Request block height
    SyncResponse(Vec<u8>),     // Chain segment
    Snapshot(Vec<u8>),         // State snapshot or compressed diff
    Custom { tag: String, data: Vec<u8> }, // Extendable
}

impl NetworkMessage {
    /// Create a new network message
    pub fn new(message_type: MessageType, payload: GossipPayload) -> Self {
        NetworkMessage {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: current_timestamp(),
            message_type,
            payload,
        }
    }
}

/// Returns current UNIX timestamp in seconds
fn current_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}
