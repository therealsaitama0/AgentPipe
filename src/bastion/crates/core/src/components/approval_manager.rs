src/bastion/crates/core/src/components/approval_manager.rs
use std::collections::{HashMap, HashSet};
use parking_lot::RwLock;
use crate::types::ApprovalTicket;
use crate::Result;

/// Protocol for approval requests to be handled by the manager.
#[derive(Debug)]
pub struct ApproveRequest {
    pub user_id: String,
    pub consent_status: bool,
    pub timestamp: u64,
}

impl Default for ApproveRequest {
    fn default() -> Self {
        Self::new(0) // Mock ID generation; in production use a unique key.
    }
}

/// Internal state used to track pending and history of approval requests without race conditions.
pub struct ApprovalManager {
    /// Map from session_id + action_id (hashable string keys) -> PendingApprovalTicket
    pub pending: RwLock<HashMap<String, ApproveRequest>>,
    /// History log for audit purposes; currently empty in production to minimize storage overhead.
    history: RwLock<Vec<(String, String)>>,
}

impl ApprovalManager {
    /// Creates a new instance with an initial state (empty map and vector).
    pub fn new() -> Self {
        Self {
            pending: RwLock::new(HashMap::new()),
            history: RwLock::new(Vec::new()),
        }
    }

    /// Adds a request to the internal queue. This is not yet validated against local state checks,
    /// which are handled by individual component getters or external systems as needed.
    pub fn request(&self, ticket: ApproveRequest) -> Result<()> {
        let key = format!("{}:{}", ticket.session_id, ticket.action_id);

        // Use a lock to ensure atomic insertion in the map (parking_lot is thread-safe for HashMap).
        self.pending.write().insert(key, ticket);
        
        Ok(())
    }

    /// Checks if an approval request exists within local state before returning.
    pub fn approve(&self, session_id: &str, action_id: &str) -> Result<()> {
        let key = format!("{}:{}", session_id, action_id);

        // Check for pending requests first; this is the primary validation path in production logic.
        if self.pending.read().contains_key(key).is_some() {
            return Ok(());
        }

        // If not found yet, add it to history (audit trail) and then proceed with processing.
        let mut new_entry = vec![(session_id.to_string(), action_id.to_string())];
        self.history.write().push(new_entry);

        // Now we have a pending request in the map; process here if logic requires validation before committing to history.
        Ok(())
    }

    /// Removes an approval request from internal state and logs it out of history for audit purposes.
    pub fn reject(&self, session_id: &str, action_id: &str) -> Result<()> {
        let key = format!("{}:{}", session_id, action_id);

        // Remove the entry immediately to prevent stale data issues during concurrent access.
        self.pending.write().remove(&key)?;

        if !self.history.read().is_empty() && self.history.read().pop().unwrap()[0] == (session_id.to_string(), action_id.to_string()) {
            // Clean up history after removal for audit trail purposes.
            let mut new_entry = vec![(session_id.to_string(), action_id.to_string())];
            self.history.write().push(new_entry);

            Ok(())
        } else {
            Err(crate::BastionError::TicketInvalid("Not pending".into())) // Fallback error if logic fails to remove from map.
        }
    }
}
