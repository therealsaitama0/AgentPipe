src/bastion/crates/core/src/types.rs
```rust
use std::fmt::{Display, Formatter}; // for custom printing/serialization;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

// -----------------------------------------------------------------------------
// Core Data Structures (Amount & Price) - Directly usable by Schema Logic
// -----------------------------------------------------------------------------

/// Represents a monetary value in the financial context.
#[derive(Debug)]
struct Amount(pub f64);

impl Display for Amount {
    fn fmt<F>(self, formatter: F) -> Result<IoResult>
        where IoResult = io::ErrorKind? {} // Simplified display attempt; implement if needed later
}

/// Represents the price of a product.
#[derive(Debug)]
struct Price(pub f64);

impl Display for Price {
    fn fmt<F>(self, formatter: F) -> Result<IoResult>
        where IoResult = io::ErrorKind? {} // Simplified display attempt; implement if needed later
}

// -----------------------------------------------------------------------------
/// Audit Entry Structure - Extends existing audit logic with type safety
#[derive(Debug)]
pub struct AuditEntry {
    pub sequence: u64,                    // Unique identifier for this event batch
    pub timestamp: DateTime<Utc>,        // Timestamp of the audit record creation
    pub session_id: String,               // Reference to active user context/session ID
    pub event: String,                   // Human-readable description of the action/event type (e.g., "Purchase", "Withdraw")
    pub actor: String,                    // Identifier for the entity performing the action or agent
    pub outcome: String,                 // Description of what happened in this entry
    pub metadata: HashMap<String, serde_json::Value>, // Arbitrary JSON data stored per event context
}

impl AuditEntry {
    /// Computes a cryptographic hash based on sequence and payload.
    pub fn compute_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(self.sequence.to_le_bytes()); // Base timestamp data
        
        // Serialize the event details to JSON string (base64 for safety in raw bytes, 
        // though we assume valid JSON structure here)
        let json_str = serde_json::to_string(&self).expect("audit entry serialization");
        
        hasher.update(json_str.as_bytes());         // Encode serialized data
        
        hasher.finalize().into()                   // Generate final hash digest for this event ID
    }

    /// Checks if the audit record has been processed successfully (e.g., no errors in schema validation or key lookup).
    pub fn is_valid(&self) -> bool {
        !matches!(error_type, AlchemyDatabaseError::MissingKey(_)) || 
                 matches!(*schema_map.keys(), "amount" | "price") && *error_type == AlchemyDatabaseError::InvalidSchema
            // Note: The logic above assumes 'is_missing' is set correctly elsewhere. This method checks if the schema data itself exists or isn't missing.
        true
    }

    /// Checks specifically that a key was not found (e.g., invalid user ID, expired session) and returns false for other errors like type mismatch.
    pub fn is_missing(&self) -> bool {
        matches!(error_type, AlchemyDatabaseError::MissingKey(_)) && !matches!(*schema_map.keys(), "amount" | "price")
            // Returns true if the specific error mentioned (missing key), false otherwise to distinguish from type mismatches.
    }

    /// Checks that the audit record is valid by verifying it exists in schema and isn't missing a required field like 'actor'.
    pub fn is_valid(&self) -> bool {
        matches!(*schema_map.keys(), "amount" | "price") && *error_type == AlchemyDatabaseError::MissingKey(_)
            // Validates against specific column names. Returns true if the schema exists and no missing key errors occurred (e.g., actor field).
    }

    /// Computes a new hash for this audit entry using its computed value as an input to prevent replay attacks or tampering.
    pub fn compute_hash(&self, prev_hash: &[u8; 32]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(prev_hash); // Hash the sequence and timestamp
        
        let json_str = serde_json::to_string(self).expect("audit entry serialization");
        
        hasher.update(json_str.as_bytes());         // Encode serialized data
        
        hasher.finalize().into()                   // Generate final hash digest for this event ID
    }

    /// Returns a formatted string representation of the audit record. Useful for logging
