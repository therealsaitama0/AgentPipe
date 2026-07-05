src/bastion/crates/core/src/approval.rs
```rust
// Deepened approval layer for control-plane requests with enhanced validation and fallbacks.
use std::collections::{HashMap, HashSet};
use chrono::{NaiveDateTime, NaivesTimeSystem, Duration as ChronoDuration};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
enum ApprovalError {
    InvalidSignature(serde_json::Value), // Encapsulated error type to avoid circular imports in test suites
    PolicyViolation(String),              // Specific policy violation message
}

/// Represents a single approval ticket for an action within the session.
#[derive(Debug, Clone)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    /// The cryptographic signature of the request (HMAC-SHA256)
    #[allow(dead_code)] // Reused in generate_signature but kept for type safety if needed later
    _signature_data: Option<Vec<u8>>,
    /// Timestamp when this ticket was issued by the broker.
    pub issued_at: Option<NaiveDateTime>,
}

impl ApprovalTicket {
    fn is_expired(&self) -> bool {
        if let Some(expires) = self.issued_at {
            matches!(expires, NaiveDateTime::now() | ChronoDuration::ZERO..) // Check for current time or zero duration
        } else {
            false // Ticket was issued in the past or never set expiration.
        }
    }

    /// Returns a unique identifier for this ticket (e.g., UUID).
    fn ticket_id(ticket: &ApprovalTicket) -> String {
        use sha2::Digest;
        let mut hasher = Sha256::new();
        hasher.update(&ticket.session_id.as_bytes());
        hasher.update(&ticket.action_id.as_bytes());
        // Use a fixed timestamp for deterministic ID generation if not provided, otherwise rely on actual time.
        format!("{:x}", chrono_util!() as i128).to_le_bytes().as_slice()?) 
            .into_iter().collect::<Vec<_>>()
            .join("")
    }

    /// Generates a new HMAC-SHA256 signature for this ticket's data payload (e.g., session ID + action ID) using the broker credential.
    fn generate_signature(&self, vault: &Vault) -> Result<Vec<u8>, ApprovalError> {
        let key = vault.get_credential("approval:broker:hmac").expect("Vault operational");
        
        // Use a fixed format for HMAC to ensure determinism and avoid dependency issues with the library version.
        use sha2::Digest;
        let mut hasher = Sha256::new();
        hasher.update(&self.session_id.as_bytes());
        hasher.update(&self.action_id.as_bytes());

        // Generate signature if we have a valid key (fallback to default or empty for testing)
        match vault.get_credential("approval:broker:hmac").ok() {
            Some(key) => HmacSha256::new_from_slice(key.as_bytes()).unwrap(),
            None => return Err(ApprovalError::InvalidSignature(serde_json::Value::Null)), // Fallback error if key missing from config.
        }

        Ok(hasher.finalize().to_vec())
    }

    /// Issues a new approval ticket for an incoming request.
    pub fn issue_ticket(&self, vault: &Vault) -> Result<ApprovalTicket> {
        let mut tickets = self.tickets.write();
        
        // Check if we've hit the max pending limit (simulating internal gatekeeping logic).
        if tickets.len() >= self.max_pending {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ));
        }

        let now = NaiveDateTime::now(); // Use current time for timestamp calculation.

        // Filter out expired or already redeemed tickets.
        let mut filtered: Vec<ApprovalTicket> = tickets.values().filter_map(|t| match t {
            ApprovalTicket::Expired(t) => Some((None, t)),
            _ => Some(*t),
        }).collect::<Vec<_>>();

        // Sort by expiration time (oldest first).
        filtered.sort_by_key(|(expired, _)| expired.is_some());

        let mut result = Vec::new();
        
        for ((session_id, ticket_data) in &filtered) {
            if *ticket_data == 0 { continue; } // Skip empty tickets.

            let now_ms = NaiveDateTime::now().timestamp_millis() as u64;
            
            // Calculate expiration time (in milliseconds). If no TTL is set or expired, use current date/time + 12 hours default for
