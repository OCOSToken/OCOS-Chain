//! OCOS-Chain: Audit Trace & Forensics Module
//!
//! Provides full transaction, VM execution, storage, governance, and event traces
//! for audit, analytics, security, and forensic investigations.

use crate::audit::types::{AuditRecord, ActionType, TraceLevel, TraceId, Timestamp, Address};
use std::collections::HashMap;

/// One full trace of a transaction, contract execution, or governance action
#[derive(Debug, Clone)]
pub struct AuditTrace {
    pub trace_id: TraceId,
    pub initiator: Address,
    pub timestamp: Timestamp,
    pub level: TraceLevel,
    pub actions: Vec<AuditRecord>,
    pub parent_trace: Option<TraceId>, // For nested/related traces
}

impl AuditTrace {
    pub fn new(
        trace_id: TraceId,
        initiator: Address,
        timestamp: Timestamp,
        level: TraceLevel,
        parent_trace: Option<TraceId>,
    ) -> Self {
        Self {
            trace_id,
            initiator,
            timestamp,
            level,
            actions: vec![],
            parent_trace,
        }
    }

    /// Record an action into this trace
    pub fn record_action(&mut self, action: AuditRecord) {
        self.actions.push(action);
    }

    /// Find all actions of a given type in the trace
    pub fn filter_actions(&self, action_type: ActionType) -> Vec<&AuditRecord> {
        self.actions.iter().filter(|a| a.action_type == action_type).collect()
    }
}

/// Audit trace registry for all traces on-chain (for demo: hashmap, real: DB/storage)
#[derive(Default)]
pub struct TraceRegistry {
    pub traces: HashMap<TraceId, AuditTrace>,
}

impl TraceRegistry {
    pub fn add_trace(&mut self, trace: AuditTrace) {
        self.traces.insert(trace.trace_id, trace);
    }

    pub fn get_trace(&self, trace_id: TraceId) -> Option<&AuditTrace> {
        self.traces.get(&trace_id)
    }

    pub fn filter_by_initiator(&self, initiator: &Address) -> Vec<&AuditTrace> {
        self.traces.values().filter(|t| &t.initiator == initiator).collect()
    }

    pub fn filter_by_level(&self, level: TraceLevel) -> Vec<&AuditTrace> {
        self.traces.values().filter(|t| t.level == level).collect()
    }
}
