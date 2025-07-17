//! OCOS-Chain: GraphQL API Main Module
//!
//! Root module for OCOS-Chain's GraphQL API. Wires up schema, context, resolvers, middleware, and subscriptions.

pub mod schema;
pub mod context;
pub mod types;
pub mod resolvers;
pub mod directives;
pub mod middleware;
pub mod subscriptions;
pub mod error;
pub mod extensions;

#[cfg(test)]
pub mod tests;

// -- Re-export top-level interfaces for external GraphQL server --
pub use schema::{build_schema, QueryRoot, MutationRoot, SubscriptionRoot};
pub use context::GraphQLContext;
pub use types::*;
pub use resolvers::*;
pub use directives::*;
pub use middleware::*;
pub use subscriptions::*;
pub use error::*;
pub use extensions::*;
