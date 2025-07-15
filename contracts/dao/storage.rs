//! OCOS-Chain DAO: On-chain Storage Module
//!
//! Provides safe persistent storage of DAO proposals, members, config, and other state
//! using deterministic keys for auditability and upgradability.

use crate::contracts::dao::types::{
    Proposal, ProposalId, ProposalStatus, MembershipInfo, MemberId, DaoConfig,
};
use crate::contracts::dao::error::DaoError;
use std::collections::HashMap;
use std::sync::Mutex;

// Simulated on-chain storage for demonstration.
// In production: use real contract storage or blockchain-provided persistent storage.
lazy_static::lazy_static! {
    static ref PROPOSAL_DB: Mutex<HashMap<ProposalId, Proposal>> = Mutex::new(HashMap::new());
    static ref MEMBER_DB: Mutex<HashMap<MemberId, MembershipInfo>> = Mutex::new(HashMap::new());
    static ref CONFIG_DB: Mutex<Option<DaoConfig>> = Mutex::new(None);
}

// ----- Proposal Storage -----

pub fn store_proposal(proposal: &Proposal) -> Result<(), DaoError> {
    PROPOSAL_DB.lock().unwrap().insert(proposal.id.clone(), proposal.clone());
    Ok(())
}

pub fn load_proposal(id: &ProposalId) -> Result<Proposal, DaoError> {
    PROPOSAL_DB.lock().unwrap()
        .get(id)
        .cloned()
        .ok_or(DaoError::ProposalNotFound)
}

pub fn list_all_proposals() -> Result<Vec<Proposal>, DaoError> {
    Ok(PROPOSAL_DB.lock().unwrap().values().cloned().collect())
}

// ----- Membership Storage -----

pub fn store_member(info: &MembershipInfo) -> Result<(), DaoError> {
    MEMBER_DB.lock().unwrap().insert(info.member_id.clone(), info.clone());
    Ok(())
}

pub fn load_member(id: &MemberId) -> Result<MembershipInfo, DaoError> {
    MEMBER_DB.lock().unwrap()
        .get(id)
        .cloned()
        .ok_or(DaoError::MemberNotFound)
}

pub fn remove_member(id: &MemberId) -> Result<(), DaoError> {
    MEMBER_DB.lock().unwrap()
        .remove(id)
        .map(|_| ())
        .ok_or(DaoError::MemberNotFound)
}

pub fn list_all_members() -> Result<Vec<MembershipInfo>, DaoError> {
    Ok(MEMBER_DB.lock().unwrap().values().cloned().collect())
}

// ----- DAO Config Storage -----

pub fn store_config(config: &DaoConfig) -> Result<(), DaoError> {
    *CONFIG_DB.lock().unwrap() = Some(config.clone());
    Ok(())
}

pub fn load_config() -> Result<DaoConfig, DaoError> {
    CONFIG_DB.lock().unwrap()
        .clone()
        .ok_or(DaoError::InvalidConfig)
}
