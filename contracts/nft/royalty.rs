//! OCOS-Chain: NFT Royalty Module (ERC-2981 Compatible)
//!
//! Handles on-chain royalty payout for creators, collection owners, or DAO
//! in NFT sales, transfers, and marketplace operations.

use crate::nft::types::{NFTId, CollectionId, Address, Amount};
use crate::nft::error::NFTError;
use std::collections::HashMap;

/// Royalty info per NFT or collection (ERC-2981 style)
#[derive(Debug, Clone)]
pub struct NFTRoyaltyInfo {
    pub recipient: Address,
    pub percentage_bps: u16, // Basis points (e.g., 500 = 5%)
}

/// Registry for all royalty settings
#[derive(Default)]
pub struct NFTRoyaltyRegistry {
    // nft_id → royalty info
    pub nft_royalties: HashMap<NFTId, NFTRoyaltyInfo>,
    // collection_id → default royalty info
    pub collection_royalties: HashMap<CollectionId, NFTRoyaltyInfo>,
}

impl NFTRoyaltyRegistry {
    /// Set royalty for an NFT (overrides collection default)
    pub fn set_nft_royalty(
        &mut self,
        nft_id: NFTId,
        recipient: Address,
        percentage_bps: u16,
    ) -> Result<(), NFTError> {
        if percentage_bps > 10_000 {
            return Err(NFTError::InvalidRoyalty);
        }
        self.nft_royalties.insert(nft_id, NFTRoyaltyInfo { recipient, percentage_bps });
        Ok(())
    }

    /// Set default royalty for a collection
    pub fn set_collection_royalty(
        &mut self,
        collection_id: CollectionId,
        recipient: Address,
        percentage_bps: u16,
    ) -> Result<(), NFTError> {
        if percentage_bps > 10_000 {
            return Err(NFTError::InvalidRoyalty);
        }
        self.collection_royalties.insert(collection_id, NFTRoyaltyInfo { recipient, percentage_bps });
        Ok(())
    }

    /// Query royalty info for a sale (NFT-level or collection-level)
    pub fn get_royalty(
        &self,
        nft_id: NFTId,
        collection_id: Option<CollectionId>,
    ) -> Option<&NFTRoyaltyInfo> {
        self.nft_royalties.get(&nft_id)
            .or_else(|| collection_id.and_then(|cid| self.collection_royalties.get(&cid)))
    }

    /// Calculate royalty payout for a given sale amount
    pub fn calculate_royalty(
        &self,
        nft_id: NFTId,
        collection_id: Option<CollectionId>,
        sale_amount: Amount,
    ) -> Option<(Address, Amount)> {
        let royalty = self.get_royalty(nft_id, collection_id)?;
        let payout = sale_amount * royalty.percentage_bps as u128 / 10_000u128;
        Some((royalty.recipient, payout))
    }
}
