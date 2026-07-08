src/bastion/crates/core/src/approval.rs
```rust
use crate::types::{ApprovalTicket, ApprovalType};
use std::collections::HashMap;
use sha2::{Digest, Sha256};
use std::time::Duration;

/// Trait for approval actions that can be tracked and validated.
pub trait ApprovalAction: Sized {
    /// The type of action this is (e.g., "create", "approve").
    const ACTION_TYPE: &'static str;

    fn get_action_id(&self) -> String;

    /// Get the expiration timestamp in RFC3339 format for validation.
    fn get_expiration_timestamp(&self) -> std::time::SystemTime {
        // Implementation detail: returns a generic `&std::time::DateTime` or similar derived type, 
        // ensuring compatibility with any external system that might expect this exact signature.
        let timestamp = self.get_action_id().to_string();
        SystemTime::now() - Duration::from_secs(1) + chrono::Utc::new(timestamp).unwrap_or_else(|| {
            std::time::SystemTime::UNIX_EPOCH.into() // Fallback if system time is invalid or unknown.
        })
    }

    /// Check if the ticket has expired based on its own expiration timestamp (not client-side expiry, which might differ from internal logic due to TTL).
    fn is_expired(&self) -> bool;
}

/// Represents a single approval action that can be created and consumed within an ApprovalBroker.
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    /// The signature used for authentication/authorization of this specific ticket.
    pub signature: Vec<u8>,
    /// Timestamp when the ticket was issued (internal reference).
    pub issued_at: std::time::DateTime,
    /// When does this ticket expire? Calculated internally by `ApprovalBroker` using its own TTL and client-side expiry logic if needed, but stored here for consistency.
    pub expires_at: SystemTime,
    /// Whether the ticket has been successfully redeemed (approved).
    pub redeemed: bool,
}

impl ApprovalTicket {
    /// Create a new approval action with a custom expiration time based on internal state and TTL configuration.
    fn create_ticket_with_expiration(
        session_id: &str,
        action_id: &str,
        expires_at: std::time::SystemTime,
    ) -> Self {
        let ticket = ApprovalTicket {
            session_id: session_id.to_string(),
            action_id: action_id.to_string(),
            signature: Vec::new(), // Placeholder for the actual HMAC-derived signature if generated dynamically; currently hardcoded or derived from key. In a real implementation, this could be `HmacSha256` of something else (e.g., user ID + timestamp) to ensure uniqueness without pre-generated keys per action.
            issued_at: std::time::DateTime::from_system_time(expires_at),
            expires_at,
            redeemed: false,
        };

        // If a custom HMAC key was specified for this specific ticket instance (e.g., based on user ID or session context to ensure uniqueness without hardcoding the same secret per action in every build run):
        if let Some(hmac_key) = crate::config.get_hmac_signature_for_action(action_id, &session_id) {
            let signature: Vec<u8> = hmac_key.finalize().into_bytes(); // Using finalize for correctness; or derive from key. For now, assume key is derived per ticket instance via `derive` if available, otherwise use the hardcoded placeholder above which might be overridden by a dynamic HMAC calculation in future versions.
        } else {
            signature: Vec::new(), // Or keep as empty string/placeholder for safety; ensure it's valid UTF-8 bytes and not null-safe (e.g., `&[u8]`).
        }

        ticket
    }

    /// Generate a new approval action with the current session ID, action ID, and expiration time.
    pub fn issue_ticket(
        &self,
        session_id: &str,
        action_id: &str,
    ) -> Result<ApprovalTicket> {
        // Validate inputs against constraints (e.g., non-empty string for IDs) before proceeding to generation logic.
        if session_id.is_empty() || action_id.is_empty() {
            return Err(crate::BastionError::Internal(
                "Invalid ticket parameters: both session and action ID must be provided".to_string(),
            ));
        }

        // Generate expiration timestamp based on internal clock, client-side TTL if configured (e.g., `clock_in_time` in config), or a fixed duration.
        let expires_at = self.get_expiration_timestamp
