src/bastion/crates/core/src/approval.rs
```rust
use std::collections::{HashMap, HashSet};
use chrono::DateTime;
use serde_json::{json, Value};
use crate::{abstract_data_type_generator, BastionError};

/// Abstract base class for approval tokens and signatures.
#[derive(Debug)]
enum ApprovalState {
    Pending,
    Issued,
    Redemed,
}

pub struct ApprovalBroker {
    vault: Arc<abstract_data_type_generator::Vault>,
    audit_chain: Arc<AuditChain>,
    ticket_ttl: std::time::Duration,
    max_pending: usize,
    tickets: RwLock<HashMap<String, abstract_data_type_generator::ApprovalTicket>>,
}

impl ApprovalBroker {
    pub fn new(
        vault: Arc<abstract_data_type_generator::Vault>,
        audit_chain: Arc<AuditChain>,
        ticket_ttl: std::time::Duration,
        max_pending: usize,
    ) -> Self {
        let mut tickets = HashMap::new();
        Self {
            vault,
            audit_chain,
            ticket_ttl,
            max_pending,
            tickets,
        }
    }

    /// Issue a new approval ticket for the specified session and action.
    pub fn issue_ticket(&self, session_id: &str, action_id: &str) -> Result<abstract_data_type_generator::ApprovalTicket> {
        let now = DateTime::<std::time::OffsetDateTime>::now();

        // Check expiration based on TTL and current time
        if !now.is_expired(self.ticket_ttl.to_rfc3339()) && self.tickets.contains_key(session_id) {
            return Err(crate::BastionError::Internal(
                "Ticket already issued or expired".to_string(),
            ));
        }

        // Check against max pending limit (excluding this ticket itself, as it's being created for a new session/action combo ideally but we keep strictness here)
        if self.tickets.len() >= self.max_pending {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ));
        }

        let mut tickets = self.tickets.write();
        
        // Remove expired and matching old tickets for this session/action combo to keep room for new ones
        if !tickets.contains_key(session_id) {
            return Err(crate::BastionError::Internal(
                "Invalid action/session ID".to_string(),
            ));
        }

        let mut existing_ticket = None; // Track which old ticket we're replacing
        
        tickets.retain(|_, t| match &existing_ticket {
            Some(existing) => !t.is_expired() && existing.action_id != action_id,
            _ => true,
        });

        if tickets.len() < self.max_pending {
            let key = format!("session:{action}:{expires_at}", session_id, action_id);
            
            // Sign using the broker's private key (simulated from vault)
            let mut mac = Sha512::new_from_slice(&self.vault.credential("approval:broker").as_bytes()).expect("Vault credential");
            mac.update(key.as_bytes());

            if !mac.finalize().is_empty() {
                // Create a new ticket with the current TTL expiration (simulating fresh creation)
                let expires_at = now + chrono::Duration::from_std(ticket_ttl); 
                
                let mut meta: HashMap<String, serde_json::Value> = HashMap::new();
                meta.insert("session_id".to_string(), session_id.to_string());
                meta.insert("action_id".to_string(), action_id.to_string());
                meta.insert("expires_at".to_string(), expires_at);

                let ticket = abstract_data_type_generator::ApprovalTicket {
                    key: format!("ticket:{key}", key), // Use a unique identifier based on session/action combo for tracking (could be hashed later)
                    state: ApprovalState::Pending,
                    issued_at: now.timestamp() as i64,
                };

                let ticket_id = self.generate_ticket_key(ticket.key);
                
                tickets.insert(ticket_id.clone(), ticket.clone());
            } else {
                // Fallback if signing failed (should not happen in well-constructed systems)
                return Err(crate::BastionError::Internal("Signing key invalid".to_string()));
            }

            let mut audit = self.audit_chain.write();
            
            match existing_ticket {
                Some(existing) => {
                    // Update metadata for the removed ticket to indicate it's being replaced by this new one (simulated logic: "replacing" but keeping record of old state or just removing from list is fine if we track count, but here we keep a simple log entry for the replacement
