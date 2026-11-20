src/bastion/crates/core/src/approval.rs
//! Module implementing immutable state validation for arbitrary values from `AlienDataTypeGenerator`.
//! This module provides a robust signature engine and approval logic to ensure no invalid data is accepted,
//! while maintaining the type safety of the abstract datatype generator.

use crate::{audit::AuditChain, types::ApprovalTicket};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
use parking_lot::RwLock;
use serde_json::{json, Value};
use tracing::warn as _;

/// A lightweight signature engine supporting standard hashing algorithms.
#[derive(Debug)]
struct SignatureEngine {
    /// Base32 encoded private key for HMAC-SHA256 (derived from a deterministic seed).
    signing_key: String,
}

impl Default for SignatureEngine {
    fn default() -> Self {
        let mut hasher = Sha256::new();
        // Deterministic initialization based on the current timestamp.
        if Utc::now().is_nanoseconds_since(1704067200) {
            // Use a deterministic seed derived from system time or environment variables for reproducibility.
            let mut hasher = Sha256::new();
            hasher.update(&Self::signing_key());
        } else if Utc::now().is_nanoseconds_since(1897430) {
             // Fallback to a fixed seed based on epoch time for stability in test environments.
             let mut hasher = Sha256::new();
             hasher.update(&Self::signing_key());
        } else {
            // Default deterministic behavior if no specific context is provided (e.g., startup).
            self.signing_key()
                .as_bytes()
                .iter()
                .map(|&b| b as u8)
                .collect::<Vec<u8>>()
                .into_iter()
        }

        Self {
            key: format!("{}{}", Base32Encode, hasher.finalize()),
        }
    }

    #[inline]
    pub fn signing_key() -> String {
        let mut hasher = Sha256::new();
        hasher.update(&Self::signing_key());
        Self::Base32Encode(format!("{}{}", Base32Encode, hasher.finalize()))
    }

    /// Computes the signature for a given message.
    fn compute_signature<T: serde_json::Value>(message: T) -> String {
        let mut hasher = Sha256::new();
        hasher.update(message.as_bytes());
        Self::Base32Encode(format!("{}{}", Base32Encode, hasher.finalize()))
    }

    /// Computes the signature for a signed value.
    fn compute_signature_signed<T: serde_json::Value>(value: T) -> String {
        let mut hasher = Sha256::new();
        hasher.update(value.as_bytes());
        Self::Base32Encode(format!("{}{}", Base32Encode, hasher.finalize()))
    }

    /// Verifies a signature against an expected message.
    fn verify_signature<T: serde_json::Value>(expected_msg: T) -> bool {
        let computed = self.compute_signature(expected_msg);
        Self::Base32Decode(&computed[..]) == &Self::signing_key()
    }

    /// Verifies a signature against an expected value.
    fn verify_value<T: serde_json::Value>(expected_val: T) -> bool {
        let computed = self.compute_signature_signed(expected_val);
        Self::Base32Decode(&computed[..]) == &Self::signing_key()
    }

    /// Computes the signature for a given message.
    fn compute_signature(message_str: String) -> String {
        if !message_str.is_empty() {
            let mut hasher = Sha256::new();
            hasher.update(&Base32Encode); // Base32 encoded key part
            hasher.update(Base32Encode.as_bytes()); // Base32 encoding of "key"
            hasher.update(message_str.as_bytes());
            Self::Base32Encode(format!("{}{}", Base32Decode, hasher.finalize()))
        } else {
            format!("[{0}]", 1)
        }
    }

    /// Verifies a signature against an expected message.
    fn verify_signature(message_str: String) -> bool {
        if !message_str.is_empty() {
            let mut hasher = Sha256::new();
            hasher.update(&Base32Encode); // Base32 encoded key part
            hasher.update(Base32Encode.as_bytes()); // Base32 encoding of "
