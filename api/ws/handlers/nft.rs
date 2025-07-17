//! OCOS-Chain: WebSocket Handler – NFT Events
//!
//! Streams real-time NFT lifecycle events: mint, transfer, burn, sale, listing, auction, etc.

use crate::ws::types::{
    WsTopic, WsMessage, NftInfo, NftTransfer, NftSale, NftListing, NftAuction, CollectionInfo,
};
use crate::ws::router::WsRouter;

/// Handles streaming of all NFT and collection-related WebSocket events.
pub struct NftHandler;

impl NftHandler {
    /// Broadcast when a new NFT is minted
    pub fn on_nft_minted(router: &WsRouter, nft: NftInfo) {
        let msg = WsMessage::NftMinted { nft: nft.clone() };
        router.broadcast(&WsTopic::Nft, &msg);
    }

    /// Broadcast when an NFT is transferred
    pub fn on_nft_transferred(router: &WsRouter, transfer: NftTransfer) {
        let msg = WsMessage::NftTransferred { transfer: transfer.clone() };
        router.broadcast(&WsTopic::Nft, &msg);
    }

    /// Broadcast when an NFT is burned
    pub fn on_nft_burned(router: &WsRouter, nft: NftInfo) {
        let msg = WsMessage::NftBurned { nft: nft.clone() };
        router.broadcast(&WsTopic::Nft, &msg);
    }

    /// Broadcast when an NFT is listed for sale
    pub fn on_nft_listed(router: &WsRouter, listing: NftListing) {
        let msg = WsMessage::NftListed { listing: listing.clone() };
        router.broadcast(&WsTopic::Nft, &msg);
    }

    /// Broadcast when an NFT is sold
    pub fn on_nft_sold(router: &WsRouter, sale: NftSale) {
        let msg = WsMessage::NftSold { sale: sale.clone() };
        router.broadcast(&WsTopic::Nft, &msg);
    }

    /// Broadcast auction events (start, bid, end)
    pub fn on_nft_auction_event(router: &WsRouter, auction: NftAuction) {
        let msg = WsMessage::NftAuctionEvent { auction: auction.clone() };
        router.broadcast(&WsTopic::Nft, &msg);
    }

    /// Broadcast collection-level events (creation, update, deletion)
    pub fn on_collection_event(router: &WsRouter, collection: CollectionInfo) {
        let msg = WsMessage::CollectionEvent { collection: collection.clone() };
        router.broadcast(&WsTopic::Nft, &msg);
    }
}

// -- Example message types for reference --

/*
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsMessage {
    NftMinted { nft: NftInfo },
    NftTransferred { transfer: NftTransfer },
    NftBurned { nft: NftInfo },
    NftListed { listing: NftListing },
    NftSold { sale: NftSale },
    NftAuctionEvent { auction: NftAuction },
    CollectionEvent { collection: CollectionInfo },
    // ... more
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftInfo {
    pub id: u64,
    pub owner: String,
    pub uri: String,
    pub collection: Option<u64>,
    pub attributes: serde_json::Value,
    pub minted_at: u64,
    // ... etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftTransfer {
    pub nft_id: u64,
    pub from: String,
    pub to: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftSale {
    pub nft_id: u64,
    pub buyer: String,
    pub seller: String,
    pub price: u128,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftListing {
    pub nft_id: u64,
    pub seller: String,
    pub price: u128,
    pub listed_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftAuction {
    pub auction_id: u64,
    pub nft_id: u64,
    pub status: String,
    pub current_bid: Option<u128>,
    pub end_time: u64,
    // ... etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionInfo {
    pub id: u64,
    pub name: String,
    pub owner: String,
    pub metadata: serde_json::Value,
    pub created_at: u64,
}
*/
