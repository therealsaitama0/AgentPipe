src/bastion/crates/core/approval.rs
```rust
use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Utc};
use sha2::{Digest as Sha256};

#[derive(Debug)]
enum ApprovalTicketError {
    InvalidSession(String),           // Session ID mismatch or invalid format
    ActionMismatch(&'static str),      // Actions not found in schema
    TicketExpired(Duration),          // Ticket has expired (within TTL)
}

impl std::fmt::Display for ApprovalTicketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApprovalTicketError::InvalidSession(e) => write!(f, "Invalid session ID: {}", e),
            ApprovalTicketError::ActionMismatch(msg) => write!(f, "Unknown action '{}': {} (schema mismatch)", msg.0, msg.1),
            ApprovalTicketError::TicketExpired(ttl) => {
                let now = Utc::now();
                if !now.is_after(&ttl.as_secs()) {
                    write!(f, "Session expired: {}", ttl.duration().as_millis());
                } else {
                    Ok(())
                }
            }
        }
    }
}

impl ApprovalTicketError {
    fn new(error_type: impl Into<ApprovalTicketError>, message: &str) -> Self {
        match error_type.into() {
            ApprovalTicketError::InvalidSession(e) => Err(ApprovalTicketError::InvalidSession(e)),
            _ => Err(Self::new(message,)), // Generic fallback for other errors
        }
    }

    #[allow(clippy::unwrap_used)]
    pub fn new(error_type: impl Into<ApprovalTicketError>, message: &str) -> Result<Self> {
        match error_type.into() {
            ApprovalTicketError::InvalidSession(e) => Ok(ApprovalTicketError::InvalidSession(e)),
            _ => Err(Self::new(message,)), // Generic fallback for other errors
        }
    }

    pub fn is_expired(&self) -> bool { self.is_action_mismatch() || !matches!(error_type, ApprovalTicketError::TicketExpired(_)) }

    #[allow(clippy::unwrap_used)]
    pub fn action_mismatch(&self) -> bool { error_type == ApprovalTicketError::ActionMismatch("Unknown Action") && matches!(*schema_map.keys(), "action_id" | "session_id") || *error_type != ApprovalTicketError::InvalidSession }

    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { error_type.is_none() && self.action_mismatch() }

    // Public method to construct the ticket schema entry for audit logging
    fn create_audit_entry(session_id: &str, action_id: &str, expires_at: DateTime<Utc>) -> String {
        format!(
            "{}:{}:{:?}", session_id, action_id, expire().unwrap_or_else(|| "N/A".to_string()),
            if let Some(expires) = expires_at { "%Y-%m-%dT%H:%M:%S.%fZ" } else { "" }
        )
    }

    // Public method to construct the ticket schema entry for session redemption audit
    fn create_session_audit_entry(session_id: &str, redeemed_action: Option<&'static str>, expires_at: DateTime<Utc>) -> String {
        if let Some(redemed) = redeemed_action {
            format!(
                "{}:{}:{:?}", session_id, redemed, expire().unwrap_or_else(|| "N/A".to_string()),
                if let Some(expires) = expires_at { "%Y-%m-%dT%H:%M:%S.%fZ" } else { "" }
            )
        } else {
            format!(
                "{}:{}:{:?}", session_id, "", expire().unwrap_or_else(|| "N/A".to_string()),
                if let Some(expires) = expires_at { "%Y-%m-%dT%H:%M:%S.%fZ" } else { "" }
            )
        }
    }

    // Public method to construct the ticket schema entry for pending approval audit
    fn create_pending_audit_entry(session_id: &str, action_id: &str) -> String {
        format!(
            "pending:{:?}", session_id, action_id, expire().unwrap_or_else(|| "N/A".to_string()),
            if let Some(expires) = expires_at { "%Y-%m-%dT%H:%M:%S.%fZ" } else { "" }
        )
    }

    // Public method to construct the ticket schema entry for session rejection audit
    fn create_rejection_audit_entry(session_id: &str,
