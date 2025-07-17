//! OCOS-Chain: WebSocket Protocol Extensions
//!
//! Provides optional extensions to the WebSocket API for tracing, analytics,
//! caching, replay, introspection, and developer tooling.

use crate::ws::types::{ClientId, WsMessage};
use std::collections::HashMap;

/// Extension trait for implementing custom WebSocket protocol features
pub trait WsExtension: Send + Sync {
    /// Optionally intercept, augment, or process outgoing messages.
    fn on_send(&self, client_id: ClientId, msg: &WsMessage);
    /// Optionally intercept, augment, or process incoming messages.
    fn on_receive(&self, client_id: ClientId, msg: &WsMessage);
}

/// Extension manager that holds all active extensions
#[derive(Default)]
pub struct ExtensionManager {
    pub extensions: Vec<Box<dyn WsExtension>>,
}

impl ExtensionManager {
    /// Call all extensions on send
    pub fn on_send(&self, client_id: ClientId, msg: &WsMessage) {
        for ext in &self.extensions {
            ext.on_send(client_id, msg);
        }
    }
    /// Call all extensions on receive
    pub fn on_receive(&self, client_id: ClientId, msg: &WsMessage) {
        for ext in &self.extensions {
            ext.on_receive(client_id, msg);
        }
    }
    /// Add an extension (chained builder)
    pub fn add<E: WsExtension + 'static>(mut self, ext: E) -> Self {
        self.extensions.push(Box::new(ext));
        self
    }
}

// --- Example Extensions ---

/// Tracing extension (Apollo-compatible) for measuring latency, message flow, etc.
pub struct TracingExtension;
impl WsExtension for TracingExtension {
    fn on_send(&self, client_id: ClientId, msg: &WsMessage) {
        // In production: record send timestamp, message type, and metrics
        println!("[WS][TRACE] send client={} type={:?}", client_id, msg);
    }
    fn on_receive(&self, client_id: ClientId, msg: &WsMessage) {
        // Record receive timestamp and processing metrics
        println!("[WS][TRACE] recv client={} type={:?}", client_id, msg);
    }
}

/// Simple replay extension for last-N events per topic
pub struct ReplayExtension {
    pub buffer: HashMap<String, Vec<WsMessage>>, // topic → recent messages
    pub capacity: usize,
}
impl ReplayExtension {
    pub fn new(capacity: usize) -> Self {
        Self { buffer: HashMap::new(), capacity }
    }
    /// Store a message in the replay buffer for a topic
    pub fn store(&mut self, topic: &str, msg: WsMessage) {
        let buf = self.buffer.entry(topic.to_string()).or_default();
        buf.push(msg);
        if buf.len() > self.capacity {
            buf.remove(0);
        }
    }
}
impl WsExtension for ReplayExtension {
    fn on_send(&self, _client_id: ClientId, _msg: &WsMessage) {
        // No-op for on_send (for demo; integrate store() as needed)
    }
    fn on_receive(&self, _client_id: ClientId, _msg: &WsMessage) {
        // No-op for on_receive
    }
}

// Usage in WebSocket core:
// let extensions = ExtensionManager::default()
//     .add(TracingExtension)
//     .add(ReplayExtension::new(100));
