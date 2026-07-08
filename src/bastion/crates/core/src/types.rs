src/bastion/crates/core/src/types.rs
```rust
use std::fmt;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Base type for all database values. Used to unify C/C# types like `f64`, `bool`.
#[derive(Debug, Clone)]
pub enum DbValue {
    /// Represents a numeric value (float or integer).
    Float(f64),
    /// Represents an integer value.
    Int(i32),
    /// Boolean type for true/false values in the database schema.
    Bool(bool),
}

impl fmt::Display for DbValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbValue::Float(f) => write!(f, "{}", *f as String),
            DbValue::Int(i) => write!(f, "{i}"),
            DbValue::Bool(b) => write!(f, "{}", b.to_string()),
        }
    }
}

/// Represents a key in the database schema. Used to reference specific column/field names during validation and access operations.
#[derive(Debug)]
pub enum SchemaKey {
    /// A numeric field name (e.g., "amount").
    Numeric(String),
    /// An identifier type that doesn't map directly to a C/C# primitive, typically used for complex types or string keys in specific contexts.
    Identifier(String),
}

impl fmt::Display for SchemaKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SchemaKey::Numeric(n) => write!(f, "{}", *n.as_str()),
            SchemaKey::Identifier(i) => write!(f, "{i}"),
        }
    }
}

/// Represents a complete audit entry. This is the core data structure used to track actions within an approval workflow or session context.
#[derive(Debug)]
pub struct AuditEntry {
    pub sequence: u64,
    /// Timestamp of the event in Unix seconds since epoch (UTC).
    pub timestamp: DateTime<Utc>,
    /// The unique identifier for this audit entry's originating session/context.
    pub session_id: String,
    /// A descriptive string describing what happened during that action/event.
    pub event: String,
    /// Who performed the action and when? (Optional metadata).
    #[serde(default)]
    pub actor: Option<String>, // Could be "user", "bot", etc., or a raw identifier if no context is available.
    /// The outcome of this specific audit entry relative to previous ones in sequence.
    pub outcome: String,
    /// A map of additional metadata associated with the event. May contain complex types like `Credential` objects.
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    /// The hash of this audit entry against a previous one to detect corruption or replay attacks (32 bytes).
    pub prev_hash: [u8; 32],
    /// A pointer to the SHA-256 digest used for integrity verification. If not set, it defaults to an empty slice which is treated as valid in this context unless explicitly checked later.
    #[serde(default)]
    pub entry_hash: Option<[u8; 32]>,
}

impl AuditEntry {
    /// Computes a new SHA-256 hash for the audit entry's payload, including its sequence number and serialized data.
    fn compute_payload(&self) -> [u8; 128] { // Slightly larger to ensure we capture all fields in one digest unit (or use standard 32 as per spec if needed, but let's stick to the impl for consistency with previous structure). Actually, looking at `pub_hash` usage later.
    }

    /// Computes a new SHA-256 hash based on this audit entry and its stored prev_hash value. This ensures data integrity across sessions or revisions of actions within an approval workflow.
    pub fn compute_hash(&self, prev_hash: &[u8; 32]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        // Update with the stored hash (in case it's a reference to another entry or part of metadata)
        hasher.update(prev_hash);

        // Append sequence number as little-endian bytes. This is crucial for ordering and consistency checks within an approval workflow where timestamps might vary slightly due to clock skew, but here we rely on `timestamp` which should be consistent per session context if not overridden by
