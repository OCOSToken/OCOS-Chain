//! OCOS-Chain: Governance Events & Log Emission Module
//!
//! Defines all events that are emitted during governance operations, such as
//! proposal creation, voting, approval, execution, delegation, and upgrade.

use crate::contracts::governance::types::{ProposalId, VoterId, VoteOption, CouncilMemberId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GovernanceEvent {
    ProposalCreated {
        proposal_id: ProposalId,
        proposer: VoterId,
        kind: String,
        description: String,
    },
    ProposalApproved {
        proposal_id: ProposalId,
    },
    ProposalRejected {
        proposal_id: ProposalId,
    },
    ProposalExecuted {
        proposal_id: ProposalId,
    },
    ProposalPassed {
        proposal_id: ProposalId,
    },
    ProposalFailed {
        proposal_id: ProposalId,
    },
    VoteCast {
        proposal_id: ProposalId,
        voter_id: VoterId,
        option: VoteOption,
    },
    WeightedVoteCast {
        proposal_id: ProposalId,
        voter_id: VoterId,
        weight: u128,
        option: VoteOption,
    },
    ReferendumVoteCast {
        proposal_id: ProposalId,
        voter_id: VoterId,
        weight: u128,
        option: VoteOption,
    },
    DelegationCreated {
        delegator: VoterId,
        delegatee: VoterId,
        weight: u128,
    },
    DelegationRevoked {
        delegator: VoterId,
    },
    CouncilVoteCast {
        proposal_id: ProposalId,
        member_id: CouncilMemberId,
        vote: VoteOption,
    },
    CouncilProposalExecuted {
        proposal_id: ProposalId,
    },
    ConfigUpdated {
        proposal_id: ProposalId,
        key: String,
        value: u128,
    },
    ProtocolUpgradeScheduled {
        proposal_id: ProposalId,
        description: String,
        scheduled_for: u64,
    },
    TreasuryActionExecuted {
        proposal_id: ProposalId,
        action: String,
    },
    // ...extendable for future governance events
}
