//! OCOS-Chain: Block Store Module
//!
//! Responsible for efficient and auditable storage and retrieval of blocks,
//! supports persistence, indexing, rollback, and chain reorg operations.

use crate::ledger::block::Block;
use std::collections::HashMap;

/// BlockStore struct: main persistent storage for all blocks
#[derive(Debug, Default)]
pub struct BlockStore {
    /// Block height → Block
    pub blocks_by_height: HashMap<u64, Block>,
    /// Block hash → Block height
    pub height_by_hash: HashMap<Vec<u8>, u64>,
}

impl BlockStore {
    /// Create a new, empty block store
    pub fn new() -> Self {
        Self {
            blocks_by_height: HashMap::new(),
            height_by_hash: HashMap::new(),
        }
    }

    /// Insert a block (returns true if new, false if replaced)
    pub fn insert_block(&mut self, block: Block, block_hash: Vec<u8>, height: u64) -> bool {
        let is_new = !self.blocks_by_height.contains_key(&height);
        self.blocks_by_height.insert(height, block);
        self.height_by_hash.insert(block_hash, height);
        is_new
    }

    /// Get block by height
    pub fn get_block_by_height(&self, height: u64) -> Option<&Block> {
        self.blocks_by_height.get(&height)
    }

    /// Get block by hash
    pub fn get_block_by_hash(&self, hash: &[u8]) -> Option<&Block> {
        if let Some(height) = self.height_by_hash.get(hash) {
            self.blocks_by_height.get(height)
        } else {
            None
        }
    }

    /// Remove block by height
    pub fn remove_block_by_height(&mut self, height: u64) -> Option<Block> {
        if let Some(block) = self.blocks_by_height.remove(&height) {
            // Remove from hash index as well
            let hash_to_remove = self.height_by_hash.iter()
                .find(|(_, &h)| h == height)
                .map(|(k, _)| k.clone());
            if let Some(hash) = hash_to_remove {
                self.height_by_hash.remove(&hash);
            }
            Some(block)
        } else {
            None
        }
    }

    /// Get the latest block (by highest height)
    pub fn get_latest_block(&self) -> Option<&Block> {
        self.blocks_by_height
            .keys()
            .max()
            .and_then(|max_height| self.blocks_by_height.get(max_height))
    }

    /// Rollback to a given block height (removes all blocks above height)
    pub fn rollback_to_height(&mut self, height: u64) {
        let heights_to_remove: Vec<u64> = self.blocks_by_height
            .keys()
            .cloned()
            .filter(|&h| h > height)
            .collect();
        for h in heights_to_remove {
            self.remove_block_by_height(h);
        }
    }

    /// Returns the number of stored blocks
    pub fn len(&self) -> usize {
        self.blocks_by_height.len()
    }

    /// Checks if the store is empty
    pub fn is_empty(&self) -> bool {
        self.blocks_by_height.is_empty()
    }
}
