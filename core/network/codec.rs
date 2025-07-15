//! OCOS-Chain: Network Message Codec
//!
//! Provides serialization and deserialization of network messages for efficient,
//! cross-platform, and auditable wire transmission. Supports pluggable formats
//! such as bincode, JSON, and Protobuf.
//!
//! Modular, future-proof, and performance-oriented for peer-to-peer blockchain messaging.

use crate::network::message::NetworkMessage;
use thiserror::Error;

/// Codec error type
#[derive(Debug, Error)]
pub enum CodecError {
    #[error("Serialization failed")]
    SerializationFailed,
    #[error("Deserialization failed")]
    DeserializationFailed,
    #[error("Unsupported codec format")]
    UnsupportedCodec,
}

/// Supported serialization formats
#[derive(Debug, Clone, Copy)]
pub enum CodecFormat {
    Bincode,
    Json,
    // Protobuf, RLP, etc. can be added here in the future
}

/// Encode a network message into bytes
pub fn encode_message(msg: &NetworkMessage, format: CodecFormat) -> Result<Vec<u8>, CodecError> {
    match format {
        CodecFormat::Bincode => bincode::serialize(msg).map_err(|_| CodecError::SerializationFailed),
        CodecFormat::Json => serde_json::to_vec(msg).map_err(|_| CodecError::SerializationFailed),
    }
}

/// Decode bytes into a network message
pub fn decode_message(bytes: &[u8], format: CodecFormat) -> Result<NetworkMessage, CodecError> {
    match format {
        CodecFormat::Bincode => bincode::deserialize(bytes).map_err(|_| CodecError::DeserializationFailed),
        CodecFormat::Json => serde_json::from_slice(bytes).map_err(|_| CodecError::DeserializationFailed),
    }
}
