//! OCOS-Chain: NFT Protocol Types & Identifiers
//!
//! Centralizes NFT, collection, marketplace, auction, and governance enums and types.

pub type NFTId = u64;
pub type CollectionId = u64;
pub type Address = [u8; 20];
pub type Amount = u128;
pub type Timestamp = u64;

/// Marketplace price (can be used for fixed, reserve, min, bid amounts)
pub type Price = u128;

/// NFT proposal and auction identifiers
pub type ProposalId = u64;
pub type AuctionId = u64;

/// Core NFT type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NFTStandard {
    ERC721,
    ERC1155,
    Custom(String),
}

/// Royalty payout basis
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoyaltyBasis {
    PerNFT,
    PerCollection,
    DAO,
}

/// Auction types for NFT sales
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuctionType {
    English,
    Dutch,
    SealedBid,
}

/// NFT approval/transfer status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApprovalStatus {
    None,
    Approved(Address),
    Operator(Address),
}

/// NFT governance proposal status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GovernanceStatus {
    Pending,
    Approved,
    Rejected,
    Executed,
    Expired,
}

/// Marketplace listing status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListingStatus {
    Active,
    Sold,
    Cancelled,
    Expired,
}

/// Core trait for NFT attribute key/value (OpenSea-compatible)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}
