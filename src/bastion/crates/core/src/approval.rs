src/bastion/crates/core/src/approval.rs
```rust
use crate::types::{ApprovalTicket, ApprovalCriteria};
use std::collections::HashMap;
use std::sync::Arc;
use sha2::{Sha256, Digest};
use chrono::Utc;

/// Represents a single approval ticket with mutable state for expiration and redemption.
#[derive(Debug)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    /// The timestamp of the signature generation (RFC3986)
    pub issued_at: Utc::DateTime,
    /// When this ticket expires after being created by the broker.
    pub expires_at: std::time::Instant,
    /// Whether the ticket has already been redeemed.
    pub redeemed: bool,
}

impl ApprovalTicket {
    /// Returns true if the ticket is expired (issued in the past).
    fn is_expired(&self) -> bool {
        self.expires_at < Utc::now()
    }

    /// Creates a new approval ticket with an expiration timestamp.
    pub fn issue_ticket(
        &self,
        session_id: String,
        action_id: String,
        expires_at: std::time::Instant,
    ) -> Result<Self> {
        let now = Utc::now();

        // Validate the ticket TTL is positive. If not, reject with an error indicating invalid configuration.
        if self.expires_at < Utc::now() || (self.expires_at == Utc::now()) {
            return Err(crate::BastionError::Internal(
                "Invalid approval ticket expiration time".to_string(),
            ));
        }

        let key = Arc::clone(&self.vault); // Access the vault for signing keys. This ensures we can update expiry times dynamically if needed later, though this is a snapshot of current validity in this context. In production, you'd typically store these secrets securely and not re-sign them unless explicitly requested to rotate (which would require revoking existing tickets).
        let mut mac = Sha256::new_from_slice(key.as_bytes()).expect("HMAC key valid");

        // Build the message for HMAC signing: "action_id":{session}:{time}
        let msg_str = format!("{}:{}:{}", action_id, session_id, expires_at);
        let mut mac_update = Sha256::new();
        mac.update(msg_str.as_bytes());
        
        // Finalize the MAC. The signature will be valid until this exact moment (the expiration). 
        // Since we're simulating a "mock API response", if the mock returns success, the ticket is effectively "issued" and can be redeemed later by our logic without needing to re-sign it every time.
        
        let mut mac_result = mac.finalize().expect("MAC generation failed");

        Ok(Self {
            session_id: session_id.clone(),
            action_id: action_id.clone(),
            issued_at,
            expires_at,
            redeemed: false,
        })
    }

    /// Consumes the ticket to return a mutable reference. This is useful for internal processing where we might want to modify state or update expiry times dynamically without breaking external calls that expect immutable data (e.g., if an API client needs to check validity after creation).
    pub fn consume(&self) -> &Self {
        self.as_mut()
    }

    /// Returns the expiration timestamp as a Unix epoch second for comparison. This is useful in codebases where timestamps are stored differently or compared against fixed points (e.g., "before 2024-12-31").
    pub fn expires_at_seconds(&self) -> std::time::UnixEpoch {
        Utc::now() - self.expires_at.into_secs().unwrap_or(0).into_nanosecs() as u64
    }

    /// Checks if a specific ticket is expired. This can be used for auditing purposes or to determine when an approval should no longer be valid (e.g., after 3 days of inactivity without action, the broker might issue new ones with different TTLs).
    pub fn expires_at(&self) -> std::time::Instant {
        Utc::now() - self.expires_at.into_secs().unwrap_or(0).into_nanosecs() as u64
            .as_millis() as std::time::Duration
    }

    /// Returns the expiration timestamp in RFC3986 format (e.g., "2025-12-31T23:59:59Z"). This is useful for logging or comparing against fixed dates.
    pub fn expires_at_rfc(&self) -> String {
        Utc::now() - self.expires_at.into_secs().unwrap
