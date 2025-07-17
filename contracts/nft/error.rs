//! OCOS-Chain: NFT Protocol Error Types
//!
//! Defines all error codes and categories for NFT, collection, royalty, governance,
//! auction, and marketplace operations.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NFTError {
    Unauthorized,
    NotFound,
    AlreadyExists,
    InvalidOperation,
    InvalidParameter,
    InvalidRoyalty,
    MetadataError,
    CollectionNotFound,
    AuctionExists,
    AuctionNotFound,
    AuctionClosed,
    AuctionOpen,
    BidTooLow,
    AlreadyVoted,
    ProposalExists,
    ProposalNotFound,
    ProposalClosed,
    StorageError(String),
    Other(String),
}

impl fmt::Display for NFTError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NFTError::Unauthorized => write!(f, "Unauthorized operation"),
            NFTError::NotFound => write!(f, "NFT or resource not found"),
            NFTError::AlreadyExists => write!(f, "Resource already exists"),
            NFTError::InvalidOperation => write!(f, "Invalid operation"),
            NFTError::InvalidParameter => write!(f, "Invalid parameter"),
            NFTError::InvalidRoyalty => write!(f, "Invalid royalty percentage or configuration"),
            NFTError::MetadataError => write!(f, "Metadata processing error"),
            NFTError::CollectionNotFound => write!(f, "NFT collection not found"),
            NFTError::AuctionExists => write!(f, "Auction already exists for this NFT"),
            NFTError::AuctionNotFound => write!(f, "Auction not found"),
            NFTError::AuctionClosed => write!(f, "Auction is closed"),
            NFTError::AuctionOpen => write!(f, "Auction is still open"),
            NFTError::BidTooLow => write!(f, "Bid is too low"),
            NFTError::AlreadyVoted => write!(f, "Address has already voted"),
            NFTError::ProposalExists => write!(f, "Governance proposal already exists"),
            NFTError::ProposalNotFound => write!(f, "Governance proposal not found"),
            NFTError::ProposalClosed => write!(f, "Governance proposal is closed"),
            NFTError::StorageError(msg) => write!(f, "Storage error: {}", msg),
            NFTError::Other(msg) => write!(f, "Other NFT error: {}", msg),
        }
    }
}

impl std::error::Error for NFTError {}
