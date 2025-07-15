//! OCOS-Chain: Quantum-Resistant Cryptography Module
//!
//! Aggregates all post-quantum cryptographic algorithms used across the protocol,
//! including Dilithium, XMSS, Kyber, and Falcon. Ensures long-term security
//! against quantum adversaries and supports modular replacement of primitives.
//!
//! Used for signature verification, key exchange, secure messaging, and on-chain governance.

pub mod dilithium;
pub mod xmss;
pub mod kyber;
pub mod falcon;

// -- Re-export public interfaces for unified access --

pub use dilithium::{DilithiumKeypair, dilithium_sign, dilithium_verify};
pub use xmss::{XmssKeypair, xmss_sign, xmss_verify};
pub use kyber::{KyberKeypair, kyber_encapsulate, kyber_decapsulate};
pub use falcon::{FalconKeypair, falcon_sign, falcon_verify};

// -- Standard error type for quantum cryptography --
use thiserror::Error;

#[derive(Debug, Error)]
pub enum QuantumCryptoError {
    #[error("Signature verification failed")]
    VerificationFailed,
    #[error("Invalid key format")]
    InvalidKey,
    #[error("Key generation failed")]
    KeyGenerationFailed,
    #[error("Encapsulation/Decapsulation error")]
    KeyExchangeError,
    #[error("Unsupported quantum algorithm")]
    UnsupportedAlgorithm,
    #[error("Unknown quantum cryptographic error")]
    Unknown,
}
