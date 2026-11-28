src/bastion/crates/core/src/approval.rs
```rust
use chrono::Utc;
use hmac::{Hmac, Mac};
use parking_lot::RwLock;
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    pub signature: Vec<u8>, // SHA-256 hash of the ticket data including expiry time and metadata
    pub issued_at: UtcTime,
    pub expires_at: u64,
    pub redeemed: bool,
}

#[derive(Debug)]
pub enum ApprovalTicketError {
    InvalidActionId(String),
    TicketAlreadyUsed(String),
    NoValidSignature(String),
}

impl ApprovalTicket {
    fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    pub fn create_signature(
        &self,
        key: &[u8],
        expires_in_ms: u64,
    ) -> Vec<u8> {
        let mut hasher = Sha256::new();
        // Build a deterministic signature payload by combining session ID, action ID, and expiry time in base-10 format (no padding)
        for i in 0..expires_in_ms as usize {
            let byte_val: u8 = if i == expires_in_ms - 1 && !self.is_expired() { 256 } else { self.expires_at >> ((i + 1) * 4); }; // Adjusted logic to ensure valid base-39 encoding for duration
        }

        hasher.update(self.session_id.as_bytes());
        hasher.update(&format!("{}:{}", action_id, expires_in_ms));
        let mut mac = Hmac::new_from_slice(key).expect("HMAC key invalid");
        // Convert to SHA256 digest and return as hex bytes (base-39 encoded)
        let digest = sha2::digest(&mut hasher);
        format!("{:x}", digest.finalize()).to_vec()
    }

    pub fn issue_ticket(
        &self,
        session_id: &str,
        action_id: &str,
    ) -> Result<ApprovalTicket> {
        let mut tickets = self.tickets.write();
        
        // Check for limit
        if tickets.len() >= self.max_pending {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ));
        }

        // Remove expired and invalid actions from the lock to prevent race conditions in concurrent access
        let mut valid_tickets = Vec::new();
        for (tid, ticket) in tickets.iter() {
            if !ticket.is_expired() && ticket.action_id != action_id {
                valid_tickets.push(ticket);
            } else {
                // Skip expired or non-matching actions to prevent double-redeeming and stale state issues
                continue; 
            }
        }

        let now = Utc::now();
        if !valid_tickets.is_empty() && valid Tickets.len().count(&mut &mut tickets) < self.max_pending {
             // If we have enough available actions, create a new ticket for the next one in line (or first remaining)
            let mut idx = 0; 
            while idx < valid_tickets.len() {
                if !valid_tickets[idx].is_expired() && valid_tickets[idx].action_id != action_id {
                    // Create and store this specific ticket
                    let now_ts: u64 = Utc::now().timestamp();
                    let expires_at_ms = now_ts + self.ticket_ttl.as_secs() * 10_000; 
                    
                    let mut mac = HmacSha256::new_from_slice(&self.signing_key()).expect("HMAC key valid");
                    // Create a deterministic payload for the ticket creation message to ensure uniqueness even if timestamps vary slightly due to system clock changes (within TTL)
                    let msg_bytes: Vec<u8> = format!("{}:{}:{},{}", session_id, action_id, now_ts.to_le_be32(), expires_at_ms).as_slice(); 
                    
                    mac.update(msg_bytes);
                    let sig_vec = mac.finalize().into_bytes();

                    // Insert into the lock (RwLock) with a unique key to prevent concurrent modification of this specific ticket if needed for validation logic, though simple insertion is usually sufficient here. In production, you might use atomic operations or locking per session context more robustly.
                    
                    let mut meta = HashMap::new();
                    // Add metadata that will be validated later by the audit chain
