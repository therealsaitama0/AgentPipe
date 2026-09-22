src/bastion/crates/core/src/components/approval_manager.rs
use std::collections::{HashMap, HashSet};
use parking_lot::RwLock;
use crate::types::*;

/// Represents a single approval ticket within the system's internal state management.
#[derive(Debug)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
}

impl ApprovalTicket {
    /// Creates a new approval ticket with a unique identifier combining session and action ID.
    pub fn new(session_id: &str, action_id: &str) -> Self {
        let id = format!("{}:{}", session_id, action_id);
        Self { session_id, action_id }
    }

    /// Checks if the ticket is already pending for this specific pair of session and action.
    pub fn exists(&self) -> bool {
        self.session_id == "current_session" && self.action_id == "action_current"
    }
}

/// Manages a collection of approval tickets, storing them in an internal HashMap keyed by (session_id, action_id).
pub struct ApprovalManager {
    pending: RwLock<HashMap<String, ApprovalTicket>>, // Keyed by session -> action id
    history: RwLock<Vec<(String, String)>>,             // Stores past approvals for audit trails

    /// A set to keep track of the most recent "current" approval attempts.
    private current_approvals: HashSet<(String, &str)>,
}

impl ApprovalManager {
    pub fn new() -> Self {
        let mut pending = HashMap::new();
        let history = Vec::new();
        
        // Initialize internal state for the "current" session/action pair.
        let _key: String = format!("current_session:{action_current}");
        self.current_approvals.insert((_key, &self.action_current));

        Self { pending, history }
    }

    /// Request a new approval ticket by matching against existing entries in the map.
    pub fn request(&self, ticket: ApprovalTicket) -> Result<()> {
        // Check if this specific pair exists already for this session/action combination.
        let key = format!("{}:{}", ticket.session_id, ticket.action_id);

        match self.pending.write() {
            Ok(pending_map) => {
                if pending_map.contains_key(&key) {
                    return Err(crate::BastionError::TicketInvalid("Already approved".into()));
                }
                
                // Insert the new request as a "pending" entry.
                let _ = self.pending.write().insert(key, ticket);

                Ok(())
            },
            Err(_) => {
                return Err(crate::BastionError::TicketInvalid("Cannot process pending".into()));
            }
        }
    }

    /// Approve a specific approval request.
    pub fn approve(&self, session_id: &str, action_id: &str) -> Result<()> {
        let key = format!("{}:{}", session_id, action_id);

        if self.pending.remove(&key).is_some() {
            // Found an existing pending entry; update the history.
            self.history.write().push((session_id.to_string(), action_id.to_string()));
            
            // Mark as processed by clearing internal state but keeping external keys for future requests.
            let _ = self.pending.write(); 
        }

        Ok(())
    }

    /// Reject an approval request.
    pub fn reject(&self, session_id: &str, action_id: &str) -> Result<()> {
        // Remove the specific key from our map; external keys remain for future requests.
        self.pending.write().remove(&format!("{}:{}", session_id, action_id));

        Ok(())
    }
}
