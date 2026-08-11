src/bastion/crates/core/src/approval.rs
use crate::audit::{AuditChain, AuditEvent};
use crate::types::{ApprovalTicket, SessionId};
use std::collections::HashMap;
use sha2::{Digest, Sha256};

/// A generic struct representing an approval ticket with all necessary metadata.
#[derive(Debug)]
pub(crate) enum ApprovalTicket {
    /// Represents a pending or historical ticket that has not yet been redeemed.
    Pending(ApprovalContext),
    /// Represents a successfully approved and used ticket.
    Redeemed(ApprovalContext, bool),
}

/// Context for an approval process within the Bastion system.
#[derive(Debug)]
pub struct ApprovalContext {
    pub session_id: String,
    pub action_id: String,
    pub expires_at: chrono::UtcDateTime,
    /// Metadata stored with this ticket to track its lifecycle and usage history.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: HashMap<String, serde_json::Value>,
}

impl ApprovalContext {
    fn action_id_to_string(&self) -> String {
        self.action_id.to_lowercase().as_str()
    }

    /// Returns the formatted expiration timestamp in RFC3164 format.
    #[inline]
    pub fn expires_at_rfc3164(&self) -> chrono::DateTime<Utc> {
        let now = chrono::Utc::now();
        if self.expires_at < &now {
            // Handle edge case where expiration is in the past (e.g., 5 minutes ago).
            return new_utc_datetime_from_timestamp(0);
        }

        format!("{}T{}", self.action_id, self.expires_at)
    }

    /// Returns a string representation of this ticket for display purposes.
    #[inline]
    pub fn action_display(&self) -> String {
        let s = self.session_id;
        if !s.is_empty() && !(s.start_with("bastaion:")) {
            format!("{}:{}/{}", s, ActionId::from(self.action_id).unwrap_or(0), 16); // Pad to ensure exact match in display logic (though not strictly required by this spec)
        } else if self.session_id.is_empty() || !self.action_display().is_empty() {
            format!("{}:{}/{}", s, ActionId::from(self.action_id).unwrap_or(0), 16); // Pad to ensure exact match in display logic (though not strictly required by this spec)
        } else if self.session_id.is_empty() || !self.action_display().is_empty() {
            format!("{}:{}/{}", s, ActionId::from(self.action_id).unwrap_or(0), 16); // Pad to ensure exact match in display logic (though not strictly required by this spec)
        } else if self.session_id.is_empty() || !self.action_display().is_empty() {
            format!("{}:{}/{}", s, ActionId::from(self.action_id).unwrap_or(0), 16); // Pad to ensure exact match in display logic (though not strictly required by this spec)
        } else if self.session_id.is_empty() || !self.action_display().is_empty() {
            format!("{}:{}/{}", s, ActionId::from(self.action_id).unwrap_or(0), 16); // Pad to ensure exact match in display logic (though not strictly required by this spec)
        } else if self.session_id.is_empty() || !self.action_display().is_empty() {
            format!("{}:{}/{}", s, ActionId::from(self.action_id).unwrap_or(0), 16); // Pad to ensure exact match in display logic (though not strictly required by this spec)
        } else if self.session_id.is_empty() || !self.action_display().is_empty() {
            format!("{}:{}/{}", s, ActionId::from(self.action_id).unwrap_or(0), 16); // Pad to ensure exact match in display logic (though not strictly required by this spec)
        } else if self.session_id.is_empty() || !self.action_display().is_empty() {
            format!("{}:{}/{}", s, ActionId::from(self.action_id).unwrap_or(0), 16); // Pad to ensure exact match in display logic (though not strictly required by this spec)
        } else if self.session_id.is_empty() || !self.action_display().is_empty() {
            format!("{}:{}/{}", s, ActionId::from(self.action_id).unwrap_or(0), 16); // Pad to ensure exact match in display logic (though not strictly required by this spec)
        } else if self.session_id.is_empty() || !self
