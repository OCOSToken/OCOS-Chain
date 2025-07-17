//! OCOS-Chain: Identity-Based Group & Membership Module
//!
//! Manages on-chain groups, roles, whitelists, membership, group airdrop/invite,
//! and integration with DAO and reputation. Suitable for social, DAO, DeFi, and NFT use cases.

use crate::identity::types::{GroupId, Address};
use crate::identity::error::IdentityError;
use std::collections::{HashMap, HashSet};

/// Group type (DAO, whitelist, role, airdrop, community, etc.)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroupType {
    DAO,
    Whitelist,
    Role,
    Airdrop,
    Community,
    Custom(String),
}

/// Group metadata and membership
#[derive(Debug, Clone)]
pub struct IdentityGroup {
    pub group_id: GroupId,
    pub group_type: GroupType,
    pub name: String,
    pub description: String,
    pub creator: Address,
    pub admins: HashSet<Address>,
    pub members: HashSet<Address>,
    pub created_at: u64,
    pub is_open: bool, // If true, anyone can join
}

#[derive(Default)]
pub struct GroupRegistry {
    // group_id → group struct
    pub groups: HashMap<GroupId, IdentityGroup>,
}

impl GroupRegistry {
    /// Create a new group
    pub fn create_group(
        &mut self,
        group_id: GroupId,
        group_type: GroupType,
        name: String,
        description: String,
        creator: Address,
        is_open: bool,
        now: u64,
    ) -> Result<(), IdentityError> {
        if self.groups.contains_key(&group_id) {
            return Err(IdentityError::AlreadyExists);
        }
        let mut admins = HashSet::new();
        admins.insert(creator);
        let group = IdentityGroup {
            group_id,
            group_type,
            name,
            description,
            creator,
            admins,
            members: HashSet::new(),
            created_at: now,
            is_open,
        };
        self.groups.insert(group_id, group);
        Ok(())
    }

    /// Add a member (admin or open group)
    pub fn add_member(
        &mut self,
        group_id: GroupId,
        member: Address,
        by: Address,
    ) -> Result<(), IdentityError> {
        let group = self.groups.get_mut(&group_id).ok_or(IdentityError::NotFound)?;
        if !group.is_open && !group.admins.contains(&by) {
            return Err(IdentityError::Unauthorized);
        }
        group.members.insert(member);
        Ok(())
    }

    /// Remove a member (admin only)
    pub fn remove_member(
        &mut self,
        group_id: GroupId,
        member: Address,
        by: Address,
    ) -> Result<(), IdentityError> {
        let group = self.groups.get_mut(&group_id).ok_or(IdentityError::NotFound)?;
        if !group.admins.contains(&by) {
            return Err(IdentityError::Unauthorized);
        }
        group.members.remove(&member);
        Ok(())
    }

    /// Query all members of a group
    pub fn members_of(&self, group_id: GroupId) -> Option<&HashSet<Address>> {
        self.groups.get(&group_id).map(|g| &g.members)
    }

    /// Check if an address is a member
    pub fn is_member(&self, group_id: GroupId, address: Address) -> bool {
        self.groups.get(&group_id)
            .map_or(false, |g| g.members.contains(&address))
    }

    /// Query group metadata
    pub fn get_group(&self, group_id: GroupId) -> Option<&IdentityGroup> {
        self.groups.get(&group_id)
    }
}
