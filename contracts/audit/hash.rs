//! OCOS-Chain: Audit Hash & Commitment Module
//!
//! Provides cryptographic hash functions and Merkle/commitment utilities
//! for immutable audit trails, on-chain proofs, and secure data integrity.

use sha3::{Digest, Sha3_256};

/// Compute a SHA3-256 hash of given data
pub fn sha3_256_hash(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&result);
    out
}

/// Compute a Merkle root from an array of hashes (binary Merkle tree)
pub fn merkle_root(mut leaves: Vec<[u8; 32]>) -> [u8; 32] {
    if leaves.is_empty() {
        return [0u8; 32];
    }
    while leaves.len() > 1 {
        let mut next_level = vec![];
        for i in (0..leaves.len()).step_by(2) {
            let pair = if i + 1 < leaves.len() {
                [leaves[i].as_ref(), leaves[i + 1].as_ref()].concat()
            } else {
                leaves[i].to_vec()
            };
            next_level.push(sha3_256_hash(&pair));
        }
        leaves = next_level;
    }
    leaves[0]
}

/// Hash a sequence of audit records into a single commitment
pub fn hash_audit_commitment(records: &[Vec<u8>]) -> [u8; 32] {
    let mut hashes: Vec<[u8; 32]> = records.iter().map(|d| sha3_256_hash(d)).collect();
    merkle_root(hashes)
}

/// Verify Merkle proof for an audit record
pub fn verify_merkle_proof(
    leaf: [u8; 32],
    proof: &[[u8; 32]],
    root: [u8; 32],
    mut index: usize,
) -> bool {
    let mut hash = leaf;
    for p in proof {
        let mut concat = if index % 2 == 0 {
            [hash.as_ref(), p.as_ref()].concat()
        } else {
            [p.as_ref(), hash.as_ref()].concat()
        };
        hash = sha3_256_hash(&concat);
        index /= 2;
    }
    hash == root
}
