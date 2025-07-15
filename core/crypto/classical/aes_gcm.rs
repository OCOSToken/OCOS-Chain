//! OCOS-Chain: AES-256-GCM Encryption Module
//!
//! Provides authenticated encryption using AES-GCM with 256-bit keys.
//! Designed for encrypting sensitive payloads (private state, messages, bridge txs).
//!
//! Ensures confidentiality, integrity and non-repudiation of data on-chain and off-chain.

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng, Payload},
    Aes256Gcm, Nonce, Key,
};
use thiserror::Error;

pub const AES_KEY_LEN: usize = 32; // 256-bit key
pub const AES_NONCE_LEN: usize = 12; // 96-bit GCM standard

#[derive(Debug, Error)]
pub enum AesError {
    #[error("Encryption failed")]
    EncryptError,
    #[error("Decryption failed")]
    DecryptError,
    #[error("Invalid key or nonce size")]
    InvalidInput,
}

/// Encrypt data using AES-256-GCM
pub fn aes_encrypt(key: &[u8], nonce: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, AesError> {
    if key.len() != AES_KEY_LEN || nonce.len() != AES_NONCE_LEN {
        return Err(AesError::InvalidInput);
    }

    let cipher = Aes256Gcm::new(Key::from_slice(key));
    let nonce = Nonce::from_slice(nonce);

    cipher
        .encrypt(nonce, Payload { msg: plaintext, aad: b"" })
        .map_err(|_| AesError::EncryptError)
}

/// Decrypt data using AES-256-GCM
pub fn aes_decrypt(key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, AesError> {
    if key.len() != AES_KEY_LEN || nonce.len() != AES_NONCE_LEN {
        return Err(AesError::InvalidInput);
    }

    let cipher = Aes256Gcm::new(Key::from_slice(key));
    let nonce = Nonce::from_slice(nonce);

    cipher
        .decrypt(nonce, Payload { msg: ciphertext, aad: b"" })
        .map_err(|_| AesError::DecryptError)
}
