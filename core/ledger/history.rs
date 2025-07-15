//! OCOS-Chain: Ledger History Module
//!
//! Stores and enables iteration over block and transaction history.
//! Supports audit, rewind, chain analysis, and explorer queries.

use crate::ledger::block::Block;
use crate::ledger::transaction::SignedTransaction;
use std::collections::BTreeMap;

/// History struct: Ordered storage for block and transaction hashes
#[derive(Debug, Default)]
pub struct History {
    pub block_hashes: BTreeMap<u64, Vec<u8>>,                 // Block height → block hash
    pub txs_by_block: BTreeMap<u64, Vec<Vec<u8>>>,            // Block height → tx hashes
    pub tx_details: BTreeMap<Vec<u8>, SignedTransaction>,      // Tx hash → tx struct
}

impl History {
    /// Yeni, boş tarixçə yaradın
    pub fn new() -> Self {
        Self::default()
    }

    /// Yeni blok əlavə edin
    pub fn add_block(&mut self, height: u64, block_hash: Vec<u8>, txs: Vec<SignedTransaction>) {
        self.block_hashes.insert(height, block_hash);
        let mut tx_hashes = vec![];
        for tx in txs {
            tx_hashes.push(tx.hash.clone());
            self.tx_details.insert(tx.hash.clone(), tx);
        }
        self.txs_by_block.insert(height, tx_hashes);
    }

    /// Blokun hash-i üzrə axtarış
    pub fn get_block_hash(&self, height: u64) -> Option<&Vec<u8>> {
        self.block_hashes.get(&height)
    }

    /// Blokda olan əməliyyatların hash-larını əldə et
    pub fn get_block_txs(&self, height: u64) -> Option<&Vec<Vec<u8>>> {
        self.txs_by_block.get(&height)
    }

    /// Əməliyyatın detallarını hash üzrə əldə et
    pub fn get_tx_by_hash(&self, hash: &[u8]) -> Option<&SignedTransaction> {
        self.tx_details.get(hash)
    }

    /// Tarixçə üzrə iterator qaytar (block-lar üzrə)
    pub fn iter_blocks(&self) -> impl Iterator<Item=(&u64, &Vec<u8>)> {
        self.block_hashes.iter()
    }

    /// Tarixçəni geri çevir (rewind) – blok hündürlüyünə qədər olan bütün məlumatı saxla
    pub fn rewind_to(&mut self, height: u64) {
        self.block_hashes.split_off(&(height + 1));
        self.txs_by_block.split_off(&(height + 1));
        // Tx details-dən artıq tx-ləri də silmək olar (əgər lazımdırsa)
    }
}

/// İstənilən blok və ya əməliyyat tarixçəsi üçün universal iterator
pub struct LedgerIterator<'a> {
    history: &'a History,
    pos: u64,
}

impl<'a> LedgerIterator<'a> {
    pub fn new(history: &'a History, start_height: u64) -> Self {
        Self { history, pos: start_height }
    }
}

impl<'a> Iterator for LedgerIterator<'a> {
    type Item = (&'a u64, &'a Vec<u8>);
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.history.block_hashes.iter().find(|(&h, _)| h == self.pos);
        self.pos += 1;
        item
    }
}
