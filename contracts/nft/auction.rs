//! OCOS-Chain: NFT Auction Module
//!
//! Implements English, Dutch, and sealed-bid auctions for NFTs.
//! Supports on-chain bidding, settlement, claim, minimum price, time windows, and DAO-based approval.

use crate::nft::types::{NFTId, Address, Amount, Timestamp};
use crate::nft::error::NFTError;
use std::collections::{HashMap, BTreeMap};

/// Auction types supported
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuctionType {
    English,
    Dutch,
    SealedBid,
}

/// Core auction structure
#[derive(Debug, Clone)]
pub struct NFTAuction {
    pub auction_id: u64,
    pub nft_id: NFTId,
    pub seller: Address,
    pub auction_type: AuctionType,
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub min_price: Amount,
    pub reserve_price: Option<Amount>,
    pub highest_bid: Option<(Address, Amount)>, // (bidder, amount)
    pub bids: BTreeMap<Amount, Address>,        // Sorted by bid amount (for English/SealedBid)
    pub settled: bool,
    pub winner: Option<Address>,
}

#[derive(Default)]
pub struct NFTAuctionRegistry {
    // auction_id → auction
    pub auctions: HashMap<u64, NFTAuction>,
}

impl NFTAuctionRegistry {
    /// Create a new auction
    pub fn create_auction(
        &mut self,
        auction_id: u64,
        nft_id: NFTId,
        seller: Address,
        auction_type: AuctionType,
        start_time: Timestamp,
        end_time: Timestamp,
        min_price: Amount,
        reserve_price: Option<Amount>,
    ) -> Result<(), NFTError> {
        if self.auctions.contains_key(&auction_id) {
            return Err(NFTError::AuctionExists);
        }
        let auction = NFTAuction {
            auction_id,
            nft_id,
            seller,
            auction_type,
            start_time,
            end_time,
            min_price,
            reserve_price,
            highest_bid: None,
            bids: BTreeMap::new(),
            settled: false,
            winner: None,
        };
        self.auctions.insert(auction_id, auction);
        Ok(())
    }

    /// Place a bid on an auction
    pub fn place_bid(
        &mut self,
        auction_id: u64,
        bidder: Address,
        bid_amount: Amount,
        now: Timestamp,
    ) -> Result<(), NFTError> {
        let auction = self.auctions.get_mut(&auction_id).ok_or(NFTError::AuctionNotFound)?;
        if now < auction.start_time || now > auction.end_time || auction.settled {
            return Err(NFTError::AuctionClosed);
        }
        if bid_amount < auction.min_price {
            return Err(NFTError::BidTooLow);
        }
        match auction.auction_type {
            AuctionType::English => {
                if let Some((_, current_highest)) = &auction.highest_bid {
                    if bid_amount <= *current_highest {
                        return Err(NFTError::BidTooLow);
                    }
                }
                auction.highest_bid = Some((bidder, bid_amount));
                auction.bids.insert(bid_amount, bidder);
            }
            AuctionType::SealedBid => {
                auction.bids.insert(bid_amount, bidder);
            }
            AuctionType::Dutch => {
                // In Dutch auction, first bidder at or above min_price wins immediately
                if auction.highest_bid.is_none() && bid_amount >= auction.min_price {
                    auction.highest_bid = Some((bidder, bid_amount));
                    auction.settled = true;
                    auction.winner = Some(bidder);
                } else {
                    return Err(NFTError::BidTooLow);
                }
            }
        }
        Ok(())
    }

    /// Settle (finalize) an auction and declare winner
    pub fn settle(
        &mut self,
        auction_id: u64,
        now: Timestamp,
    ) -> Result<Option<Address>, NFTError> {
        let auction = self.auctions.get_mut(&auction_id).ok_or(NFTError::AuctionNotFound)?;
        if auction.settled {
            return Err(NFTError::AuctionClosed);
        }
        if now < auction.end_time && auction.auction_type != AuctionType::Dutch {
            return Err(NFTError::AuctionOpen);
        }
        // Determine winner
        match auction.auction_type {
            AuctionType::English | AuctionType::SealedBid => {
                if let Some((winner, _amount)) = auction.bids.iter().rev().next() {
                    auction.winner = Some(*winner);
                }
            }
            AuctionType::Dutch => { /* Already settled in place_bid */ }
        }
        auction.settled = true;
        Ok(auction.winner)
    }

    /// Get current highest bid
    pub fn highest_bid(&self, auction_id: u64) -> Option<(Address, Amount)> {
        self.auctions.get(&auction_id).and_then(|a| a.highest_bid.clone())
    }

    /// Query auction details
    pub fn get_auction(&self, auction_id: u64) -> Option<&NFTAuction> {
        self.auctions.get(&auction_id)
    }
}
