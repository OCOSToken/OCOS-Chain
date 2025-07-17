//! OCOS-Chain: Audit Storage & Retrieval Module
//!
//! Persistent storage and retrieval for all audit logs, traces, events, compliance checks,
//! and proofs. Designed for full-chain, off-chain, and hybrid audit infrastructure.

use std::collections::HashMap;
use crate::audit::types::{
    TraceId, AuditLogEntry, AuditEvent, ComplianceCheck, MerkleProof, CommitmentProof, ZkProof,
};

#[derive(Default)]
pub struct AuditStorage {
    // TraceId → Log Entries
    pub logs: HashMap<TraceId, Vec<AuditLogEntry>>,
    // TraceId → Audit Events
    pub events: HashMap<TraceId, Vec<AuditEvent>>,
    // TraceId → Compliance Checks
    pub compliance: HashMap<TraceId, Vec<ComplianceCheck>>,
    // TraceId → Merkle Proofs
    pub merkle_proofs: HashMap<TraceId, MerkleProof>,
    // TraceId → Commitment Proofs
    pub commitment_proofs: HashMap<TraceId, CommitmentProof>,
    // TraceId → Zk Proofs
    pub zk_proofs: HashMap<TraceId, ZkProof>,
}

impl AuditStorage {
    pub fn add_log(&mut self, trace_id: TraceId, entry: AuditLogEntry) {
        self.logs.entry(trace_id).or_default().push(entry);
    }
    pub fn get_logs(&self, trace_id: TraceId) -> Option<&Vec<AuditLogEntry>> {
        self.logs.get(&trace_id)
    }

    pub fn add_event(&mut self, trace_id: TraceId, event: AuditEvent) {
        self.events.entry(trace_id).or_default().push(event);
    }
    pub fn get_events(&self, trace_id: TraceId) -> Option<&Vec<AuditEvent>> {
        self.events.get(&trace_id)
    }

    pub fn add_compliance(&mut self, trace_id: TraceId, check: ComplianceCheck) {
        self.compliance.entry(trace_id).or_default().push(check);
    }
    pub fn get_compliance(&self, trace_id: TraceId) -> Option<&Vec<ComplianceCheck>> {
        self.compliance.get(&trace_id)
    }

    pub fn add_merkle_proof(&mut self, trace_id: TraceId, proof: MerkleProof) {
        self.merkle_proofs.insert(trace_id, proof);
    }
    pub fn get_merkle_proof(&self, trace_id: TraceId) -> Option<&MerkleProof> {
        self.merkle_proofs.get(&trace_id)
    }

    pub fn add_commitment_proof(&mut self, trace_id: TraceId, proof: CommitmentProof) {
        self.commitment_proofs.insert(trace_id, proof);
    }
    pub fn get_commitment_proof(&self, trace_id: TraceId) -> Option<&CommitmentProof> {
        self.commitment_proofs.get(&trace_id)
    }

    pub fn add_zk_proof(&mut self, trace_id: TraceId, proof: ZkProof) {
        self.zk_proofs.insert(trace_id, proof);
    }
    pub fn get_zk_proof(&self, trace_id: TraceId) -> Option<&ZkProof> {
        self.zk_proofs.get(&trace_id)
    }
}
