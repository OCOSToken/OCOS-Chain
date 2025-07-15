//! OCOS-Chain: Network Layer Core Tests
//!
//! Comprehensive unit and integration tests for peer management, messaging,
//! codec integrity, node status updates, and sync logic.

use super::*;
use crate::network::{
    p2p::{PeerManager, PeerInfo, PeerStatus},
    message::{NetworkMessage, MessageType, GossipPayload},
    codec::{encode_message, decode_message, CodecFormat},
    node::{NodeInfo, NodeStatus},
    sync::SyncEngine,
};

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;

    fn demo_peer_info(id: &str) -> PeerInfo {
        PeerInfo {
            id: id.into(),
            address: "127.0.0.1:4040".parse().unwrap_or(SocketAddr::from(([127,0,0,1], 4040))),
            status: PeerStatus::Connected,
            last_seen: std::time::Instant::now(),
            latency: Some(std::time::Duration::from_millis(42)),
            protocol_version: "1.0".to_string(),
            client_agent: "OCOS-Client".to_string(),
        }
    }

    #[test]
    fn test_peer_registration_and_ban() {
        let mut pm = PeerManager::new();
        let peer = demo_peer_info("peer1");
        pm.register_peer(peer.clone());
        assert!(pm.get_peer(&"peer1".to_string()).is_some());

        pm.ban_peer(&"peer1".to_string());
        assert!(pm.is_banned(&"peer1".to_string()));
        assert!(pm.get_peer(&"peer1".to_string()).is_none());
    }

    #[test]
    fn test_message_encode_decode_bincode() {
        let msg = NetworkMessage::new(
            MessageType::Ping,
            GossipPayload::Ping,
        );
        let encoded = encode_message(&msg, CodecFormat::Bincode).unwrap();
        let decoded = decode_message(&encoded, CodecFormat::Bincode).unwrap();
        assert_eq!(msg.message_type as u8, decoded.message_type as u8);
    }

    #[test]
    fn test_node_status_and_trust() {
        let mut node = NodeInfo::new("node1".to_string(), "127.0.0.1:5050".to_string(), "1.1".to_string(), "OCOS-Agent".to_string());
        node.set_online();
        assert_eq!(node.status, NodeStatus::Online);

        node.set_offline();
        assert_eq!(node.status, NodeStatus::Offline);

        node.increase_trust(20);
        assert_eq!(node.trust, 70);

        node.decrease_trust(100);
        assert_eq!(node.trust, 0);
    }

    #[test]
    fn test_sync_engine_update() {
        let mut sync = SyncEngine::new();
        sync.start_sync(100);
        assert_eq!(sync.state, crate::network::sync::SyncState::Syncing);

        sync.update_height(50);
        assert_eq!(sync.known_height, 50);
        assert_eq!(sync.state, crate::network::sync::SyncState::Syncing);

        sync.update_height(100);
        assert_eq!(sync.state, crate::network::sync::SyncState::Complete);
    }
}
