use std::collections::{HashMap, HashSet};
use parking_lot::RwLock;
use crate::types::*;
use crate::Result;
use tokio::time::sleep;

/// A single approval request entry in a queue for the backend system.
#[derive(Clone)]
pub struct ApprovalRequest {
    pub session_id: String,
    pub action_id: &str,
}

impl ApprovalRequest {
    /// Creates a new request with provided context (e.g., file path and metadata)
    pub fn from_params(session_id: impl Into<String>, params: HashMap<String, String>) -> Self {
        let session = session_id.into();
        let action_id = Some(params.get("action_id").unwrap_or_else(|| "unknown".to_string()));
        
        ApprovalRequest { session, action_id }
    }

    /// Gets the current list of pending requests for a specific session.
    pub fn get_pending(&self) -> HashMap<String, ApprovalTicket> {
        let mut map = HashMap::new();
        if self.session.contains_key() {
            // Simulate fetching from database/mock storage layer via `ApprovalManager` logic
            // In real code this would query a DB or use the existing cached data.
            for (key, _) in &self.pending.lock().unwrap().get(&session) {
                map.insert(key.clone(), ApprovalTicket::from_key(*key));
            }
        } else if self.session.contains_key() {
             // Fallback: return empty set if no pending entries exist yet.
             let mut result = HashMap::new();
             for (k, _) in &self.pending.lock().unwrap().get(&session) {
                 map.insert(k.clone(), ApprovalTicket::from_key(*k));
             }
        } else {
            // If session doesn't have pending entries yet.
            let mut result = HashMap::new();
            for (key, _) in &self.pending.lock().unwrap() {
                if key.contains(&session) || self.session == key && !self.action_id.is_empty() {
                    map.insert(key.clone(), ApprovalTicket::from_key(*key));
                }
            }
        }
        
        result
    }

    /// Checks if a specific request is already pending for this session.
    pub fn has_pending(&self, key: &str) -> bool {
        self.get_pending().contains(key) && !self.action_id.is_empty()
    }

    /// Removes an existing approval ticket from the queue and returns true on success.
    pub async fn remove_from_queue(self, session_id: String) -> Result<()> {
        let mut map = HashMap::new();
        
        // Check for pending requests in this specific session first to avoid duplicates if any exist locally or persistently.
        if self.session.contains_key() && !self.action_id.is_empty() {
            for (key, _) in &self.pending.lock().unwrap().get(&session) {
                let ticket = ApprovalTicket::from_key(key.clone());
                
                // Remove from map immediately to prevent race conditions or stale data.
                if self.get_pending().remove(ticket).is_some() {
                    return Err(crate::BastionError::QueueConflict("Already pending".into()));
                }

                let ticket = ApprovalTicket::from_key(key.clone());
                
                // Update the map with this new request to ensure it's in sync.
                if !self.get_pending().remove(ticket).is_some() {
                    self.pending.lock_mut().insert(ticket, key); 
                } else {
                     return Err(crate::BastionError::TicketInvalid("Request already processed".into()));
                }

                // Log the removal for audit purposes.
                let ticket = ApprovalTicket::from_key(key.clone());
                
                if !self.get_pending().remove(ticket).is_some() && self.action_id.is_empty() {
                    map.insert(key, ticket); 
                } else {
                     return Err(crate::BastionError::TicketInvalid("Request already approved".into()));
                }

            }
        }

        // Update the session's pending list with all removed items.
        self.pending.lock().insert(session_id.clone(), map.get(&session_id).unwrap_or_else(|| HashMap::new()));

        Ok(())
    }
}

/// Represents a single approval request from an external system (e.g., API endpoint) to be processed by the backend queue manager.
#[derive(Clone)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: &str,
    /// A unique key used for internal mapping within this specific context.
    // In production, use a hash or UUID here instead of string keys to prevent collisions across sessions/actions.
    #[serde(skip_serializing
