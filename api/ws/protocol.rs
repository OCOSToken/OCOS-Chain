//! OCOS-Chain: WebSocket Protocol Definitions
//!
//! Defines standard protocol messages, opcodes, status codes, and helpers for the WebSocket API.

use serde::{Serialize, Deserialize};
use crate::ws::types::WsTopic;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsProtocolOp {
    /// Client requests to subscribe to a topic (e.g., "blocks", "governance").
    Subscribe { topic: WsTopic },

    /// Client requests to unsubscribe from a topic.
    Unsubscribe { topic: WsTopic },

    /// Server acknowledges a successful subscription/unsubscription.
    Ack { topic: WsTopic },

    /// Client sends a heartbeat to keep the connection alive.
    Ping,

    /// Server responds with Pong.
    Pong,

    /// General application-level error (invalid request, unauthorized, etc.).
    Error { code: u16, message: String },

    /// Initial handshake when connection is established.
    Handshake { protocol_version: String },

    /// Session close (with optional reason).
    Close { reason: Option<String> },

    /// Custom/user-defined operations (extensible).
    Custom { op: String, payload: serde_json::Value },
}

/// Example status codes for WsProtocolOp::Error
pub mod ws_status {
    pub const OK: u16 = 1000;
    pub const INVALID_REQUEST: u16 = 1400;
    pub const UNAUTHORIZED: u16 = 1401;
    pub const FORBIDDEN: u16 = 1403;
    pub const NOT_FOUND: u16 = 1404;
    pub const RATE_LIMITED: u16 = 1429;
    pub const SERVER_ERROR: u16 = 1500;
    pub const BAD_PROTOCOL: u16 = 1501;
}

impl WsProtocolOp {
    /// Utility for building a protocol error message
    pub fn error(code: u16, message: impl Into<String>) -> Self {
        WsProtocolOp::Error { code, message: message.into() }
    }

    /// Utility for handshake (negotiating protocol version, etc.)
    pub fn handshake(version: &str) -> Self {
        WsProtocolOp::Handshake { protocol_version: version.to_string() }
    }
}
