//! OCOS-Chain: Identity Governance Module
//!
//! On-chain identity, reputation, group, and KYC-based DAO governance:
//! proposals, voting, config updates, recovery approvals, and attestation management.

use crate::identity::types::{ProposalId, Address, GroupId, IdentityId};
use crate::identity::error::IdentityError;
use std::collections::{HashMap, HashSet};

/// Proposal types for identity governance
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentityProposalKind {
    UpdateConfig { key: String, value: u128 },
    AddGuardian { identity_id: IdentityId, new_guardian: Address },
    RemoveGuardian { identity_id: IdentityId, guardian: Address },
    RecoveryApproval { identity_id: IdentityId, new_owner: Address },
    GroupUpgrade { group_id: GroupId, change: String },
    Custom { description: String },
}

/// Identity governance proposal structure
#[derive(Debug, Clone)]
pub struct IdentityGovernanceProposal {
    pub proposal_id: ProposalId,
    pub kind: IdentityProposalKind,
    pub creator: Address,
    pub created_at: u64,
    pub voting_end: u64,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub voters: HashSet<Address>,
    pub executed: bool,
    pub status: ProposalStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProposalStatus {
    Pending,
    Approved,
    Rejected,
    Executed,
    Expired,
}

#[derive(Default)]
pub struct IdentityGovernance {
    // proposal_id → proposal struct
    pub proposals: HashMap<ProposalId, IdentityGovernanceProposal>,
}

impl IdentityGovernance {
    /// Submit a new proposal
    pub fn submit_proposal(
        &mut self,
        proposal_id: ProposalId,
        kind: IdentityProposalKind,
        creator: Address,
        created_at: u64,
        voting_end: u64,
    ) -> Result<(), IdentityError> {
        if self.proposals.contains_key(&proposal_id) {
            return Err(IdentityError::ProposalExists);
        }
        let proposal = IdentityGovernanceProposal {
            proposal_id,
            kind,
            creator,
            created_at,
            voting_end,
            yes_votes: 0,
            no_votes: 0,
            voters: HashSet::new(),
            executed: false,
            status: ProposalStatus::Pending,
        };
        self.proposals.insert(proposal_id, proposal);
        Ok(())
    }

    /// Vote on proposal (one address, one vote)
    pub fn vote(
        &mut self,
        proposal_id: ProposalId,
        voter: Address,
        approve: bool,
        now: u64,
    ) -> Result<(), IdentityError> {
        let proposal = self.proposals.get_mut(&proposal_id).ok_or(IdentityError::ProposalNotFound)?;
        if proposal.status != ProposalStatus::Pending || now > proposal.voting_end {
            return Err(IdentityError::ProposalClosed);
        }
        if proposal.voters.contains(&voter) {
            return Err(IdentityError::AlreadyVoted);
        }
        proposal.voters.insert(voter);
        if approve {
            proposal.yes_votes += 1;
        } else {
            proposal.no_votes += 1;
        }
        Ok(())
    }

    /// Execute proposal after voting ends
    pub fn execute(
        &mut self,
        proposal_id: ProposalId,
        now: u64,
    ) -> Result<(), IdentityError> {
        let proposal = self.proposals.get_mut(&proposal_id).ok_or(IdentityError::ProposalNotFound)?;
        if proposal.executed || now < proposal.voting_end {
            return Err(IdentityError::ProposalClosed);
        }
        if proposal.yes_votes > proposal.no_votes {
            proposal.status = ProposalStatus::Executed;
        } else {
            proposal.status = ProposalStatus::Rejected;
        }
        proposal.executed = true;
        Ok(())
    }

    /// List all proposals for monitoring/audit
    pub fn list_proposals(&self) -> Vec<&IdentityGovernanceProposal> {
        self.proposals.values().collect()
    }
}
