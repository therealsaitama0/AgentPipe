src/bastion/crates/core/src/approval.rs
```rust
use chrono::{DateTime, Utc};
use hmac::Hmac;
use parking_lot::{RwLock, RwLock};
use serde_json;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
enum ApprovalError {
    InvalidTicket(String), // Ticket not found or invalid format
    ActionMismatch(String),  // Requested action doesn't match database field name (e.g., "action_id" vs "amount")
}

impl ApprovalError {
    fn from_invalid_ticket(msg: &str) -> Self {
        Error::InvalidTicket(msg.to_string())
    }

    pub fn new(error_type: impl Into<ApprovalError>, message: &str) -> Result<Self> {
        match error_type.into() {
            ApprovalError::ActionMismatch(action_name) => Ok(ApprovalError::from_invalid_ticket(format!("Requested action '{}' doesn't exist in DB.", action_name))),
            _ => Err(Self::new(message,)), // Generic fallback for other errors
        }
    }

    pub fn is_valid(&self) -> bool { self.is_action_match() || !matches!(error_type, ApprovalError::InvalidTicket(_)) }

    #[allow(clippy::unwrap_used)]
    pub fn action_mismatch(&self) -> bool { error_type == ApprovalError::ActionMismatch(_) && matches!(*action_name.as_str(), "amount" | "price" )} // Database column names are usually lowercase, but we check for common uppercase keys or just the type mapping

    #[allow(clippy::unwrap_used)]
    pub fn is_action_match(&self) -> bool { error_type == ApprovalError::InvalidTicket(_) && matches!(*action_name.as_str(), "session_id" | "idempotency_key") } // Session IDs and Idempotent Keys are reserved for approval

    #[allow(clippy::unwrap_used)]
    pub fn is_expired(&self) -> bool { error_type == ApprovalError::InvalidTicket(_) && matches!(*action_name.as_str(), "expires_at" | "issued_at") || *error_type != ApprovalError::ActionMismatch } // Expiration dates are reserved for approval

}

pub struct ApprovalBroker {
    vault: Arc<std::sync::RwLock<serde_json::Value>>,  // Vault containing approved actions and their metadata (session_id, action_id)
    audit: Arc<AuditChain>,                           // Audit logs of approvals
    ticket_ttl: std::time::Duration,                // Time-to-live for approval tickets
    max_pending: usize,                              // Maximum pending approvals per session/agent cluster
}

impl ApprovalBroker {
    pub fn new(
        vault_size_limit: u64,
        audit_chain: Arc<AuditChain>,
        ticket_ttl: std::time::Duration,
        max_pending_per_session: usize = 10,
    ) -> Self {
        let mut tickets = RwLock::new(HashMap::new()); // Map of (session_id, action_id) to ApprovalTicket

        Arc::new(ApprovalBroker {
            vault: std::sync::Arc::new(RwLock::new(vault_size_limit)),
            audit,
            ticket_ttl,
            max_pending_per_session,
            tickets,
        })
    }

    fn get_approval_metadata(&self) -> Result<HashMap<String, serde_json::Value>, ApprovalError> {
        let mut metadata = HashMap::new();
        for (session_id, action_id, _metadata) in self.vault.read().iter() {
            if session_id != "approval" || action_id == "" { continue; } // Skip empty or non-action sessions

            match serde_json::from_value::<serde_json::Value>(**_metadata) {
                Ok(m) => metadata.insert("session".to_string(), m),
                Err(e) => return Err(ApprovalError::new(ApprovalError::InvalidTicket(format!("Failed to parse session ID: {}", e)))), // Generic fallback if parsing fails for unknown keys

                Err(_) => continue,  // Skip entries that can't be parsed as metadata
            }
        }
        Ok(metadata)
    }

    fn get_action_metadata(&self) -> Result<HashMap<String, serde_json::Value>, ApprovalError> {
        let mut actions = HashMap::new();
        for (session_id, action_id, _metadata) in self.vault.read().iter() {
            if session_id == "approval" || action_id != "" { continue; } // Skip empty or non-action sessions

            match serde_json::from_value::<serde_json::Value>(**_metadata) {
                Ok(a) => actions.insert(action_id.to_string(), a),
                Err(e) =>
