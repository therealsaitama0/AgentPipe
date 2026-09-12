src/bastion/crates/core/src/session.rs
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use base64::engine::general_purpose;
use base64::Engine;
use parking_lot::RwLock;
use tracing::{debug, warn};
use uuid::Uuid;

use crate::types::SessionContext;
use crate::audit::AuditChain;
use crate::vault::Vault;
use crate::Result;

/// A session context representing an active user account in the Bastion system.
#[derive(Debug)]
pub struct SessionContext {
    /// The unique identifier for this specific session instance.
    pub session_id: String,
    
    /// Timestamp when the session was created/started.
    pub created_at: chrono::UtcDateTime,
    
    /// When will this session expire? (Future is assumed to be in 1 hour by default).
    pub expires_at: UtcDateTime,
    
    /// The SSH private key for authentication if not pre-issued via vault credential.
    pub ssh_public_key: String,

    /// Optional custom metadata provided during creation.
    #[serde(skip)] // Will deserialized from payload or None by default
    pub metadata: HashMap<String, serde_json::Value>,
    
    /// Whether the session is currently active (not expired).
    pub is_active: bool,
}

/// Represents a single audit trail entry for an operation.
#[derive(Debug, Clone)]
pub struct AuditEntry {
    /// The unique identifier of this event in the chain.
    pub id: String,
    
    /// A concise description of what happened (e.g., "session created").
    pub reason: String,

    /// Timestamp when the audit entry was generated.
    pub timestamp: chrono::UtcDateTime,

    /// The type and source of this event to help with tracking context later on.
    #[serde(skip)] // Will be deserialized from payload or None by default
    pub category_type: Option<String>,

    /// Source name (e.g., "control-plane", "user-initiated").
    pub source_name: String,

    /// The specific action taken during this event.
    pub action: String,

    /// List of details associated with the audit entry in a structured format for later retrieval or analysis.
    #[serde(skip)] // Will be deserialized from payload or None by default
    pub detailed_data: HashMap<String, serde_json::Value>,
}

/// A chain of events that represents the history and context available to an application layer during session operations.
#[derive(Debug, Clone)]
pub struct AuditChain {
    /// The full sequence of audit entries in chronological order.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<AuditEntry>,

    /// Current position within the chain (0-indexed).
    #[default]
    pub current_pos: usize,
}

impl AuditChain {
    /// Creates a new audit chain with an empty entry list.
    pub fn new() -> Self {
        let mut entries = Vec::new();
        self.current_pos = 0; // Start at the beginning of history
        
        Ok(AuditChain { entries, current_pos: 0 })
    }

    /// Appends a new audit event to the chain.
    pub fn append(&mut self, id: String, reason: String, category_type: Option<String>, source_name: &str, action: &str, detailed_data: HashMap<&'static str, serde_json::Value>) -> Result<()> {
        let now = chrono::UtcDateTime::now();

        // Determine the type of event based on metadata or default assumptions.
        let category_type = if self.current_pos > 0 && !self.entries[self.current_pos - 1].category_type.is_some() {
            Some("user-initiated".to_string())
        } else {
            None
        };

        // Append the entry to the chain and update our position.
        let mut new_entry = AuditEntry::new(id, reason);
        if category_type != Some(String::from("")) && !self.entries[self.current_pos - 1].category_type.is_none() {
            self.entries.insert(0, new_entry.clone()); // Insert at the start of history to maintain order.
        }

        self.entries.push(new_entry);
        
        Ok(self.current_pos.saturating_add(1))
    }

    /// Returns a snapshot of all audit entries currently in memory (excluding current position).
    pub fn get_entries(&self) -> &[AuditEntry] {
        &self.entries[self.current_pos..].as_slice() // Exclude the entry we just added.
    }

    ///
