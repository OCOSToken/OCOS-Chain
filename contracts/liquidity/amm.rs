//! OCOS-Chain: Automated Market Maker (AMM) Module
//!
//! Implements the core AMM logic for token swaps, price discovery,
//! fee application, and invariant enforcement.

use crate::contracts::liquidity::types::{PoolId, TokenId, Amount};
use crate::contracts::liquidity::pool::LiquidityPool;
use crate::contracts::liquidity::error::LiquidityError;
use crate::contracts::liquidity::events::LiquidityEvent;

/// AMM swap direction
#[derive(Debug, Clone, Copy)]
pub enum SwapDirection {
    AtoB,
    BtoA,
}

pub struct AMM;

impl AMM {
    /// Swap token A for token B (or vice versa) in a given pool
    pub fn swap(
        pool: &mut LiquidityPool,
        amount_in: Amount,
        direction: SwapDirection,
        min_amount_out: Amount,
        events: &mut Vec<LiquidityEvent>,
    ) -> Result<Amount, LiquidityError> {
        if amount_in == 0 {
            return Err(LiquidityError::InvalidAmount);
        }

        let (reserve_in, reserve_out) = match direction {
            SwapDirection::AtoB => (&mut pool.reserve_a, &mut pool.reserve_b),
            SwapDirection::BtoA => (&mut pool.reserve_b, &mut pool.reserve_a),
        };

        // Apply fee (e.g., 0.3%)
        let amount_in_with_fee = amount_in * (10_000 - pool.fee_basis_points as u128) / 10_000;

        // Constant product invariant: (x + dx) * (y - dy) = x * y
        let numerator = amount_in_with_fee * *reserve_out;
        let denominator = *reserve_in + amount_in_with_fee;
        let amount_out = numerator / denominator;

        if amount_out < min_amount_out || amount_out == 0 {
            return Err(LiquidityError::SlippageExceeded);
        }

        *reserve_in += amount_in;
        *reserve_out -= amount_out;

        events.push(LiquidityEvent::TokenSwapped {
            pool_id: pool.id,
            direction,
            amount_in,
            amount_out,
        });

        Ok(amount_out)
    }

    /// Calculate spot price for a swap (A->B or B->A)
    pub fn spot_price(pool: &LiquidityPool, direction: SwapDirection) -> Option<f64> {
        match direction {
            SwapDirection::AtoB => {
                if pool.reserve_a == 0 { None } else { Some(pool.reserve_b as f64 / pool.reserve_a as f64) }
            }
            SwapDirection::BtoA => {
                if pool.reserve_b == 0 { None } else { Some(pool.reserve_a as f64 / pool.reserve_b as f64) }
            }
        }
    }
}
