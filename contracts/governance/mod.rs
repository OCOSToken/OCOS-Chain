//! OCOS-Chain: Governance Main Module
//!
//! Re-exports all governance contract submodules for unified integration and use.
//! Provides the central entry point and public interface for on-chain governance logic.

pub mod council;
pub mod weighted_vote;
pub mod delegation;
pub mod referendum;
pub mod proposal;
pub mod config;
pub mod execution;
pub mod registry;
pub mod error;
pub mod types;
pub mod events;
pub mod storage;

#[cfg(test)]
pub mod tests;

// -- Re-export common public types and traits for external use --
pub use council::*;
pub use weighted_vote::*;
pub use delegation::*;
pub use referendum::*;
pub use proposal::*;
pub use config::*;
pub use execution::*;
pub use registry::*;
pub use error::*;
pub use types::*;
pub use events::*;
pub use storage::*;
