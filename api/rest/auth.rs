//! OCOS-Chain: REST API Authentication & Authorization Layer
//!
//! Middleware and helpers for JWT/token auth, API key, session validation, and role/permission checks.

use axum::{async_trait, extract::{FromRequestParts, TypedHeader}, headers::Authorization, http::request::Parts, middleware::Next, response::Response};
use axum::http::StatusCode;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, TokenData};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub roles: Vec<String>,
}

#[derive(Clone, Default)]
pub struct AuthLayer {
    pub jwt_secret: Arc<String>,
}

#[async_trait]
impl<B> axum::middleware::Middleware<B> for AuthLayer
where
    B: Send + 'static,
{
    async fn handle(&self, req: axum::http::Request<B>, next: Next<B>) -> Response {
        // Example: Validate JWT in Authorization header
        let auth_header = req.headers().get("authorization").and_then(|h| h.to_str().ok());
        if let Some(header) = auth_header {
            if let Some(token) = header.strip_prefix("Bearer ") {
                if let Ok(claims) = validate_jwt(token, &self.jwt_secret) {
                    // Optionally, check roles/permissions here
                    return next.run(req).await;
                }
            }
        }
        Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(axum::body::Body::from("Unauthorized"))
            .unwrap()
    }
}

// --- Helper: JWT validation ---
fn validate_jwt(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(Algorithm::HS256);
    let token_data: TokenData<Claims> = decode::<Claims>(token, &decoding_key, &validation)?;
    Ok(token_data.claims)
}
