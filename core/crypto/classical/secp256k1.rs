//! OCOS-Chain: secp256k1 Signature Module
//!
//! Provides ECDSA over secp256k1 for signing and verifying messages.
//! Commonly used in Ethereum, Bitcoin, and other EVM-compatible systems.
//!
//! This module supports key generation, signature creation and verification
//! with public-private key pairs using the secp256k1 elliptic curve.

use k256::{
    ecdsa::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    },
    elliptic_curve::sec1::ToEncodedPoint,
};
use rand_core::OsRng;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Secp256k1Error {
    #[error("Signature verification failed")]
    VerificationFailed,
    #[error("Invalid key format")]
    InvalidKey,
    #[error("Keypair generation failed")]
    KeyGenerationFailed,
}

/// A wrapper for a secp256k1 keypair
#[derive(Debug, Clone)]
pub struct Secp256k1Keypair {
    pub public: Vec<u8>,
    pub private: Vec<u8>,
}

impl Secp256k1Keypair {
    /// Generate a new keypair
    pub fn generate() -> Result<Self, Secp256k1Error> {
        let signing_key = SigningKey::random(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        let public = verifying_key.to_encoded_point(false).as_bytes().to_vec();
        let private = signing_key.to_bytes().to_vec();

        Ok(Secp256k1Keypair { public, private })
    }

    /// Construct keypair from private key (32 bytes)
    pub fn from_private_key(sk_bytes: &[u8]) -> Result<Self, Secp256k1Error> {
        let signing_key = SigningKey::from_bytes(sk_bytes.try_into().map_err(|_| Secp256k1Error::InvalidKey)?)
            .map_err(|_| Secp256k1Error::InvalidKey)?;
        let verifying_key = signing_key.verifying_key();
        let public = verifying_key.to_encoded_point(false).as_bytes().to_vec();

        Ok(Secp256k1Keypair {
            public,
            private: sk_bytes.to_vec(),
        })
    }
}

/// Sign a message using secp256k1 ECDSA
pub fn secp_sign(private_key: &[u8], message: &[u8]) -> Result<Vec<u8>, Secp256k1Error> {
    let signing_key = SigningKey::from_bytes(private_key.try_into().map_err(|_| Secp256k1Error::InvalidKey)?)
        .map_err(|_| Secp256k1Error::InvalidKey)?;
    let signature: Signature = signing_key.sign(message);
    Ok(signature.to_der().as_bytes().to_vec())
}

/// Verify a secp256k1 signature
pub fn secp_verify(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    let verifying_key = match VerifyingKey::from_sec1_bytes(public_key) {
        Ok(vk) => vk,
        Err(_) => return false,
    };

    let sig = match Signature::from_der(signature) {
        Ok(s) => s,
        Err(_) => return false,
    };

    verifying_key.verify(message, &sig).is_ok()
}
