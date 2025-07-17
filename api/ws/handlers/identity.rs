//! OCOS-Chain: WebSocket Handler – Identity Events
//!
//! Streams real-time identity, reputation, verification, soulbound, and KYC/attestation events.

use crate::ws::types::{
    WsTopic, WsMessage, IdentityInfo, ReputationEvent, AttestationEvent, SoulboundEvent,
};
use crate::ws::router::WsRouter;

/// Handles streaming of all on-chain identity and reputation events.
pub struct IdentityHandler;

impl IdentityHandler {
    /// Broadcast when a new on-chain identity is registered or updated
    pub fn on_identity_registered(router: &WsRouter, identity: IdentityInfo) {
        let msg = WsMessage::IdentityRegistered { identity: identity.clone() };
        router.broadcast(&WsTopic::Identity, &msg);
    }

    /// Broadcast when a reputation or badge is updated (e.g. level, score)
    pub fn on_reputation_update(router: &WsRouter, rep_event: ReputationEvent) {
        let msg = WsMessage::ReputationUpdated { event: rep_event.clone() };
        router.broadcast(&WsTopic::Identity, &msg);
    }

    /// Broadcast when a new KYC/KYB attestation or verification is completed
    pub fn on_attestation_event(router: &WsRouter, attest: AttestationEvent) {
        let msg = WsMessage::AttestationEvent { event: attest.clone() };
        router.broadcast(&WsTopic::Identity, &msg);
    }

    /// Broadcast when a soulbound (non-transferable) token is issued or updated
    pub fn on_soulbound_event(router: &WsRouter, soulbound: SoulboundEvent) {
        let msg = WsMessage::SoulboundEvent { event: soulbound.clone() };
        router.broadcast(&WsTopic::Identity, &msg);
    }

    /// (Optional) Custom identity-related events (social recovery, zkID, etc.)
    pub fn on_custom_event(router: &WsRouter, payload: serde_json::Value) {
        let msg = WsMessage::IdentityCustomEvent { payload };
        router.broadcast(&WsTopic::Identity, &msg);
    }
}

// -- Example message types for reference --

/*
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsMessage {
    IdentityRegistered { identity: IdentityInfo },
    ReputationUpdated { event: ReputationEvent },
    AttestationEvent { event: AttestationEvent },
    SoulboundEvent { event: SoulboundEvent },
    IdentityCustomEvent { payload: serde_json::Value },
    // ... more
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityInfo {
    pub id: u64,
    pub address: String,
    pub did: String,
    pub display_name: Option<String>,
    pub profile: serde_json::Value,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationEvent {
    pub identity_id: u64,
    pub score: i32,
    pub badge: Option<String>,
    pub reason: Option<String>,
    pub updated_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationEvent {
    pub identity_id: u64,
    pub attestor: String,
    pub attestation_type: String,
    pub status: String,
    pub metadata: serde_json::Value,
    pub issued_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulboundEvent {
    pub identity_id: u64,
    pub token_id: u64,
    pub attributes: serde_json::Value,
    pub issued_at: u64,
}
*/
