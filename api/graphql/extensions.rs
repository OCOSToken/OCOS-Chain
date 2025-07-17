//! OCOS-Chain: GraphQL API Extensions Module
//!
//! Provides schema- and request-level extensions for tracing, metrics, caching, introspection,
//! and analytics—compatible with Apollo and OpenTelemetry for dashboards and production observability.

use async_graphql::{
    Extension, ServerResult, Response, ServerError, Request, Variables,
    extensions::{Tracing, Logger, ApolloTracing, ExtensionFactory},
};
use std::sync::Arc;

/// Tracing extension (OpenTelemetry-compatible)
pub struct OcosTracingExtension;

impl ExtensionFactory for OcosTracingExtension {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(Tracing)
    }
}

/// Logging extension (per-request)
pub struct OcosLoggerExtension;

impl ExtensionFactory for OcosLoggerExtension {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(Logger)
    }
}

/// Apollo tracing extension (performance/latency for Apollo Studio)
pub struct OcosApolloTracingExtension;

impl ExtensionFactory for OcosApolloTracingExtension {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(ApolloTracing)
    }
}

/// Introspection extension (optionally enable/disable for production)
pub struct OcosIntrospectionExtension {
    pub enabled: bool,
}

impl ExtensionFactory for OcosIntrospectionExtension {
    fn create(&self) -> Arc<dyn Extension> {
        if self.enabled {
            Arc::new(async_graphql::extensions::Introspection)
        } else {
            // Return a no-op extension or an error if introspection is forbidden
            Arc::new(NoopExtension)
        }
    }
}

/// No-op extension (for disabled features)
pub struct NoopExtension;

impl Extension for NoopExtension {}

/// Extension registration helper
pub fn register_extensions(
    schema: async_graphql::SchemaBuilder<impl Send + Sync, impl Send + Sync, impl Send + Sync>,
    enable_apollo: bool,
    enable_tracing: bool,
    enable_introspection: bool,
) -> async_graphql::SchemaBuilder<impl Send + Sync, impl Send + Sync, impl Send + Sync> {
    let mut schema = schema
        .extension(OcosLoggerExtension)
        .extension(OcosTracingExtension);

    if enable_apollo {
        schema = schema.extension(OcosApolloTracingExtension);
    }
    schema = schema.extension(OcosIntrospectionExtension { enabled: enable_introspection });
    schema
}
