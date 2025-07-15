//! OCOS-Chain: Governance Proposal Management Module
//!
//! Provides proposal lifecycle management: creation, status, voting periods,
//! approval/rejection, execution, expiration, and history tracking.

use crate::contracts::governance::types::{ProposalId, VoterId, ProposalStatus, ProposalKind, VoteOption};
use crate::contracts::governance::error::GovernanceError;
use crate::contracts::governance::events::GovernanceEvent;
use crate::contracts::governance::storage::ProposalStorage;

/// Governance proposal structure
#[derive(Debug, Clone)]
pub struct Proposal {
    pub id: ProposalId,
    pub proposer: VoterId,
    pub kind: ProposalKind,
    pub description: String,
    pub status: ProposalStatus,
    pub created_at: u64,         // timestamp
    pub voting_start: u64,       // timestamp
    pub voting_end: u64,         // timestamp
    pub executed_at: Option<u64>,
    pub yes_votes: u128,
    pub no_votes: u128,
    pub data: Option<Vec<u8>>,   // encoded proposal payload
}

impl Proposal {
    pub fn new(
        id: ProposalId,
        proposer: VoterId,
        kind: ProposalKind,
        description: String,
        created_at: u64,
        voting_start: u64,
        voting_end: u64,
        data: Option<Vec<u8>>,
    ) -> Self {
        Proposal {
            id,
            proposer,
            kind,
            description,
            status: ProposalStatus::Pending,
            created_at,
            voting_start,
            voting_end,
            executed_at: None,
            yes_votes: 0,
            no_votes: 0,
            data,
        }
    }

    /// Mark proposal as approved and record event
    pub fn approve(
        &mut self,
        now: u64,
        events: &mut Vec<GovernanceEvent>,
    ) {
        self.status = ProposalStatus::Approved;
        self.executed_at = Some(now);
        events.push(GovernanceEvent::ProposalApproved { proposal_id: self.id });
    }

    /// Mark proposal as rejected and record event
    pub fn reject(
        &mut self,
        now: u64,
        events: &mut Vec<GovernanceEvent>,
    ) {
        self.status = ProposalStatus::Rejected;
        self.executed_at = Some(now);
        events.push(GovernanceEvent::ProposalRejected { proposal_id: self.id });
    }

    /// Execute proposal (calls on-chain action or upgrade)
    pub fn execute(
        &mut self,
        now: u64,
        storage: &mut ProposalStorage,
        events: &mut Vec<GovernanceEvent>,
    ) -> Result<(), GovernanceError> {
        if self.status != ProposalStatus::Approved {
            return Err(GovernanceError::ProposalNotApproved);
        }
        self.status = ProposalStatus::Executed;
        self.executed_at = Some(now);
        storage.mark_as_executed(self.id)?;
        events.push(GovernanceEvent::ProposalExecuted { proposal_id: self.id });
        Ok(())
    }

    /// Check if voting is open at a given timestamp
    pub fn voting_open(&self, now: u64) -> bool {
        self.status == ProposalStatus::Pending
            && now >= self.voting_start
            && now <= self.voting_end
    }
}
