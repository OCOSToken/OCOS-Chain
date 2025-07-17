//! OCOS-Chain: Structured Audit Log Module
//!
//! Provides structured access and action logging for all chain, governance, identity,
//! and VM events. Ensures every critical action is tracked with metadata for audit/compliance.

use crate::audit::types::{AuditLogEntry, LogLevel, ActionType, Timestamp, Address, TraceId};
use std::collections::VecDeque;

/// Structured log record for any audited event
#[derive(Debug, Clone)]
pub struct AuditLog {
    pub entries: VecDeque<AuditLogEntry>,
    pub capacity: usize,
}

impl AuditLog {
    pub fn new(capacity: usize) -> Self {
        AuditLog {
            entries: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    /// Record a new audit log entry
    pub fn record(
        &mut self,
        timestamp: Timestamp,
        trace_id: TraceId,
        actor: Address,
        action_type: ActionType,
        level: LogLevel,
        details: String,
    ) {
        let entry = AuditLogEntry {
            timestamp,
            trace_id,
            actor,
            action_type,
            level,
            details,
        };
        if self.entries.len() >= self.capacity {
            self.entries.pop_front(); // oldest removed
        }
        self.entries.push_back(entry);
    }

    /// Query logs by actor (user or contract address)
    pub fn by_actor(&self, actor: &Address) -> Vec<&AuditLogEntry> {
        self.entries.iter().filter(|e| &e.actor == actor).collect()
    }

    /// Query logs by action type
    pub fn by_action(&self, action_type: ActionType) -> Vec<&AuditLogEntry> {
        self.entries.iter().filter(|e| e.action_type == action_type).collect()
    }

    /// Query logs by log level (INFO, WARN, ERROR, etc.)
    pub fn by_level(&self, level: LogLevel) -> Vec<&AuditLogEntry> {
        self.entries.iter().filter(|e| e.level == level).collect()
    }
}
