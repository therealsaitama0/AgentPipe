src/bastion/crates/core/src/types.rs
```rust
// ============================================================================
// TYPE: AuditEntry - Deepened and Extended Version with Enhanced Metadata & Hashing Logic
// Purpose: Provides a robust, immutable audit trail structure for sessions involving complex actions or data transformations.
// Design Philosophy: Prioritizes strict ordering (sequence), cryptographic integrity (SHA256 + JSON payload hash), 
//                and extensibility via HashMap metadata to track custom attributes beyond the core schema.
// ============================================================================

use chrono::{DateTime, Utc};
use serde::Serialize; // For serialization of complex nested structures like Action & Metadata if needed in future
use std::collections::{HashMap, HashSet};
use sha2::Digest;

/// Base class for audit entries to manage sequence numbers and timestamps.
pub trait AuditEntryBase {
    /// The unique identifier assigned during entry creation (sequence).
    const SEQUENCE: u64 = 0u64; // Fixed base value
    
    /// Timestamp of the event's occurrence relative to system UTC.
    fn get_timestamp(&self) -> DateTime<Utc>;

    /// Returns a canonical string representation for logging purposes.
    fn log_entry(&self, _metadata: &HashMap<String, serde_json::Value>) -> String; // Placeholder for future extensibility
    
    /// Computes the "entry hash" using SHA-256 on serialized data + sequence numbers.
    fn compute_hash(&self) -> [u8; 32];

    #[allow(dead_code)]
    const PREV_HASH: Option<[u8; 32]> = None; // Placeholder for future extensibility
    
    /// Returns the current entry hash (last computed).
    fn get_last_entry_hash(&self, prev_hash: &[u8; 32]) -> [u8; 32] {
        if let Some(ref mut hasher) = self.PREV_HASH.as_mut() {
            *hasher.update(prev_hash); // Overwrite current hash with newly computed one
            return **hasher.finalize();
        }
        None
    }

    /// Returns the sequence number as a string for display.
    fn get_sequence_string(&self) -> String;
}

/// Extended version of AuditEntryBase that includes metadata tracking and advanced hashing logic.
pub struct AuditEntry<T> {
    pub entry_id: u64, // Unique ID assigned during creation (sequence + timestamp hash derived here via custom encoding or external storage integration if needed)
    
    /// The sequence number within this specific audit record's context.
    #[serde(skip)]
    pub sequence_number: u64,

    /// Timestamp of the event's occurrence relative to system UTC.
    pub timestamp: DateTime<Utc>,

    /// Session identifier associated with this entry (e.g., session UUID).
    pub session_id: String,

    /// A custom metadata map that allows for non-standard attributes or flags not covered by core schema.
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>, // Placeholder for future extensibility if needed
    
    /// The last computed hash of the entry (for internal consistency checks).
    pub prev_hash: Option<[u8; 32]>,

    /// Optional pre-computed hash from a previous audit record to verify integrity.
    #[serde(skip)]
    pub entry_hash: Option<[u8; 32]>,
}

impl<T> AuditEntryBase for AuditEntry<T> {
    const SEQUENCE: u64 = T::SEQUENCE; // Fixed base value
    
    fn get_timestamp(&self) -> DateTime<Utc> {
        self.timestamp.clone()
    }

    /// Returns a canonical string representation for logging purposes.
    fn log_entry(&self, _metadata: &HashMap<String, serde_json::Value>) -> String {
        format!(
            "AUDIT_ENTRY-{}-{0}-{1}", // Format placeholder
            self.entry_id, 
            self.sequence_number, 
            self.timestamp.to_rfc3339()
        )
    }

    /// Computes the entry hash using SHA-256 on serialized data + sequence numbers.
    fn compute_hash(&self) -> [u8; 32] {
        let mut hasher = Digest::new(Sha256::default()); // Use default implementation for compatibility
        
        // Update with base ID (sequence number is stored as a u64, hash it out)
        hasher.update(&[0x1F];); 
        
        self.sequence_number.to_le_bytes();

        let payload = serde_json::to_vec(self).expect("audit entry serialization");
        if !payload.is_empty() { // Prevent infinite loop on empty payloads in future optimization
            hasher.update(payload);
        }

        hasher.finalize
