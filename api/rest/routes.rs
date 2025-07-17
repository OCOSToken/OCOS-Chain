//! OCOS-Chain: REST API Router
//!
//! Wires up all top-level REST endpoints and mounts submodules for each API namespace.

use crate::api::rest::handlers::{
    ledger::*, governance::*, identity::*, liquidity::*, dao::*, metrics::*,
};
use crate::api::rest::{auth::AuthLayer, error::ApiError};
use axum::{Router, routing::{get, post, put, delete}};

pub fn register_routes() -> Router {
    Router::new()
        // --- Ledger (blockchain data) endpoints ---
        .route("/blocks/:height", get(get_block_by_height))
        .route("/blocks/latest", get(get_latest_block))
        .route("/tx/:hash", get(get_transaction))
        .route("/state/:address", get(get_account_state))

        // --- Governance endpoints ---
        .route("/governance/proposals", get(list_proposals).post(submit_proposal))
        .route("/governance/proposals/:id", get(get_proposal))
        .route("/governance/vote", post(cast_vote))

        // --- Identity endpoints ---
        .route("/identity/:address", get(get_identity))
        .route("/identity/kyc/verify", post(kyc_verify))
        .route("/identity/reputation/:address", get(get_reputation))

        // --- Liquidity/DeFi endpoints ---
        .route("/liquidity/pools", get(list_pools).post(create_pool))
        .route("/liquidity/pools/:id", get(get_pool))
        .route("/liquidity/swap", post(perform_swap))
        .route("/liquidity/farming/stake", post(stake_lp_tokens))
        .route("/liquidity/farming/harvest", post(harvest_rewards))

        // --- DAO endpoints ---
        .route("/dao/list", get(list_daos))
        .route("/dao/:id", get(get_dao))
        .route("/dao/:id/members", get(list_dao_members))

        // --- Metrics/monitoring endpoints ---
        .route("/metrics/gas", get(get_gas_usage))
        .route("/metrics/performance", get(get_performance_stats))

        // --- Health check and version ---
        .route("/health", get(health_check))
        .route("/version", get(api_version))
        
        // --- Auth-protected layer (can wrap protected endpoints) ---
        .layer(AuthLayer::default())
}
