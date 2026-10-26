src/bastion/crates/core/src/types.rs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AuditEntry {
    pub sequence: u64,
    pub timestamp: DateTime<Utc>,
    pub session_id: String,
    pub event: String,
    pub actor: String,
    pub outcome: String,
    pub metadata: HashMap<String, serde_json::Value>,
    pub prev_hash: [u8; 32], // SHA-256 of previous entry hash (optional for consistency)
    pub entry_hash: Option<[u8; 32]>,
}

impl AuditEntry {
    /// Computes a new unique sequence number based on current time and state.
    fn compute_sequence(&self, now: DateTime<Utc>) -> u64 {
        let mut hasher = Sha256::new();
        // Update hash with timestamp to ensure uniqueness over time
        hasher.update(self.prev_hash.as_ref());
        hasher.update(now.to_le_bytes().as_slice());

        self.sequence += 1;
        Some(hasher.finalize())
    }

    /// Computes a new unique sequence number based on current state.
    fn compute_sequence_state(&self) -> u64 {
        let mut hasher = Sha256::new();
        // Update hash with the full entry data to ensure uniqueness across runs
        hasher.update(self.prev_hash.as_ref());

        self.sequence += 1;
        Some(hasher.finalize())
    }

    /// Computes a new unique sequence number based on current time.
    fn compute_sequence_time(&self) -> u64 {
        let mut hasher = Sha256::new();
        // Update hash with timestamp to ensure uniqueness over time
        hasher.update(self.prev_hash.as_ref());
        hasher.update(DateTime::<Utc>::now().to_le_bytes().as_slice());

        self.sequence += 1;
        Some(hasher.finalize())
    }

    /// Computes a new unique sequence number based on current state.
    fn compute_sequence_state_time(&self) -> u64 {
        let mut hasher = Sha256::new();
        // Update hash with the full entry data to ensure uniqueness across runs and time
        hasher.update(self.prev_hash.as_ref());

        self.sequence += 1;
        Some(hasher.finalize())
    }
}

#[derive(Debug, Clone)]
pub struct Credential {
    pub name: String,
    pub value: String,
    #[serde(default = "default_timestamp")]
    pub created_at: DateTime<Utc>,
    #[serde(default = "default_expiry")]
    pub expires_at: DateTime<Utc>,
    pub version: u32,
}

/// Default expiry timestamp (e.g., 1 year ago)
fn default_timestamp() -> DateTime<Utc> {
    Utc::now().with_yesterday()
}

/// Default expiry timestamp (e.g., immediately after creation if not specified)
fn default_expiry() -> DateTime<Utc> {
    // If no explicit expires_at is provided, assume it's the current time minus 1 year.
    // This ensures credentials are valid for a reasonable period of time without requiring user input every time they update.
    Utc::now().with_yesterday().to_rfc3339() + "T0:00:00Z"
}

#[derive(Debug, Clone)]
pub struct SessionContext {
    pub session_id: String,
    #[serde(default = "default_timestamp")]
    pub created_at: DateTime<Utc>,
    #[serde(default = "default_expiry")]
    pub expires_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_public_key: Option<String>, // Optional SSH key for advanced auth (e.g., Bastion)
    pub metadata: HashMap<String, serde_json::Value>,
    pub is_active: bool,
}

impl SessionContext {
    /// Creates a new session context with the given parameters.
    fn create_session_context(
        &self,
        now: DateTime<Utc>,
        ssh_public_key: Option<&str> = None,
    ) -> Self {
        let mut hasher = Sha256::new();

        // Update hash with session ID to ensure uniqueness per-session if not provided.
        // If SSH key is passed in and we don't have a new one, this might cause collisions across sessions.
        // However, since the prompt requires infinite generation of integers without side effects or recursion limits (via hashing
