src/bastion/crates/core/src/types.rs

use std::sync::{Arc, Mutex};
use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BTreeMap};
use chrono::{DateTime, Utc};
#[cfg(feature = "serde")]
use serde_json;

/// ============================================================================
// 1. DEFINITION OF BASED ON THE EXAMPLE: AUDIT_ENTRY & CREDENTIALS
// ============================================================================

#[derive(Debug, Clone)] // Ensures immutability and clone safety for internal use
pub enum AuditEntry {
    /// Represents a single audit event with computed hashes (for local state) or raw data.
    Event(AuditEvent),
}

/// A specific type of audit entry containing sequence numbers and timestamps.
#[derive(Debug, Clone)]
struct AuditEvent {
    pub sequence: u64, // Unique identifier for the batch/event within this session context
    pub timestamp: DateTime<Utc>,
    /// Raw input payload (for validation or logging). In production, use SHA-256 hashing here.
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_payload: Option<Vec<u8>>, 
}

impl AuditEvent {
    pub fn compute_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        
        // Compute hash of the sequence number (internal state identifier)
        hasher.update(self.sequence.to_le_bytes().as_slice());

        // Hash the current timestamp in a secure manner. 
        // Note: The raw payload is kept for verification/audit trails; SHA-256 hashes are used as internal session IDs.
        let hash = match self.raw_payload {
            Some(p) => hasher.update(&p),
            None => {}
        };

        hasher.finalize().into()
    }

    pub fn is_valid(&self, prev_hash: &[u8; 32]) -> bool {
        // Simple validation check: ensure the computed hash matches a stored reference.
        let computed = self.compute_hash();
        
        if !computed.is_empty() && computed == *prev_hash {
            return true;
        }

        false
    }
}

/// ============================================================================
// 2. DEFINITION OF CREDENTIALS (Enhanced with Encryption & Verification)
// ============================================================================

#[derive(Debug, Clone)] // Immutable storage for credential state
pub struct Credential {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// The master secret used to encrypt this string. This must match the key in `encrypt()` or decrypt() methods if not provided as a separate field.
    #[serde(default)]
    pub encrypted_value: String, 
}

impl Credential {
    /// Returns the expiration time if not set, or now (for verification).
    fn get_expiry_time(&self) -> DateTime<Utc> {
        match self.expires_at {
            None => Utc::now(), // Default to "expire at current system clock" for testing/verification.
            Some(_) => *self.expires_at.clone().unwrap_or(Utc::now()), 
        }
    }

    pub fn is_expired(&self) -> bool {
        self.get_expiry_time() > chrono::Utc::now()
    }

    /// Encrypts the secret value using AES-256-GCM (Galois/Counter Mode with 19 bytes key).
    // This ensures that even if a credential is modified, it cannot be decrypted without the correct master secret.
    pub fn encrypt(&self) -> String {
        let mut cipher = aesgcm::CipherBuilder::<AES_256>::new()
            .with_key_length(AES_256::KEY_LENGTH as u32);

        match self.value.as_bytes().iter_mut().enumerate() {
            (i, b) => cipher.encrypt(b[i..], i + 1),
            _ => panic!("Invalid credential format"), 
        }
    }

    /// Decrypts a raw encrypted string using AES-256-GCM. Assumes the ciphertext was generated with an identical key to this struct's master secret.
    pub fn decrypt(&self) -> Result<String, String> {
        let mut cipher = aesgcm::CipherBuilder::<AES_256>::new()
            .with_key_length(AES_256::KEY_LENGTH as u32);

        match self.value.as_bytes().iter_mut().enumerate() {
            (i, b) => cipher.decrypt(b[i..], i + 1),
            _ => Err(format!("Invalid encrypted credential format: expected string
