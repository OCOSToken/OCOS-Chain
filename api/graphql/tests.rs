//! OCOS-Chain: GraphQL API Integration & Unit Tests
//!
//! Tests schema queries, mutations, subscriptions, error formatting, and extension/middleware integration.

use async_graphql::{Schema, Request, Value};
use crate::api::graphql::{
    schema::{build_schema, QueryRoot, MutationRoot, SubscriptionRoot},
    context::GraphQLContext,
    types::*,
};

fn setup_schema() -> Schema<QueryRoot, MutationRoot, SubscriptionRoot> {
    build_schema()
}

#[tokio::test]
async fn test_block_query() {
    let schema = setup_schema();
    let query = r#"{ block(id: "0xabc") { number hash proposer } }"#;
    let resp = schema.execute(Request::new(query)).await;
    assert!(resp.is_ok(), "Block query failed");
    let data = resp.data.into_json().unwrap();
    assert!(data["block"]["number"].as_u64().unwrap() > 0);
}

#[tokio::test]
async fn test_proposal_query_and_vote() {
    let schema = setup_schema();
    let proposal_query = r#"{ proposal(id: 1) { id status description } }"#;
    let resp = schema.execute(Request::new(proposal_query)).await;
    let data = resp.data.into_json().unwrap();
    assert_eq!(data["proposal"]["id"].as_u64().unwrap(), 1);

    let vote_mutation = r#"
        mutation {
            vote(input: { voter: "0x1", proposalId: 1, approve: true, amount: 500 }) {
                voter proposalId approve amount
            }
        }
    "#;
    let resp = schema.execute(Request::new(vote_mutation)).await;
    let data = resp.data.into_json().unwrap();
    assert_eq!(data["vote"]["approve"].as_bool().unwrap(), true);
}

#[tokio::test]
async fn test_identity_profile_query() {
    let schema = setup_schema();
    let profile_query = r#"{ identityProfile(address: "0xDIDowner") { address did username kycStatus } }"#;
    let resp = schema.execute(Request::new(profile_query)).await;
    let data = resp.data.into_json().unwrap();
    assert_eq!(data["identityProfile"]["address"].as_str().unwrap(), "0xDIDowner");
    assert_eq!(data["identityProfile"]["kycStatus"].as_str().unwrap(), "VERIFIED");
}

#[tokio::test]
async fn test_liquidity_pool_query_and_swap() {
    let schema = setup_schema();
    let pool_query = r#"{ pool(id: 1) { id tokenA { symbol } tokenB { symbol } reserveA reserveB } }"#;
    let resp = schema.execute(Request::new(pool_query)).await;
    let data = resp.data.into_json().unwrap();
    assert_eq!(data["pool"]["id"].as_u64().unwrap(), 1);

    let swap_mutation = r#"
        mutation {
            swap(input: { poolId: 1, fromToken: 1, toToken: 2, amountIn: 1000, minOut: 950, recipient: "0xuser" }) {
                poolId amountIn amountOut feePaid txHash
            }
        }
    "#;
    let resp = schema.execute(Request::new(swap_mutation)).await;
    let data = resp.data.into_json().unwrap();
    assert!(data["swap"]["amountOut"].as_u64().unwrap() > 0);
}

#[tokio::test]
async fn test_error_handling_for_invalid_proposal() {
    let schema = setup_schema();
    let invalid_query = r#"{ proposal(id: 9999) { id status } }"#;
    let resp = schema.execute(Request::new(invalid_query)).await;
    assert!(resp.errors.len() > 0, "Expected error for invalid proposal");
    let err = &resp.errors[0];
    assert!(err.message.contains("not found") || err.message.contains("NotFound"));
}

#[tokio::test]
async fn test_subscription_block_stream() {
    // For async-graphql: test subscriptions with WebSocket or test helpers
    // Here, just a sketch - in real test, use a test server/client
    // let schema = setup_schema();
    // let mut stream = schema.execute_stream(Request::new("subscription { blockStream { number } }"));
    // assert!(stream.next().await.is_some());
}
