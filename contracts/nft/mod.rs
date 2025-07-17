//! OCOS-Chain: NFT Main Module
//!
//! Exports all submodules for NFT, collections, royalty, governance, marketplace,
//! auctions, and events. Provides the unified entry point and public API for the
//! OCOS NFT protocol.

pub mod token;
pub mod metadata;
pub mod collection;
pub mod royalty;
pub mod governance;
pub mod marketplace;
pub mod auction;
pub mod error;
pub mod events;
pub mod types;
pub mod storage;

#[cfg(test)]
pub mod tests;

// -- Re-export commonly used types and interfaces for external modules --
pub use token::*;
pub use metadata::*;
pub use collection::*;
pub use royalty::*;
pub use governance::*;
pub use marketplace::*;
pub use auction::*;
pub use error::*;
pub use events::*;
pub use types::*;
pub use storage::*;
