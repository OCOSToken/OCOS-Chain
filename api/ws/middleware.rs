//! OCOS-Chain: WebSocket Middleware Pipeline
//!
//! Provides pluggable middleware layers for authentication, logging, CORS, rate-limiting, etc.

use crate::ws::types::{ClientId, WsRequest, WsResponse};
use std::collections::HashMap;

/// Context passed to each middleware (holds connection/session state, metadata, etc.)
#[derive(Default)]
pub struct WsContext {
    pub client_ip: Option<String>,
    pub session_data: HashMap<String, String>,
    // More fields: auth status, tracing info, rate-limit counters, etc.
}

/// The trait every middleware must implement
pub trait WsMiddleware: Send + Sync {
    /// Inspect/transform requests. Return Some(response) to halt, or None to continue.
    fn handle(&self, client_id: ClientId, req: &WsRequest, ctx: &mut WsContext) -> Option<WsResponse>;
}

/// A pipeline of middleware layers, executed in order
#[derive(Default)]
pub struct MiddlewareChain {
    pub layers: Vec<Box<dyn WsMiddleware>>,
}

impl MiddlewareChain {
    /// Run all middleware layers for a request. If any returns Some, short-circuit.
    pub fn handle(&self, client_id: ClientId, req: &WsRequest, ctx: &mut WsContext) -> Option<WsResponse> {
        for layer in &self.layers {
            if let Some(resp) = layer.handle(client_id, req, ctx) {
                return Some(resp);
            }
        }
        None
    }

    /// Add a middleware layer (chained builder pattern)
    pub fn add<L: WsMiddleware + 'static>(mut self, layer: L) -> Self {
        self.layers.push(Box::new(layer));
        self
    }
}

// --- Example middleware implementations ---

/// Simple authentication middleware
pub struct AuthMiddleware;
impl WsMiddleware for AuthMiddleware {
    fn handle(&self, _client_id: ClientId, req: &WsRequest, ctx: &mut WsContext) -> Option<WsResponse> {
        // Example: require authentication for all non-Ping requests
        if let WsRequest::Ping = req {
            return None;
        }
        if ctx.session_data.get("authed") == Some(&"true".to_string()) {
            None
        } else {
            Some(WsResponse::Error("Unauthorized".into()))
        }
    }
}

/// Simple logging middleware
pub struct LoggingMiddleware;
impl WsMiddleware for LoggingMiddleware {
    fn handle(&self, client_id: ClientId, req: &WsRequest, _ctx: &mut WsContext) -> Option<WsResponse> {
        println!("[WS] client={} request={:?}", client_id, req);
        None
    }
}

/// Simple rate-limiting middleware (placeholder)
pub struct RateLimitMiddleware;
impl WsMiddleware for RateLimitMiddleware {
    fn handle(&self, _client_id: ClientId, req: &WsRequest, ctx: &mut WsContext) -> Option<WsResponse> {
        let count = ctx.session_data.entry("rate".to_string()).or_insert("0".to_string());
        let n = count.parse::<u32>().unwrap_or(0) + 1;
        *count = n.to_string();
        if n > 100 {
            Some(WsResponse::Error("Rate limited".into()))
        } else {
            None
        }
    }
}

// Usage in WsRouter:
// let middleware = MiddlewareChain::default()
//     .add(LoggingMiddleware)
//     .add(AuthMiddleware)
//     .add(RateLimitMiddleware);
