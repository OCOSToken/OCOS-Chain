//! OCOS-Chain: NFT Governance & DAO Module
//!
//! Enables NFT-based voting, collection-level governance, whitelist management,
//! allowlist/airdrop, and upgrade proposals for on-chain NFT communities.

use crate::nft::types::{NFTId, CollectionId, Address};
use crate::nft::error::NFTError;
use std::collections::{HashMap, HashSet};

/// NFT-based DAO proposal structure
#[derive(Debug, Clone)]
pub struct NFTGovernanceProposal {
    pub proposal_id: u64,
    pub collection_id: Option<CollectionId>,
    pub creator: Address,
    pub description: String,
    pub yes_votes: u128,
    pub no_votes: u128,
    pub status: ProposalStatus,
    pub created_at: u64,
    pub voting_end: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProposalStatus {
    Pending,
    Approved,
    Rejected,
    Executed,
}

#[derive(Default)]
pub struct NFTGovernance {
    /// proposal_id → proposal struct
    pub proposals: HashMap<u64, NFTGovernanceProposal>,
    /// collection_id → voter addresses (NFT holders, allowlist, etc.)
    pub collection_voters: HashMap<CollectionId, HashSet<Address>>,
    /// proposal_id → voted addresses (prevent double voting)
    pub votes: HashMap<u64, HashSet<Address>>,
}

impl NFTGovernance {
    /// Submit a new governance proposal for a collection or global NFT community
    pub fn submit_proposal(
        &mut self,
        proposal_id: u64,
        collection_id: Option<CollectionId>,
        creator: Address,
        description: String,
        created_at: u64,
        voting_end: u64,
    ) -> Result<(), NFTError> {
        if self.proposals.contains_key(&proposal_id) {
            return Err(NFTError::ProposalExists);
        }
        let proposal = NFTGovernanceProposal {
            proposal_id,
            collection_id,
            creator,
            description,
            yes_votes: 0,
            no_votes: 0,
            status: ProposalStatus::Pending,
            created_at,
            voting_end,
        };
        self.proposals.insert(proposal_id, proposal);
        Ok(())
    }

    /// Vote on a proposal (NFT-holder voting power is 1 per NFT)
    pub fn vote(
        &mut self,
        proposal_id: u64,
        voter: Address,
        approve: bool,
        voting_power: u128,
        now: u64,
    ) -> Result<(), NFTError> {
        let proposal = self.proposals.get_mut(&proposal_id).ok_or(NFTError::ProposalNotFound)?;
        if proposal.status != ProposalStatus::Pending || now > proposal.voting_end {
            return Err(NFTError::ProposalClosed);
        }
        let voted_set = self.votes.entry(proposal_id).or_default();
        if voted_set.contains(&voter) {
            return Err(NFTError::AlreadyVoted);
        }
        voted_set.insert(voter);
        if approve {
            proposal.yes_votes += voting_power;
        } else {
            proposal.no_votes += voting_power;
        }
        Ok(())
    }

    /// Execute proposal if passed
    pub fn execute(
        &mut self,
        proposal_id: u64,
        now: u64,
    ) -> Result<(), NFTError> {
        let proposal = self.proposals.get_mut(&proposal_id).ok_or(NFTError::ProposalNotFound)?;
        if proposal.status != ProposalStatus::Pending || now < proposal.voting_end {
            return Err(NFTError::ProposalClosed);
        }
        if proposal.yes_votes > proposal.no_votes {
            proposal.status = ProposalStatus::Executed;
        } else {
            proposal.status = ProposalStatus::Rejected;
        }
        Ok(())
    }

    /// Add an address to collection-level voter list (airdrop, allowlist, etc.)
    pub fn add_voter(
        &mut self,
        collection_id: CollectionId,
        addr: Address,
    ) {
        self.collection_voters.entry(collection_id).or_default().insert(addr);
    }

    /// Query all proposals
    pub fn list_proposals(&self) -> Vec<&NFTGovernanceProposal> {
        self.proposals.values().collect()
    }
}
