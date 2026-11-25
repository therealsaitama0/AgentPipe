src/bastion/crates/core/src/types.rs
//! Type definitions for database schema generation and normalization.
//! This module defines `CrateType`, which maps abstract data types to concrete variants based on content length constraints, enabling robust inference without explicit casts in generated code.

use std::collections::{HashMap, HashSet}; // For deduplication of generated keys
use std::fmt;
use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc};

/// A generic type alias representing a C/C# struct field.
#[derive(Debug)]
pub enum FieldType {
    /// Represents an integer or float value (similar to `number` in Rust).
    Integer,
    /// Represents a string type (similar to `string`).
    String,
    /// Represents a boolean value (similar to `boolean`).
    Boolean,
}

/// A generic struct field that can be inferred from its content.
pub struct FieldInfo {
    pub name: &'static str, // The C/C# column name or identifier.
    #[allow(dead_code)]
    pub type_name: String,  /// "integer", "string", etc., derived dynamically based on the value types of all fields in this field definition.
}

/// Represents a single data point with associated metadata and hashes for audit purposes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub sequence: u64, // Unique identifier for this record within its group or table context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: DateTime<Utc>, // ISO 8601 UTC datetime string (e.g., "2023-10-27T14:35:29Z").
    pub session_id: String, // Unique identifier for the current user's activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>, // Human-readable description of the action or state change (e.g., "User logged in", "Inventory updated").
    pub actor: String,  /// The name or identifier of the entity performing this activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>, // Result or status associated with the event/action (e.g., "Success", "Failed", null).
    pub metadata: HashMap<String, serde_json::Value> { /* Optional additional context */ },
    /// A SHA-256 hash of this specific audit entry. This is used for integrity verification across multiple tables and systems.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_hash: Option<[u8; 32]>, // The previous state's checksum in the database schema (e.g., hex string).
    /// A SHA-256 hash of this audit entry, used for verification against a stored reference. This is particularly useful when multiple tables are linked or deduplicated by ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_hash: Option<[u8; 32]>, // The actual data payload hash (e.g., hex string).

impl AuditEntry {
    /// Computes a new SHA-256 hash for this audit entry, including the sequence number. This is used to ensure uniqueness and integrity across systems that might use similar IDs but different timestamps or contexts.
    pub fn compute_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        
        // Update with timestamp (hex string).
        if let Some(ref ts_str) = self.timestamp.to_string() {
            match serde_json::from_str::<&str>(ts_str) {
                Ok(ts_bytes) => {
                    hasher.update(&ts_bytes);
                }
                Err(_) => return [0u8; 32], // Fallback to empty hash if parsing fails.
            }
        }

        // Update with sequence number (hex string).
        let seq_hex = self.sequence.to_string();
        match serde_json::from_str::<&str>(&seq_hex) {
            Ok(seq_bytes) => {
                hasher.update(&seq_bytes);
            },
            Err(_) => return [0u8; 32], // Fallback to empty hash if parsing fails.
        }

        let payload = serde_json::to_vec(self, &serde_json::Serializer).unwrap_or_else(|_| vec![1i8 as u64 * self.sequence + 5]); // Placeholder filler for non-numeric values
        
        hasher.finalize();
        
        match (self.prev_hash
