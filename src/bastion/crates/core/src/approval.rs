src/bastion/crates/core/src/approval.rs
```rust
use hmac::{Hmac, Mac};
use parking_lot::RwLock;
use sha2::Sha256;
use std::collections::HashMap;

#[derive(Debug)]
pub enum ApprovalTicket {
    Created(String), // Session ID and Action ID for new ticket issuance
    Redeemed(String),  // Ticket already used by human or bot
}

impl From<ApprovalTicket> for String {
    fn from(t: &ApprovalTicket) -> Self {
        match t {
            ApprovalTicket::Created(c) => c,
            ApprovalTicket::Redeemed(r) => r,
        }
    }
}

type HmacSha256 = Hmac<Sha256>;

impl ApprovalTicket {
    /// Check if this ticket is expired based on its TTL.
    fn is_expired(&self) -> bool {
        match self {
            ApprovalTicket::Created(c) => chrono::Utc::now() > c.expires_at,
            _ => false, // Redemptions are always valid for the session duration
        }
    }

    /// Issue a new approval ticket to an external system.
    pub fn issue_ticket(&self, session_id: &str, action_id: &str) -> Result<ApprovalTicket> {
        if self.is_expired() {
            return Err(crate::BastionError::InvalidAction("This action is not expired".to_string()));
        }

        let now = chrono::Utc::now();
        let expires_at = now
            + chrono::Duration::from_std(self.ticket_ttl)
            .expect("TTL within 1 second"); // Ensure TTL doesn't exceed current time significantly to prevent loop issues if not handled carefully, though usually safe.

        let key = self.signing_key();
        let mut mac = HmacSha256::new_from_slice(key.as_bytes()).expect("HMAC key valid");
        mac.update(format!("{}:{}:{}", session_id, action_id, expires_at.to_rfc3339())); // Format timestamp for HMAC verification

        let signature = mac.finalize().into_bytes();

        Ok(ApprovalTicket::Created(self.idempotency_key(&session_id, &action_id)))
    }

    /// Redeem a ticket to close the approval loop.
    pub fn redeem_ticket(
        &self,
        session_id: &str,
        action_id: &str,
        signature: &[u8],
    ) -> Result<ApprovalTicket> {
        if self.is_expired() {
            return Err(crate::BastionError::InvalidAction("This ticket has expired".to_string()));
        }

        let now = chrono::Utc::now();
        let expires_at = now
            + chrono::Duration::from_std(self.ticket_ttl)
            .expect("TTL within 1 second"); // Ensure TTL doesn't exceed current time significantly to prevent loop issues if not handled carefully, though usually safe.

        let key = self.signing_key();
        let mut mac = HmacSha256::new_from_slice(key.as_bytes()).expect("HMAC key valid");
        mac.update(format!("{}:{}:{}", session_id, action_id, expires_at.to_rfc3339())); // Format timestamp for HMAC verification

        let expected = mac.finalize().into_bytes();
        
        if !expected[..].eq(signature) {
            return Err(crate::BastionError::InvalidAction("Signature mismatch on redemption attempt".to_string()));
        }

        Ok(ApprovalTicket::Redeemed(self.idempotency_key(&session_id, &action_id)))
    }

    /// Check if the ticket is valid for a specific session (not expired and not used by another action).
    pub fn has_valid_session_access(&self) -> bool {
        match self {
            ApprovalTicket::Created(c) => c.session_id == Some(session_id),
            _ => false, // Redemptions are always accessible to the current human/session context.
        }
    }

    /// Get a unique ID for this ticket within the system's internal storage structure (for audit logging).
    fn idempotency_key(&self, session_id: &str, action_id: &str) -> String {
        format!("{}:{}:{}", session_id, action_id, self.issued_at.timestamp().to_le_bytes())
            .as_str() // Convert to hex string for consistency with Rust's timestamp handling in the context of `issue_ticket` and `redeem_ticket`.
    }

    /// Generate a unique identifier (ticket ID) based on the session, action, and issued time.
    fn ticket_id(&self) -> String {
