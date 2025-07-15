//! OCOS-Chain: Referendum (On-chain Plebiscite) Module
//!
//! Enables DAO-wide referenda with support for binary (yes/no), approval, and
//! quadratic voting modes. Used for protocol upgrades, key parameter changes,
//! and any governance decision requiring broad legitimacy.

use crate::contracts::governance::types::{ProposalId, VoterId, VoteOption};
use crate::contracts::governance::error::GovernanceError;
use crate::contracts::governance::events::GovernanceEvent;
use crate::contracts::governance::storage::{VoteStorage, ProposalStorage};

/// Referendum voting modes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReferendumMode {
    Binary,     // Yes/No
    Approval,   // Multiple choices
    Quadratic,  // Quadratic cost voting (e.g. 1,4,9 weight)
}

/// Structure for referendum configuration
#[derive(Debug, Clone)]
pub struct ReferendumConfig {
    pub mode: ReferendumMode,
    pub quorum: u128,
    pub threshold: u128,
    pub options: Vec<VoteOption>, // Used for approval voting
}

impl ReferendumConfig {
    pub fn default_binary() -> Self {
        ReferendumConfig {
            mode: ReferendumMode::Binary,
            quorum: 0,
            threshold: 0,
            options: vec![VoteOption::Yes, VoteOption::No],
        }
    }
}

/// Referendum voting logic
pub struct ReferendumEngine {
    pub config: ReferendumConfig,
}

impl ReferendumEngine {
    pub fn new(config: ReferendumConfig) -> Self {
        ReferendumEngine { config }
    }

    /// Cast a referendum vote (binary, approval, quadratic)
    pub fn cast_vote(
        &self,
        voter_id: VoterId,
        proposal_id: ProposalId,
        selected_option: VoteOption,
        weight: u128,
        vote_storage: &mut VoteStorage,
        events: &mut Vec<GovernanceEvent>,
    ) -> Result<(), GovernanceError> {
        // Prevent double voting
        if vote_storage.has_voted(proposal_id, voter_id) {
            return Err(GovernanceError::AlreadyVoted);
        }
        let effective_weight = match self.config.mode {
            ReferendumMode::Quadratic => (weight as f64).sqrt().round() as u128,
            _ => weight,
        };
        vote_storage.record_weighted_vote(proposal_id, crate::contracts::governance::weighted_vote::WeightedVote {
            voter_id,
            weight: effective_weight,
            option: selected_option.clone(),
        })?;
        events.push(GovernanceEvent::ReferendumVoteCast {
            proposal_id,
            voter_id,
            weight: effective_weight,
            option: selected_option,
        });
        Ok(())
    }

    /// Tally referendum votes and decide outcome
    pub fn tally(
        &self,
        proposal_id: ProposalId,
        vote_storage: &VoteStorage,
        proposal_storage: &mut ProposalStorage,
        events: &mut Vec<GovernanceEvent>,
    ) -> Result<bool, GovernanceError> {
        let (yes_weight, total_weight) = vote_storage.tally_votes(proposal_id, VoteOption::Yes);
        if total_weight < self.config.quorum {
            return Err(GovernanceError::QuorumNotMet);
        }
        let passed = yes_weight >= self.config.threshold;
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
