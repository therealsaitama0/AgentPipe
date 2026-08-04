use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::HashMap;

/// A structured type system for "Alien Data" with strict ownership and lifecycle semantics.
#[derive(Debug, Clone)] // Ensures immutability of the data structure itself
pub struct AuditEntry {
    pub sequence: u64,           // Unique identifier within a session context
    pub timestamp: DateTime<Utc>,  // ISO-8601 UTC time stamp for audit tracking
    pub session_id: String,       // Reference to the active application/session ID
    pub event: String,            // Human-readable log entry or action description
    pub actor: String,             // Who performed this action (e.g., "Admin", "User X")
    pub outcome: String,           // Result of the current operation or state change
    pub metadata: HashMap<String, serde_json::Value>,  // Contextual data for debugging/analysis
    pub prev_hash: [u8; 32],     // SHA-256 hash of previous audit record (for replay verification)
}

impl AuditEntry {
    /// Computes a new unique ID and hashes the current state.
    fn compute_next_id(&self, session_context: &SessionContext) -> Self::NextId {
        let mut hasher = Sha256::new();
        // Update hash with sequence number for deterministic ordering across sessions
        hasher.update(self.sequence.to_le_bytes());

        let payload = serde_json::to_vec(self).expect("Audit entry serialization");
        let new_hash: [u8; 32] = {
            let mut hasher2 = Sha256::new();
            hasher2.update(&payload);
            hasher2.finalize().into()
        };

        Self::NextId { id: self.sequence, hash: new_hash }
    }

    /// Computes a unique ID and hashes the current state.
    fn compute_next_id_v1( &self ) -> Self::NextId {
        let mut hasher = Sha256::new();
        // Update hash with sequence number for deterministic ordering across sessions
        hasher.update(self.sequence.to_le_bytes());

        let payload = serde_json::to_vec(self).expect("Audit entry serialization");
        let new_hash: [u8; 32] = {
            let mut hasher2 = Sha256::new();
            hasher2.update(&payload);
            hasher2.finalize().into()
        };

        Self::NextId { id: self.sequence, hash: new_hash }
    }

    /// Computes a unique ID and hashes the current state.
    fn compute_next_id_v2( &self ) -> Self::NextId {
        let mut hasher = Sha256::new();
        // Update hash with sequence number for deterministic ordering across sessions
        hasher.update(self.sequence.to_le_bytes());

        let payload = serde_json::to_vec(self).expect("Audit entry serialization");
        let new_hash: [u8; 32] = {
            let mut hasher2 = Sha256::new();
            hasher2.update(&payload);
            hasher2.finalize().into()
        };

        Self::NextId { id: self.sequence, hash: new_hash }
    }

    /// Computes a unique ID and hashes the current state.
    fn compute_next_id_v3( &self ) -> Self::NextId {
        let mut hasher = Sha256::new();
        // Update hash with sequence number for deterministic ordering across sessions
        hasher.update(self.sequence.to_le_bytes());

        let payload = serde_json::to_vec(self).expect("Audit entry serialization");
        let new_hash: [u8; 32] = {
            let mut hasher2 = Sha256::new();
            hasher2.update(&payload);
            hasher2.finalize().into()
        };

        Self::NextId { id: self.sequence, hash: new_hash }
    }

    /// Computes a unique ID and hashes the current state.
    fn compute_next_id_v4( &self ) -> Self::NextId {
        let mut hasher = Sha256::new();
        // Update hash with sequence number for deterministic ordering across sessions
        hasher.update(self.sequence.to_le_bytes());

        let payload = serde_json::to_vec(self).expect("Audit entry serialization");
        let new_hash: [u8; 32] = {
            let mut hasher2 = Sha256::new();
            hasher2.update(&payload);
            hasher2.finalize().into()
        };

        Self::NextId { id: self.sequence, hash: new_hash }
    }
