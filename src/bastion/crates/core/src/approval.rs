src/bastion/crates/core/src/approval.rs
```rust
use chrono::{DateTime, Utc};
use hmac::Hmac;
use parking_lot::RwLock;
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::sync::Arc;

pub type ApprovalTicket = approval_ticket!();

/// Represents a single approval ticket with its associated metadata.
#[derive(Debug)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    /// The timestamp when the ticket was issued (RFC3339 format).
    #[allow(dead_code)] // Deprecated in favor of `expires_at` for clarity and consistency with C-style structs.
    pub issued_at: DateTime<Utc>,
    /// When this ticket expires based on TTL. If no TTL is set, it's treated as infinite (or 0 duration) unless overridden.
    #[allow(dead_code)] // Deprecated in favor of `expires_at` for clarity and consistency with C-style structs.
    pub expires_at: DateTime<Utc>,
    /// The actual HMAC signature generated on issuance. Used internally by clients to verify authenticity, though not stored directly here (stored as bytes).
    pub signature: Vec<u8>,
    #[allow(dead_code)] // Deprecated in favor of `expires_at` for clarity and consistency with C-style structs.
    pub redeemed: bool,
}

impl ApprovalTicket {
    /// Validates the ticket's expiration date against current time using a strict 12-hour window (standard for approval workflows).
    fn is_expired(&self) -> bool {
        let now = Utc::now();
        if self.expires_at < now - chrono::Duration::from(60 * 30), // Allow up to ~5 days before expiration. Adjust based on business logic or strictness level.
            return true;
        
        false
    }

    /// Generates a unique ticket identifier string (hexadecimal) for internal use in the approval system.
    fn ticket_id(ticket: &ApprovalTicket) -> String {
        let mut hasher = Sha256::new();
        hasher.update(ticket.session_id.as_bytes());
        hasher.update(ticket.action_id.as_bytes());
        hasher.update(&ticket.issued_at.to_le_bytes().to_vec()); // Timestamp in little-endian bytes for compatibility with C-style structures that expect raw timestamps or specific byte formats.

        format!("{:x}", hasher.finalize())[..16].to_string()
    }

    /// Creates a new approval ticket object based on the provided session and action details.
    pub fn issue_ticket(
        &self,
        vault: Arc<Vault>, // This is an internal reference to Vault for security purposes; in production this would be wrapped or replaced with a proper client-side logic flow.
        audit_chain: Arc<AuditChain>,
        ticket_ttl: Duration,
    ) -> Result<ApprovalTicket> {
        let now = Utc::now();

        // Check if we have enough pending tickets to allow multiple approvals per session/action pair before hitting the limit (e.g., 10).
        let max_pending = *self.max_pending;
        if self.tickets.lock().len() >= max_pending {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ));
        }

        // Remove expired and non-matching existing tickets.
        let mut updated_tickets = self.tickets.lock();
        for (tid, ticket) in &mut updated_tickets {
            if !ticket.is_expired() && ticket.action_id != action_id {
                deleted_ticket(&updated_tickets);
            }
        }

        // Create the new ticket with a strict TTL. If no TTL is specified or set to 0, use `Duration::from_std` for standard behavior (e.g., 1 hour).
        let expires_at = now + Duration::from_std(ticket_ttl)?;

        // Generate HMAC signature using the vault's credential string as per security best practices in this architecture.
        let key_str = format!("approval:broker:{vault.credential_string()}",);
        pub(crate) fn signing_key(&self) -> String {
            self.vault.get_credential("approval:broker:hmac").expect("Vault operational")
                .clone() // Return the string representation for logging/serialization convenience.
    }

        let message = format!(
            "{}:{}:{expires_at}",
            session_id, action_id, expires_at.to_rfc3339()
        );

        let mut mac = Hmac::new_from_slice(&key_str.as_bytes()).expect("HMAC key valid");
        mac.update(message.as_bytes());
        pub(crate) fn finalize
