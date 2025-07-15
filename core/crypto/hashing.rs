//! OCOS-Chain: Cryptographic Hashing Module
//!
//! Provides universal hashing functions used throughout the protocol,
//! including SHA3-256, BLAKE3, Keccak256 (optional), and domain-specific utilities.
//!
//! All hashing operations in OCOS are auditable, deterministic, and chain-consistent.

use sha3::{Sha3_256, Digest};
use blake3;
use thiserror::Error;

/// Hashing error type
#[derive(Debug, Error)]
pub enum HashingError {
    #[error("Hash computation failed")]
    HashFailed,
}

/// Hash with SHA3-256
pub fn sha3_256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// Hash with BLAKE3
pub fn blake3_256(data: &[u8]) -> Vec<u8> {
    blake3::hash(data).as_bytes().to_vec()
}

/// Hash + Salt (e.g. DAO proposal ID or private key derivation)
pub fn sha3_256_salted(data: &[u8], salt: &[u8]) -> Vec<u8> {
    let mut hasher = Sha3_256::new();
    hasher.update(salt);
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// Double SHA3 (e.g. address derivation)
pub fn double_sha3_256(data: &[u8]) -> Vec<u8> {
    sha3_256(&sha3_256(data))
}

/// Fixed-length digest helper (truncate to 20 bytes — Ethereum-style address)
pub fn hash_to_20_bytes(data: &[u8]) -> [u8; 20] {
    let full_hash = sha3_256(data);
    let mut short = [0u8; 20];
    short.copy_from_slice(&full_hash[..20]);
    short
}
