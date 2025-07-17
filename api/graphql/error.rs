//! OCOS-Chain: GraphQL API Error Handling
//!
//! Standardizes all GraphQL API error types, formatting, and mapping from protocol to client errors.

use async_graphql::{ErrorExtensions, ServerError, ServerResult, Error, Value};

/// Unified API error enum for business logic
#[derive(Debug, Clone)]
pub enum ApiError {
    NotFound(String),
    Unauthorized(String),
    Forbidden(String),
    Validation(String),
    Conflict(String),
    RateLimitExceeded,
    Internal(String),
    NotImplemented(String),
    ProtocolError(String),
}

impl ErrorExtensions for ApiError {
    fn extend(&self) -> Value {
        match self {
            ApiError::NotFound(msg) => Value::object(vec![("code", "NOT_FOUND".into()), ("message", msg.clone().into())]),
            ApiError::Unauthorized(msg) => Value::object(vec![("code", "UNAUTHORIZED".into()), ("message", msg.clone().into())]),
            ApiError::Forbidden(msg) => Value::object(vec![("code", "FORBIDDEN".into()), ("message", msg.clone().into())]),
            ApiError::Validation(msg) => Value::object(vec![("code", "VALIDATION".into()), ("message", msg.clone().into())]),
            ApiError::Conflict(msg) => Value::object(vec![("code", "CONFLICT".into()), ("message", msg.clone().into())]),
            ApiError::RateLimitExceeded => Value::object(vec![("code", "RATE_LIMIT".into()), ("message", "Rate limit exceeded".into())]),
            ApiError::Internal(msg) => Value::object(vec![("code", "INTERNAL".into()), ("message", msg.clone().into())]),
            ApiError::NotImplemented(msg) => Value::object(vec![("code", "NOT_IMPLEMENTED".into()), ("message", msg.clone().into())]),
            ApiError::ProtocolError(msg) => Value::object(vec![("code", "PROTOCOL".into()), ("message", msg.clone().into())]),
        }
    }
}

impl From<ApiError> for ServerError {
    fn from(err: ApiError) -> Self {
        let mut server_err = ServerError::new(format!("{:?}", err), None);
        server_err.extensions = Some(err.extend());
        server_err
    }
}

impl From<ApiError> for ServerResult<()> {
    fn from(err: ApiError) -> Self {
        Err(ServerError::from(err))
    }
}

/// Helper for wrapping any error in an internal server error
pub fn internal_error<E: std::fmt::Display>(err: E) -> ServerError {
    ApiError::Internal(err.to_string()).into()
}

/// Utility for validation errors
pub fn validation_error<E: std::fmt::Display>(err: E) -> ServerError {
    ApiError::Validation(err.to_string()).into()
}

/// Utility for unauthorized errors
pub fn unauthorized_error<E: std::fmt::Display>(err: E) -> ServerError {
    ApiError::Unauthorized(err.to_string()).into()
}

// Example usage in resolvers:
// return Err(api::graphql::error::not_found("Proposal not found"));

pub fn not_found<S: Into<String>>(msg: S) -> ServerError {
    ApiError::NotFound(msg.into()).into()
}
