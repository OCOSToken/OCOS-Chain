//! OCOS-Chain: Vote Delegation (Liquid Democracy / Proxy Voting) Module
//!
//! Allows users to delegate their voting power to trusted representatives.
//! Supports stake- or reputation-weighted delegation with cycle detection.

use crate::contracts::governance::types::{VoterId, ProposalId, VoteOption};
use crate::contracts::governance::error::GovernanceError;
use crate::contracts::governance::events::GovernanceEvent;
use crate::contracts::governance::storage::{DelegationStorage, VoteStorage};

/// Represents a delegation relationship
#[derive(Debug, Clone)]
pub struct Delegation {
    pub delegator: VoterId,
    pub delegatee: VoterId,
    pub weight: u128, // stake, token, or reputation
}

/// Delegation logic for governance
pub struct DelegationEngine;

impl DelegationEngine {
    /// Delegate voting power to another voter
    pub fn delegate(
        delegator: VoterId,
        delegatee: VoterId,
        weight: u128,
        storage: &mut DelegationStorage,
        events: &mut Vec<GovernanceEvent>,
    ) -> Result<(), GovernanceError> {
        // Prevent self-delegation
        if delegator == delegatee {
            return Err(GovernanceError::InvalidDelegation);
        }
        // Prevent delegation cycles (direct)
        if storage.is_delegatee(delegator, delegatee) {
            return Err(GovernanceError::DelegationCycle);
        }
        storage.record_delegation(Delegation {
            delegator,
            delegatee,
            weight,
        })?;
        events.push(GovernanceEvent::DelegationCreated { delegator, delegatee, weight });
        Ok(())
    }

    /// Revoke delegation
    pub fn revoke(
        delegator: VoterId,
        storage: &mut DelegationStorage,
        events: &mut Vec<GovernanceEvent>,
    ) -> Result<(), GovernanceError> {
        storage.remove_delegation(delegator)?;
        events.push(GovernanceEvent::DelegationRevoked { delegator });
        Ok(())
    }

    /// Apply all delegations to vote tally for a proposal
    pub fn apply_delegations(
        proposal_id: ProposalId,
        vote_option: VoteOption,
        delegation_storage: &DelegationStorage,
        vote_storage: &mut VoteStorage,
    ) -> Result<(), GovernanceError> {
        let delegations = delegation_storage.all();
        for d in delegations {
            if !vote_storage.has_voted(proposal_id, d.delegator)
                && !vote_storage.has_voted(proposal_id, d.delegatee)
            {
                // Only count if neither party already voted
                vote_storage.record_delegated_vote(proposal_id, d.delegatee, d.weight, vote_option.clone())?;
            }
        }
        Ok(())
    }
}
