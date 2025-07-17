//! OCOS-Chain: Audit Event Normalization & Decoding Module
//!
//! Provides normalization, decoding, and translation of raw on-chain events/logs
//! into structured, audit-friendly, and analytics-ready event records.

use crate::audit::types::{AuditEvent, AuditEventKind, Timestamp, Address, TraceId};

/// Structured event record decoded from on-chain logs
#[derive(Debug, Clone)]
pub struct EventRecord {
    pub timestamp: Timestamp,
    pub trace_id: Option<TraceId>,
    pub actor: Option<Address>,
    pub kind: AuditEventKind,
    pub details: String,
}

impl EventRecord {
    pub fn new(
        timestamp: Timestamp,
        trace_id: Option<TraceId>,
        actor: Option<Address>,
        kind: AuditEventKind,
        details: String,
    ) -> Self {
        EventRecord {
            timestamp,
            trace_id,
            actor,
            kind,
            details,
        }
    }
}

/// Event decoder: from raw blockchain log data to EventRecord
pub struct EventDecoder;

impl EventDecoder {
    /// Decodes a raw on-chain log or protocol event (as bytes or JSON) into EventRecord
    pub fn decode(raw_data: &[u8]) -> Option<EventRecord> {
        // Placeholder: Actual implementation would parse JSON, ABI, or log topic mapping.
        // For now, simulate decoding for demo/test:
        if raw_data.is_empty() {
            return None;
        }
        // Example: If data is valid, construct a sample event.
        Some(EventRecord {
            timestamp: 0,
            trace_id: None,
            actor: None,
            kind: AuditEventKind::Custom("demo".to_string()),
            details: "Decoded event".to_string(),
        })
    }

    /// Normalizes events from multiple contract standards to unified AuditEventKind
    pub fn normalize(event: &EventRecord) -> AuditEvent {
        AuditEvent {
            timestamp: event.timestamp,
            trace_id: event.trace_id,
            actor: event.actor,
            kind: event.kind.clone(),
            details: event.details.clone(),
        }
    }
}
