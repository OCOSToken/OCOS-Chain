//! OCOS-Chain DAO: Events Module
//!
//! Defines and emits auditable events for all significant DAO state changes:
//! proposal creation, vote casting, execution, membership changes, and treasury ops.

use crate::contracts::dao::types::{
    Proposal, ProposalId, ProposalStatus, MembershipInfo, MemberId,
    ExecutionReceipt, TreasuryEvent,
};

/// Emit event when a new proposal is created
pub fn emit_proposal_created(proposal: &Proposal) {
    println!(
        "[EVENT] ProposalCreated: id={:?} proposer={:?} status={:?} title=\"{}\"",
        proposal.id, proposal.proposer, proposal.status, proposal.title
    );
}

/// Emit event when a proposal is updated (status changed)
pub fn emit_proposal_updated(proposal: &Proposal) {
    println!(
        "[EVENT] ProposalUpdated: id={:?} status={:?}",
        proposal.id, proposal.status
    );
}

/// Emit event when a proposal is executed
pub fn emit_proposal_executed(proposal: &Proposal) {
    println!(
        "[EVENT] ProposalExecuted: id={:?} outcome={:?}",
        proposal.id, proposal.execution_result
    );
}

/// Emit execution receipt for audit purposes
pub fn emit_execution_receipt(receipt: &ExecutionReceipt) {
    println!(
        "[EVENT] ExecutionReceipt: proposal_id={:?} status={:?} outcome={:?} message={:?}",
        receipt.proposal_id, receipt.status, receipt.outcome, receipt.message
    );
}

/// Emit event when a new member is added
pub fn emit_member_added(info: &MembershipInfo) {
    println!(
        "[EVENT] MemberAdded: id={:?} role={:?} stake={}",
        info.member_id, info.role, info.stake
    );
}

/// Emit event when a member is removed
pub fn emit_member_removed(info: &MembershipInfo) {
    println!(
        "[EVENT] MemberRemoved: id={:?}",
        info.member_id
    );
}

/// Emit treasury operation event
pub fn emit_treasury_event(event: &TreasuryEvent) {
    println!(
        "[EVENT] TreasuryEvent: type={:?} details={:?}",
        event.event_type, event.details
    );
}
