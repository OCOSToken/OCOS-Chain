//! OCOS-Chain: NFT Protocol Events Module
//!
//! Defines all on-chain events for NFTs, collections, marketplace, auctions, royalties, and governance.

use crate::nft::types::{NFTId, CollectionId, Address, Amount};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NFTEvent {
    Minted {
        nft_id: NFTId,
        to: Address,
    },
    Burned {
        nft_id: NFTId,
        by: Address,
    },
    Transferred {
        nft_id: NFTId,
        from: Address,
        to: Address,
    },
    Approved {
        nft_id: NFTId,
        owner: Address,
        approved: Address,
    },
    MetadataUpdated {
        nft_id: NFTId,
        key: String,
        value: String,
    },
    CollectionCreated {
        collection_id: CollectionId,
        creator: Address,
    },
    AddedToCollection {
        nft_id: NFTId,
        collection_id: CollectionId,
    },
    RoyaltySet {
        nft_id: Option<NFTId>,
        collection_id: Option<CollectionId>,
        recipient: Address,
        percentage_bps: u16,
    },
    RoyaltyPaid {
        nft_id: NFTId,
        sale_amount: Amount,
        recipient: Address,
        amount: Amount,
    },
    ListedOnMarketplace {
        nft_id: NFTId,
        seller: Address,
        price: Amount,
    },
    SoldOnMarketplace {
        nft_id: NFTId,
        buyer: Address,
        price: Amount,
    },
    AuctionCreated {
        auction_id: u64,
        nft_id: NFTId,
        seller: Address,
    },
    BidPlaced {
        auction_id: u64,
        bidder: Address,
        bid_amount: Amount,
    },
    AuctionSettled {
        auction_id: u64,
        winner: Option<Address>,
        final_price: Option<Amount>,
    },
    GovernanceProposalCreated {
        proposal_id: u64,
        creator: Address,
        description: String,
    },
    GovernanceVoted {
        proposal_id: u64,
        voter: Address,
        approve: bool,
        voting_power: u128,
    },
    GovernanceExecuted {
        proposal_id: u64,
        status: String,
    },
    // Extendable for future NFT/marketplace events
}
