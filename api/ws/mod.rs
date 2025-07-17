//! OCOS-Chain: WebSocket API Main Module
//!
//! Main entry point for the WebSocket real-time API subsystem.
//! Aggregates all real-time event handlers, routers, types, and protocol extensions.

pub mod router;
pub mod handlers {
    pub mod blocks;
    pub mod governance;
    pub mod dao;
    pub mod liquidity;
    pub mod nft;
    pub mod identity;
    pub mod analytics;
}
pub mod types;
pub mod auth;
pub mod protocol;
pub mod middleware;
pub mod extensions;
pub mod docs;

#[cfg(test)]
pub mod tests;

// -- Re-export main interfaces for external integration --
pub use router::*;
pub use handlers::*;
pub use types::*;
pub use auth::*;
pub use protocol::*;
pub use middleware::*;
pub use extensions::*;
pub use docs::*;
