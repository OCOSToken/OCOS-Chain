//! OCOS-Chain: Audit & Tracing Module Tests
//!
//! Tests core audit functions: trace, log, event, compliance, proof, storage, and error handling.

use crate::audit::{
    trace::{AuditTrace, TraceRegistry},
    log::AuditLog,
    event::{EventRecord, EventDecoder},
    compliance::{ComplianceEngine, CompliancePolicy, ComplianceResult},
    hash::{sha3_256_hash, merkle_root, verify_merkle_proof},
    proof::{MerkleProof, CommitmentProof, ZkProof},
    storage::AuditStorage,
    error::AuditError,
    types::*,
};

fn sample_actor() -> Address {
    [1u8; 20]
}

#[test]
fn test_trace_and_log() {
    let mut trace = AuditTrace::new(1, sample_actor(), 123, TraceLevel::Info, None);
    let action = AuditRecord {
        id: 1,
        action_type: ActionType::Transfer,
        details: "Transferred 100 tokens".into(),
    };
    trace.record_action(action.clone());
    assert_eq!(trace.actions.len(), 1);

    let mut registry = TraceRegistry::default();
    registry.add_trace(trace.clone());
    let fetched = registry.get_trace(1).unwrap();
    assert_eq!(fetched.initiator, sample_actor());
    assert_eq!(fetched.actions[0].details, "Transferred 100 tokens");

    // Log
    let mut log = AuditLog::new(10);
    log.record(123, 1, sample_actor(), ActionType::Transfer, LogLevel::Info, "Log entry".into());
    let logs = log.by_actor(&sample_actor());
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].details, "Log entry");
}

#[test]
fn test_event_decode_and_normalize() {
    let raw_data = vec![42u8; 10];
    let event = EventDecoder::decode(&raw_data).unwrap();
    assert_eq!(event.details, "Decoded event");
    let audit_event = EventDecoder::normalize(&event);
    assert_eq!(audit_event.details, "Decoded event");
}

#[test]
fn test_compliance_check_kyc() {
    let result = ComplianceEngine::check_action(
        sample_actor(),
        ActionType::Transfer,
        &[CompliancePolicy::KycRequired],
        None,
        None,
    );
    assert!(matches!(result, ComplianceResult::Rejected(_)));
}

#[test]
fn test_hash_and_merkle_proof() {
    let data1 = b"hello".to_vec();
    let data2 = b"world".to_vec();
    let hash1 = sha3_256_hash(&data1);
    let hash2 = sha3_256_hash(&data2);
    let root = merkle_root(vec![hash1, hash2]);
    let proof = vec![hash2];
    let merkle_proof = MerkleProof { leaf: hash1, proof: proof.clone(), root, index: 0 };
    assert!(merkle_proof.verify());
}

#[test]
fn test_commitment_proof_and_zkproof() {
    let record = AuditRecord { id: 1, action_type: ActionType::Mint, details: "Minted".into() };
    let records = vec![record.clone()];
    let hash = sha3_256_hash(&record.encode());
    let commitment = merkle_root(vec![hash]);
    let proof = CommitmentProof { commitment, records_hashes: vec![hash] };
    assert!(proof.verify(&records));

    let zk_proof = ZkProof { statement: vec![], proof_bytes: vec![] };
    assert!(zk_proof.verify(&[]));
}

#[test]
fn test_audit_storage_and_error() {
    let mut storage = AuditStorage::default();
    let log_entry = AuditLogEntry {
        timestamp: 1,
        trace_id: 7,
        actor: sample_actor(),
        action_type: ActionType::StorageWrite,
        level: LogLevel::Info,
        details: "Write".into(),
    };
    storage.add_log(7, log_entry.clone());
    let logs = storage.get_logs(7).unwrap();
    assert_eq!(logs[0].details, "Write");

    let error = AuditError::TraceNotFound;
    assert_eq!(format!("{}", error), "Audit trace not found");
}
