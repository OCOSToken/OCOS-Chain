//! OCOS-Chain DAO: Membership Management Module
//!
//! Handles DAO member lifecycle: addition, removal, role assignment, reputation, and querying.

use crate::contracts::dao::types::{MemberId, MemberRole, MembershipInfo};
use crate::contracts::dao::storage::{store_member, load_member, remove_member, list_all_members};
use crate::contracts::dao::error::DaoError;
use crate::contracts::dao::events::{emit_member_added, emit_member_removed};

/// Add a new member to the DAO
pub fn add_member(
    member_id: MemberId,
    role: MemberRole,
    stake: u128,
) -> Result<(), DaoError> {
    // Prevent duplicate membership
    if load_member(&member_id).is_ok() {
        return Err(DaoError::MemberExists);
    }

    let info = MembershipInfo {
        member_id,
        role,
        stake,
        joined_at: now_unix(),
        reputation: 0,
        active: true,
    };

    store_member(&info)?;
    emit_member_added(&info);
    Ok(())
}

/// Remove a member from the DAO
pub fn remove_member(
    member_id: &MemberId,
) -> Result<(), DaoError> {
    let info = load_member(member_id)?;
    remove_member(member_id)?;
    emit_member_removed(&info);
    Ok(())
}

/// Get a member's information
pub fn get_member(
    member_id: &MemberId,
) -> Result<MembershipInfo, DaoError> {
    load_member(member_id)
}

/// List all active DAO members
pub fn list_members() -> Result<Vec<MembershipInfo>, DaoError> {
    let all = list_all_members()?;
    Ok(all.into_iter().filter(|m| m.active).collect())
}

/// Helper to get current UNIX timestamp
fn now_unix() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}
