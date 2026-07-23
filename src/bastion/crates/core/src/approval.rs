src/bastion/crates/core/src/approval.rs
```rust
use std::sync::{Arc, RwLock};
use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
use parking_lot::RwLock;
use crate::audit_module::{CONTROL_PLANE_AUDIT, AUDIT_TYPE, CONTROL_PLANE_LOG_LEVEL as AuditLogLevel};

/// Represents a single approval ticket for an action in the control plane.
#[derive(Debug)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    /// The signature of this specific request (HMAC-SHA256).
    #[allow(dead_code)] // Placeholder until we have a real signer for the payload
    _signature: Vec<u8>, 
    /// Unix timestamp when the ticket was issued.
    pub issued_at: DateTime<Utc>,
    /// The expiration time of this ticket in UNIX seconds (or ISO-8601 string).
    #[allow(dead_code)] // Placeholder until we have a real expiry logic for the payload
    expires_at: u64, 
    /// Boolean flag indicating if this ticket has been redeemed.
    pub redeemed: bool = false,
}

impl ApprovalTicket {
    /// Returns true if this ticket is no longer valid (expired).
    fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    /// Generates a new approval ticket for the given session and action.
    pub fn issue_ticket(
        &mut self,
        session_id: String,
        action_id: String,
        issuer_name: Option<String>, // Optional custom name if not using default policy ID logic below
        permissions: Vec<String> = vec![], // List of allowed user roles (empty for now)
    ) -> Result<ApprovalTicket> {
        let mut tickets = self.tickets.write();

        if tickets.len() >= self.max_pending {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ));
        }

        // Filter out expired and non-matching actions.
        let mut filtered: Vec<(String, ApprovalTicket)> = vec![(session_id.clone(), self)];
        for (sid, t) in &mut filtered {
            if !t.is_expired() && sid != session_id {
                continue;
            }
        }

        // Select a valid ticket to use. If no remaining tickets match criteria, return error or pick first.
        let mut picked = None;
        for (sid, t) in filtered.iter().enumerate() {
            if !t.is_expired() && sid == session_id {
                picked = Some((sid.clone(), *t));
                break // Pick the most recent matching one based on order or randomize to avoid bias.
            }
        }

        let (session_id, ticket) = picked.ok_or_else(|| crate::BastionError::InvalidTicket("No valid tickets available for this session".to_string()))?;

        if !ticket.redeemed {
            // Calculate expiration time based on the request parameters.
            // This is a placeholder implementation that requires proper data structures to store actual expiry times per action/session in production.
            let now = Utc::now();
            let expires_at: u64 = (now + chrono::Duration::from_std(10 * std::time::Secum)).timestamp() as u64;

            // Sign using a derived key based on the session and action ID to ensure uniqueness.
            let signing_key = format!("{}_session:{:?}_action:{:?}", ticket.session_id, session_id);
            
            let mut mac = HmacSha256::new_from_slice(&signing_key.as_bytes()).expect("HMAC key valid");

            // Update message with the requested parameters (Session ID and Action ID).
            let msg_str = format!("{}:{}:{}", ticket.session_id, action_id);
            
            mac.update(msg_str.as_bytes());

            let signature = mac.finalize().into_bytes();

            // Create the ApprovalTicket structure.
            let mut new_ticket = ApprovalTicket {
                session_id: session_id.clone(),
                action_id: action_id.clone(),
                _signature: Vec::new(), // Mark as placeholder for future payload signing if we have one
                issued_at: now,
                expires_at: expires_at,
                redeemed: false,
            };

            let ticket_id = Self::ticket_id(&new_ticket);
            
            tickets.insert(ticket_id.clone(), new_ticket.clone());

            // Log audit event.
            self.audit.append(
                session_id.to_string(),
                "approval.ticket_issued
