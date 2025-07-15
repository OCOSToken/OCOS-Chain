//! OCOS-Chain: Liquidity & DEX Governance Module
//!
//! Enables DAO-driven governance for liquidity pools, AMMs, farming, rewards, oracle settings,
//! and protocol upgrades. Proposals, voting, and execution follow DAO rules.

use crate::contracts::liquidity::config::LiquidityConfig;
use crate::contracts::liquidity::types::{ProposalId, ProposalStatus, GovernanceProposalKind, Address, Amount};
use crate::contracts::liquidity::error::LiquidityError;
use crate::contracts::liquidity::events::LiquidityEvent;
use std::collections::HashMap;

/// Governance proposal for the liquidity module
#[derive(Debug, Clone)]
pub struct LiquidityGovernanceProposal {
    pub id: ProposalId,
    pub proposer: Address,
    pub kind: GovernanceProposalKind,
    pub status: ProposalStatus,
    pub created_at: u64,
    pub voting_end: u64,
    pub yes_votes: Amount,
    pub no_votes: Amount,
    pub executed: bool,
    pub data: Option<Vec<u8>>, // Custom data for config/upgrade proposals
}

/// Governance registry and execution logic for liquidity proposals
pub struct LiquidityGovernance {
    pub proposals: HashMap<ProposalId, LiquidityGovernanceProposal>,
    pub config: LiquidityConfig,
}

impl LiquidityGovernance {
    pub fn new(config: LiquidityConfig) -> Self {
        Self {
            proposals: HashMap::new(),
            config,
        }
    }

    /// Submit a new proposal (e.g., change swap fee, launch new pool, etc.)
    pub fn submit_proposal(
        &mut self,
        id: ProposalId,
        proposer: Address,
        kind: GovernanceProposalKind,
        created_at: u64,
        voting_end: u64,
        data: Option<Vec<u8>>,
        events: &mut Vec<LiquidityEvent>,
    ) -> Result<(), LiquidityError> {
        if self.proposals.contains_key(&id) {
            return Err(LiquidityError::ProposalExists);
        }
        let proposal = LiquidityGovernanceProposal {
            id,
            proposer,
            kind: kind.clone(),
            status: ProposalStatus::Pending,
            created_at,
            voting_end,
            yes_votes: 0,
            no_votes: 0,
            executed: false,
            data,
        };
        self.proposals.insert(id, proposal);
        events.push(LiquidityEvent::GovernanceProposalCreated { proposal_id: id, kind });
        Ok(())
    }

    /// Vote on a proposal
    pub fn vote(
        &mut self,
        proposal_id: ProposalId,
        voter: Address,
        amount: Amount,
        approve: bool,
        events: &mut Vec<LiquidityEvent>,
    ) -> Result<(), LiquidityError> {
        let proposal = self.proposals.get_mut(&proposal_id).ok_or(LiquidityError::ProposalNotFound)?;
        if proposal.executed || proposal.status != ProposalStatus::Pending {
            return Err(LiquidityError::ProposalClosed);
        }
        if approve {
            proposal.yes_votes += amount;
        } else {
            proposal.no_votes += amount;
        }
        events.push(LiquidityEvent::GovernanceVoteCast { proposal_id, voter, amount, approve });
        Ok(())
    }

    /// Execute a successful proposal (called after voting period)
    pub fn execute(
        &mut self,
        proposal_id: ProposalId,
        now: u64,
        events: &mut Vec<LiquidityEvent>,
    ) -> Result<(), LiquidityError> {
        let proposal = self.proposals.get_mut(&proposal_id).ok_or(LiquidityError::ProposalNotFound)?;
        if proposal.executed || proposal.status != ProposalStatus::Pending || now < proposal.voting_end {
            return Err(LiquidityError::ProposalClosed);
        }
        if proposal.yes_votes <= proposal.no_votes {
            proposal.status = ProposalStatus::Rejected;
            events.push(LiquidityEvent::GovernanceProposalRejected { proposal_id });
            return Ok(());
        }
        // Example: Handle config update proposals
        if let GovernanceProposalKind::ConfigUpdate { key } = &proposal.kind {
            if let Some(ref data) = proposal.data {
                let value = Self::parse_u128_payload(data)?;
                self.config.update(key, value)?;
            }
        }
        proposal.status = ProposalStatus::Executed;
        proposal.executed = true;
        events.push(LiquidityEvent::GovernanceProposalExecuted { proposal_id });
        Ok(())
    }

    fn parse_u128_payload(data: &Vec<u8>) -> Result<u128, LiquidityError> {
        if data.len() == 16 {
            let mut arr = [0u8; 16];
            arr.copy_from_slice(&data[..16]);
            Ok(u128::from_be_bytes(arr))
        } else {
            Err(LiquidityError::InvalidPayload)
        }
    }
}
