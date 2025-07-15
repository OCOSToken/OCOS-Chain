//! OCOS-Chain: Transaction Pool (Mempool) Module
//!
//! Handles pending transactions prior to block inclusion. Supports filtering, prioritization,
//! expiration, replay protection, and mempool auditing.

use crate::ledger::transaction::SignedTransaction;
use std::collections::{HashMap, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH, Duration};

/// TxPool struct: holds all pending transactions
pub struct TxPool {
    /// Address → Nonce → Transaction
    pub pending: HashMap<Vec<u8>, HashMap<u64, TxPoolEntry>>,
    /// Expiration time in seconds for each tx
    pub tx_ttl: Duration,
    /// Ordered queue for eviction & auditing
    pub ordered_queue: VecDeque<TxPoolEntry>,
}

/// TxPoolEntry: Stores transaction with timestamp
#[derive(Clone)]
pub struct TxPoolEntry {
    pub tx: SignedTransaction,
    pub timestamp: u64, // UNIX epoch seconds
}

impl TxPool {
    /// Create new, empty mempool with specified tx TTL (seconds)
    pub fn new(ttl_seconds: u64) -> Self {
        Self {
            pending: HashMap::new(),
            tx_ttl: Duration::from_secs(ttl_seconds),
            ordered_queue: VecDeque::new(),
        }
    }

    /// Add a new transaction (replay and duplicate check)
    pub fn add_transaction(&mut self, tx: SignedTransaction) -> Result<(), &'static str> {
        let sender = tx.sender.clone();
        let nonce = tx.nonce;
        let now = current_unix_timestamp();

        // Replay/duplicate protection
        if let Some(map) = self.pending.get(&sender) {
            if map.contains_key(&nonce) {
                return Err("Duplicate/replay transaction");
            }
        }
        let entry = TxPoolEntry { tx: tx.clone(), timestamp: now };

        self.pending.entry(sender.clone())
            .or_insert_with(HashMap::new)
            .insert(nonce, entry.clone());

        self.ordered_queue.push_back(entry);
        Ok(())
    }

    /// Get next valid transaction for a given sender (by nonce)
    pub fn get_next_tx(&self, sender: &[u8], current_nonce: u64) -> Option<&SignedTransaction> {
        self.pending.get(sender)
            .and_then(|map| map.get(&(current_nonce + 1)))
            .map(|entry| &entry.tx)
    }

    /// Remove a transaction after block inclusion
    pub fn remove_transaction(&mut self, sender: &[u8], nonce: u64) -> Option<SignedTransaction> {
        if let Some(map) = self.pending.get_mut(sender) {
            if let Some(entry) = map.remove(&nonce) {
                return Some(entry.tx);
            }
        }
        None
    }

    /// Evict expired transactions (audit for dropped tx)
    pub fn evict_expired(&mut self) {
        let now = current_unix_timestamp();
        while let Some(entry) = self.ordered_queue.front() {
            if now - entry.timestamp > self.tx_ttl.as_secs() {
                // Remove from pending map
                let sender = entry.tx.sender.clone();
                let nonce = entry.tx.nonce;
                self.remove_transaction(&sender, nonce);
                self.ordered_queue.pop_front();
            } else {
                break;
            }
        }
    }

    /// Get current pool size
    pub fn len(&self) -> usize {
        self.ordered_queue.len()
    }

    /// Check if the pool is empty
    pub fn is_empty(&self) -> bool {
        self.ordered_queue.is_empty()
    }
}

/// Helper: current UNIX timestamp (seconds)
fn current_unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_secs()
}
