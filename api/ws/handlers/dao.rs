//! OCOS-Chain: WebSocket Handler – DAO Events
//!
//! Streams real-time DAO lifecycle and membership events over WebSocket channels.

use crate::ws::types::{WsTopic, WsMessage, DaoInfo, MemberInfo};
use crate::ws::router::WsRouter;

/// Handles all DAO-related, organization-wide event streaming.
pub struct DaoHandler;

impl DaoHandler {
    /// Broadcast when a new DAO is created or registered
    pub fn on_dao_created(router: &WsRouter, dao: DaoInfo) {
        let msg = WsMessage::DaoCreated { dao: dao.clone() };
        router.broadcast(&WsTopic::Dao, &msg);
    }

    /// Broadcast when DAO metadata or settings are updated (e.g. name, governance policy)
    pub fn on_dao_metadata_update(router: &WsRouter, dao: DaoInfo) {
        let msg = WsMessage::DaoMetadataUpdated { dao: dao.clone() };
        router.broadcast(&WsTopic::Dao, &msg);
    }

    /// Broadcast when a new member joins the DAO
    pub fn on_member_joined(router: &WsRouter, dao_id: u64, member: MemberInfo) {
        let msg = WsMessage::DaoMemberJoined {
            dao_id,
            member: member.clone(),
        };
        router.broadcast(&WsTopic::Dao, &msg);
    }

    /// Broadcast when a member leaves or is removed from the DAO
    pub fn on_member_left(router: &WsRouter, dao_id: u64, member: MemberInfo) {
        let msg = WsMessage::DaoMemberLeft {
            dao_id,
            member: member.clone(),
        };
        router.broadcast(&WsTopic::Dao, &msg);
    }

    /// (Optional) Custom DAO actions or status updates
    pub fn on_custom_action(router: &WsRouter, dao_id: u64, action: serde_json::Value) {
        let msg = WsMessage::DaoCustomEvent { dao_id, action };
        router.broadcast(&WsTopic::Dao, &msg);
    }
}

// -- Example message types for reference --

/*
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsMessage {
    DaoCreated { dao: DaoInfo },
    DaoMetadataUpdated { dao: DaoInfo },
    DaoMemberJoined { dao_id: u64, member: MemberInfo },
    DaoMemberLeft { dao_id: u64, member: MemberInfo },
    DaoCustomEvent { dao_id: u64, action: serde_json::Value },
    // ... other variants
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaoInfo {
    pub id: u64,
    pub name: String,
    pub metadata: serde_json::Value,
    pub created_at: u64,
    // ... further DAO fields
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberInfo {
    pub address: String,
    pub role: String,
    pub joined_at: u64,
    // ... further member data
}
*/
