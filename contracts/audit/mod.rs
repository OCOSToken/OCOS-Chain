//! OCOS-Chain: Audit Main Module
//!
//! Re-exports all audit submodules for unified integration and use.
//! Provides the central entry point and public interface for all on-chain/off-chain audit logic.

pub mod trace;
pub mod log;
pub mod event;
pub mod compliance;
pub mod hash;
pub mod proof;
pub mod index;
pub mod metrics;
pub mod types;
pub mod storage;
pub mod error;

#[cfg(test)]
pub mod tests;

// -- Re-export common public types and traits for external use --
pub use trace::*;
pub use log::*;
pub use event::*;
pub use compliance::*;
pub use hash::*;
pub use proof::*;
pub use index::*;
pub use metrics::*;
pub use types::*;
pub use storage::*;
pub use error::*;
