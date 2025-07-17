//! OCOS-Chain: REST API Version and Health Endpoint
//!
//! Provides API version metadata, chain info, commit/build references, and health status.

use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiVersion {
    pub api_version: String,
    pub chain_id: String,
    pub commit_hash: String,
    pub build_date: String,
}

// --- Handler: Get current API version and metadata ---
pub async fn api_version() -> Json<ApiVersion> {
    Json(ApiVersion {
        api_version: "v1.0.0".to_string(),
        chain_id: "ocos-mainnet-001".to_string(),
        commit_hash: "ab12cdef9876".to_string(),
        build_date: "2025-07-18T12:00:00Z".to_string(),
    })
}

// --- Handler: Health check endpoint (simple version) ---
#[derive(Serialize)]
pub struct HealthCheck {
    pub status: String,
    pub uptime_sec: u64,
}

pub async fn health_check() -> Json<HealthCheck> {
    // In real code, calculate uptime dynamically.
    Json(HealthCheck {
        status: "ok".to_string(),
        uptime_sec: 1_234_567,
    })
}
