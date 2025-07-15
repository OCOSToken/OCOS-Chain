//! OCOS-Chain: Cross-Chain Bridge Module
//!
//! Manages minting, burning, and transferring tokens/assets across multiple chains.
//! Supports locking/unlocking, proofs, event emission, and oracle verification.

use crate::contracts::liquidity::types::{BridgeId, TokenId, Amount, Address, ChainId, Proof, Timestamp};
use crate::contracts::liquidity::error::LiquidityError;
use crate::contracts::liquidity::events::LiquidityEvent;
use std::collections::HashMap;

/// Bridge lock/burn event record
#[derive(Debug, Clone)]
pub struct BridgeTransfer {
    pub bridge_id: BridgeId,
    pub token_id: TokenId,
    pub amount: Amount,
    pub from: Address,
    pub to_chain: ChainId,
    pub to_address: Address,
    pub locked: bool,
    pub timestamp: Timestamp,
    pub proof: Option<Proof>, // Optional: off-chain proof (SPV, ZKP, signature, etc.)
}

/// Cross-chain bridge registry and logic
pub struct BridgeRegistry {
    pub records: HashMap<BridgeId, BridgeTransfer>,
    pub authorized_oracles: Vec<Address>,
}

impl BridgeRegistry {
    pub fn new() -> Self {
        BridgeRegistry {
            records: HashMap::new(),
            authorized_oracles: vec![],
        }
    }

    /// Lock (or burn) tokens for bridging to another chain
    pub fn lock_or_burn(
        &mut self,
        bridge_id: BridgeId,
        token_id: TokenId,
        amount: Amount,
        from: Address,
        to_chain: ChainId,
        to_address: Address,
        now: Timestamp,
        events: &mut Vec<LiquidityEvent>,
    ) -> Result<(), LiquidityError> {
        let record = BridgeTransfer {
            bridge_id,
            token_id,
            amount,
            from,
            to_chain,
            to_address,
            locked: true,
            timestamp: now,
            proof: None,
        };
        self.records.insert(bridge_id, record);
        events.push(LiquidityEvent::BridgeLocked {
            bridge_id,
            token_id,
            amount,
            from,
            to_chain,
            to_address,
            timestamp: now,
        });
        Ok(())
    }

    /// Unlock (or mint) tokens after proof is verified
    pub fn unlock_or_mint(
        &mut self,
        bridge_id: BridgeId,
        proof: Proof,
        oracle: Address,
        now: Timestamp,
        events: &mut Vec<LiquidityEvent>,
    ) -> Result<(), LiquidityError> {
        // Security: Only authorized oracles can unlock
        if !self.authorized_oracles.contains(&oracle) {
            return Err(LiquidityError::Unauthorized);
        }
        let record = self.records.get_mut(&bridge_id).ok_or(LiquidityError::BridgeNotFound)?;
        if !record.locked {
            return Err(LiquidityError::InvalidOperation);
        }
        // Optional: Additional proof verification logic here (SPV, ZKP, etc.)
        record.locked = false;
        record.proof = Some(proof.clone());
        events.push(LiquidityEvent::BridgeUnlocked {
            bridge_id,
            token_id: record.token_id,
            amount: record.amount,
            to_address: record.to_address,
            by_oracle: oracle,
            timestamp: now,
            proof,
        });
        Ok(())
    }

    /// Authorize a new oracle for bridge verification
    pub fn authorize_oracle(&mut self, oracle: Address) {
        if !self.authorized_oracles.contains(&oracle) {
            self.authorized_oracles.push(oracle);
        }
    }
}
