//! OCOS-Chain: NFT Metadata Module
//!
//! Manages on-chain and off-chain metadata URIs, attributes, and
//! extensible data for NFT collections. Compatible with ERC-721/1155 metadata standards.

use crate::nft::types::{NFTId, CollectionId};
use crate::nft::error::NFTError;
use std::collections::HashMap;

/// Metadata for a single NFT
#[derive(Debug, Clone)]
pub struct NFTMetadata {
    pub nft_id: NFTId,
    pub collection_id: Option<CollectionId>,
    pub name: String,
    pub description: String,
    pub image_uri: String,
    pub external_uri: Option<String>,
    pub attributes: HashMap<String, String>, // trait_type → value
    pub created_by: String, // Creator address or identifier
}

#[derive(Default)]
pub struct NFTMetadataRegistry {
    // nft_id → metadata
    pub metadata_map: HashMap<NFTId, NFTMetadata>,
}

impl NFTMetadataRegistry {
    /// Register or update NFT metadata
    pub fn set_metadata(
        &mut self,
        nft_id: NFTId,
        metadata: NFTMetadata,
    ) -> Result<(), NFTError> {
        self.metadata_map.insert(nft_id, metadata);
        Ok(())
    }

    /// Query metadata for a given NFT
    pub fn get_metadata(&self, nft_id: NFTId) -> Option<&NFTMetadata> {
        self.metadata_map.get(&nft_id)
    }

    /// Update a specific attribute for an NFT
    pub fn update_attribute(
        &mut self,
        nft_id: NFTId,
        key: &str,
        value: &str,
    ) -> Result<(), NFTError> {
        let meta = self.metadata_map.get_mut(&nft_id).ok_or(NFTError::NotFound)?;
        meta.attributes.insert(key.to_string(), value.to_string());
        Ok(())
    }

    /// Set or update the image URI for an NFT
    pub fn set_image_uri(
        &mut self,
        nft_id: NFTId,
        uri: String,
    ) -> Result<(), NFTError> {
        let meta = self.metadata_map.get_mut(&nft_id).ok_or(NFTError::NotFound)?;
        meta.image_uri = uri;
        Ok(())
    }
}
