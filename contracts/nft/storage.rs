//! OCOS-Chain: NFT Storage Module
//!
//! Provides persistent storage for NFTs, metadata, collections, royalties,
//! marketplace listings, auctions, and governance proposals.

use std::collections::{HashMap, HashSet};
use crate::nft::types::{NFTId, CollectionId, Address, ProposalId, AuctionId};
use crate::nft::token::NFTLedger;
use crate::nft::metadata::{NFTMetadata, NFTMetadataRegistry};
use crate::nft::collection::{NFTCollectionMetadata, NFTCollectionRegistry};
use crate::nft::royalty::{NFTRoyaltyInfo, NFTRoyaltyRegistry};
use crate::nft::marketplace::MarketplaceListing;
use crate::nft::auction::NFTAuction;
use crate::nft::governance::{NFTGovernanceProposal, NFTGovernance};

#[derive(Default)]
pub struct NFTStorage {
    // NFT ownership and approval
    pub ledger: NFTLedger,
    // nft_id → NFTMetadata
    pub metadata: HashMap<NFTId, NFTMetadata>,
    // collection_id → Collection metadata & NFT list
    pub collections: NFTCollectionRegistry,
    // Royalty registry
    pub royalties: NFTRoyaltyRegistry,
    // marketplace listing_id → listing struct
    pub marketplace_listings: HashMap<u64, MarketplaceListing>,
    // auction_id → NFTAuction
    pub auctions: HashMap<AuctionId, NFTAuction>,
    // proposal_id → NFTGovernanceProposal
    pub governance_proposals: HashMap<ProposalId, NFTGovernanceProposal>,
    // Governance engine logic/state (e.g., votes, allowlists)
    pub governance: NFTGovernance,
}

impl NFTStorage {
    // --- NFT Metadata Logic ---
    pub fn set_metadata(&mut self, nft_id: NFTId, metadata: NFTMetadata) {
        self.metadata.insert(nft_id, metadata);
    }
    pub fn get_metadata(&self, nft_id: NFTId) -> Option<&NFTMetadata> {
        self.metadata.get(&nft_id)
    }

    // --- Collection Logic ---
    pub fn set_collection(&mut self, meta: NFTCollectionMetadata) {
        self.collections.set_collection(meta).ok();
    }
    pub fn add_nft_to_collection(&mut self, collection_id: CollectionId, nft_id: NFTId) {
        self.collections.add_nft_to_collection(collection_id, nft_id).ok();
    }

    // --- Royalty Logic ---
    pub fn set_nft_royalty(&mut self, nft_id: NFTId, info: NFTRoyaltyInfo) {
        self.royalties.nft_royalties.insert(nft_id, info);
    }
    pub fn set_collection_royalty(&mut self, collection_id: CollectionId, info: NFTRoyaltyInfo) {
        self.royalties.collection_royalties.insert(collection_id, info);
    }

    // --- Marketplace Listing Logic ---
    pub fn add_listing(&mut self, listing_id: u64, listing: MarketplaceListing) {
        self.marketplace_listings.insert(listing_id, listing);
    }
    pub fn get_listing(&self, listing_id: u64) -> Option<&MarketplaceListing> {
        self.marketplace_listings.get(&listing_id)
    }

    // --- Auction Logic ---
    pub fn add_auction(&mut self, auction_id: AuctionId, auction: NFTAuction) {
        self.auctions.insert(auction_id, auction);
    }
    pub fn get_auction(&self, auction_id: AuctionId) -> Option<&NFTAuction> {
        self.auctions.get(&auction_id)
    }

    // --- Governance Logic ---
    pub fn add_governance_proposal(&mut self, proposal: NFTGovernanceProposal) {
        self.governance_proposals.insert(proposal.proposal_id, proposal);
    }
    pub fn get_governance_proposal(&self, proposal_id: ProposalId) -> Option<&NFTGovernanceProposal> {
        self.governance_proposals.get(&proposal_id)
    }
}
