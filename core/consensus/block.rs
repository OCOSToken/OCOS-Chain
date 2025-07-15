//! OCOS-Chain: Block & Header Definitions
//!
//! Defines the canonical block and header structures, supporting both
//! quantum-resistant and classical cryptography. Includes hashing, signature
//! validation and metadata for full auditability.

use sha3::{Digest, Sha3_256};

/// Canonical block header structure
#[derive(Debug, Clone)]
pub struct BlockHeader {
    pub parent_hash: Vec<u8>,
    pub proposer_public_key: Vec<u8>,
    pub state_root: Vec<u8>,
    pub tx_root: Vec<u8>,
    pub height: u64,
    pub timestamp: u64,
    pub metadata: Option<Vec<u8>>, // extensible: e.g., for consensus params, governance info
}

impl BlockHeader {
    /// Create a new block header
    pub fn new(parent_hash: Vec<u8>, proposer_public_key: Vec<u8>, txs: &[u8]) -> Self {
        let now = Self::current_unix_timestamp();
        BlockHeader {
            parent_hash,
            proposer_public_key,
            state_root: Self::calc_state_root(txs),
            tx_root: Self::calc_tx_root(txs),
            height: 0, // to be set by consensus engine
            timestamp: now,
            metadata: None,
        }
    }

    /// Calculate block header hash (chain ID, height, roots, etc.)
    pub fn hash(&self) -> Vec<u8> {
        let mut hasher = Sha3_256::new();
        hasher.update(&self.parent_hash);
        hasher.update(&self.proposer_public_key);
        hasher.update(&self.state_root);
        hasher.update(&self.tx_root);
        hasher.update(&self.height.to_be_bytes());
        hasher.update(&self.timestamp.to_be_bytes());
        if let Some(meta) = &self.metadata {
            hasher.update(meta);
        }
        hasher.finalize().to_vec()
    }

    /// Simulated state root (placeholder)
    fn calc_state_root(_txs: &[u8]) -> Vec<u8> {
        // Real implementation: Merkle/Patricia trie root of state after txs applied
        vec![0u8; 32]
    }

    /// Simulated tx root (placeholder)
    fn calc_tx_root(txs: &[u8]) -> Vec<u8> {
        // Real implementation: Merkle root of all transactions
        let mut hasher = Sha3_256::new();
        hasher.update(txs);
        hasher.finalize().to_vec()
    }

    /// Helper: current UNIX timestamp (seconds)
    fn current_unix_timestamp() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }
}

/// Canonical OCOS-Chain block structure
#[derive(Debug, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub signature: Vec<u8>,        // Block signature (quantum/classical)
    pub proposer: String,          // Validator address
    pub transactions: Vec<u8>,     // Transactions (batch, for simplicity)
}

impl Block {
    /// Block hash = header hash (chain standard)
    pub fn hash(&self) -> Vec<u8> {
        self.header.hash()
    }

    /// Validate block header signature (via quantum/classical handler)
    pub fn validate_signature<F>(&self, verify_fn: F) -> bool
    where
        F: Fn(&[u8], &[u8], &[u8]) -> bool,
    {
        verify_fn(
            &self.header.proposer_public_key,
            &self.header.hash(),
            &self.signature,
        )
    }
}
