//! OCOS-Chain: Governance Proposal Execution Module
//!
//! Handles execution of approved proposals: parameter changes, upgrades,
//! DAO operations, and on-chain governance effects. Ensures atomicity and
//! proper event logging for every execution path.

use crate::contracts::governance::types::{ProposalId, ProposalKind};
use crate::contracts::governance::error::GovernanceError;
use crate::contracts::governance::events::GovernanceEvent;
use crate::contracts::governance::config::GovernanceConfig;

/// Executor for on-chain governance actions
pub struct ExecutionEngine<'a> {
    pub config: &'a mut GovernanceConfig,
}

impl<'a> ExecutionEngine<'a> {
    /// Execute a governance proposal based on its type and payload
    pub fn execute_proposal(
        &mut self,
        proposal_id: ProposalId,
        kind: ProposalKind,
        payload: Option<Vec<u8>>,
        now: u64,
        events: &mut Vec<GovernanceEvent>,
    ) -> Result<(), GovernanceError> {
        match kind {
            ProposalKind::ConfigUpdate { ref key } => {
                let value = Self::parse_u128_payload(&payload)?;
                self.config.update(key, value)?;
                events.push(GovernanceEvent::ConfigUpdated {
                    proposal_id,
                    key: key.clone(),
                    value,
                });
                Ok(())
            }
            ProposalKind::Upgrade { ref description } => {
                // Placeholder: In real system, triggers on-chain contract or runtime upgrade.
                events.push(GovernanceEvent::ProtocolUpgradeScheduled {
                    proposal_id,
                    description: description.clone(),
                    scheduled_for: now,
                });
                Ok(())
            }
            ProposalKind::Treasury { ref action } => {
                // Placeholder: In real system, executes DAO treasury logic (transfer/grant)
                events.push(GovernanceEvent::TreasuryActionExecuted {
                    proposal_id,
                    action: action.clone(),
                });
                Ok(())
            }
            // Add more on-chain proposal types as needed...
            _ => Err(GovernanceError::ExecutionNotSupported),
        }
    }

    /// Parse u128 from payload
    fn parse_u128_payload(payload: &Option<Vec<u8>>) -> Result<u128, GovernanceError> {
        if let Some(bytes) = payload {
            if bytes.len() == 16 {
                let mut arr = [0u8; 16];
                arr.copy_from_slice(&bytes[..16]);
                Ok(u128::from_be_bytes(arr))
            } else {
                Err(GovernanceError::InvalidPayload)
            }
        } else {
            Err(GovernanceError::InvalidPayload)
        }
    }
}
