//! OCOS-Chain: Peer-to-Peer (P2P) Networking Layer
//!
//! Handles peer identity, discovery, handshake protocol, connection state,
//! and peer health management. Critical for secure and efficient gossip,
//! sync, and validator messaging.

use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::time::{Duration, Instant};

/// Unique identifier for a peer (could be derived from pubkey hash)
pub type PeerId = String;

/// Peer connection status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PeerStatus {
    Connected,
    Handshaking,
    Disconnected,
    Banned,
}

/// Metadata and state tracking for a peer
#[derive(Debug, Clone)]
pub struct PeerInfo {
    pub id: PeerId,
    pub address: SocketAddr,
    pub status: PeerStatus,
    pub last_seen: Instant,
    pub latency: Option<Duration>,
    pub protocol_version: String,
    pub client_agent: String,
}

/// Core peer management structure
#[derive(Debug, Default)]
pub struct PeerManager {
    peers: HashMap<PeerId, PeerInfo>,
    banned: HashSet<PeerId>,
}

impl PeerManager {
    /// Create new peer manager
    pub fn new() -> Self {
        Self {
            peers: HashMap::new(),
            banned: HashSet::new(),
        }
    }

    /// Register or update a peer
    pub fn register_peer(&mut self, info: PeerInfo) {
        if !self.banned.contains(&info.id) {
            self.peers.insert(info.id.clone(), info);
        }
    }

    /// Mark peer as connected
    pub fn mark_connected(&mut self, id: &PeerId) {
        if let Some(peer) = self.peers.get_mut(id) {
            peer.status = PeerStatus::Connected;
            peer.last_seen = Instant::now();
        }
    }

    /// Mark peer as disconnected
    pub fn mark_disconnected(&mut self, id: &PeerId) {
        if let Some(peer) = self.peers.get_mut(id) {
            peer.status = PeerStatus::Disconnected;
        }
    }

    /// Ban a peer (malicious or unstable)
    pub fn ban_peer(&mut self, id: &PeerId) {
        self.peers.remove(id);
        self.banned.insert(id.clone());
    }

    /// Check if a peer is banned
    pub fn is_banned(&self, id: &PeerId) -> bool {
        self.banned.contains(id)
    }

    /// Get list of active peers
    pub fn active_peers(&self) -> Vec<&PeerInfo> {
        self.peers
            .values()
            .filter(|p| p.status == PeerStatus::Connected)
            .collect()
    }

    /// Get peer by ID
    pub fn get_peer(&self, id: &PeerId) -> Option<&PeerInfo> {
        self.peers.get(id)
    }
}
