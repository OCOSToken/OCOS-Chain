//! OCOS-Chain: Networking Layer Root Module
//!
//! This module aggregates all submodules related to OCOS-Chain's
//! peer-to-peer communication, transport abstraction, message handling,
//! codec definition, node identity, and sync logic.
//!
//! Designed for high-performance decentralized networks, modular upgrades,
//! auditability, and security.

pub mod p2p;
pub mod transport;
pub mod message;
pub mod codec;
pub mod node;
pub mod sync;

pub use p2p::{PeerId, PeerInfo, PeerStatus, PeerManager};
pub use transport::{Transport, Connection, NetworkAddress};
pub use message::{NetworkMessage, MessageType, GossipPayload};
pub use codec::{encode_message, decode_message};
pub use node::{NodeInfo, NodeId, NodeStatus};
pub use sync::{SyncEngine, SyncState};
