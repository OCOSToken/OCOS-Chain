//! OCOS-Chain: Governance Council (Multisig Committee) Module
//!
//! Implements a council/committee-based governance model supporting
//! member rotation, proposal management, council votes, and multisig approval
//! for high-impact protocol decisions.

use crate::contracts::governance::types::{ProposalId, CouncilMemberId, VoteOption};
use crate::contracts::governance::error::GovernanceError;
use crate::contracts::governance::events::GovernanceEvent;
use crate::contracts::governance::storage::{CouncilStorage, ProposalStorage};

/// Council member data
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CouncilMember {
    pub id: CouncilMemberId,
    pub address: [u8; 20],
    pub since_block: u64,
    pub reputation: u64,
    pub active: bool,
}

/// Governance council structure
#[derive(Debug, Clone)]
pub struct GovernanceCouncil {
    pub members: Vec<CouncilMember>,
    pub threshold: u8,         // Multisig: approvals needed for execution
    pub max_members: u8,       // Maximum allowed council size
}

impl GovernanceCouncil {
    /// Create a new council with initial members
    pub fn new(members: Vec<CouncilMember>, threshold: u8) -> Self {
        GovernanceCouncil {
            max_members: members.len() as u8,
            members,
            threshold,
        }
    }

    /// Propose addition of a new council member (subject to approval)
    pub fn propose_add_member(
        &mut self,
        proposer: &CouncilMember,
        new_member: CouncilMember,
        storage: &mut CouncilStorage,
    ) -> Result<ProposalId, GovernanceError> {
        if self.members.len() as u8 >= self.max_members {
            return Err(GovernanceError::CouncilFull);
        }
        // Save proposal for adding member (for council voting)
        let proposal_id = storage.create_member_add_proposal(proposer.id, new_member.clone())?;
        Ok(proposal_id)
    }

    /// Vote on a council proposal (add/remove/upgrade)
    pub fn vote_on_proposal(
        &mut self,
        member_id: CouncilMemberId,
        proposal_id: ProposalId,
        vote: VoteOption,
        storage: &mut ProposalStorage,
        events: &mut Vec<GovernanceEvent>,
    ) -> Result<(), GovernanceError> {
        // Only active council members can vote
        let member = self.members.iter().find(|m| m.id == member_id && m.active)
            .ok_or(GovernanceError::Unauthorized)?;

        storage.record_vote(proposal_id, member.id, vote.clone())?;
        events.push(GovernanceEvent::CouncilVoteCast { proposal_id, member_id, vote });

        // Check for proposal approval threshold
        if storage.count_approvals(proposal_id, VoteOption::Yes) >= self.threshold as usize {
            storage.execute_proposal(proposal_id)?;
            events.push(GovernanceEvent::CouncilProposalExecuted { proposal_id });
        }
        Ok(())
    }

    /// Rotate council: retire members, add new ones
    pub fn rotate_members(
        &mut self,
        retire_ids: Vec<CouncilMemberId>,
        add_members: Vec<CouncilMember>,
    ) {
        for id in retire_ids {
            if let Some(member) = self.members.iter_mut().find(|m| m.id == id) {
                member.active = false;
            }
        }
        for new_member in add_members {
            if self.members.len() < self.max_members as usize {
                self.members.push(new_member);
            }
        }
    }

    /// Check if an address is a current, active council member
    pub fn is_member(&self, address: &[u8; 20]) -> bool {
        self.members.iter().any(|m| &m.address == address && m.active)
    }
}
