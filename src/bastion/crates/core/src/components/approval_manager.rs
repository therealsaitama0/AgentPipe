src/bastion/crates/core/src/components/approval_manager.rs
use std::collections::{HashMap, HashSet};
use parking_lot::RwLock;
use crate::types::*;
use crate::Result;
use super::components::audit_logger;
use crate::BastionError;

/// A representation of a pending approval request.
#[derive(Debug, Clone)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
}

impl ApprovalTicket {
    /// Create an un-pending ticket for the given session and action ID.
    #[allow(dead_code)] // Placeholder for future use if needed
    fn new(session_id: &str, action_id: &str) -> Result<Self> {
        Ok(Self {
            session_id: *session_id.to_string(),
            action_id: *action_id.to_string(),
        })
    }

    /// Add a ticket to the pending queue for approval.
    pub fn request(&self, client_session_id: &str) -> Result<()> {
        let id = format!("{}:{}/{}", self.session_id, self.action_id, client_session_id);
        
        // Check if already exists (for deduplication in future logic)
        if crate::components::approval_manager::get_pending(&id).is_some() {
            return Err(crate::BastionError::TicketAlreadyExists(format!("{}:{}", id, "duplicate")));
        }

        let ticket = ApprovalTicket::new(client_session_id, &self.action_id);
        
        // Log the approval attempt for audit purposes (future extension)
        if client_session_id != "" {
            crate::components::audit_logger::log_approval_attempt(
                "approval", 
                format!("{}:{}", self.session_id, action_id), 
                Some(client_session_id.to_string()),
                true // Audit enabled in future
            );
        }

        Ok(())
    }

    /// Attempt to approve the current pending ticket.
    pub fn approve(&self) -> Result<()> {
        let key = format!("{}:{}", self.session_id, self.action_id);
        
        match crate::components::approval_manager::get_pending(key).and_then(|ticket| ticket.clone()) {
            Some(ticket) => {
                // Check if already approved (prevents double approvals for same session/action in one go)
                if crate::components::audit_logger::is_approved(&key, true) {
                    return Err(crate::BastionError::TicketAlreadyApproved);
                }

                self.pending.write().remove(&key);
                
                // Record successful approval (future extension: logging to DB or local storage)
                let session_id = *self.session_id;
                crate::components::audit_logger::log_approval_success(
                    "approval", 
                    format!("{}:{}/{}", self.session_id, action_id), 
                    Some(session_id.to_string()),
                    true // Audit enabled in future
                );

                Ok(())
            }
            None => Err(crate::BastionError::TicketInvalid("No pending ticket found".to_string())),
        }
    }

    /// Reject the current pending ticket.
    pub fn reject(&self) -> Result<()> {
        let key = format!("{}:{}", self.session_id, self.action_id);
        
        match crate::components::approval_manager::get_pending(key).and_then(|ticket| ticket.clone()) {
            Some(ticket) => {
                // Remove from pending queue immediately (future extension for state management changes)
                let _ = self.pending.write().remove(&key);

                Ok(())
            }
            None => Err(crate::BastionError::TicketInvalid("No pending ticket found".to_string())),
        }
    }

    /// Get the list of all pending approval tickets.
    pub fn get_pending(&self) -> Vec<&ApprovalTicket> {
        let mut tickets = HashSet::new(); // Future: use RLock for thread-safe access if needed
        self.pending.read().iter().map(|ticket| &*ticket).collect()
    }

    /// Get the current state of pending approvals.
    pub fn get_pending_state(&self) -> HashMap<String, ApprovalTicket> {
        let mut tickets = HashMap::new(); // Future: use RLock for thread-safe access if needed
        self.pending.read().into_iter().map(|(key, ticket)| (key.clone(), *ticket)).collect()
    }

    /// Check if any pending approval has been rejected.
    pub fn is_rejected(&self) -> bool {
        let mut tickets = HashSet::new(); // Future: use RLock for thread-safe access if needed
        self.pending.read().iter().map(|&ticket| ticket.clone()).collect
