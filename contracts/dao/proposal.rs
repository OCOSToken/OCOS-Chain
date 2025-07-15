//! OCOS-Chain DAO: Proposal Management Module
//!
//! Handles the lifecycle of DAO proposals: creation, tracking, status transitions,
//! listing, and detailed retrieval. Proposals are subject to voting, execution, and auditing.

use crate::contracts::dao::types::{ProposalId, ProposalStatus, ProposalKind, Proposal, MemberId};
use crate::contracts::dao::error::DaoError;
use crate::contracts::dao::storage::{store_proposal, load_proposal, list_all_proposals};
use crate::contracts::dao::events::{emit_proposal_created, emit_proposal_updated};
use std::time::{SystemTime, UNIX_EPOCH};

/// Create a new DAO proposal
pub fn create_proposal(
    proposer: MemberId,
    kind: ProposalKind,
    title: String,
    description: String,
    params: Vec<u8>,
) -> Result<ProposalId, DaoError> {
    // Generate unique proposal ID (timestamp + proposer)
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let id = ProposalId::new(proposer, timestamp);

    let proposal = Proposal {
        id,
        proposer,
        kind,
        title,
        description,
        params,
        status: ProposalStatus::Pending,
        created_at: timestamp,
        updated_at: timestamp,
        votes_for: 0,
        votes_against: 0,
        quorum: 0,
        execution_result: None,
    };

    store_proposal(&proposal)?;
    emit_proposal_created(&proposal);

    Ok(proposal.id)
}

/// Get details of a specific proposal by ID
pub fn get_proposal(id: &ProposalId) -> Result<Proposal, DaoError> {
    load_proposal(id)
}

/// List all proposals (optionally filtered by status or proposer)
pub fn list_proposals(
    status: Option<ProposalStatus>,
    proposer: Option<MemberId>,
) -> Result<Vec<Proposal>, DaoError> {
    let all = list_all_proposals()?;
    let filtered = all
        .into_iter()
        .filter(|p| status.map_or(true, |s| p.status == s))
        .filter(|p| proposer.map_or(true, |m| p.proposer == m))
        .collect();
    Ok(filtered)
}

/// Update proposal status (e.g., after voting or execution)
pub fn update_proposal_status(
    id: &ProposalId,
    new_status: ProposalStatus,
) -> Result<(), DaoError> {
    let mut proposal = load_proposal(id)?;
    proposal.status = new_status;
    proposal.updated_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    store_proposal(&proposal)?;
    emit_proposal_updated(&proposal);
    Ok(())
}
