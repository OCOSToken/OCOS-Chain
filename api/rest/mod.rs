//! OCOS-Chain: REST API Main Module
//!
//! This module serves as the entry point for the OCOS REST API.
//! It wires together all HTTP route handlers, middleware, versioning,
//! and authentication layers.

pub mod routes;
pub mod handlers;
pub mod types;
pub mod auth;
pub mod error;
pub mod version;
pub mod docs;
pub mod middleware;

#[cfg(test)]
pub mod tests;

// Publicly re-export routes and types for application use
pub use routes::register_routes;
pub use types::*;
pub use auth::AuthLayer;
pub use error::ApiError;
pub use version::ApiVersion;
pub use docs::openapi_spec;
