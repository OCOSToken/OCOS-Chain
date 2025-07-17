//! OCOS-Chain: WebSocket Router
//!
//! Handles subscription management, topic-based event routing, and dispatches
//! real-time messages to connected WebSocket clients.

use crate::ws::types::{WsMessage, WsRequest, WsResponse, WsTopic, ClientId};
use crate::ws::handlers;
use crate::ws::auth::AuthGuard;
use crate::ws::middleware::{MiddlewareChain, WsContext};
use std::collections::{HashMap, HashSet};

/// State tracking for all connected clients and their topic subscriptions
pub struct WsRouter {
    pub clients: HashMap<ClientId, ClientSession>,
    pub topic_subs: HashMap<WsTopic, HashSet<ClientId>>,
    pub middleware: MiddlewareChain,
}

#[derive(Debug)]
pub struct ClientSession {
    pub id: ClientId,
    pub authed: bool,
    pub subscriptions: HashSet<WsTopic>,
    // More fields: last ping, user metadata, etc.
}

impl WsRouter {
    pub fn new() -> Self {
        WsRouter {
            clients: HashMap::new(),
            topic_subs: HashMap::new(),
            middleware: MiddlewareChain::default(),
        }
    }

    /// Register a new client connection
    pub fn register_client(&mut self, client_id: ClientId) {
        self.clients.insert(client_id, ClientSession {
            id: client_id,
            authed: false,
            subscriptions: HashSet::new(),
        });
    }

    /// Handle incoming WebSocket messages (subscribe, unsubscribe, ping, etc.)
    pub fn handle_message(&mut self, client_id: ClientId, msg: WsRequest, ctx: &mut WsContext) -> WsResponse {
        // Optionally run middleware (auth, logging, rate-limit, etc.)
        if let Some(resp) = self.middleware.handle(client_id, &msg, ctx) {
            return resp;
        }

        match msg {
            WsRequest::Subscribe { topic } => self.subscribe(client_id, topic),
            WsRequest::Unsubscribe { topic } => self.unsubscribe(client_id, topic),
            WsRequest::Ping => WsResponse::Pong,
            // ... other protocol methods
            _ => WsResponse::Error("Unknown request".into()),
        }
    }

    /// Subscribe a client to a topic (e.g., "blocks", "governance", "dao", etc.)
    pub fn subscribe(&mut self, client_id: ClientId, topic: WsTopic) -> WsResponse {
        self.topic_subs.entry(topic.clone()).or_default().insert(client_id);
        self.clients.get_mut(&client_id).map(|sess| {
            sess.subscriptions.insert(topic.clone());
        });
        WsResponse::Ack { topic }
    }

    /// Unsubscribe a client from a topic
    pub fn unsubscribe(&mut self, client_id: ClientId, topic: WsTopic) -> WsResponse {
        if let Some(subs) = self.topic_subs.get_mut(&topic) {
            subs.remove(&client_id);
        }
        self.clients.get_mut(&client_id).map(|sess| {
            sess.subscriptions.remove(&topic);
        });
        WsResponse::Ack { topic }
    }

    /// Broadcast an event to all clients subscribed to the topic
    pub fn broadcast(&self, topic: &WsTopic, msg: &WsMessage) {
        if let Some(subscribers) = self.topic_subs.get(topic) {
            for client_id in subscribers {
                self.send_to_client(client_id, msg);
            }
        }
    }

    /// Send a message to a single client (stub for integration with actual WS server)
    fn send_to_client(&self, client_id: &ClientId, msg: &WsMessage) {
        // Implementation: push to WS server, event queue, or channel
        // For production, integrate with async runtime, task manager, etc.
        // Example (pseudo-code): ws_server.send(client_id, msg)
    }
}
