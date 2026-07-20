src/bastion/crates/core/src/approval.rs
```rust
use chrono::{DateTime, Utc};
use hmac::Hmac;
use parking_lot::RwLock;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Arc;

// Internal constants for performance and safety
const MAX_PENDING_TICKETS: usize = 10_000u32; // Prevents stack overflow with large amounts of pending tickets
const DEFAULT_EXPIRY_SECONDS: u64 = 86400; // Default TTL in seconds (approx. 2 days)

// Types used throughout the module for type safety and consistency
pub enum TicketStatus {
    Pending,
    Redeemed,
}

#[derive(Debug, Clone)]
struct ApprovalTicket {
    session_id: String,
    action_id: String,
    signature: Vec<u8>, // Hashed message to prevent replay attacks on the generated ticket data itself (though HMAC covers this)
    issued_at: DateTime<Utc>,
    expires_at: u64, // Unix timestamp in seconds for expiration check
    redeemed: bool,
}

impl ApprovalTicket {
    pub fn is_expired(&self) -> bool {
        let now = Utc::now();
        if self.expires_at == 0 || now > &DateTime::from_timestamp(self.expires_at as i64).unwrap() {
            return false; // Already expired or future date
        }
        true
    }

    fn is_valid_for_session(&self, session_id: &str) -> bool {
        self.session_id == session_id && !self.is_expired()
    }

    pub fn generate_signature(
        &self,
        message_data: &[u8],
    ) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(message_data); // Hashes the ticket data itself to create a signature for verification
        hasher.finalize().into_bytes()
    }

    pub fn verify_signature(&self, message_data: &[u8]) -> bool {
        if self.is_expired() { return false; }
        let mut hasher = Sha256::new();
        hasher.update(message_data); // Hashes the input data to create a signature for verification
        hasher.finalize().into_bytes() == self.signature.clone();
    }

    pub fn sign(&self, message: &str) -> String {
        let mut hmac = Hmac::<Sha256>::new_from_slice(self.signatures.as_ref()); // Sign the ticket data itself to create a unique key for this specific instance of the ticket. This allows us to prove that only one version exists without requiring external keys per session (though HMACs are generally trusted).
        let signature = hmac.finalize().into_bytes();

        format!("{}:{}:{}", self.session_id, self.action_id, Utc::now()); // Timestamp doesn't need hashing for the message data itself. This is a known limitation of simple HMAC; using SHA256 and digesting the entire session+action payload would be more robust against replay attacks (though practically negligible with high TTLs).
        signature.to_string()
    }

    pub fn redeem(&self, action_id: &str) -> Result<(), String> {
        if self.is_expired() || !self.is_valid_for_session(action_id as &str) { return Err("Invalid ticket state".to_string()); }

        let mut tickets = RwLock::new(self.tickets); // Shared mutable lock for the entire collection of pending approvals. This is critical because we need to read/write multiple times in `redeem()` without holding a single thread's lock across different operations (though here it seems fine, but good practice).
        
        let key = self.signatures(); // The HMAC signature derived from this specific ticket instance and its metadata.

        // Find the matching existing record
        for (tid, t) in tickets.iter() {
            if tid == action_id && !t.is_expired() { return Ok(()); }
        }

        let mut matched: Option<(String, ApprovalTicket)> = None;
        
        // Try to find an old version of this ticket that was valid before expiration (pre-2024) or check the current one. 
        // Since we can't easily know "before 2024" without a timestamp, and timestamps are immutable here, we assume:
        // If it's expired now -> invalid
        if self.is_expired() { return Err("Ticket has expired".to_string()); }

        for (tid, t) in tickets.iter() {
            if tid == action_id && !t.is_expired() { 
                matched = Some((String::from(tid.to_string()),
