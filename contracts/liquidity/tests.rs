//! OCOS-Chain: Liquidity & DEX Module Tests
//!
//! Covers pool operations, AMM swaps, staking/rewards, oracles, and governance integration.

use crate::contracts::liquidity::{
    types::*,
    pool::LiquidityPool,
    amm::{AMM, SwapDirection},
    rewards::RewardsEngine,
    oracle::OracleRegistry,
    governance::{LiquidityGovernance, GovernanceProposalKind},
    config::LiquidityConfig,
    storage::LiquidityStorage,
    error::LiquidityError,
};

fn basic_pool() -> LiquidityPool {
    LiquidityPool::new(1, 10, 11, 101, 30)
}

#[test]
fn test_pool_deposit_and_withdraw() {
    let mut pool = basic_pool();
    let mut events = vec![];
    let lp = pool.deposit(1_000, 1_000, 1_000, &mut events).unwrap();
    assert!(lp > 0);
    let (a, b) = pool.withdraw(lp, 1, 1, &mut events).unwrap();
    assert_eq!(a, 1_000);
    assert_eq!(b, 1_000);
}

#[test]
fn test_amm_swap_and_invariant() {
    let mut pool = basic_pool();
    let mut events = vec![];
    pool.deposit(10_000, 20_000, 1, &mut events).unwrap();
    let out = AMM::swap(&mut pool, 1_000, SwapDirection::AtoB, 1, &mut events).unwrap();
    assert!(out > 0);
    // Ensure invariant (x * y ~ constant)
    let k = pool.reserve_a * pool.reserve_b;
    let approx_k = (10_000 + 1_000) * (20_000 - out);
    assert!((k as i128 - approx_k as i128).abs() < 1000);
}

#[test]
fn test_staking_and_rewards() {
    let mut engine = RewardsEngine::new(crate::contracts::liquidity::rewards::RewardCampaign {
        pool_id: 1,
        reward_token: [2u8; 20],
        start_block: 0,
        end_block: 100,
        reward_per_block: 100,
        total_allocated: 10_000,
        claimed: 0,
    });
    let mut events = vec![];
    let user = [1u8; 20];
    engine.deposit(user, 1_000, 1, &mut events).unwrap();
    engine.harvest(user, 10, &mut events).unwrap();
    let info = engine.user_info.get(&user).unwrap();
    assert!(info.pending_rewards == 0);
}

#[test]
fn test_oracle_registry_update_and_query() {
    let mut registry = OracleRegistry::new();
    let updater = "relayer1".to_string();
    registry.authorize_updater(updater.clone());
    registry.register_feed(1, 10, "binance".to_string(), 100_000, 0);
    let mut events = vec![];
    assert!(registry.update_price(1, 101_000, &updater, 2, &mut events).is_ok());
    let price = registry.get_price(1).unwrap();
    assert_eq!(price, 101_000);
}

#[test]
fn test_governance_proposal_submit_vote_execute() {
    let mut governance = LiquidityGovernance::new(LiquidityConfig::default());
    let mut events = vec![];
    let proposal_id = 99;
    let proposer = [5u8; 20];
    governance.submit_proposal(
        proposal_id,
        proposer,
        GovernanceProposalKind::ConfigUpdate { key: "min_liquidity".to_string() },
        1,
        10,
        Some(1_234_567_890u128.to_be_bytes().to_vec()),
        &mut events,
    ).unwrap();
    let voter = [6u8; 20];
    governance.vote(proposal_id, voter, 10_000, true, &mut events).unwrap();
    governance.execute(proposal_id, 11, &mut events).unwrap();
    assert_eq!(governance.proposals.get(&proposal_id).unwrap().status, ProposalStatus::Executed);
}
