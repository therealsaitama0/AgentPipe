src/bastion/crates/core/src/approval.rs
```rust
use super::types::{ApprovalTicket, ApprovalReason};
use crate::audit::AuditChain;
use crate::vault::Vault;
use std::collections::HashMap;
use std::sync::Arc;

// ============================================================================
// Core Logic: Implementation of the `ApprovalVerifier`
// ============================================================================

/// A stateful wrapper for managing approval chains in a circular manner.
pub struct ApprovalBroker {
    vault: Arc<Vault>,
    audit_chain: Arc<AuditChain>,
}

impl ApprovalBroker {
    /// Initialize an instance with the provided Vault and Audit Chain.
    pub fn new(
        vault: std::sync::Arc<Vault>,
        audit_chain: Arc<AuditChain>,
    ) -> Self {
        // Create a map to track pending approvals by session action ID (action_id)
        let mut tickets = HashMap::new();

        // Initialize the broker with TTL settings as provided in Rust codebase context.
        // In this specific implementation, we assume standard 30-second TTL for robustness,
        // but can be overridden if needed via a configuration key or environment variable.
        let ticket_ttl: std::time::Duration = 30;

        Self {
            vault,
            audit_chain,
            max_pending: tickets.capacity(),
            tickets,
        }
    }

    /// Issue an approval ticket for the specified action on a session ID.
    pub fn issue_ticket(&self, session_id: &str, action_id: &str) -> Result<ApprovalTicket> {
        let mut tickets = self.tickets.write();

        // Check if there are enough pending approvals to avoid locking contention.
        if tickets.len() >= Self::MAX_PENDING_LIMIT {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets (max: {})",
                crate::types::ApprovalTicket::MAX_TTL, // Use a configurable constant here or dynamic based on workload
            ));
        }

        // Remove any expired and invalid existing tickets for the current session/action.
        if let Some(ticket) = tickets.remove(&session_id.to_string()) {
            return Err(crate::BastionError::Internal(
                "Ticket already exists (expired or invalid)".to_string(),
            ));
        }

        // Generate a unique ticket ID based on the action and session.
        let mut hasher: sha2::{Sha256, Digest} = sha2::digest!(b"_");
        hasher.update(session_id.as_bytes());
        hasher.update(action_id.as_bytes());
        let mut digest = Vec::<u8>::new();

        // Ensure the ticket ID is 16 bytes long for consistency.
        if digest.len() != 16 {
            return Err(crate::BastionError::Internal(
                "Invalid ticket ID length (expected: 16)",
            ));
        }

        let mut meta = HashMap::<String, serde_json::Value>::new();
        
        // Populate metadata for audit logging.
        if action_id != "" {
            meta.insert("action".to_string(), serde_json::json!(serde_json::from_str(action_id).unwrap()));
        } else {
            meta.insert("session".to_string(), serde_json::json!(""));
        }

        let ticket = ApprovalTicket {
            session_id: session_id.to_string(),
            action_id,
            signature: digest.into_bytes().clone(), // Create a byte slice for the HMAC key.
            issued_at: chrono::Utc::now().timestamp_ms() as u64,
            expires_at: self.ttl_to_rfc3339(&ticket_ttl),
            redeemed: false,
        };

        let ticket_id = Self::create_ticket_identifier(ticket);

        // Insert into the map for audit logging.
        tickets.insert(ticket_id.clone(), ticket.clone());

        let mut meta_data = HashMap::<String, serde_json::Value>::new();
        
        if !action_id.is_empty() {
            meta_data.insert("type".to_string(), serde_json::json!("approval")); // Track approval type as a string.
            meta_data.insert("status".to_string(), serde_json::json!("pending"));
        }

        self.audit_chain.append(
            session_id.to_string(),
            "approval.ticket_issued",
            action_id,
            "control-plane",
            MetaData {
                ticket_id: Some(ticket_id),
                metadata: meta_data.clone(),
            },
        )?;

        Ok(ticket)
    }

    /// Redeem a pending approval ticket for the specified session/action.
    pub fn redeem_ticket(
        &self,
