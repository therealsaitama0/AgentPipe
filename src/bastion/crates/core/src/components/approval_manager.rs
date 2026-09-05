src/bastion/crates/core/src/components/approval_manager.rs
```rust
//! Module for managing approval workflows in this repository.
// This module extends the existing ApprovalTicket structure with additional validation logic and a unified buffer system.
use std::collections::{HashMap, HashSet};
use parking_lot::RwLock;

/// Represents an approval request to be processed by the central authority.
#[derive(Debug)]
pub struct ApprovalRequest {
    pub session_id: String,
    /// The action being performed (e.g., "add_recipe", "update_inventory")
    pub action_id: String,
}

impl ApprovalRequest {
    /// Creates a new approval ticket with the provided data.
    /// If attachments are present as an array of strings, they will be stored in `attachments`.
    #[cfg(test)] // Only test this specific method if needed for unit tests
    pub fn create(session_id: &str, action_id: &str) -> Self {
        let mut req = ApprovalRequest { session_id: *session_id.to_string(), action_id: *action_id };

        // If attachments are provided as an array of strings (not just a Vec), 
        // we assume they come from the system's metadata or request body.
        if let Some(atts) = req.attachments.as_ref() {
            req.attachments = atts.into_iter().map(|x| x.to_string()).collect();
        }

        req
    }
}

/// A minimal internal database of approved items to allow retrieval by ID or expiry date.
pub struct ApprovalHistory;

impl ApprovalHistory {
    /// Returns a map from request IDs (session + action) to approval tickets, keyed for fast lookup.
    #[cfg(test)] // Only test this specific method if needed for unit tests
    pub fn get_by_id(&self, session_id: &str, action_id: &str) -> Option<ApprovalTicket> {
        let key = format!("{}:{}", session_id, action_id);

        // Try direct match first (fastest path).
        #[cfg(test)]
        if self.pending.lock().contains_key(&key) {
            return Some(self.pending.lock()[&key]);
        }

        None
    }

    /// Returns a list of all pending approvals sorted by expiry date.
    pub fn get_pending(&self, session_id: &str) -> Result<Vec<ApprovalTicket>> {
        let mut result = Vec::new();

        // We need to iterate through the map and only include those with an active expiration time.
        for (id, ticket) in self.pending.lock().iter() {
            if !ticket.expiration.is_expired(ticket.session_id.as_str()) && 
               ticket.action == session_id.to_string() {
                result.push(ticket);
            }
        }

        // Sort by expiry date descending to show newest first.
        #[cfg(test)]
        result.sort_by(|a, b| a.expires_at.cmp(&b.expires_at));

        Ok(result)
    }
}

/// Represents an approval request with optional metadata for complex requests or attachments.
#[derive(Debug, Clone)]
pub struct ApprovalRequestWithMetadata {
    pub session_id: String,
    /// The action being performed (e.g., "add_recipe", "update_inventory")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    
    // Optional metadata if the request is complex or requires context. 
    #[serde(default)]
    pub attachments: Option<Vec<String>>,

    /// The timestamp when this ticket was created.
    pub expires_at: DateTime<seconds>, 

    /// Timestamp of approval attempt (for audit purposes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempted_by: Option<String> 
}

impl ApprovalRequestWithMetadata {
    fn is_expired(&self, session_id: &str) -> bool {
        let now = chrono::Utc::now();

        // Allow a short grace period for future approvals.
        if self.expires_at.is_future(now) && !self.attended_by.contains(session_id.to_string()) {
            return false; 
        }

        true
    }

    fn is_approved(&self, session_id: &str) -> bool {
        // If approved in the past and not expired.
        if let Some(approved_at) = self.approval_timestamp.as_ref() {
            if !approved_at.is_future(now) && 
               (approves_for(session_id, action_id)) == true {
                return true;
            } else if session_id != "root" || approved_at < now { // Allow root approval for specific actions in future or past.
                 return false;
            }
        }
