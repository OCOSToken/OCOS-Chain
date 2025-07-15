//! OCOS-Chain: On-Chain Governance Hook & Proposal System
//!
//! Modular on-chain governance engine supporting proposals, voting,
//! dynamic consensus/config updates, and audit logging. Designed for
//! fully decentralized, DAO-driven protocol management.

use std::collections::HashMap;

/// Governance action status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProposalStatus {
    Pending,
    Approved,
    Rejected,
    Executed,
    Expired,
}

/// Governance proposal structure
#[derive(Debug, Clone)]
pub struct Proposal {
    pub id: u64,
    pub proposer: String,
    pub title: String,
    pub description: String,
    pub action_key: String,     // Target config/protocol parameter (e.g., "block_time")
    pub action_value: String,   // Proposed new value
    pub votes_for: u64,
    pub votes_against: u64,
    pub start_time: u64,
    pub end_time: u64,
    pub status: ProposalStatus,
    pub metadata: Option<HashMap<String, String>>, // Custom (KYC, tags, etc)
}

/// Governance state and hooks
#[derive(Debug, Default, Clone)]
pub struct GovernanceHook {
    pub proposals: HashMap<u64, Proposal>,
    pub current_config: HashMap<String, String>, // On-chain key-value config store
    pub next_proposal_id: u64,
}

impl GovernanceHook {
    /// Create a new governance hook
    pub fn default() -> Self {
        GovernanceHook {
            proposals: HashMap::new(),
            current_config: HashMap::new(),
            next_proposal_id: 1,
        }
    }

    /// Submit a new proposal (returns proposal ID)
    pub fn submit_proposal(&mut self, proposer: String, title: String, desc: String, key: String, value: String, voting_period_secs: u64) -> u64 {
        let now = Self::current_unix_timestamp();
        let prop = Proposal {
            id: self.next_proposal_id,
            proposer,
            title,
            description: desc,
            action_key: key,
            action_value: value,
            votes_for: 0,
            votes_against: 0,
            start_time: now,
            end_time: now + voting_period_secs,
            status: ProposalStatus::Pending,
            metadata: None,
        };
        self.proposals.insert(self.next_proposal_id, prop);
        self.next_proposal_id += 1;
        self.next_proposal_id - 1
    }

    /// Vote on a proposal (true = for, false = against)
    pub fn vote(&mut self, proposal_id: u64, for_vote: bool) -> Option<ProposalStatus> {
        let prop = self.proposals.get_mut(&proposal_id)?;
        if prop.status != ProposalStatus::Pending || Self::current_unix_timestamp() > prop.end_time {
            return None;
        }
        if for_vote {
            prop.votes_for += 1;
        } else {
            prop.votes_against += 1;
        }
        Some(prop.status.clone())
    }

    /// Finalize a proposal (can be called by anyone after voting period)
    pub fn finalize_proposal(&mut self, proposal_id: u64) -> Option<ProposalStatus> {
        let prop = self.proposals.get_mut(&proposal_id)?;
        let now = Self::current_unix_timestamp();
        if prop.status != ProposalStatus::Pending || now < prop.end_time {
            return None;
        }
        if prop.votes_for > prop.votes_against {
            prop.status = ProposalStatus::Approved;
        } else {
            prop.status = ProposalStatus::Rejected;
        }
        Some(prop.status.clone())
    }

    /// Apply an approved proposal (on-chain config update)
    pub fn apply_update(&mut self, key: &str, value: &str) -> bool {
        self.current_config.insert(key.to_string(), value.to_string());
        true
    }

    /// Governance policy check for updates (can be extended with multi-sig, DAO token voting, etc)
    pub fn validate_update(&self, _key: &str, _value: &str) -> bool {
        // Always true. Real: Check DAO rules, token weights, KYC, etc.
        true
    }

    /// Get current config parameter value
    pub fn get_config(&self, key: &str) -> Option<&String> {
        self.current_config.get(key)
    }

    /// Helper: UNIX timestamp
    fn current_unix_timestamp() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }
}
