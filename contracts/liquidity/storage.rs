//! OCOS-Chain: Liquidity Protocol Storage Module
//!
//! Implements persistent storage for pools, stakes, farming, oracle feeds,
//! rewards history, governance proposals, and bridge records.

use std::collections::HashMap;
use crate::contracts::liquidity::types::{
    PoolId, TokenId, Address, OracleId, BridgeId, ProposalId
};
use crate::contracts::liquidity::pool::LiquidityPool;
use crate::contracts::liquidity::rewards::{RewardCampaign, UserStakeInfo};
use crate::contracts::liquidity::oracle::OracleFeed;
use crate::contracts::liquidity::bridge::BridgeTransfer;
use crate::contracts::liquidity::governance::LiquidityGovernanceProposal;

/// All on-chain liquidity data (for demo: separated for real DB/contract storage)
#[derive(Default)]
pub struct LiquidityStorage {
    // PoolId → LiquidityPool
    pub pools: HashMap<PoolId, LiquidityPool>,
    // (PoolId, UserAddress) → UserStakeInfo
    pub stakes: HashMap<(PoolId, Address), UserStakeInfo>,
    // PoolId → RewardCampaign
    pub reward_campaigns: HashMap<PoolId, RewardCampaign>,
    // OracleId → OracleFeed
    pub oracles: HashMap<OracleId, OracleFeed>,
    // BridgeId → BridgeTransfer
    pub bridges: HashMap<BridgeId, BridgeTransfer>,
    // ProposalId → GovernanceProposal
    pub governance_proposals: HashMap<ProposalId, LiquidityGovernanceProposal>,
}

impl LiquidityStorage {
    // --- Pool Logic ---
    pub fn add_pool(&mut self, pool: LiquidityPool) {
        self.pools.insert(pool.id, pool);
    }
    pub fn get_pool(&self, pool_id: PoolId) -> Option<&LiquidityPool> {
        self.pools.get(&pool_id)
    }

    // --- Stake Logic ---
    pub fn stake(&mut self, pool_id: PoolId, user: Address, stake_info: UserStakeInfo) {
        self.stakes.insert((pool_id, user), stake_info);
    }
    pub fn get_stake(&self, pool_id: PoolId, user: Address) -> Option<&UserStakeInfo> {
        self.stakes.get(&(pool_id, user))
    }

    // --- Reward/Farming Campaigns ---
    pub fn add_reward_campaign(&mut self, pool_id: PoolId, campaign: RewardCampaign) {
        self.reward_campaigns.insert(pool_id, campaign);
    }
    pub fn get_reward_campaign(&self, pool_id: PoolId) -> Option<&RewardCampaign> {
        self.reward_campaigns.get(&pool_id)
    }

    // --- Oracle Feeds ---
    pub fn add_oracle_feed(&mut self, oracle: OracleFeed) {
        self.oracles.insert(oracle.oracle_id, oracle);
    }
    pub fn get_oracle_feed(&self, oracle_id: OracleId) -> Option<&OracleFeed> {
        self.oracles.get(&oracle_id)
    }

    // --- Bridge Transfers ---
    pub fn add_bridge(&mut self, bridge: BridgeTransfer) {
        self.bridges.insert(bridge.bridge_id, bridge);
    }
    pub fn get_bridge(&self, bridge_id: BridgeId) -> Option<&BridgeTransfer> {
        self.bridges.get(&bridge_id)
    }

    // --- Governance Proposals ---
    pub fn add_governance_proposal(&mut self, proposal: LiquidityGovernanceProposal) {
        self.governance_proposals.insert(proposal.id, proposal);
    }
    pub fn get_governance_proposal(&self, proposal_id: ProposalId) -> Option<&LiquidityGovernanceProposal> {
        self.governance_proposals.get(&proposal_id)
    }
}
