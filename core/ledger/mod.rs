//! OCOS-Chain: Ledger Root Module
//!
//! Defines the main accounting and persistent data structures for the blockchain ledger.
//! Integrates blocks, transactions, account state, receipts, Merkle proofs, and audit hooks.

pub mod block;
pub mod transaction;
pub mod state;
pub mod account;
pub mod receipt;
pub mod merkle;
pub mod history;
pub mod audit;

// -- Public re-exports for ease of use across protocol layers --
pub use block::{Block, BlockHeader};
pub use transaction::{Transaction, SignedTransaction};
pub use state::{State, StateUpdate, StorageProof};
pub use account::{Account, Balance, Nonce};
pub use receipt::{Receipt, EventLog};
pub use merkle::{MerkleTree, MerkleProof};
pub use history::{History, LedgerIterator};
pub use audit::{LedgerAudit, AuditLog, AuditError};
