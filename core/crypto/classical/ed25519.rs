//! OCOS-Chain: Ed25519 Signature Module
//!
//! Implements Ed25519 key generation, message signing, and signature verification.
//! Built with auditability, modularity, and cryptographic safety in mind.
//!
//! Ed25519 is used for user wallets, DAO voting, validator authentication, and light clients.

use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer, Verifier, SECRET_KEY_LENGTH};
use rand_core::OsRng;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Ed25519Error {
    #[error("Signature verification failed")]
    VerificationFailed,
    #[error("Invalid key data")]
    InvalidKey,
    #[error("Keypair generation failed")]
    KeyGenerationFailed,
}

/// A wrapper for an Ed25519 keypair
#[derive(Debug, Clone)]
pub struct Ed25519Keypair {
    pub public: Vec<u8>,
    pub private: Vec<u8>,
}

impl Ed25519Keypair {
    /// Generate a new Ed25519 keypair
    pub fn generate() -> Result<Self, Ed25519Error> {
        let mut csprng = OsRng;
        let keypair = Keypair::generate(&mut csprng);
        Ok(Ed25519Keypair {
            public: keypair.public.to_bytes().to_vec(),
            private: keypair.secret.to_bytes().to_vec(),
        })
    }

    /// Construct from raw private key (32 bytes)
    pub fn from_private_key(sk_bytes: &[u8]) -> Result<Self, Ed25519Error> {
        if sk_bytes.len() != SECRET_KEY_LENGTH {
            return Err(Ed25519Error::InvalidKey);
        }
        let secret = SecretKey::from_bytes(sk_bytes).map_err(|_| Ed25519Error::InvalidKey)?;
        let public = PublicKey::from(&secret);
        Ok(Ed25519Keypair {
            public: public.to_bytes().to_vec(),
            private: secret.to_bytes().to_vec(),
        })
    }
}

/// Sign a message with an Ed25519 private key
pub fn ed25519_sign(private_key: &[u8], message: &[u8]) -> Vec<u8> {
    let secret = SecretKey::from_bytes(private_key).expect("Invalid private key");
    let public = PublicKey::from(&secret);
    let keypair = Keypair { secret, public };
    keypair.sign(message).to_bytes().to_vec()
}

/// Verify an Ed25519 signature
pub fn ed25519_verify(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    match PublicKey::from_bytes(public_key)
        .and_then(|pk| Signature::from_bytes(signature).map(|sig| (pk, sig)))
    {
        Ok((pk, sig)) => pk.verify(message, &sig).is_ok(),
        Err(_) => false,
    }
}
