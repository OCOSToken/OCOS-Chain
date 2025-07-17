//! OCOS-Chain: WebSocket Handler – Governance
//!
//! Broadcasts real-time governance and DAO events: proposals, voting, execution, council, etc.

use crate::ws::types::{WsTopic, WsMessage, ProposalInfo, VoteInfo, GovernanceStatus};
use crate::ws::router::WsRouter;

/// Handles and streams governance-related events over WebSocket channels.
pub struct GovernanceHandler;

impl GovernanceHandler {
    /// Called when a new governance proposal is created
    pub fn on_proposal_created(router: &WsRouter, proposal: ProposalInfo) {
        let msg = WsMessage::GovernanceProposalCreated { proposal: proposal.clone() };
        router.broadcast(&WsTopic::Governance, &msg);
    }

    /// Called when a vote is cast on a proposal
    pub fn on_vote_cast(router: &WsRouter, vote: VoteInfo) {
        let msg = WsMessage::GovernanceVoteCast { vote: vote.clone() };
        router.broadcast(&WsTopic::Governance, &msg);
    }

    /// Called when a proposal is executed
    pub fn on_proposal_executed(router: &WsRouter, proposal_id: u64, status: GovernanceStatus) {
        let msg = WsMessage::GovernanceProposalExecuted {
            proposal_id,
            status,
        };
        router.broadcast(&WsTopic::Governance, &msg);
    }

    /// (Optional) Called when council/committee actions are performed
    pub fn on_council_action(router: &WsRouter, council_event: serde_json::Value) {
        let msg = WsMessage::CouncilEvent { event: council_event };
        router.broadcast(&WsTopic::Governance, &msg);
    }

    /// (Optional) Notify about governance config or parameter changes
    pub fn on_config_update(router: &WsRouter, config_update: serde_json::Value) {
        let msg = WsMessage::GovernanceConfigUpdate { config: config_update };
        router.broadcast(&WsTopic::Governance, &msg);
    }
}

// -- Example message types for reference --

/*
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsMessage {
    GovernanceProposalCreated { proposal: ProposalInfo },
    GovernanceVoteCast { vote: VoteInfo },
    GovernanceProposalExecuted { proposal_id: u64, status: GovernanceStatus },
    CouncilEvent { event: serde_json::Value },
    GovernanceConfigUpdate { config: serde_json::Value },
    // ... other variants
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalInfo {
    pub id: u64,
    pub proposer: String,
    pub kind: String,
    pub description: String,
    pub status: GovernanceStatus,
    pub created_at: u64,
    pub voting_end: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteInfo {
    pub proposal_id: u64,
    pub voter: String,
    pub amount: u128,
    pub option: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GovernanceStatus {
    Pending,
    Approved,
    Rejected,
    Executed,
    Expired,
}
*/
