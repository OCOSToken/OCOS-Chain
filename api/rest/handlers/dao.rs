//! OCOS-Chain: REST API DAO Handlers
//!
//! Implements HTTP handlers for DAO metadata, membership, permissions, and registry queries.

use axum::{extract::Path, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct DaoResponse {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub founder: String,
    pub created_at: u64,
    pub member_count: u32,
    pub governance_model: String,
    // ... extendable: custom metadata, avatar, treasury balance, etc.
}

#[derive(Serialize)]
pub struct DaoMemberResponse {
    pub address: String,
    pub joined_at: u64,
    pub role: String,
    pub reputation: u64,
}

#[derive(Serialize)]
pub struct DaoPermissionResponse {
    pub address: String,
    pub permissions: Vec<String>,
}

// --- Handler: List all DAOs ---
pub async fn list_daos() -> Json<Vec<DaoResponse>> {
    // In real code, fetch from DAO registry
    Json(vec![
        DaoResponse {
            id: 101,
            name: "OCOS Core DAO".to_string(),
            description: "Main governance DAO for OCOS-Chain".to_string(),
            founder: "0xFounder...".to_string(),
            created_at: 1_650_000_000,
            member_count: 42,
            governance_model: "Council+Referendum".to_string(),
        },
    ])
}

// --- Handler: Get DAO details by id ---
pub async fn get_dao(Path(id): Path<u64>) -> Json<DaoResponse> {
    // In real code, fetch details for DAO
    Json(DaoResponse {
        id,
        name: "OCOS Core DAO".to_string(),
        description: "Main governance DAO for OCOS-Chain".to_string(),
        founder: "0xFounder...".to_string(),
        created_at: 1_650_000_000,
        member_count: 47,
        governance_model: "Council+Referendum".to_string(),
    })
}

// --- Handler: List members of a DAO ---
pub async fn list_dao_members(Path(id): Path<u64>) -> Json<Vec<DaoMemberResponse>> {
    // In real code, fetch member list from DAO registry
    Json(vec![
        DaoMemberResponse {
            address: "0xAlice...".to_string(),
            joined_at: 1_650_100_000,
            role: "Council".to_string(),
            reputation: 100,
        },
        DaoMemberResponse {
            address: "0xBob...".to_string(),
            joined_at: 1_650_200_000,
            role: "Member".to_string(),
            reputation: 55,
        },
    ])
}

// --- Handler: Get DAO member permissions ---
pub async fn get_dao_member_permissions(Path((id, address)): Path<(u64, String)>) -> Json<DaoPermissionResponse> {
    // In real code, fetch permissions from DAO governance module
    Json(DaoPermissionResponse {
        address: address.clone(),
        permissions: vec!["vote".to_string(), "propose".to_string()],
    })
}
