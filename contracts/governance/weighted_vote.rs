//! OCOS-Chain: Weighted Voting Module (Token/Stake/Reputation-based)
//!
//! Implements a weighted voting system for governance, supporting
//! stake, token, or reputation based voting power with quorum and threshold checks.

use crate::contracts::governance::types::{ProposalId, VoterId, VoteOption};
use crate::contracts::governance::error::GovernanceError;
use crate::contracts::governance::events::GovernanceEvent;
use crate::contracts::governance::storage::{VoteStorage, ProposalStorage};

/// Weighted vote structure for a proposal
#[derive(Debug, Clone)]
pub struct WeightedVote {
    pub voter_id: VoterId,
    pub weight: u128,
    pub option: VoteOption,
}

/// Weighted voting logic
pub struct WeightedVoting {
    pub quorum: u128,       // Minimum total voting power required
    pub threshold: u128,    // Votes required for proposal to pass
}

impl WeightedVoting {
    pub fn new(quorum: u128, threshold: u128) -> Self {
        WeightedVoting { quorum, threshold }
    }

    /// Cast a weighted vote
    pub fn cast_vote(
        &self,
        voter_id: VoterId,
        proposal_id: ProposalId,
        weight: u128,
        option: VoteOption,
        vote_storage: &mut VoteStorage,
        events: &mut Vec<GovernanceEvent>,
    ) -> Result<(), GovernanceError> {
        // Prevent double voting
        if vote_storage.has_voted(proposal_id, voter_id) {
            return Err(GovernanceError::AlreadyVoted);
        }
        let vote = WeightedVote { voter_id, weight, option: option.clone() };
        vote_storage.record_weighted_vote(proposal_id, vote.clone())?;
        events.push(GovernanceEvent::WeightedVoteCast {
            proposal_id,
            voter_id,
            weight,
            option,
        });
        Ok(())
    }

    /// Tally votes and determine if proposal passes
    pub fn tally(
        &self,
        proposal_id: ProposalId,
        vote_storage: &VoteStorage,
        proposal_storage: &mut ProposalStorage,
        events: &mut Vec<GovernanceEvent>,
    ) -> Result<bool, GovernanceError> {
        let (yes_weight, total_weight) = vote_storage.tally_votes(proposal_id, VoteOption::Yes);
        if total_weight < self.quorum {
            return Err(GovernanceError::QuorumNotMet);
        }
        let passed = yes_weight >= self.threshold;
        if passed {
            proposal_storage.mark_as_passed(proposal_id)?;
            events.push(GovernanceEvent::ProposalPassed { proposal_id });
        } else {
            proposal_storage.mark_as_failed(proposal_id)?;
            events.push(GovernanceEvent::ProposalFailed { proposal_id });
        }
        Ok(passed)
    }
}
