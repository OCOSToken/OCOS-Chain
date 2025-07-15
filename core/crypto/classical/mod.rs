//! OCOS-Chain: Classical Cryptography Module
//!
//! Provides widely adopted cryptographic algorithms used in legacy and modern blockchains,
//! including Ed25519, secp256k1, and AES-GCM for symmetric encryption.
//!
//! These primitives are used across OCOS components such as wallet signing, bridge encryption,
//! validator authentication, and smart contract logic.

pub mod ed25519;
pub mod secp256k1;
pub mod aes_gcm;

// -- Universal re-exports for classical usage --

pub use ed25519::{Ed25519Keypair, ed25519_sign, ed25519_verify};
pub use secp256k1::{Secp256k1Keypair, secp_sign, secp_verify};
pub use aes_gcm::{aes_encrypt, aes_decrypt};
