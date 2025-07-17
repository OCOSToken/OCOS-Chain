//! OCOS-Chain: Identity Protocol Types & Identifiers
//!
//! Centralizes all identity, DID, SBT, reputation, group, and governance enums and types.

pub type IdentityId = u64;
pub type Address = [u8; 20];
pub type GroupId = u64;
pub type SBTId = u64;
pub type DID = String;
pub type ProposalId = u64;

/// DID Document structure (W3C compatible, simplified)
#[derive(Debug, Clone)]
pub struct DIDDocument {
    pub did: DID,
    pub controller: Address,
    pub verification_methods: Vec<VerificationMethod>,
    pub services: Vec<ServiceEndpoint>,
    pub updated_at: u64,
    pub attested_by: Vec<(Address, Vec<u8>)>, // (attestor, proof)
}

/// Verification method (public key or zk-proof)
#[derive(Debug, Clone)]
pub struct VerificationMethod {
    pub key_type: String,   // e.g., "Ed25519", "secp256k1", "ZK"
    pub public_key: Vec<u8>,
    pub fragment: Option<String>, // "#controller" or other reference
}

/// DID service endpoint
#[derive(Debug, Clone)]
pub struct ServiceEndpoint {
    pub id: String,
    pub service_type: String,
    pub endpoint: String,
}

/// Soulbound token metadata
#[derive(Debug, Clone)]
pub struct SBTMetadata {
    pub sbt_id: SBTId,
    pub name: String,
    pub description: String,
    pub issued_by: Address,
    pub issued_at: u64,
    pub expiry: Option<u64>,
    pub attributes: Vec<(String, String)>, // trait_type, value
}

/// Identity profile (base)
#[derive(Debug, Clone)]
pub struct IdentityProfile {
    pub identity_id: IdentityId,
    pub address: Address,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
    pub email: Option<String>,
    pub created_at: u64,
}

/// Attestation (DAO, KYC, etc.)
#[derive(Debug, Clone)]
pub struct Attestation {
    pub identity_id: IdentityId,
    pub attestor: Address,
    pub data: String,
    pub timestamp: u64,
}

/// Reputation history event
#[derive(Debug, Clone)]
pub struct ReputationEvent {
    pub address: Address,
    pub score_before: u64,
    pub score_after: u64,
    pub delta: i64,
    pub reason: String,
    pub timestamp: u64,
}
