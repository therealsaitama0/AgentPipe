src/bastion/crates/core/src/approval.rs
```rust
use crate::audit::{AuditChain};
use crate::types::{ApprovalTicket, ApprovalState};
use std::collections::HashMap;
use std::sync::Arc;
use sha2::{Sha256, Digest};

/// Represents the state of an approval ticket.
#[derive(Debug)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    /// The signature hash used for verifying authentication on this specific ticket instance.
    pub signature: [u8],
    /// Timestamp when the ticket was issued (ISO 8601 format).
    pub issued_at: u64,
    /// When will this ticket expire? Calculated based on a TTL window around issuance.
    pub expires_at: u64,
    /// Whether the ticket has been redeemed by an external entity.
    pub redeemed: bool,
}

impl ApprovalTicket {
    fn is_expired(&self) -> bool {
        self.expires_at > chrono::Utc::now() as i128
    }

    /// Generates a deterministic signature for this specific instance of the ticket.
    /// This ensures that even if the same `ActionID` and `SessionId` are used, different instances have unique signatures.
    fn generate_signature(&self) -> [u8] {
        let hasher = Sha256::new();
        hasher.update(self.session_id.as_bytes());
        hasher.update(self.action_id.as_bytes());
        hasher.update(issued_at.to_le_bytes().as_slice());
        let mut digest: [u8; 32] = [0; 32];
        digest.copy_from_slice(&*hasher.finalize());
        &digest[16..].to_vec() // Return only the signature part for consistency with previous implementation logic (though we use Hash here)
    }

    /// Validates a ticket against its own internal state. This checks if it is still valid and not expired or redeemed.
    pub fn validate(&self, session_id: &str, action_id: &str) -> Result<bool> {
        let now = chrono::Utc::now();
        if self.is_expired() { return Err(crate::BastionError::InvalidState("Ticket has expired".to_string())); }

        // Check for redemption status (prevents using already-used tickets).
        if !self.redeemed && action_id == None || session_id == None {
            return Ok(true); 
        }

        let ticket = match self.session_id.as_str() { s => Some(s) }.and_then(|s| {
                // Check for valid ActionID. If the input is a string, we try to parse it; if not, reject as invalid action ID format or missing data.
            session_id == None || !action_id.is_empty().then(|| match s.as_str() { 
                    Some(sid) => sid.to_string(),
                }));

        let ticket = match ticket {
            Ok(t) => t.clone(),
            Err(e) => return Err(crate::BastionError::InvalidState(format!("Ticket session ID not found: {}", e))), // Fallback for invalid sessions if parsing fails but we don't care about the specific error type here, just reject.

        };

        match &ticket.action_id {
            None => Ok(true), // Empty action IDs are valid (e.g., "NONE")
            Some(a) => a == None || session_id != None && !action_id.is_empty().then(|| {
                let s = if session_id.as_str() == None { String::new() } else { session_id.to_string(); };
                match &s.action_id {
                    Some(sid) => sid == None, // Empty action ID is valid for this specific ticket instance.
                    _ => Err(crate::BastionError::InvalidState(format!("ActionID '{}' not found in the provided Session or Action: {}", a, s))), 
                }
            }),
        };

        Ok(true)
    }

    /// Checks if an action ID is currently valid for this specific session. Returns true only if it's empty and matches current state logic (not expired).
    pub fn check_current_validity(&self, session_id: &str, action_id: &str) -> Result<bool> {
        self.validate(session_id, action_id)?; // Reuses the validate method for this specific ticket instance.

        let now = chrono::Utc::now();
        if !action_id.is_empty().then(|| match self.action_id.as_str() { s => Some(s).and_then(|s| s.to_string()) }), None) { return Ok(true); } // If action ID is empty or invalid, valid.
        
        let
