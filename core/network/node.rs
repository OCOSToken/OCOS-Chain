//! OCOS-Chain: Node Identity and Status Module
//!
//! Represents network node identity, versioning, status, trust management,
//! and heartbeat monitoring for OCOS-Chain.
//!
//! Used for peer management, consensus participation, governance and node health.

use std::time::{Instant, SystemTime, UNIX_EPOCH};

/// Unique identifier for each node (e.g., pubkey hash or random UUID)
pub type NodeId = String;

/// Current status of a node in the network
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeStatus {
    Online,
    Offline,
    Syncing,
    Banned,
    Unknown,
}

/// Detailed information about a node
#[derive(Debug, Clone)]
pub struct NodeInfo {
    pub id: NodeId,
    pub address: String,          // e.g., "127.0.0.1:3030" or DNS multiaddr
    pub version: String,          // OCOS software version
    pub agent: String,            // Node client identifier (e.g., "ocosd/1.0")
    pub status: NodeStatus,
    pub last_heartbeat: Instant,  // Last time node was seen active
    pub trust: u8,                // Trust score (0-100, for governance/whitelist)
}

impl NodeInfo {
    /// Create a new node info
    pub fn new(id: NodeId, address: String, version: String, agent: String) -> Self {
        Self {
            id,
            address,
            version,
            agent,
            status: NodeStatus::Unknown,
            last_heartbeat: Instant::now(),
            trust: 50,
        }
    }

    /// Mark node as online and update heartbeat
    pub fn set_online(&mut self) {
        self.status = NodeStatus::Online;
        self.last_heartbeat = Instant::now();
    }

    /// Mark node as offline
    pub fn set_offline(&mut self) {
        self.status = NodeStatus::Offline;
    }

    /// Increase trust score (max 100)
    pub fn increase_trust(&mut self, by: u8) {
        self.trust = (self.trust + by).min(100);
    }

    /// Decrease trust score (min 0)
    pub fn decrease_trust(&mut self, by: u8) {
        self.trust = self.trust.saturating_sub(by);
    }
}
