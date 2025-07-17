//! OCOS-Chain: NFT Collection & Group Module
//!
//! Manages NFT collections, series, and group attributes.
//! Supports on-chain metadata, creator/owner, and DAO-based collection governance.

use crate::nft::types::{CollectionId, NFTId, Address};
use crate::nft::error::NFTError;
use std::collections::{HashMap, HashSet};

/// Metadata for a collection of NFTs
#[derive(Debug, Clone)]
pub struct NFTCollectionMetadata {
    pub collection_id: CollectionId,
    pub name: String,
    pub description: String,
    pub image_uri: String,
    pub external_uri: Option<String>,
    pub creator: Address,
    pub attributes: HashMap<String, String>, // Collection-level traits
    pub is_verified: bool,
}

/// Registry and management for all NFT collections
#[derive(Default)]
pub struct NFTCollectionRegistry {
    // collection_id → collection metadata
    pub collections: HashMap<CollectionId, NFTCollectionMetadata>,
    // collection_id → all NFT ids in collection
    pub collection_tokens: HashMap<CollectionId, HashSet<NFTId>>,
}

impl NFTCollectionRegistry {
    /// Register or update a new NFT collection
    pub fn set_collection(
        &mut self,
        meta: NFTCollectionMetadata,
    ) -> Result<(), NFTError> {
        self.collections.insert(meta.collection_id, meta);
        Ok(())
    }

    /// Add NFT to a collection (creator or DAO-gated)
    pub fn add_nft_to_collection(
        &mut self,
        collection_id: CollectionId,
        nft_id: NFTId,
    ) -> Result<(), NFTError> {
        if !self.collections.contains_key(&collection_id) {
            return Err(NFTError::CollectionNotFound);
        }
        self.collection_tokens.entry(collection_id).or_default().insert(nft_id);
        Ok(())
    }

    /// Query all NFTs in a collection
    pub fn nfts_of_collection(&self, collection_id: CollectionId) -> Option<Vec<NFTId>> {
        self.collection_tokens.get(&collection_id).map(|set| set.iter().cloned().collect())
    }

    /// Verify a collection (e.g., by DAO or admin)
    pub fn verify_collection(
        &mut self,
        collection_id: CollectionId,
        by: Address,
    ) -> Result<(), NFTError> {
        let collection = self.collections.get_mut(&collection_id).ok_or(NFTError::CollectionNotFound)?;
        collection.is_verified = true;
        Ok(())
    }

    /// Query collection metadata
    pub fn get_metadata(&self, collection_id: CollectionId) -> Option<&NFTCollectionMetadata> {
        self.collections.get(&collection_id)
    }
}
