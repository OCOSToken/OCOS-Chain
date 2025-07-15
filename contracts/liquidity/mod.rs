//! OCOS-Chain: Liquidity Main Module
//!
//! Entry point and public interface for all on-chain liquidity contracts,
//! including pools, AMMs, staking, farming, and DEX logic.

pub mod pool;
pub mod amm;
pub mod staking;
pub mod farming;
pub mod router;
pub mod fee;
pub mod types;
pub mod error;
pub mod events;
pub mod storage;

#[cfg(test)]
pub mod tests;

// -- Re-export key public interfaces for external integration --
pub use pool::*;
pub use amm::*;
pub use staking::*;
pub use farming::*;
pub use router::*;
pub use fee::*;
pub use types::*;
pub use error::*;
pub use events::*;
pub use storage::*;
