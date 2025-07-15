//! OCOS-Chain: Liquidity Pool Core Module
//!
//! Defines the base pool structure and logic for token pairs, LP tokens,
//! deposit/withdraw, and core invariant checks (e.g., constant product).

use crate::contracts::liquidity::types::{PoolId, TokenId, Amount, LPTokenId};
use crate::contracts::liquidity::error::LiquidityError;
use crate::contracts::liquidity::events::LiquidityEvent;

/// Basic token pair pool
#[derive(Debug, Clone)]
pub struct LiquidityPool {
    pub id: PoolId,
    pub token_a: TokenId,
    pub token_b: TokenId,
    pub reserve_a: Amount,
    pub reserve_b: Amount,
    pub lp_token: LPTokenId,
    pub total_lp_supply: Amount,
    pub fee_basis_points: u16, // e.g., 30 = 0.3%
}

impl LiquidityPool {
    /// Create a new pool
    pub fn new(
        id: PoolId,
        token_a: TokenId,
        token_b: TokenId,
        lp_token: LPTokenId,
        fee_basis_points: u16,
    ) -> Self {
        LiquidityPool {
            id,
            token_a,
            token_b,
            reserve_a: 0,
            reserve_b: 0,
            lp_token,
            total_lp_supply: 0,
            fee_basis_points,
        }
    }

    /// Deposit tokens to the pool (adds liquidity, mints LP tokens)
    pub fn deposit(
        &mut self,
        amount_a: Amount,
        amount_b: Amount,
        min_lp: Amount,
        events: &mut Vec<LiquidityEvent>,
    ) -> Result<Amount, LiquidityError> {
        // Invariant: maintain proportionality (simplified)
        let lp_minted = if self.total_lp_supply == 0 {
            // First deposit
            (amount_a * amount_b).integer_sqrt()
        } else {
            std::cmp::min(
                amount_a * self.total_lp_supply / self.reserve_a,
                amount_b * self.total_lp_supply / self.reserve_b,
            )
        };

        if lp_minted < min_lp {
            return Err(LiquidityError::SlippageExceeded);
        }
        self.reserve_a += amount_a;
        self.reserve_b += amount_b;
        self.total_lp_supply += lp_minted;
        events.push(LiquidityEvent::LiquidityAdded {
            pool_id: self.id,
            amount_a,
            amount_b,
            lp_minted,
        });
        Ok(lp_minted)
    }

    /// Withdraw tokens by burning LP tokens
    pub fn withdraw(
        &mut self,
        lp_amount: Amount,
        min_a: Amount,
        min_b: Amount,
        events: &mut Vec<LiquidityEvent>,
    ) -> Result<(Amount, Amount), LiquidityError> {
        if lp_amount > self.total_lp_supply || lp_amount == 0 {
            return Err(LiquidityError::InvalidAmount);
        }
        let amount_a = lp_amount * self.reserve_a / self.total_lp_supply;
        let amount_b = lp_amount * self.reserve_b / self.total_lp_supply;
        if amount_a < min_a || amount_b < min_b {
            return Err(LiquidityError::SlippageExceeded);
        }
        self.reserve_a -= amount_a;
        self.reserve_b -= amount_b;
        self.total_lp_supply -= lp_amount;
        events.push(LiquidityEvent::LiquidityRemoved {
            pool_id: self.id,
            amount_a,
            amount_b,
            lp_burned: lp_amount,
        });
        Ok((amount_a, amount_b))
    }

    /// Get price of token A in terms of token B (spot price)
    pub fn spot_price_a_to_b(&self) -> Option<f64> {
        if self.reserve_a == 0 || self.reserve_b == 0 {
            None
        } else {
            Some(self.reserve_b as f64 / self.reserve_a as f64)
        }
    }
}

// Integer square root (utility)
trait IntegerSqrt {
    fn integer_sqrt(self) -> Self;
}

impl IntegerSqrt for u128 {
    fn integer_sqrt(self) -> Self {
        (self as f64).sqrt().round() as u128
    }
}
