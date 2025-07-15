//! OCOS-Chain: Network Synchronization Engine
//!
//! Responsible for block/tx sync, fork detection, chain state negotiation, and
//! live network convergence. Used by nodes for catching up, fast bootstrapping,
//! and maintaining blockchain consistency across all peers.

use std::collections::HashMap;
use crate::network::node::{NodeId, NodeInfo};
use crate::network::message::{NetworkMessage, MessageType, GossipPayload};

/// Synchronization state of a node
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncState {
    Idle,
    Syncing,
    Waiting,
    Complete,
    Failed,
}

/// Core sync engine struct
#[derive(Debug)]
pub struct SyncEngine {
    pub state: SyncState,
    pub known_height: u64,
    pub peers: HashMap<NodeId, NodeInfo>,
    pub target_height: Option<u64>,
}

impl SyncEngine {
    /// Initialize new SyncEngine
    pub fn new() -> Self {
        SyncEngine {
            state: SyncState::Idle,
            known_height: 0,
            peers: HashMap::new(),
            target_height: None,
        }
    }

    /// Start sync process toward target block height
    pub fn start_sync(&mut self, target: u64) {
        self.state = SyncState::Syncing;
        self.target_height = Some(target);
    }

    /// Update known blockchain height (on new block received)
    pub fn update_height(&mut self, height: u64) {
        if height > self.known_height {
            self.known_height = height;
        }
        if let Some(target) = self.target_height {
            if self.known_height >= target {
                self.state = SyncState::Complete;
            }
        }
    }

    /// Handle a network sync message (from peer)
    pub fn handle_message(&mut self, msg: &NetworkMessage) {
        match &msg.message_type {
            MessageType::SyncResponse | MessageType::Block => {
                if let GossipPayload::SyncResponse(chain_data) = &msg.payload {
                    // In real implementation: parse chain_data and update state
                    self.state = SyncState::Syncing;
                    // ...block validation, state update, etc.
                }
            }
            MessageType::Snapshot => {
                // Snapshot/state diff logic (for DAO and light client)
                self.state = SyncState::Syncing;
            }
            _ => {}
        }
    }

    /// Add or update a peer in sync context
    pub fn add_peer(&mut self, node: NodeInfo) {
        self.peers.insert(node.id.clone(), node);
    }

    /// Get best peer (highest block height, online status, etc.)
    pub fn best_peer(&self) -> Option<&NodeInfo> {
        self.peers.values()
            .filter(|n| n.status == crate::network::node::NodeStatus::Online)
            .max_by_key(|n| n.trust)
    }
}
