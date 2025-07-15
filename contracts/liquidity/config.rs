//! OCOS-Chain: Liquidity Protocol Config Module
//!
//! On-chain adjustable parameters for pools, AMMs, farming, rewards, and oracles.
//! All config changes are governance-driven and fully auditable.

use crate::contracts::liquidity::error::LiquidityError;

/// Liquidity/AMM/DEX config structure
#[derive(Debug, Clone)]
pub struct LiquidityConfig {
    pub default_swap_fee_bps: u16,     // Swap fee (basis points, e.g. 30 = 0.3%)
    pub min_liquidity: u128,           // Minimum required liquidity for pool
    pub farming_reward_per_block: u128,// Default farming reward/block
    pub max_pools: u16,                // Maximum number of active pools
    pub oracle_update_interval: u64,   // Min interval (seconds) between price updates
    pub governance_param: u64,         // Reserved for governance-controlled parameter
}

impl LiquidityConfig {
    pub fn default() -> Self {
        LiquidityConfig {
            default_swap_fee_bps: 30,        // 0.3%
            min_liquidity: 1_000u128,
            farming_reward_per_block: 10u128,
            max_pools: 100,
            oracle_update_interval: 60,       // 1 minute
            governance_param: 0,
        }
    }

    /// Update a liquidity parameter by name (governance action)
    pub fn update(&mut self, key: &str, value: u128) -> Result<(), LiquidityError> {
        match key {
            "default_swap_fee_bps" => { self.default_swap_fee_bps = value as u16; }
            "min_liquidity" => { self.min_liquidity = value; }
            "farming_reward_per_block" => { self.farming_reward_per_block = value; }
            "max_pools" => { self.max_pools = value as u16; }
            "oracle_update_interval" => { self.oracle_update_interval = value as u64; }
            "governance_param" => { self.governance_param = value as u64; }
            _ => return Err(LiquidityError::InvalidParameter),
        }
        Ok(())
    }
}
