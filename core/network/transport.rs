//! OCOS-Chain: Transport Layer Abstraction
//!
//! Defines the interface for underlying network transport (e.g., TCP, WebSocket).
//! Provides unified connection handling, secure I/O, and async read/write operations.

use std::net::SocketAddr;
use async_trait::async_trait;

/// Unified transport address
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NetworkAddress(pub String);

/// Abstract connection object for any transport (TCP, TLS, WebSocket, etc.)
#[async_trait]
pub trait Connection: Send + Sync {
    async fn send(&mut self, data: Vec<u8>) -> Result<(), TransportError>;
    async fn receive(&mut self) -> Result<Vec<u8>, TransportError>;
    fn remote_addr(&self) -> NetworkAddress;
}

/// Transport protocol abstraction layer
#[async_trait]
pub trait Transport: Send + Sync {
    async fn connect(&self, addr: &NetworkAddress) -> Result<Box<dyn Connection>, TransportError>;
    async fn listen(&self, bind_addr: &NetworkAddress) -> Result<(), TransportError>;
}

/// Transport-specific error types
#[derive(Debug)]
pub enum TransportError {
    ConnectionFailed(String),
    SendFailed(String),
    ReceiveFailed(String),
    UnsupportedProtocol,
}
