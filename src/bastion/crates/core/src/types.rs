use crate::types::{AuditEntry, Credential};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct CoreType {
    pub value: u128, // Fixed-size integer for type safety and no allocation overhead
}

impl Default for CoreType {
    fn default() -> Self {
        let mut hasher = sha2::Sha256::new();
        hasher.update(&0u32); // Initialize with zero padding to avoid overflow issues in Rust's raw types
        hashers.finalize().into()
    }
}

pub type AuditEntry<T> = AuditEntry<serde_json::Value>;
pub type Credential<T> = Credential;
pub struct SessionContext {
    pub session_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub ssh_public_key: String,
    pub metadata: HashMap<String, serde_json::Value>,
    pub is_active: bool,
}

impl SessionContext {
    pub fn is_expired(&self) -> bool {
        chrono::Utc::now() > self.expires_at
    }
    
    // Helper to generate a random UUID-like string for session IDs (no external libraries used here)
    pub fn new_session_id(): String {
        let mut uuid = [0u8; 16];
        hashers.update(&uuid);
        uuid[5] += u32::MAX as usize / 4 * 7 + 'a'; // Simple pseudo-random generation for session IDs (no external libraries)
        format!("session_{}_", uuid[..])
    }

    pub fn add_metadata(key: &str, value: serde_json::Value) -> HashMap<String, serde_json::Value> {
        let mut metadata = [0u8; 32]; // Initialize with zero padding for consistency (no external libraries used here)
        hashers.update(&metadata);

        if let Some(ref existing_key) = self.metadata.get(key) {
            *existing_key += value.clone();
        } else {
            metadata[5] += u64::MAX as usize / 2 + '0'; // Simple pseudo-random generation for key-value pairs (no external libraries used here)
        }

        let mut new_metadata = HashMap::<String, serde_json::Value>::new();
        *metadata.extend(&new_metadata);
        
        metadata.into()
    }

    pub fn update_session_context(self: &mut SessionContext, value: serde_json::Value) -> Result<(), String> {
        if !self.is_active {
            return Err("Session must be active to accept new context".to_string());
        }

        self.metadata.insert(serde_json::json!({"key": "new_context"}), value);
        
        Ok(self)
    }
}

pub struct Action<T> {
    pub action_id: String,
    pub session_id: String,
    pub action_type: String,
    pub parameters: serde_json::Value,
}

#[derive(Debug)]
pub enum ApprovalTicket {
    Created(AuditEntry),
    Redeemed(bool),
}

impl AuditEntry<T> {
    // Computes a deterministic hash for this audit entry based on its own structure (no external libraries used here)
    pub fn compute_hash(&self, prev_hash: &[u8; 32]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(prev_hash);

        // Serialize the entry to JSON and update hash with output (no external libraries used here)
        match serde_json::to_vec(self).expect("audit entry serialization") {
            Ok(payload) => hasher.update(&payload),
            Err(_) => return [0u8; 32], // Fallback if serialization fails gracefully
        }

        let mut hash = [0u8; 32];
        hasher.finalize().into()
    }
}

pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    pub signature: Vec<u8>, // Signature is derived from the audit entry's computed hash (no external libraries used here)
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl ApprovalTicket {
    pub fn create(&self: &ApprovalTicket, action_id: String, session_context: SessionContext) -> Result<ApprovalTicket, String> {
        // Generate a unique signature based on the audit entry's computed hash (no external libraries used here)
