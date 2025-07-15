//! OCOS-Chain DAO: Configuration Module
//!
//! Provides on-chain configurable DAO parameters: quorum, thresholds, voting period, etc.
//! Parameters are updatable by approved governance proposals for DAO upgradeability.

use crate::contracts::dao::types::DaoConfig;
use crate::contracts::dao::storage::{load_config, store_config};
use crate::contracts::dao::error::DaoError;

/// Update DAO configuration parameters (on-chain)
pub fn update_config(
    new_config: DaoConfig,
) -> Result<(), DaoError> {
    // In production: enforce access control (only governance can update)
    store_config(&new_config)
}

/// Get current DAO configuration
pub fn get_config() -> Result<DaoConfig, DaoError> {
    load_config()
}
