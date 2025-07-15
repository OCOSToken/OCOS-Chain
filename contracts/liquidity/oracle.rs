//! OCOS-Chain: Liquidity Oracle Module
//!
//! Provides secure and updatable price feeds, index values, and external data
//! for on-chain DeFi, DEX, and treasury operations.

use crate::contracts::liquidity::types::{TokenId, Price, Timestamp, OracleId};
use crate::contracts::liquidity::error::LiquidityError;
use crate::contracts::liquidity::events::LiquidityEvent;
use std::collections::HashMap;

/// Structure representing a single oracle feed
#[derive(Debug, Clone)]
pub struct OracleFeed {
    pub oracle_id: OracleId,
    pub token_id: TokenId,
    pub price: Price,
    pub last_updated: Timestamp,
    pub source: String,
}

/// Oracle registry and update logic
pub struct OracleRegistry {
    pub feeds: HashMap<OracleId, OracleFeed>,
    pub authorized_updaters: Vec<String>, // e.g., authorized relayer or oracle node addresses
}

impl OracleRegistry {
    pub fn new() -> Self {
        OracleRegistry {
            feeds: HashMap::new(),
            authorized_updaters: vec![],
        }
    }

    /// Register a new oracle feed
    pub fn register_feed(
        &mut self,
        oracle_id: OracleId,
        token_id: TokenId,
        source: String,
        initial_price: Price,
        now: Timestamp,
    ) {
        let feed = OracleFeed {
            oracle_id,
            token_id,
            price: initial_price,
            last_updated: now,
            source,
        };
        self.feeds.insert(oracle_id, feed);
    }

    /// Authorize an updater (oracle node, relayer, etc.)
    pub fn authorize_updater(&mut self, updater: String) {
        if !self.authorized_updaters.contains(&updater) {
            self.authorized_updaters.push(updater);
        }
    }

    /// Update price for an oracle feed
    pub fn update_price(
        &mut self,
        oracle_id: OracleId,
        new_price: Price,
        updater: &str,
        now: Timestamp,
        events: &mut Vec<LiquidityEvent>,
    ) -> Result<(), LiquidityError> {
        // Security: Only authorized updaters
        if !self.authorized_updaters.contains(&updater.to_string()) {
            return Err(LiquidityError::Unauthorized);
        }
        let feed = self.feeds.get_mut(&oracle_id).ok_or(LiquidityError::OracleNotFound)?;
        feed.price = new_price;
        feed.last_updated = now;
        events.push(LiquidityEvent::OraclePriceUpdated {
            oracle_id,
            token_id: feed.token_id,
            new_price,
            updater: updater.to_string(),
        });
        Ok(())
    }

    /// Query price for a given token/oracle
    pub fn get_price(&self, oracle_id: OracleId) -> Option<Price> {
        self.feeds.get(&oracle_id).map(|f| f.price)
    }
}
