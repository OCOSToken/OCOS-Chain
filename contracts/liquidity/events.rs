//! OCOS-Chain: Liquidity Protocol Events Module
//!
//! Defines all on-chain events emitted by the liquidity, AMM, DEX, farming, oracle, and bridge contracts.

use crate::contracts::liquidity::types::{PoolId, TokenId, Amount, Address, OracleId, Price, BridgeId, ChainId, Timestamp};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiquidityEvent {
    PoolCreated {
        pool_id: PoolId,
        token_a: TokenId,
        token_b: TokenId,
    },
    LiquidityAdded {
        pool_id: PoolId,
        amount_a: Amount,
        amount_b: Amount,
        lp_minted: Amount,
    },
    LiquidityRemoved {
        pool_id: PoolId,
        amount_a: Amount,
        amount_b: Amount,
        lp_burned: Amount,
    },
    TokenSwapped {
        pool_id: PoolId,
        direction: crate::contracts::liquidity::amm::SwapDirection,
        amount_in: Amount,
        amount_out: Amount,
    },
    Staked {
        user: Address,
        amount: Amount,
    },
    Unstaked {
        user: Address,
        amount: Amount,
    },
    RewardsHarvested {
        user: Address,
        rewards: Amount,
    },
    OraclePriceUpdated {
        oracle_id: OracleId,
        token_id: TokenId,
        new_price: Price,
        updater: String,
    },
    BridgeLocked {
        bridge_id: BridgeId,
        token_id: TokenId,
        amount: Amount,
        from: Address,
        to_chain: ChainId,
        to_address: Address,
        timestamp: Timestamp,
    },
    BridgeUnlocked {
        bridge_id: BridgeId,
        token_id: TokenId,
        amount: Amount,
        to_address: Address,
        by_oracle: Address,
        timestamp: Timestamp,
        proof: Vec<u8>,
    },
    GovernanceProposalCreated {
        proposal_id: u64,
        kind: crate::contracts::liquidity::types::GovernanceProposalKind,
    },
    GovernanceVoteCast {
        proposal_id: u64,
        voter: Address,
        amount: Amount,
        approve: bool,
    },
    GovernanceProposalExecuted {
        proposal_id: u64,
    },
    GovernanceProposalRejected {
        proposal_id: u64,
    },
    // ...extendable for any new protocol events
}
