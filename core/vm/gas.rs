//! OCOS-Chain: Gas Metering Module
//!
//! Provides deterministic gas tracking and enforcement for all VM operations.
//! Ensures that execution is limited in compute and storage cost, and prevents
//! denial-of-service and runaway loops within smart contracts.

/// Maximum possible gas value for u64 (safety guard)
pub const MAX_GAS: u64 = u64::MAX;

/// Represents a gas limit assigned to a transaction or contract call
pub type GasLimit = u64;

/// Gas meter to track consumed gas during VM execution
#[derive(Debug, Clone)]
pub struct GasMeter {
    limit: GasLimit,
    consumed: u64,
}

impl GasMeter {
    /// Create a new gas meter with a given limit
    pub fn new(limit: GasLimit) -> Self {
        GasMeter {
            limit,
            consumed: 0,
        }
    }

    /// Consume a specific amount of gas units
    pub fn consume(&mut self, amount: u64) -> Result<(), GasError> {
        if self.consumed.saturating_add(amount) > self.limit {
            return Err(GasError::OutOfGas);
        }
        self.consumed += amount;
        Ok(())
    }

    /// Return how much gas has been used
    pub fn used(&self) -> u64 {
        self.consumed
    }

    /// Return how much gas remains
    pub fn remaining(&self) -> u64 {
        self.limit.saturating_sub(self.consumed)
    }

    /// Check if enough gas is available
    pub fn check(&self, amount: u64) -> Result<(), GasError> {
        if self.remaining() < amount {
            Err(GasError::OutOfGas)
        } else {
            Ok(())
        }
    }
}

/// Gas-related errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GasError {
    OutOfGas,
}
