//! OCOS-Chain: Audit Index & Query Module
//!
//! Provides indexing and query capabilities for audit traces, logs, events, and compliance records.
//! Designed for analytics, monitoring, and integration with external tools or dashboards.

use crate::audit::trace::{AuditTrace, TraceRegistry};
use crate::audit::log::AuditLog;
use crate::audit::event::EventRecord;
use crate::audit::compliance::ComplianceCheck;
use crate::audit::types::{TraceId, ActionType, LogLevel, AuditEventKind, Address, Timestamp};

#[derive(Default)]
pub struct AuditIndex {
    pub trace_index: Vec<TraceId>,
    pub action_index: Vec<ActionType>,
    pub log_level_index: Vec<LogLevel>,
    pub event_kind_index: Vec<AuditEventKind>,
    pub actor_index: Vec<Address>,
    pub timestamp_index: Vec<Timestamp>,
}

pub struct AuditIndexer<'a> {
    pub trace_registry: &'a TraceRegistry,
    pub audit_log: &'a AuditLog,
    pub events: &'a [EventRecord],
    pub compliance_checks: &'a [ComplianceCheck],
}

impl<'a> AuditIndexer<'a> {
    /// Query all traces by action type
    pub fn traces_by_action(&self, action: ActionType) -> Vec<&AuditTrace> {
        self.trace_registry
            .traces
            .values()
            .filter(|trace| trace.actions.iter().any(|a| a.action_type == action))
            .collect()
    }

    /// Query all logs by log level
    pub fn logs_by_level(&self, level: LogLevel) -> Vec<&crate::audit::types::AuditLogEntry> {
        self.audit_log.by_level(level)
    }

    /// Query all events by event kind
    pub fn events_by_kind(&self, kind: AuditEventKind) -> Vec<&EventRecord> {
        self.events.iter().filter(|ev| ev.kind == kind).collect()
    }

    /// Query all compliance checks by actor
    pub fn compliance_by_actor(&self, actor: &Address) -> Vec<&ComplianceCheck> {
        self.compliance_checks.iter().filter(|cc| &cc.actor == actor).collect()
    }

    /// Query all actions/events/logs by timestamp range
    pub fn by_time_range(&self, from: Timestamp, to: Timestamp) -> AuditIndex {
        let trace_index = self
            .trace_registry
            .traces
            .values()
            .filter(|trace| trace.timestamp >= from && trace.timestamp <= to)
            .map(|t| t.trace_id)
            .collect();

        let log_level_index = self
            .audit_log
            .entries
            .iter()
            .filter(|log| log.timestamp >= from && log.timestamp <= to)
            .map(|log| log.level)
            .collect();

        let event_kind_index = self
            .events
            .iter()
            .filter(|ev| ev.timestamp >= from && ev.timestamp <= to)
            .map(|ev| ev.kind.clone())
            .collect();

        let actor_index = self
            .audit_log
            .entries
            .iter()
            .filter(|log| log.timestamp >= from && log.timestamp <= to)
            .map(|log| log.actor)
            .collect();

        let action_index = self
            .audit_log
            .entries
            .iter()
            .filter(|log| log.timestamp >= from && log.timestamp <= to)
            .map(|log| log.action_type)
            .collect();

        let timestamp_index = self
            .audit_log
            .entries
            .iter()
            .filter(|log| log.timestamp >= from && log.timestamp <= to)
            .map(|log| log.timestamp)
            .collect();

        AuditIndex {
            trace_index,
            action_index,
            log_level_index,
            event_kind_index,
            actor_index,
            timestamp_index,
        }
    }
}
