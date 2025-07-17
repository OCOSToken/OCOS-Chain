//! OCOS-Chain: REST API Error Types and Handlers
//!
//! Centralizes API error codes, HTTP status mapping, and response formatting.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiErrorBody {
    pub code: u16,
    pub message: String,
    pub details: Option<String>,
}

#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    Unauthorized,
    Forbidden,
    BadRequest(String),
    Validation(String),
    Conflict(String),
    RateLimit,
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            ApiError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                ApiErrorBody { code: 404, message: msg, details: None },
            ),
            ApiError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                ApiErrorBody { code: 401, message: "Unauthorized".to_string(), details: None },
            ),
            ApiError::Forbidden => (
                StatusCode::FORBIDDEN,
                ApiErrorBody { code: 403, message: "Forbidden".to_string(), details: None },
            ),
            ApiError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                ApiErrorBody { code: 400, message: msg, details: None },
            ),
            ApiError::Validation(msg) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ApiErrorBody { code: 422, message: msg, details: None },
            ),
            ApiError::Conflict(msg) => (
                StatusCode::CONFLICT,
                ApiErrorBody { code: 409, message: msg, details: None },
            ),
            ApiError::RateLimit => (
                StatusCode::TOO_MANY_REQUESTS,
                ApiErrorBody { code: 429, message: "Rate limit exceeded".to_string(), details: None },
            ),
            ApiError::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorBody { code: 500, message: "Internal server error".to_string(), details: Some(msg) },
            ),
        };
        (status, Json(body)).into_response()
    }
}

// Example: Map other error types (DB, chain, validation) to ApiError
impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError::Internal(err.to_string())
    }
}
