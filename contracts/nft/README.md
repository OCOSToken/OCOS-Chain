# OCOS-Chain: NFT Module

**ERC-721 / ERC-1155 Compatible | Metadata | Royalty | Collection | Marketplace | Governance | Events**

---

## Overview

The `/nft` module implements a full-featured, modular, and auditable NFT infrastructure for OCOS-Chain.  
It provides core logic for ERC-721/1155 compatible NFTs, metadata, collection and royalty management, DAO-driven marketplace functionality, and governance for both creators and collectors.

---

## Features

- **NFT Ledger:** Ownership, transfer, burn, mint, and approval system
- **Metadata Registry:** On-chain and IPFS-compatible metadata structures with extensible attributes
- **Royalty Management:** Per-token and per-collection royalty with ERC-2981-style calculations
- **Collections:** Logical grouping of NFTs with shared attributes, creator, and verification
- **Marketplace Logic:** Listing, buying, and selling of NFTs with price validation
- **Auction Engine:** English, Dutch, and sealed-bid auctions fully integrated with DAO governance
- **Governance:** Weighted vote, delegation, proposal execution for NFT-based DAOs
- **Events:** Chain event logs for monitoring, analytics, and third-party indexers
- **Types & Errors:** Cleanly abstracted types and errors for SDK and contract integration

---

## Directory Structure

```
nft/
│
├── mod.rs          # Main entry; re-exports all modules
├── token.rs        # Core NFT ledger: mint, burn, transfer, approval
├── metadata.rs     # Metadata management for NFTs
├── collection.rs   # NFT collection logic and grouping
├── royalty.rs      # Royalty distribution system
├── marketplace.rs  # Fixed-price NFT sales and listings
├── auction.rs      # Auction engine for NFT bidding
├── governance.rs   # NFT-based DAO governance tools
├── error.rs        # NFT-specific error definitions
├── events.rs       # NFT-related event structures
├── types.rs        # Shared NFT types and enums
├── storage.rs      # NFT storage layer for all registries
├── tests.rs        # Unit and integration tests
```

---

## Integration

- Tightly integrated with `/identity` for SBTs, DAO roles, and permissioned NFTs  
- Compatible with `/core/consensus` for validator NFT identity systems  
- Designed to work with `/liquidity` and `/governance` modules for DeFi and DAO interaction

---

## Security & Compliance

- Compliant with ERC-721, ERC-1155, and ERC-2981 standards  
- Event-driven, auditable, and DAO-controllable  
- Designed for secure use in NFT marketplaces, airdrops, DeFi vaults, and DAO treasuries

---

## License

This module is part of the OCOS-Chain protocol. See [LICENSE](../../LICENSE) for terms.
