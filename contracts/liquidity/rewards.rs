//! OCOS-Chain: Liquidity Rewards Module
//!
//! Handles reward distribution for liquidity providers and stakers,
//! including real-time accrual, harvesting, and farming campaigns.

use crate::contracts::liquidity::types::{PoolId, Address, Amount};
use crate::contracts::liquidity::events::LiquidityEvent;
use crate::contracts::liquidity::error::LiquidityError;

/// Information about a reward campaign
#[derive(Debug, Clone)]
pub struct RewardCampaign {
    pub pool_id: PoolId,
    pub reward_token: Address,
    pub start_block: u64,
    pub end_block: u64,
    pub reward_per_block: Amount,
    pub total_allocated: Amount,
    pub claimed: Amount,
}

/// State of a user's stake and reward accrual
#[derive(Debug, Clone, Default)]
pub struct UserStakeInfo {
    pub amount_staked: Amount,
    pub reward_debt: Amount,
    pub pending_rewards: Amount,
    pub last_reward_block: u64,
}

/// Rewards engine for a liquidity pool or farming contract
pub struct RewardsEngine {
    pub campaign: RewardCampaign,
    pub user_info: std::collections::HashMap<Address, UserStakeInfo>,
    pub total_staked: Amount,
}

impl RewardsEngine {
    pub fn new(campaign: RewardCampaign) -> Self {
        RewardsEngine {
            campaign,
            user_info: Default::default(),
            total_staked: 0,
        }
    }

    /// Deposit (stake) LP tokens to participate in rewards
    pub fn deposit(
        &mut self,
        user: Address,
        amount: Amount,
        current_block: u64,
        events: &mut Vec<LiquidityEvent>,
    ) -> Result<(), LiquidityError> {
        self.update_rewards(user, current_block)?;
        let info = self.user_info.entry(user).or_default();
        info.amount_staked += amount;
        self.total_staked += amount;
        events.push(LiquidityEvent::Staked { user, amount });
        Ok(())
    }

    /// Withdraw LP tokens and harvest pending rewards
    pub fn withdraw(
        &mut self,
        user: Address,
        amount: Amount,
        current_block: u64,
        events: &mut Vec<LiquidityEvent>,
    ) -> Result<Amount, LiquidityError> {
        let info = self.user_info.get_mut(&user).ok_or(LiquidityError::NoStake)?;
        if info.amount_staked < amount {
            return Err(LiquidityError::InvalidAmount);
        }
        self.update_rewards(user, current_block)?;
        info.amount_staked -= amount;
        self.total_staked -= amount;
        let rewards = info.pending_rewards;
        info.pending_rewards = 0;
        events.push(LiquidityEvent::Unstaked { user, amount });
        events.push(LiquidityEvent::RewardsHarvested { user, rewards });
        Ok(rewards)
    }

    /// View and harvest accrued rewards
    pub fn harvest(
        &mut self,
        user: Address,
        current_block: u64,
        events: &mut Vec<LiquidityEvent>,
    ) -> Result<Amount, LiquidityError> {
        self.update_rewards(user, current_block)?;
        let info = self.user_info.get_mut(&user).ok_or(LiquidityError::NoStake)?;
        let rewards = info.pending_rewards;
        info.pending_rewards = 0;
        info.reward_debt = self.accumulated_reward(user, current_block);
        events.push(LiquidityEvent::RewardsHarvested { user, rewards });
        Ok(rewards)
    }

    /// Update a user's accrued rewards up to the current block
    fn update_rewards(&mut self, user: Address, current_block: u64) -> Result<(), LiquidityError> {
        let info = self.user_info.entry(user).or_default();
        if current_block <= info.last_reward_block {
            return Ok(());
        }
        let staked = info.amount_staked;
        if staked > 0 && self.total_staked > 0 {
            let blocks = current_block - info.last_reward_block;
            let reward = blocks as u128 * self.campaign.reward_per_block * staked / self.total_staked;
            info.pending_rewards += reward;
            self.campaign.claimed += reward;
        }
        info.last_reward_block = current_block;
        Ok(())
    }

    /// Calculate total accumulated rewards for a user (for view/harvest)
    fn accumulated_reward(&self, user: Address, current_block: u64) -> Amount {
        let info = self.user_info.get(&user).unwrap_or(&UserStakeInfo::default());
        if info.amount_staked == 0 || self.total_staked == 0 {
            0
        } else {
            let blocks = current_block - info.last_reward_block;
            info.pending_rewards + (blocks as u128 * self.campaign.reward_per_block * info.amount_staked / self.total_staked)
        }
    }
}
