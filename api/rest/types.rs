//! OCOS-Chain: REST API Shared Types & Schemas
//!
//! Defines request/response models, pagination, filter, and generic API response envelopes.

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination {
    pub page: u32,
    pub page_size: u32,
    pub total: Option<u64>,
    pub has_next: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FilterQuery {
    pub q: Option<String>,
    pub from: Option<u64>,
    pub to: Option<u64>,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
    pub details: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdRequest {
    pub id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddressRequest {
    pub address: String,
}

// Example: Wrapper for list responses
#[derive(Serialize, Deserialize, Debug)]
pub struct ListResponse<T> {
    pub items: Vec<T>,
    pub pagination: Option<Pagination>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionResponse {
    pub api_version: String,
    pub chain_id: String,
    pub commit_hash: String,
    pub build_date: String,
}
