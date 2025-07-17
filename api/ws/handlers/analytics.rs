//! OCOS-Chain: WebSocket Handler – Analytics & Metrics Events
//!
//! Streams real-time protocol analytics, performance metrics, chain health, and DAO KPIs.

use crate::ws::types::{WsTopic, WsMessage, AnalyticsMetric, HealthStatus, DaoKpi};
use crate::ws::router::WsRouter;

/// Handles real-time streaming of analytics, KPIs, and health status events.
pub struct AnalyticsHandler;

impl AnalyticsHandler {
    /// Broadcast general protocol or chain-wide analytics metrics
    pub fn on_metric(router: &WsRouter, metric: AnalyticsMetric) {
        let msg = WsMessage::AnalyticsMetric { metric: metric.clone() };
        router.broadcast(&WsTopic::Analytics, &msg);
    }

    /// Broadcast real-time protocol or node health status
    pub fn on_health_status(router: &WsRouter, status: HealthStatus) {
        let msg = WsMessage::HealthStatus { status: status.clone() };
        router.broadcast(&WsTopic::Analytics, &msg);
    }

    /// Broadcast DAO or governance-level KPI updates (e.g. voter turnout, treasury, etc.)
    pub fn on_dao_kpi(router: &WsRouter, kpi: DaoKpi) {
        let msg = WsMessage::DaoKpi { kpi: kpi.clone() };
        router.broadcast(&WsTopic::Analytics, &msg);
    }

    /// (Optional) Broadcast custom analytics events (user-defined or plugin metrics)
    pub fn on_custom_event(router: &WsRouter, payload: serde_json::Value) {
        let msg = WsMessage::AnalyticsCustomEvent { payload };
        router.broadcast(&WsTopic::Analytics, &msg);
    }
}

// -- Example message types for reference --

/*
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsMessage {
    AnalyticsMetric { metric: AnalyticsMetric },
    HealthStatus { status: HealthStatus },
    DaoKpi { kpi: DaoKpi },
    AnalyticsCustomEvent { payload: serde_json::Value },
    // ... more
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsMetric {
    pub name: String,
    pub value: f64,
    pub unit: Option<String>,
    pub tags: Option<Vec<String>>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub component: String,
    pub status: String, // e.g., "healthy", "degraded", "critical"
    pub details: Option<serde_json::Value>,
    pub checked_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaoKpi {
    pub dao_id: u64,
    pub metric: String,
    pub value: f64,
    pub updated_at: u64,
}
*/
