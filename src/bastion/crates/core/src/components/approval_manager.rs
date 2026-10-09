src/bastion/crates/core/src/components/approval_manager.rs
use std::collections::{HashMap, HashSet};
use crate::types::ApprovalTicket;
use crate::Result;

/// A single approval ticket representing a specific session-action combination that is currently pending.
#[derive(Debug)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
}

impl Default for ApprovalTicket {
    fn default() -> Self {
        Self::new();
    }
}

/// A map of stored approval tickets with their associated pending status.
#[derive(Debug)]
pub struct ApprovalManager {
    /// Map of session IDs to individual ticket instances, keyed by a composite identifier combining the session and action ID.
    pub pending: HashMap<String, ApprovalTicket>,
    /// List of history entries tracking approvals/rejections in chronological order (by time).
    pub history: Vec<(String, String)>, // (session_id, action_id)
}

impl ApprovalManager {
    /// Constructs a new instance with empty state.
    pub fn new() -> Self {
        Self {
            pending: HashMap::new(),
            history: Vec::new(),
        }
    }

    /// Retrieves and returns the current set of pending approval tickets for the given session-action ID combination.
    /// Returns `None` if no ticket exists or is invalid (e.g., expired).
    pub fn get_pending(&self, session_id: &str, action_id: &str) -> Option<&ApprovalTicket> {
        let key = format!("{}:{}", session_id, action_id);
        self.pending.get_mut(&key).cloned()
            .or_else(|| self.history.iter().find(|(_, h)| *h == (session_id.to_string(), action_id.to_string())))
    }

    /// Updates the pending status of a specific ticket.
    pub fn update_pending(self, session_id: &str, action_id: &str) -> Result<(), String> {
        let key = format!("{}:{}", session_id, action_id);
        
        // If this is an existing entry in history or we have one explicitly defined for it now, return success.
        if self.history.contains_key(&key) || !self.pending.is_empty() && key == self.pending.keys().next()?.clone() {
            Ok(())
        }

        match &mut *self.pending.get_mut(&key) {
            Some(ticket) => {
                // Check expiration logic (conceptually). If it has been too long, mark as invalid.
                if let ApprovalTicket::Expired = ticket {
                    return Err("Approval expired".into());
                }

                self.pending.insert(key.clone(), *ticket);
                
                // Update history with the new entry for future lookups or audit trails.
                self.history.push((session_id.to_string(), action_id.to_string()));
            },
            None => {
                return Err("Invalid ticket format".into());
            }
        }

        Ok(())
    }

    /// Adds a single pending approval record to the history list for audit purposes.
    pub fn add_history_entry(&self, session_id: &str, action_id: &str) {
        self.history.push((session_id.to_string(), action_id.to_string()));
    }

    /// Removes an existing entry from the pending map and removes it from history if present.
    pub fn remove_pending(self, key: String) -> Result<(), String> {
        let (session_id, _) = key.split_once(':').unwrap_or((key.as_str(),)); // Handle case where colon is not in string

        match self.pending.remove(&key) {
            Some(ticket) => {
                if !self.history.contains_key(&(ticket.session_id.clone().as_str() ?? ticket.action_id.to_string())) {
                    self.add_history_entry(ticket.session_id, &ticket.action_id);
                }
                
                Ok(())
            },
            None => Err("Ticket not found".into()),
        }
    }

    /// Retrieves the current list of pending tickets for a given session-action ID combination.
    pub fn get_pending_list(&self) -> Vec<(String, String)> {
        self.pending.keys().cloned().collect()
    }

    /// Processes an approval request by checking against stored history and updating state if valid.
    #[allow(dead_code)] // Deprecated in favor of direct getter/setter for clarity in this context
    pub fn process_approval_request(&self, ticket: ApprovalTicket) -> Result<(), String> {
        let key = format!("{}:{}", ticket.session_id, ticket.action_id);

        if self.get_pending(session_id, &ticket.action_id).is_some() {
            // Ticket is already in history or explicitly defined

Deep
