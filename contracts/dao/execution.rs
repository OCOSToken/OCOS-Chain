//! OCOS-Chain DAO: Proposal Execution Module
//!
//! Executes approved DAO proposals on-chain, handling result tracking,
//! receipt emission, and audit logging.

use crate::contracts::dao::types::{ProposalId, ExecutionResult, ProposalStatus};
use crate::contracts::dao::storage::{load_proposal, store_proposal};
use crate::contracts::dao::error::DaoError;
use crate::contracts::dao::events::{emit_proposal_executed, emit_execution_receipt};
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents the receipt/result of proposal execution
#[derive(Debug, Clone)]
pub struct ExecutionReceipt {
    pub proposal_id: ProposalId,
    pub status: ProposalStatus,
    pub outcome: ExecutionResult,
    pub executed_at: u64,
    pub message: Option<String>,
}

/// Executes a DAO proposal (if approved and pending execution)
pub fn execute_proposal(
    proposal_id: &ProposalId,
) -> Result<ExecutionReceipt, DaoError> {
    let mut proposal = load_proposal(proposal_id)?;
    if proposal.status != ProposalStatus::Approved {
        return Err(DaoError::ProposalNotExecutable);
    }

    // Simulate execution logic (should route to on-chain VM or handler)
    // Here, just a placeholder "success"
    let outcome = ExecutionResult::Success;

    proposal.status = ProposalStatus::Executed;
    proposal.execution_result = Some(outcome.clone());
    proposal.updated_at = now_unix();
    store_proposal(&proposal)?;

    let receipt = ExecutionReceipt {
        proposal_id: proposal.id.clone(),
        status: proposal.status,
        outcome,
        executed_at: proposal.updated_at,
        message: Some("Proposal executed successfully".to_string()),
    };

    emit_proposal_executed(&proposal);
    emit_execution_receipt(&receipt);

    Ok(receipt)
}

/// Helper to get current UNIX timestamp
fn now_unix() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}
