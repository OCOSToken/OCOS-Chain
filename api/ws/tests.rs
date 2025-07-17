//! OCOS-Chain: WebSocket API – Handler & Router Tests
//!
//! Covers subscription logic, message routing, handler invocation, and protocol compliance.

use crate::ws::router::{WsRouter, ClientSession};
use crate::ws::types::{ClientId, WsRequest, WsResponse, WsTopic, WsMessage, BlockInfo};

fn setup_router() -> WsRouter {
    let mut router = WsRouter::new();
    router.register_client(1);
    router.register_client(2);
    router
}

#[test]
fn test_subscription_and_broadcast() {
    let mut router = setup_router();

    // Client 1 subscribes to "Blocks"
    let resp = router.handle_message(1, WsRequest::Subscribe { topic: WsTopic::Blocks }, &mut Default::default());
    assert!(matches!(resp, WsResponse::Ack { topic } if topic == WsTopic::Blocks));
    assert!(router.clients.get(&1).unwrap().subscriptions.contains(&WsTopic::Blocks));

    // Client 2 subscribes to "Blocks"
    router.handle_message(2, WsRequest::Subscribe { topic: WsTopic::Blocks }, &mut Default::default());
    assert!(router.clients.get(&2).unwrap().subscriptions.contains(&WsTopic::Blocks));

    // Simulate a block event
    let block = BlockInfo {
        number: 1001,
        hash: "0xabc".to_string(),
        timestamp: 1234567890,
        tx_count: 7,
        producer: "validator1".to_string(),
    };
    let msg = WsMessage::NewBlock { block: block.clone() };

    // We don't have a real ws_server, so we just ensure router.broadcast doesn't panic
    router.broadcast(&WsTopic::Blocks, &msg);
}

#[test]
fn test_unsubscribe() {
    let mut router = setup_router();
    router.handle_message(1, WsRequest::Subscribe { topic: WsTopic::Dao }, &mut Default::default());
    assert!(router.clients.get(&1).unwrap().subscriptions.contains(&WsTopic::Dao));

    // Unsubscribe
    let resp = router.handle_message(1, WsRequest::Unsubscribe { topic: WsTopic::Dao }, &mut Default::default());
    assert!(matches!(resp, WsResponse::Ack { topic } if topic == WsTopic::Dao));
    assert!(!router.clients.get(&1).unwrap().subscriptions.contains(&WsTopic::Dao));
}

#[test]
fn test_middleware_short_circuit() {
    use crate::ws::middleware::{MiddlewareChain, WsContext, WsMiddleware};

    struct TestRejectAll;
    impl WsMiddleware for TestRejectAll {
        fn handle(&self, _client_id: ClientId, _req: &WsRequest, _ctx: &mut WsContext) -> Option<WsResponse> {
            Some(WsResponse::Error("Rejected".into()))
        }
    }

    let mut router = WsRouter::new();
    router.register_client(1);
    router.middleware = MiddlewareChain::default().add(TestRejectAll);

    let resp = router.handle_message(1, WsRequest::Ping, &mut Default::default());
    assert!(matches!(resp, WsResponse::Error(reason) if reason == "Rejected"));
}

#[test]
fn test_invalid_request() {
    let mut router = WsRouter::new();
    router.register_client(1);

    // Send an unknown request (simulate error)
    let resp = router.handle_message(1, WsRequest::Custom { payload: serde_json::json!({"foo":"bar"}) }, &mut Default::default());
    assert!(matches!(resp, WsResponse::Error(_)));
}
