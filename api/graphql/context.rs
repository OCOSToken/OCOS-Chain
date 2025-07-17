//! OCOS-Chain: GraphQL API Execution Context
//!
//! Defines shared context (auth, state, metadata, config) for all GraphQL resolvers and middleware.

use async_graphql::Context as GqlContext;
use std::sync::Arc;

/// Example state handles (customize as needed)
pub struct DbHandle;        // Replace with actual DB/State/Storage types
pub struct ChainHandle;     // Blockchain node handle (RPC, query, etc.)

/// Auth info for current request (session, user, permissions)
#[derive(Clone, Debug)]
pub struct AuthInfo {
    pub address: Option<String>,
    pub is_admin: bool,
    pub scopes: Vec<String>,
    pub api_key: Option<String>,
    // Additional JWT/session/token info as needed
}

/// OCOS-Chain GraphQL context (injected into all resolvers)
#[derive(Clone)]
pub struct GraphQLContext {
    pub auth: AuthInfo,
    pub db: Arc<DbHandle>,
    pub chain: Arc<ChainHandle>,
    pub request_id: String,
    // You can add logger, metrics, cache, etc. here
}

impl Default for GraphQLContext {
    fn default() -> Self {
        GraphQLContext {
            auth: AuthInfo {
                address: None,
                is_admin: false,
                scopes: vec![],
                api_key: None,
            },
            db: Arc::new(DbHandle),
            chain: Arc::new(ChainHandle),
            request_id: uuid::Uuid::new_v4().to_string(),
        }
    }
}

// Example helper to get context from resolver
pub fn get_context<'a>(ctx: &'a GqlContext<'_>) -> &'a GraphQLContext {
    ctx.data_unchecked::<GraphQLContext>()
}
