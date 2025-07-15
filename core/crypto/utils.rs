//! OCOS-Chain: Cryptographic Utilities Module
//!
//! Universal helpers for randomness, encoding, byte manipulation and secure operations.
//! Used across the protocol for wallet generation, address encoding, key zeroization, etc.

use rand_core::{OsRng, RngCore};
use base64::{engine::general_purpose, Engine as _};
use bs58;
use zeroize::Zeroize;

/// Generate cryptographically secure random bytes
pub fn random_bytes(len: usize) -> Vec<u8> {
    let mut buf = vec![0u8; len];
    OsRng.fill_bytes(&mut buf);
    buf
}

/// Encode data as base64 (URL-safe)
pub fn base64_encode(data: &[u8]) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(data)
}

/// Decode base64 (URL-safe)
pub fn base64_decode(data: &str) -> Result<Vec<u8>, base64::DecodeError> {
    general_purpose::URL_SAFE_NO_PAD.decode(data)
}

/// Encode data as base58 (Bitcoin-style)
pub fn base58_encode(data: &[u8]) -> String {
    bs58::encode(data).into_string()
}

/// Decode base58 (Bitcoin-style)
pub fn base58_decode(data: &str) -> Result<Vec<u8>, bs58::decode::Error> {
    bs58::decode(data).into_vec()
}

/// Convert hex string to bytes
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(hex)
}

/// Convert bytes to hex string (lowercase)
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

/// Securely zeroize a byte buffer (clears sensitive data from memory)
pub fn zeroize_bytes(buf: &mut [u8]) {
    buf.zeroize();
}
