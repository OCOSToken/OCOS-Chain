//! OCOS-Chain: Governance/DAO Storage Module
//!
//! Implements all persistent storage for proposals, votes, delegations, council,
//! and DAO state. Supports querying and audit-friendly access.

use std::collections::{HashMap, HashSet};
use crate::contracts::governance::types::{
    ProposalId, VoterId, CouncilMemberId, ProposalStatus, VoteOption,
};
use crate::contracts::governance::weighted_vote::WeightedVote;
use crate::contracts::governance::delegation::Delegation;
use crate::contracts::governance::error::GovernanceError;

/// Proposal storage with status and index
#[derive(Default)]
pub struct ProposalStorage {
    pub proposals: HashMap<ProposalId, super::proposal::Proposal>,
}

impl ProposalStorage {
    pub fn add(&mut self, proposal: super::proposal::Proposal) -> Result<(), GovernanceError> {
        if self.proposals.contains_key(&proposal.id) {
            return Err(GovernanceError::ProposalExists);
        }
        self.proposals.insert(proposal.id, proposal);
        Ok(())
    }
    pub fn get(&self, proposal_id: ProposalId) -> Option<&super::proposal::Proposal> {
        self.proposals.get(&proposal_id)
    }
    pub fn mark_as_passed(&mut self, proposal_id: ProposalId) -> Result<(), GovernanceError> {
        self.proposals
            .get_mut(&proposal_id)
            .map(|p| p.status = ProposalStatus::Approved)
            .ok_or(GovernanceError::ProposalNotFound)
    }
    pub fn mark_as_failed(&mut self, proposal_id: ProposalId) -> Result<(), GovernanceError> {
        self.proposals
            .get_mut(&proposal_id)
            .map(|p| p.status = ProposalStatus::Failed)
            .ok_or(GovernanceError::ProposalNotFound)
    }
    pub fn mark_as_executed(&mut self, proposal_id: ProposalId) -> Result<(), GovernanceError> {
        self.proposals
            .get_mut(&proposal_id)
            .map(|p| p.status = ProposalStatus::Executed)
            .ok_or(GovernanceError::ProposalNotFound)
    }
    // Extendable with more audit/query logic
}

/// Voting storage
#[derive(Default)]
pub struct VoteStorage {
    // proposal_id -> voter_id -> WeightedVote
    pub votes: HashMap<ProposalId, HashMap<VoterId, WeightedVote>>,
}

impl VoteStorage {
    pub fn has_voted(&self, proposal_id: ProposalId, voter_id: VoterId) -> bool {
        self.votes.get(&proposal_id)
            .map_or(false, |vmap| vmap.contains_key(&voter_id))
    }
    pub fn record_weighted_vote(
        &mut self,
        proposal_id: ProposalId,
        vote: WeightedVote,
    ) -> Result<(), GovernanceError> {
        let entry = self.votes.entry(proposal_id).or_default();
        if entry.contains_key(&vote.voter_id) {
            return Err(GovernanceError::AlreadyVoted);
        }
        entry.insert(vote.voter_id, vote);
        Ok(())
    }
    pub fn record_delegated_vote(
        &mut self,
        proposal_id: ProposalId,
        voter_id: VoterId,
        weight: u128,
        option: VoteOption,
    ) -> Result<(), GovernanceError> {
        let entry = self.votes.entry(proposal_id).or_default();
        if entry.contains_key(&voter_id) {
            return Err(GovernanceError::AlreadyVoted);
        }
        entry.insert(voter_id, WeightedVote { voter_id, weight, option });
        Ok(())
    }
    pub fn tally_votes(
        &self,
        proposal_id: ProposalId,
        option: VoteOption,
    ) -> (u128, u128) {
        let mut for_weight = 0u128;
        let mut total = 0u128;
        if let Some(vmap) = self.votes.get(&proposal_id) {
            for vote in vmap.values() {
                total += vote.weight;
                if vote.option == option {
                    for_weight += vote.weight;
                }
            }
        }
        (for_weight, total)
    }
}

/// Delegation storage
#[derive(Default)]
pub struct DelegationStorage {
    // delegator -> Delegation
    pub delegations: HashMap<VoterId, Delegation>,
}

impl DelegationStorage {
    pub fn record_delegation(&mut self, d: Delegation) -> Result<(), GovernanceError> {
        self.delegations.insert(d.delegator, d);
        Ok(())
    }
    pub fn remove_delegation(&mut self, delegator: VoterId) -> Result<(), GovernanceError> {
        self.delegations.remove(&delegator);
        Ok(())
    }
    pub fn is_delegatee(&self, delegator: VoterId, delegatee: VoterId) -> bool {
        self.delegations.get(&delegator).map_or(false, |d| d.delegatee == delegatee)
    }
    pub fn all(&self) -> Vec<&Delegation> {
        self.delegations.values().collect()
    }
}

/// Council storage (active council members)
#[derive(Default)]
pub struct CouncilStorage {
    pub members: HashMap<CouncilMemberId, super::council::CouncilMember>,
}

impl CouncilStorage {
    pub fn add_member(&mut self, m: super::council::CouncilMember) {
        self.members.insert(m.id, m);
    }
    pub fn remove_member(&mut self, id: CouncilMemberId) {
        self.members.remove(&id);
    }
    pub fn get_member(&self, id: CouncilMemberId) -> Option<&super::council::CouncilMember> {
        self.members.get(&id)
    }
    pub fn create_member_add_proposal(
        &mut self,
        proposer_id: CouncilMemberId,
        new_member: super::council::CouncilMember,
    ) -> Result<ProposalId, GovernanceError> {
        // In a real implementation, this would create and track a proposal for council expansion.
        // Here, we just simulate ID assignment.
        let new_id = rand::random::<u64>();
        self.add_member(new_member);
        Ok(new_id)
    }
}
