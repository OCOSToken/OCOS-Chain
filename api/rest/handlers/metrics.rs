//! OCOS-Chain: REST API Metrics Handlers
//!
//! Implements HTTP handlers for chain, governance, and system metrics queries.

use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct GasUsageMetrics {
    pub avg_gas_per_tx: f64,
    pub total_gas_24h: u64,
    pub max_gas_block: u64,
    pub block_height: u64,
}

#[derive(Serialize)]
pub struct PerformanceMetrics {
    pub avg_block_time_ms: f64,
    pub txs_per_second: f64,
    pub block_time_24h: f64,
    pub last_block_height: u64,
}

#[derive(Serialize)]
pub struct DaoPerformanceMetrics {
    pub dao_id: u64,
    pub proposals_last_30d: u32,
    pub votes_last_30d: u32,
    pub participation_rate: f64,
    pub pass_rate: f64,
}

// --- Handler: Get gas usage metrics ---
pub async fn get_gas_usage() -> Json<GasUsageMetrics> {
    Json(GasUsageMetrics {
        avg_gas_per_tx: 32000.0,
        total_gas_24h: 21_000_000,
        max_gas_block: 4_000_000,
        block_height: 1_234_567,
    })
}

// --- Handler: Get chain performance metrics ---
pub async fn get_performance_stats() -> Json<PerformanceMetrics> {
    Json(PerformanceMetrics {
        avg_block_time_ms: 1750.0,
        txs_per_second: 34.5,
        block_time_24h: 1800.0,
        last_block_height: 1_234_567,
    })
}

// --- Handler: Get DAO performance metrics ---
pub async fn get_dao_metrics() -> Json<DaoPerformanceMetrics> {
    Json(DaoPerformanceMetrics {
        dao_id: 101,
        proposals_last_30d: 14,
        votes_last_30d: 390,
        participation_rate: 87.2,
        pass_rate: 78.6,
    })
}
