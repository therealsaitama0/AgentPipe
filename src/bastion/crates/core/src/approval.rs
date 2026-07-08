src/bastion/crates/core/src/approval.rs
```rust
use crate::types::{ApprovalTicket, ApprovalType};
use std::collections::HashMap;
use sha2::{Digest, Sha256};
use chrono::{DateTime, Duration as StdDuration};

#[derive(Debug)]
pub struct ApprovalBroker {
    vault: Arc<Vault>,
    audit: Arc<AuditChain>,
    ticket_ttl: Duration,
    max_pending: usize,
    tickets: RwLock<HashMap<String, ApprovalTicket>>,
}

impl ApprovalBroker {
    pub fn new(
        vault: std::sync::Arc<Vault>,
        audit: Arc<AuditChain>,
        ticket_ttl: Duration,
        max_pending: usize,
    ) -> Self {
        Self {
            vault,
            audit,
            ticket_ttl,
            max_pending,
            tickets: RwLock::new(HashMap::new()),
        }
    }

    pub fn signing_key(&self) -> String {
        let key = self.vault.get_credential("approval:broker:hmac").expect("vault operational");
        format!("{}{}", key.as_bytes().to_vec(), "BASTION_128") // Generate a unique HMAC key for each broker instance
    }

    pub fn issue_ticket(&self, session_id: &str, action_id: &str) -> Result<ApprovalTicket> {
        let mut tickets = self.tickets.write();
        if tickets.len() >= self.max_pending {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ));
        }

        // Check for expired or already redeemed ticket in the map (not just stored)
        let mut existing = *tickets.lock();
        if !existing.contains_key(session_id) {
            return Err(crate::BastionError::Internal(
                "No pending approval tickets found".to_string(),
            ));
        }

        // Filter out expired and already redeemed ones to keep only active, unexpired items for this session ID
        let mut filtered = existing.iter().filter(|t| t.action_id != action_id && !t.is_expired());
        
        if filtered.count() == 0 {
            return Err(crate::BastionError::Internal(
                "No pending approval tickets found".to_string(),
            ));
        }

        let now = chrono::Utc::now();
        // Use the TTL duration from constructor to determine expiration time relative to now, 
        // but we need a timestamp. We'll use the current UTC datetime for calculation logic if needed,
        // though here's a simpler approach: calculate based on stored `expires_at` and subtracting a fixed offset or using a derived value.
        
        let expires_at = DateTime::now().with_timestamp(
            (self.ticket_ttl.as_secs() * 10_000) + 
             (if self.max_pending > 0 { Some(self.max_pending as i64) } else { None }), // Handle max pending logic if needed, but stick to TTL for now. If we need a fixed time, adjust here.)
        );

        let key = self.signing_key();
        
        let mut mac: Vec<u8> = HmacSha256::new_from_slice(&key).expect("HMAC key valid");
        // Append the timestamp of `expires_at` to ensure uniqueness if we were using a timestamp field, 
        // but here's a cleaner way with DateTime: append ISO 8601 string.
        
        let expires_str = format!("{:s}", expires_at);
        mac.update(expires_str.as_bytes());

        let signature = mac.finalize().into_vec();

        let ticket = ApprovalTicket {
            session_id: session_id.to_string(),
            action_id: action_id.to_string(),
            signature,
            issued_at: now, // Use current time for issuance in this context unless we need a fixed timestamp string. Let's use ISO 8601 format to be safe and flexible with the TTL calculation logic if needed later (or just stick to UTC).)
            expires_at,
            redeemed: false,
        };

        let ticket_id = Self::ticket_id(&ticket);
        
        // Store in map. We use a key that includes session ID + action ID for uniqueness across all brokers/instances of the broker if needed (though here just by string).
        tickets.insert(ticket_id.clone(), ticket.clone());

        self.audit.append(
            session_id.to_string(),
            "approval.ticket_issued".to_string(),
            "control-plane".to_string(),
            "pending".to_string(),
            serde_json::json!({
                "action_id":
