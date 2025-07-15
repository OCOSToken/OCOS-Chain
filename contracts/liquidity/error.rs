//! OCOS-Chain: Liquidity Protocol Error Types
//!
//! Defines all error codes for pools, AMMs, DEX, farming, oracle, and rewards.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiquidityError {
    Unauthorized,
    PoolExists,
    PoolNotFound,
    InsufficientLiquidity,
    SlippageExceeded,
    InvalidAmount,
    InvalidParameter,
    AlreadyStaked,
    NoStake,
    RewardNotAvailable,
    OracleNotFound,
    BridgeNotFound,
    InvalidProof,
    ProposalExists,
    ProposalNotFound,
    ProposalClosed,
    InvalidPayload,
    GovernanceRejected,
    NotAllowed,
    StorageError(String),
    Other(String),
}

impl fmt::Display for LiquidityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiquidityError::Unauthorized => write!(f, "Unauthorized operation"),
            LiquidityError::PoolExists => write!(f, "Liquidity pool already exists"),
            LiquidityError::PoolNotFound => write!(f, "Liquidity pool not found"),
            LiquidityError::InsufficientLiquidity => write!(f, "Insufficient liquidity"),
            LiquidityError::SlippageExceeded => write!(f, "Slippage tolerance exceeded"),
            LiquidityError::InvalidAmount => write!(f, "Invalid amount specified"),
            LiquidityError::InvalidParameter => write!(f, "Invalid configuration parameter"),
            LiquidityError::AlreadyStaked => write!(f, "User has already staked"),
            LiquidityError::NoStake => write!(f, "User has no stake in this pool"),
            LiquidityError::RewardNotAvailable => write!(f, "Reward not yet available"),
            LiquidityError::OracleNotFound => write!(f, "Oracle feed not found"),
            LiquidityError::BridgeNotFound => write!(f, "Bridge record not found"),
            LiquidityError::InvalidProof => write!(f, "Bridge or oracle proof is invalid"),
            LiquidityError::ProposalExists => write!(f, "Governance proposal already exists"),
            LiquidityError::ProposalNotFound => write!(f, "Governance proposal not found"),
            LiquidityError::ProposalClosed => write!(f, "Governance proposal is closed"),
            LiquidityError::InvalidPayload => write!(f, "Invalid payload data"),
            LiquidityError::GovernanceRejected => write!(f, "Governance rejected this operation"),
            LiquidityError::NotAllowed => write!(f, "Operation not allowed"),
            LiquidityError::StorageError(msg) => write!(f, "Storage error: {}", msg),
            LiquidityError::Other(msg) => write!(f, "Other liquidity error: {}", msg),
        }
    }
}

impl std::error::Error for LiquidityError {}
