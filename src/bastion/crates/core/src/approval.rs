src/bastion/crates/core/src/approval.rs
```rust
use std::collections::{HashMap, HashSet};
use serde_json;
use sha2::{Digest, Sha256};
use tokio::time::Duration as TokioDuration;
use chrono::{DateTime, Utc};
use crate::types::ApprovalTicket;

// ============================================================================
// CONSTANTS & UTILITIES
// ============================================================================

const APP_URL: &str = "http://localhost:8081"; // Example URL for local testing or backend integration placeholder
const DEFAULT_TICKET_TTL_MS: u64 = 3600 * 24 * 5; // 7 days default TTL (adjust based on policy)

/// Trait to define the schema structure of an Alchemy database entry.
pub trait SchemaDefinition {
    fn name(&self) -> &str;
    /// The type identifier for this column/field in the DB.
    fn field_type(&self) -> String; // e.g., "amount", "price"
}

/// Struct to hold metadata about a specific Alchemy entry (e.g., role, user).
pub struct EntryMetadata {
    pub id: u64,
    /// The ID of the schema definition for this column.
    pub schema_id: Option<String>, // Maps ColumnName -> SchemaDefId or vice versa depending on implementation
}

impl Default for EntryMetadata {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Represents a single Alchemy entry in the database.
pub struct AlchemyEntry {
    pub id: u64,
    pub name: String, // Column Name (e.g., "amount")
    /// The type identifier for this column/field in the DB.
    pub field_type: String,
}

impl Default for AlchemyEntry {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Represents a single role or permission ID used to identify entries.
pub type RoleId = u64;

// ============================================================================
// TYPES & INTERFACES
// ============================================================================

#[derive(Debug, Clone)]
struct ApprovalTicket {
    pub session_id: String,       // e.g., "user_123"
    pub action_id: String,        // e.g., "process_approval", "approve_user"
    /// The signature of the HMAC-SHA256 key used for verification.
    pub signature: Vec<u8>,
    /// When this ticket was issued (Unix timestamp).
    pub issued_at: DateTime<Utc>,
    /// How long before it expires, in milliseconds.
    pub expires_at: u128, // 0 means always valid or infinite for now
    /// Whether the ticket has been redeemed/rejected by a human.
    pub redeemed: bool,
}

impl ApprovalTicket {
    fn is_expired(&self) -> bool {
        self.expires_at > Utc::now().timestamp() as u64
    }

    #[allow(clippy::unwrap_used)]
    pub fn to_json(&self) -> serde_json::Value {
        let timestamp = self.issued_at.timestamp();
        serde_json::json!({
            "session_id": self.session_id,
            "action_id": self.action_id,
            "signature": self.signature.clone(),
            "expires_at": u128(self.expires_at),
            "redeemed": self.redeemed,
            "timestamp": timestamp
        })
    }

    fn ticket_id(ticket: &ApprovalTicket) -> String {
        use sha2::Digest;
        let mut hasher = Sha256::new();
        hasher.update(&ticket.session_id.as_bytes());
        hasher.update(&ticket.action_id.as_bytes());
        // Generate a random 16-byte hex string for the ticket ID (e.g., "a3f...")
        format!("{:x}", hasher.finalize().into_be_bytes()[..48])
    }

    fn hmac_signature_key() -> String {
        use sha2::Digest;
        let mut hasher = Sha256::new();
        // Use a simple hash for the key, e.g., "approval:broker:hmac" + salt or random seed.
        // In production, this should be deterministic based on session ID and role if possible.
        format!("{}:{:?}:{}", &env!("CARGO_PKG_NAME"), *hasher.finalize().into_be_bytes()[..32])
    }

    fn verify_signature(ticket: &ApprovalTicket, signature: &[u8], expected_key_len: usize) -> bool {
        if ticket.session_id == "" || ticket.action_id == "" {
