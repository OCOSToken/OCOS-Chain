//! OCOS-Chain: Reputation & Trust Score Module
//!
//! Tracks on-chain reputation scores for users, DAOs, or contracts.
//! Supports staking, voting, peer review, attestations, and trust-based governance.

use crate::identity::types::Address;
use std::collections::HashMap;

/// Reputation registry mapping address to score
#[derive(Default)]
pub struct ReputationRegistry {
    pub scores: HashMap<Address, u64>,
    // Optionally: history, DAO weighting, social proof
}

impl ReputationRegistry {
    /// Get current reputation score for an address
    pub fn get(&self, address: Address) -> u64 {
        *self.scores.get(&address).unwrap_or(&0)
    }

    /// Set or update a reputation score (e.g., via governance, DAO vote)
    pub fn set(&mut self, address: Address, new_score: u64) {
        self.scores.insert(address, new_score);
    }

    /// Increase reputation by delta (reward, social proof, etc.)
    pub fn reward(&mut self, address: Address, delta: u64) {
        let entry = self.scores.entry(address).or_insert(0);
        *entry = entry.saturating_add(delta);
    }

    /// Decrease reputation by delta (slashing, penalization, etc.)
    pub fn slash(&mut self, address: Address, delta: u64) {
        let entry = self.scores.entry(address).or_insert(0);
        *entry = entry.saturating_sub(delta);
    }

    /// Peer review or attestation (e.g., DAO upvote/downvote)
    pub fn peer_review(&mut self, address: Address, upvote: bool, weight: u64) {
        if upvote {
            self.reward(address, weight);
        } else {
            self.slash(address, weight);
        }
    }
}
