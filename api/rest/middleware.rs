//! OCOS-Chain: REST API Middleware Layer
//!
//! Universal middleware for CORS, logging, compression, rate-limiting, error handling, and monitoring.

use axum::{
    http::{HeaderValue, Method},
    middleware::{self, Next},
    response::Response,
    Router,
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
    compression::CompressionLayer,
    limit::RequestBodyLimitLayer,
};
use std::time::Instant;

// --- Setup all middleware for API server ---
pub fn setup_middleware(router: Router) -> Router {
    router
        // CORS policy for browser-based apps and clients
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                .allow_origin(Any)
                .allow_headers(Any)
        )
        // Gzip/Deflate compression for faster responses
        .layer(CompressionLayer::new())
        // Logging and tracing for request/response monitoring
        .layer(TraceLayer::new_for_http())
        // Limit request body size (e.g., max 1MB)
        .layer(RequestBodyLimitLayer::new(1024 * 1024))
        // Custom global middleware: response time header
        .layer(middleware::from_fn(response_time_middleware))
}

// --- Example: Custom middleware for response time header ---
async fn response_time_middleware<B>(req: axum::http::Request<B>, next: Next<B>) -> Response {
    let start = Instant::now();
    let mut res = next.run(req).await;
    let elapsed = start.elapsed().as_millis().to_string();
    res.headers_mut().insert(
        "x-response-time-ms",
        HeaderValue::from_str(&elapsed).unwrap(),
    );
    res
}
