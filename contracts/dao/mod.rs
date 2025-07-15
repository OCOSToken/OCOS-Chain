//! OCOS-Chain DAO Smart Contract: Module Root
//!
//! Orchestrates all DAO governance components: proposals, voting,
//! membership, execution, treasury, configuration, and error handling.

pub mod proposal;
pub mod voting;
pub mod membership;
pub mod execution;
pub mod config;
pub mod treasury;
pub mod error;
pub mod types;
pub mod storage;
pub mod events;

#[cfg(test)]
pub mod tests;

// DAO public API surface — used by the runtime or VM engine
pub use proposal::{
    create_proposal, get_proposal, list_proposals, ProposalStatus,
};
pub use voting::{
    cast_vote, tally_votes, Vote, VoteWeight, QuorumConfig,
};
pub use membership::{
    add_member, remove_member, get_member, MemberRole, MembershipInfo,
};
pub use execution::{
    execute_proposal, ExecutionReceipt,
};
pub use treasury::{
    deposit, withdraw, grant_funds, TreasuryEvent,
};
pub use config::{
    update_config, get_config, DaoConfig,
};
pub use error::DaoError;
pub use types::*;
pub use storage::*;
pub use events::*;
