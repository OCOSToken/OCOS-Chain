//! OCOS-Chain: Identity Protocol Main Module
//!
//! Exports all submodules for on-chain identities, verifiable credentials, KYC/KYB,
//! soulbound tokens, reputation, and recovery. Provides the main entry point and
//! unified API for identity management in OCOS.

pub mod profile;
pub mod kyc;
pub mod soulbound;
pub mod reputation;
pub mod recovery;
pub mod attestation;
pub mod privacy;
pub mod error;
pub mod events;
pub mod types;
pub mod storage;

#[cfg(test)]
pub mod tests;

// -- Re-export key interfaces for external use --
pub use profile::*;
pub use kyc::*;
pub use soulbound::*;
pub use reputation::*;
pub use recovery::*;
pub use attestation::*;
pub use privacy::*;
pub use error::*;
pub use events::*;
pub use types::*;
pub use storage::*;
