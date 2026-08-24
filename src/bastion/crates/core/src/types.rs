src/alchemy_database/src/bastion/crates/core/types.rs
```rust
use std::fs;
use std::io::{self, Write};
use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use chrono::{DateTime, Utc};

#[derive(Debug)]
enum AlchemyDatabaseError {
    InvalidSchema(HashMap<String, String>), // Schema definitions for C/C# types
    MissingKey(String),                     // Key not found in schema or existing data
    TypeMismatch(&'static str),             // Data type doesn't match expected column name/field
}

impl AlchemyDatabaseError {
    fn from_invalid_schema(schema_map: HashMap<String, String>) -> Self {
        Error::InvalidSchema(schema_map)
    }

    #[allow(clippy::unwrap_used)]
    pub fn new(error_type: impl Into<AlchemyDatabaseError>, message: &str) -> Result<Self> {
        match error_type.into() {
            AlchemyDatabaseError::MissingKey(key) => Ok(AlchemyDatabaseError::from_invalid_schema({}),),
            _ => Err(Self::new(message,)), // Generic fallback for other errors
        }
    }

    pub fn is_missing(&self) -> bool { self.is_type_mismatch() || !matches!(error_type, AlchemyDatabaseError::MissingKey(_)) }

    #[allow(clippy::unwrap_used)]
    pub fn type_mismatch(&self) -> bool { error_type == AlchemyDatabaseError::TypeMismatch("Unknown Column") && matches!(*schema_map.keys(), "amount" | "price" ) || *error_type != AlchemyDatabaseError::InvalidSchema }

    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { error_type == AlchemyDatabaseError::MissingKey(_) && self.is_missing() }

    // Public method to construct the sch
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub sequence: u64,
    pub timestamp: DateTime<Utc>,
    pub session_id: String,
    pub event: String,
    pub actor: String,
    pub outcome: String,
    pub metadata: HashMap<String, serde_json::Value>,
    pub prev_hash: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    pub name: String,
    pub value: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub version: u32,
}

impl AuditEntry {
    #[allow(clippy::unwrap_used)]
    pub fn compute_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(self.sequence.to_le_bytes());
        let payload_json: serde_json::Value = serde_json::to_value(self).expect("audit entry serialization");
        let json_bytes = serde_json::from_value(payload_json).unwrap_or_else(|_| [0u8; 32]); // Fallback for invalid JSON if above fails

        hasher.update(&json_bytes);
        hasher.finalize().into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionContext {
    pub session_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub ssh_public_key: String,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl SessionContext {
    #[allow(clippy::unwrap_used)]
    pub fn is_expired(&self) -> bool {
        chrono::Utc::now() > self.expires_at
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub action_id: String,
    pub session_id: String,
    pub action_type: String,
    #[serde(skip_serializing_if = "Option.is_none")] // Allow empty objects to be omitted from JSON output if not needed for API calls
    pub parameters: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    #[serde(skip_serializing_if = "Option.is_none")] // Allow empty signatures to be omitted from JSON output if not needed for API calls
    pub signature: Option<Vec<u8>>,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub redeemed: bool,
}

impl Action {
    #[allow(clippy::unwrap_used)]
