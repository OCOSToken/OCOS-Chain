//! OCOS-Chain: REST API Governance Handlers
//!
//! Implements HTTP handlers for governance proposals, voting, council, referenda, and DAO audit endpoints.

use axum::{extract::Path, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ProposalResponse {
    pub id: u64,
    pub proposer: String,
    pub kind: String,
    pub status: String,
    pub created_at: u64,
    pub voting_end: u64,
    pub yes_votes: u128,
    pub no_votes: u128,
    // ... extendable: execution, metadata, custom fields
}

#[derive(Deserialize)]
pub struct SubmitProposalRequest {
    pub proposer: String,
    pub kind: String,
    pub description: String,
    pub data: Option<serde_json::Value>,
    pub voting_end: u64,
}

#[derive(Deserialize)]
pub struct CastVoteRequest {
    pub proposal_id: u64,
    pub voter: String,
    pub amount: u128,
    pub approve: bool,
}

// --- Handler: List proposals ---
pub async fn list_proposals() -> Json<Vec<ProposalResponse>> {
    // In real code, query from governance storage/db
    Json(vec![
        ProposalResponse {
            id: 1,
            proposer: "0xabc...".to_string(),
            kind: "ConfigUpdate".to_string(),
            status: "Pending".to_string(),
            created_at: 1_650_000_000,
            voting_end: 1_650_086_400,
            yes_votes: 1234,
            no_votes: 10,
        },
    ])
}

// --- Handler: Get proposal by ID ---
pub async fn get_proposal(Path(id): Path<u64>) -> Json<ProposalResponse> {
    // In real code, query proposal by id
    Json(ProposalResponse {
        id,
        proposer: "0xabc...".to_string(),
        kind: "ConfigUpdate".to_string(),
        status: "Approved".to_string(),
        created_at: 1_650_000_000,
        voting_end: 1_650_086_400,
        yes_votes: 1500,
        no_votes: 10,
    })
}

// --- Handler: Submit a proposal ---
pub async fn submit_proposal(Json(req): Json<SubmitProposalRequest>) -> Json<ProposalResponse> {
    // In real code, validate, persist and return the new proposal
    Json(ProposalResponse {
        id: 2,
        proposer: req.proposer,
        kind: req.kind,
        status: "Pending".to_string(),
        created_at: 1_660_000_000,
        voting_end: req.voting_end,
        yes_votes: 0,
        no_votes: 0,
    })
}

// --- Handler: Cast a vote ---
pub async fn cast_vote(Json(req): Json<CastVoteRequest>) -> Json<ProposalResponse> {
    // In real code, validate, update votes, and return updated proposal status
    Json(ProposalResponse {
        id: req.proposal_id,
        proposer: "0xabc...".to_string(),
        kind: "ConfigUpdate".to_string(),
        status: "Pending".to_string(),
        created_at: 1_650_000_000,
        voting_end: 1_650_086_400,
        yes_votes: if req.approve { 1500 } else { 0 },
        no_votes: if req.approve { 0 } else { 1500 },
    })
}
