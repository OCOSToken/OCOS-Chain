//! OCOS-Chain: Audit Metrics & Analytics Module
//!
//! Collects and exposes chain, VM, DAO, and protocol performance metrics for
//! analytics, optimization, compliance, and governance reporting.

use crate::audit::types::{Address, Timestamp, ActionType, TraceId};

#[derive(Debug, Clone, Default)]
pub struct MetricSnapshot {
    pub timestamp: Timestamp,
    pub block_number: u64,
    pub gas_used: u64,
    pub tx_count: u32,
    pub avg_gas_per_tx: f64,
    pub avg_block_time: f64,
    pub dao_participation: f64,  // percent of eligible DAO voters
    pub governance_proposals: u32,
    pub avg_vote_turnout: f64,
    pub failed_tx_count: u32,
    pub avg_vm_exec_time_ms: f64,
    pub risk_score: f64,         // calculated/aggregated by risk model
}

#[derive(Debug, Default)]
pub struct MetricsRegistry {
    pub snapshots: Vec<MetricSnapshot>,
}

impl MetricsRegistry {
    pub fn record_snapshot(&mut self, snapshot: MetricSnapshot) {
        self.snapshots.push(snapshot);
    }

    pub fn latest(&self) -> Option<&MetricSnapshot> {
        self.snapshots.last()
    }

    /// Calculate average metric values over all snapshots
    pub fn averages(&self) -> Option<MetricSnapshot> {
        let n = self.snapshots.len() as f64;
        if n == 0.0 {
            return None;
        }
        let mut total = MetricSnapshot::default();
        for snap in &self.snapshots {
            total.gas_used += snap.gas_used;
            total.tx_count += snap.tx_count;
            total.avg_gas_per_tx += snap.avg_gas_per_tx;
            total.avg_block_time += snap.avg_block_time;
            total.dao_participation += snap.dao_participation;
            total.governance_proposals += snap.governance_proposals;
            total.avg_vote_turnout += snap.avg_vote_turnout;
            total.failed_tx_count += snap.failed_tx_count;
            total.avg_vm_exec_time_ms += snap.avg_vm_exec_time_ms;
            total.risk_score += snap.risk_score;
        }
        Some(MetricSnapshot {
            timestamp: 0,
            block_number: 0,
            gas_used: (total.gas_used as f64 / n) as u64,
            tx_count: (total.tx_count as f64 / n) as u32,
            avg_gas_per_tx: total.avg_gas_per_tx / n,
            avg_block_time: total.avg_block_time / n,
            dao_participation: total.dao_participation / n,
            governance_proposals: (total.governance_proposals as f64 / n) as u32,
            avg_vote_turnout: total.avg_vote_turnout / n,
            failed_tx_count: (total.failed_tx_count as f64 / n) as u32,
            avg_vm_exec_time_ms: total.avg_vm_exec_time_ms / n,
            risk_score: total.risk_score / n,
        })
    }
}
