//! OCOS-Chain: GraphQL Middleware & Interceptors
//!
//! Provides modular middleware layers for authentication, logging, error handling,
//! rate limiting, metrics, and request tracing on GraphQL queries and mutations.

use async_graphql::{ServerError, ServerResult, ErrorExtensions, Middleware, Next, Request, Response};
use std::sync::Arc;
use std::time::Instant;

use crate::api::graphql::context::GraphQLContext;

/// Authentication middleware (JWT, API key, session)
pub struct AuthMiddleware;

#[async_trait::async_trait]
impl Middleware<GraphQLContext> for AuthMiddleware {
    async fn call(
        &self,
        ctx: &GraphQLContext,
        req: Request,
        next: Next<'_, GraphQLContext>,
    ) -> ServerResult<Response> {
        // Example: check API key or JWT in ctx.auth
        if ctx.auth.address.is_none() {
            return Err(ServerError::new("Unauthorized", None));
        }
        next.run(ctx, req).await
    }
}

/// Logging middleware (request, response, timing, and audit)
pub struct LoggingMiddleware;

#[async_trait::async_trait]
impl Middleware<GraphQLContext> for LoggingMiddleware {
    async fn call(
        &self,
        ctx: &GraphQLContext,
        req: Request,
        next: Next<'_, GraphQLContext>,
    ) -> ServerResult<Response> {
        let start = Instant::now();
        let req_id = &ctx.request_id;
        log::info!("[GraphQL][{}] Incoming query: {:?}", req_id, req.query);
        let resp = next.run(ctx, req).await;
        log::info!(
            "[GraphQL][{}] Completed in {:?}, status: {}",
            req_id,
            start.elapsed(),
            if resp.is_ok() { "OK" } else { "ERR" }
        );
        resp
    }
}

/// Error formatting middleware (customizes API errors for clients)
pub struct ErrorFormatMiddleware;

#[async_trait::async_trait]
impl Middleware<GraphQLContext> for ErrorFormatMiddleware {
    async fn call(
        &self,
        ctx: &GraphQLContext,
        req: Request,
        next: Next<'_, GraphQLContext>,
    ) -> ServerResult<Response> {
        match next.run(ctx, req).await {
            Ok(resp) => Ok(resp),
            Err(err) => {
                let msg = format!("API error: {}", err.message());
                let err = ServerError::new(msg, None).extend_with(|_e, e| {
                    e.set("request_id", &ctx.request_id);
                });
                Err(err)
            }
        }
    }
}

/// Rate limiting middleware (per IP/user/session)
pub struct RateLimitMiddleware;

#[async_trait::async_trait]
impl Middleware<GraphQLContext> for RateLimitMiddleware {
    async fn call(
        &self,
        ctx: &GraphQLContext,
        req: Request,
        next: Next<'_, GraphQLContext>,
    ) -> ServerResult<Response> {
        // Example: Check rate limit for ctx.auth.address or IP
        // if rate_limit_exceeded() { return Err(ServerError::new("Rate limit exceeded", None)); }
        next.run(ctx, req).await
    }
}

/// Request tracing middleware (for analytics and performance)
pub struct TracingMiddleware;

#[async_trait::async_trait]
impl Middleware<GraphQLContext> for TracingMiddleware {
    async fn call(
        &self,
        ctx: &GraphQLContext,
        req: Request,
        next: Next<'_, GraphQLContext>,
    ) -> ServerResult<Response> {
        let span = tracing::info_span!("graphql_request", request_id = %ctx.request_id);
        let _guard = span.enter();
        let resp = next.run(ctx, req).await;
        drop(_guard);
        resp
    }
}

// --- Registration helper ---
pub fn register_middlewares(schema: async_graphql::SchemaBuilder<impl Send + Sync, impl Send + Sync, impl Send + Sync>)
    -> async_graphql::SchemaBuilder<impl Send + Sync, impl Send + Sync, impl Send + Sync>
{
    schema
        .middleware(Arc::new(AuthMiddleware))
        .middleware(Arc::new(LoggingMiddleware))
        .middleware(Arc::new(ErrorFormatMiddleware))
        .middleware(Arc::new(RateLimitMiddleware))
        .middleware(Arc::new(TracingMiddleware))
}
