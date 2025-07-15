//! OCOS-Chain: Governance Config & Parameters Module
//!
//! Provides on-chain adjustable parameters for all governance mechanisms:
//! - voting durations, quorums, thresholds, council sizes, delegation limits, etc.

use crate::contracts::governance::error::GovernanceError;

/// Config structure holding all governance parameters
#[derive(Debug, Clone)]
pub struct GovernanceConfig {
    pub min_voting_period: u64,        // Minimum voting period in seconds
    pub max_voting_period: u64,        // Maximum voting period in seconds
    pub proposal_quorum: u128,         // Minimum total voting power required
    pub proposal_threshold: u128,      // Votes required to pass
    pub council_size: u8,              // Number of council seats
    pub delegation_limit: u8,          // Max number of active delegations per account
    pub min_stake: u128,               // Minimum stake to create/vote proposal
    pub exec_delay: u64,               // Delay before execution after passing (seconds)
}

impl GovernanceConfig {
    pub fn default() -> Self {
        GovernanceConfig {
            min_voting_period: 60 * 60 * 24,         // 1 day
            max_voting_period: 60 * 60 * 24 * 14,    // 14 days
            proposal_quorum: 10_000,
            proposal_threshold: 5_000,
            council_size: 9,
            delegation_limit: 3,
            min_stake: 1_000,
            exec_delay: 60 * 60,                     // 1 hour
        }
    }

    /// Update a governance parameter by name (DAO proposal-driven)
    pub fn update(&mut self, key: &str, value: u128) -> Result<(), GovernanceError> {
        match key {
            "min_voting_period" => { self.min_voting_period = value as u64; }
            "max_voting_period" => { self.max_voting_period = value as u64; }
            "proposal_quorum" => { self.proposal_quorum = value; }
            "proposal_threshold" => { self.proposal_threshold = value; }
            "council_size" => { self.council_size = value as u8; }
            "delegation_limit" => { self.delegation_limit = value as u8; }
            "min_stake" => { self.min_stake = value; }
            "exec_delay" => { self.exec_delay = value as u64; }
            _ => return Err(GovernanceError::InvalidParameter),
        }
        Ok(())
    }
}
