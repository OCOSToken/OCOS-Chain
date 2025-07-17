//! OCOS-Chain: GraphQL Schema Definition
//!
//! Defines the Query, Mutation, and Subscription roots for the entire protocol.
//! This is the entry point for the GraphQL server.

use crate::api::graphql::{
    context::GraphQLContext,
    resolvers::{
        ledger::{LedgerQuery, LedgerMutation},
        governance::{GovernanceQuery, GovernanceMutation},
        identity::{IdentityQuery, IdentityMutation},
        liquidity::{LiquidityQuery, LiquidityMutation},
        dao::{DaoQuery, DaoMutation},
        analytics::{AnalyticsQuery},
    },
    subscriptions::{OcosSubscription},
};
use async_graphql::{Schema, MergedObject, MergedSubscription};

/// The combined Query root object, exposing all main protocol modules.
#[derive(MergedObject, Default)]
pub struct QueryRoot(
    LedgerQuery,
    GovernanceQuery,
    IdentityQuery,
    LiquidityQuery,
    DaoQuery,
    AnalyticsQuery,
);

/// The combined Mutation root object.
#[derive(MergedObject, Default)]
pub struct MutationRoot(
    LedgerMutation,
    GovernanceMutation,
    IdentityMutation,
    LiquidityMutation,
    DaoMutation,
);

/// The combined Subscription root object (real-time events).
#[derive(MergedSubscription, Default)]
pub struct SubscriptionRoot(OcosSubscription);

/// Build and return the OCOS-Chain GraphQL schema.
pub fn build_schema() -> Schema<QueryRoot, MutationRoot, SubscriptionRoot> {
    Schema::build(QueryRoot::default(), MutationRoot::default(), SubscriptionRoot::default())
        .data(GraphQLContext::default())
        // Optionally add extensions, tracing, and limiters here
        .finish()
}
