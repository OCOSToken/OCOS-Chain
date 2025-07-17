//! OCOS-Chain: REST API Integration & Endpoint Tests
//!
//! Covers key REST endpoints for ledger, governance, identity, liquidity, DAO, and metrics.

use axum::http::{Request, StatusCode};
use axum::body::Body;
use axum::Router;
use serde_json::json;
use tower::ServiceExt;

use crate::api::rest::{register_routes, types::*};

#[tokio::test]
async fn test_get_latest_block() {
    let app = register_routes();
    let resp = app
        .oneshot(Request::builder().uri("/blocks/latest").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = hyper::body::to_bytes(resp.into_body()).await.unwrap();
    let block: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(block.get("height").is_some());
}

#[tokio::test]
async fn test_list_proposals() {
    let app = register_routes();
    let resp = app
        .oneshot(Request::builder().uri("/governance/proposals").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = hyper::body::to_bytes(resp.into_body()).await.unwrap();
    let proposals: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(proposals.is_array());
}

#[tokio::test]
async fn test_liquidity_swap() {
    let app = register_routes();
    let swap_req = json!({
        "pool_id": 1,
        "amount_in": "100",
        "direction": "AtoB",
        "min_amount_out": "0.47"
    });
    let resp = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/liquidity/swap")
                .header("content-type", "application/json")
                .body(Body::from(swap_req.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = hyper::body::to_bytes(resp.into_body()).await.unwrap();
    let swap: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(swap.get("amount_out").is_some());
}

#[tokio::test]
async fn test_api_version() {
    let app = register_routes();
    let resp = app
        .oneshot(Request::builder().uri("/version").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = hyper::body::to_bytes(resp.into_body()).await.unwrap();
    let version: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(version.get("api_version").unwrap(), "v1.0.0");
}
