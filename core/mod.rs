//! OCOS-Chain: Unified Cryptography Root Module
//!
//! Aggregates classical and quantum-resistant cryptographic subsystems,
//! universal hash functions, key derivation, and utilities. Designed
//! for modular security, easy extensibility, and blockchain auditability.

pub mod classical;
pub mod quantum;
pub mod hashing;
pub mod kdf;
pub mod utils;

// -- Universal re-exports for global cryptographic use --

pub use classical::{
    ed25519::{Ed25519Keypair, ed25519_sign, ed25519_verify},
    secp256k1::{Secp256k1Keypair, secp_sign, secp_verify},
    aes_gcm::{aes_encrypt, aes_decrypt},
};

pub use quantum::{
    dilithium::{DilithiumKeypair, dilithium_sign, dilithium_verify},
    xmss::{XmssKeypair, xmss_sign, xmss_verify},
    kyber::{KyberKeypair, kyber_encapsulate, kyber_decapsulate},
    falcon::{FalconKeypair, falcon_sign, falcon_verify},
};

pub use hashing::{sha3_256, blake3_256};
pub use kdf::{pbkdf2_derive, argon2_derive, scrypt_derive};
pub use utils::{random_bytes, base58_encode, base64_encode};

// -- Common crypto error type --
#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("Invalid key or signature")]
    InvalidSignature,
    #[error("Encryption/Decryption failed")]
    EncryptionFailed,
    #[error("Key derivation failed")]
    KeyDerivationFailed,
    #[error("Unsupported algorithm")]
    UnsupportedAlgorithm,
    #[error("Unknown cryptographic error")]
    Unknown,
}
