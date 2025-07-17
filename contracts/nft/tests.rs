//! OCOS-Chain: NFT Module Tests
//!
//! Unit & integration tests for NFT minting, transfer, collection, metadata, royalty,
//! marketplace listing, governance and event logic.

use crate::nft::{
    token::NFTLedger,
    metadata::{NFTMetadata, NFTMetadataRegistry},
    collection::{NFTCollectionMetadata, NFTCollectionRegistry},
    royalty::NFTRoyaltyRegistry,
    governance::{NFTGovernance, NFTGovernanceProposal, ProposalStatus},
    auction::{NFTAuctionRegistry, AuctionType},
    error::NFTError,
    types::*,
};

fn dummy_address(n: u8) -> Address {
    let mut arr = [0u8; 20];
    arr[0] = n;
    arr
}

#[test]
fn test_mint_and_transfer_nft() {
    let mut ledger = NFTLedger::default();
    let mut events = vec![];

    // Mint NFT to user1
    let nft_id = 1;
    let user1 = dummy_address(1);
    assert!(ledger.mint(nft_id, user1, &mut events).is_ok());
    assert_eq!(ledger.owner_of(nft_id).unwrap(), user1);

    // Approve and transfer to user2
    let user2 = dummy_address(2);
    assert!(ledger.approve(nft_id, user1, user2, &mut events).is_ok());
    assert!(ledger.transfer(nft_id, user2, user2, &mut events).is_ok());
    assert_eq!(ledger.owner_of(nft_id).unwrap(), user2);

    // Burn NFT
    assert!(ledger.burn(nft_id, user2, &mut events).is_ok());
    assert!(ledger.owner_of(nft_id).is_none());
}

#[test]
fn test_metadata_and_attribute_update() {
    let mut registry = NFTMetadataRegistry::default();
    let nft_id = 42;
    let meta = NFTMetadata {
        nft_id,
        collection_id: None,
        name: "Art #42".to_string(),
        description: "Unique piece".to_string(),
        image_uri: "ipfs://somehash".to_string(),
        external_uri: None,
        attributes: Default::default(),
        created_by: "creator_addr".to_string(),
    };
    assert!(registry.set_metadata(nft_id, meta.clone()).is_ok());
    assert_eq!(registry.get_metadata(nft_id).unwrap().name, "Art #42");

    // Update attribute
    assert!(registry.update_attribute(nft_id, "Color", "Red").is_ok());
    assert_eq!(
        registry.get_metadata(nft_id).unwrap().attributes.get("Color").unwrap(),
        "Red"
    );
}

#[test]
fn test_collection_and_nft_add() {
    let mut registry = NFTCollectionRegistry::default();
    let collection_id = 10;
    let meta = NFTCollectionMetadata {
        collection_id,
        name: "SuperArt".to_string(),
        description: "Legendary".to_string(),
        image_uri: "ipfs://coll".to_string(),
        external_uri: None,
        creator: dummy_address(9),
        attributes: Default::default(),
        is_verified: false,
    };
    assert!(registry.set_collection(meta).is_ok());
    assert!(registry.add_nft_to_collection(collection_id, 1001).is_ok());
    let nfts = registry.nfts_of_collection(collection_id).unwrap();
    assert!(nfts.contains(&1001));
}

#[test]
fn test_royalty_registry_and_payout() {
    let mut registry = NFTRoyaltyRegistry::default();
    let nft_id = 5;
    let recipient = dummy_address(7);
    assert!(registry.set_nft_royalty(nft_id, recipient, 500).is_ok()); // 5%
    let (to, payout) = registry.calculate_royalty(nft_id, None, 10_000).unwrap();
    assert_eq!(to, recipient);
    assert_eq!(payout, 500);
}

#[test]
fn test_nft_governance_proposal_and_voting() {
    let mut gov = NFTGovernance::default();
    let proposal_id = 77;
    let collection_id = Some(8);
    let creator = dummy_address(1);

    assert!(gov.submit_proposal(proposal_id, collection_id, creator, "Change banner".to_string(), 0, 10).is_ok());
    let voter = dummy_address(2);

    assert!(gov.vote(proposal_id, voter, true, 2, 5).is_ok()); // 2 NFT voting power
    assert!(gov.execute(proposal_id, 11).is_ok());
    assert_eq!(gov.proposals.get(&proposal_id).unwrap().status, ProposalStatus::Executed);
}

#[test]
fn test_auction_english_flow() {
    let mut registry = NFTAuctionRegistry::default();
    let auction_id = 333;
    let nft_id = 55;
    let seller = dummy_address(4);
    assert!(registry.create_auction(auction_id, nft_id, seller, AuctionType::English, 0, 100, 1_000, None).is_ok());

    // Bidder1, Bidder2
    let bidder1 = dummy_address(8);
    let bidder2 = dummy_address(9);
    assert!(registry.place_bid(auction_id, bidder1, 1_500, 10).is_ok());
    assert!(registry.place_bid(auction_id, bidder2, 2_000, 11).is_ok());

    // Settle auction
    assert!(registry.settle(auction_id, 101).unwrap().is_some());
}
