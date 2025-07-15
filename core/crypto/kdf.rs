//! OCOS-Chain: Key Derivation Functions (KDF) Module
//!
//! Provides industry-standard key derivation schemes for password hashing,
//! wallet seed expansion, identity hardening, and encrypted storage access.
//!
//! Supported: PBKDF2, Argon2id, Scrypt

use pbkdf2::{pbkdf2_hmac};
use argon2::{self, Config as Argon2Config, Variant};
use scrypt::{scrypt, Params as ScryptParams};
use sha2::Sha256;
use rand_core::{OsRng, RngCore};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KdfError {
    #[error("Key derivation failed")]
    DerivationFailed,
    #[error("Invalid parameter")]
    InvalidParameter,
}

/// Derive a key using PBKDF2 (HMAC-SHA256)
pub fn pbkdf2_derive(password: &[u8], salt: &[u8], iterations: u32, length: usize) -> Vec<u8> {
    let mut derived = vec![0u8; length];
    pbkdf2_hmac::<Sha256>(password, salt, iterations, &mut derived);
    derived
}

/// Derive a key using Argon2id (memory-hard)
pub fn argon2_derive(password: &[u8], salt: &[u8], length: usize) -> Result<Vec<u8>, KdfError> {
    let config = Argon2Config {
        variant: Variant::Argon2id,
        mem_cost: 65536,    // 64 MiB
        time_cost: 3,
        lanes: 4,
        hash_length: length as u32,
        ..Default::default()
    };
    argon2::hash_raw(password, salt, &config).map_err(|_| KdfError::DerivationFailed)
}

/// Derive a key using scrypt
pub fn scrypt_derive(password: &[u8], salt: &[u8], length: usize) -> Result<Vec<u8>, KdfError> {
    let params = ScryptParams::recommended();
    let mut output = vec![0u8; length];
    scrypt(password, salt, &params, &mut output).map_err(|_| KdfError::DerivationFailed)?;
    Ok(output)
}

/// Generate secure random salt
pub fn generate_salt(len: usize) -> Vec<u8> {
    let mut salt = vec![0u8; len];
    OsRng.fill_bytes(&mut salt);
    salt
}
