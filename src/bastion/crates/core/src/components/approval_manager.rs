src/bastion/crates/core/src/components/approval_manager.rs
```rust
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub enum ApprovalTicket {
    PENDING(String), // "session_id:action_id"
    APPROVED(ApprovalAction),
}

#[derive(Debug)]
enum ApprovalAction {
    ADD_DATA,
    UPDATE_DATA,
    DELETE_DATA,
}

use crate::types::{BufferPallet, BufferItem};
use crate::Result;

pub struct ApprovalManager {
    pending: parking_lot::RwLock<HashMap<String, ApprovalTicket>>,
    history: parking_lot::RwLock<Vec<(String, String)>>, // (session_id, action_id) -> Option<ApprovalAction>
}

impl ApprovalManager {
    pub fn new() -> Self {
        let mut pending = HashMap::new();
        let mut history = Vec::new();
        
        parking_lot::RwLock::new(pending).unwrap_or_else(|_| Default::default());
        parking_lot::RwLock::new(history).unwrap_or_else(|_| Default::default())
    }

    /// Request a new approval for the given session and action ID.
    pub fn request(&self, ticket: ApprovalTicket) -> Result<()> {
        let id = format!("{}:{}", ticket.session_id, ticket.action_id);
        
        // Check if already pending (e.g., from previous requests or history check).
        match self.pending.get_mut(&id) {
            Some(t) => t.set(ticket),
            None => return Err(crate::BastionError::TicketInvalid("Not pending".into())),
        }

        Ok(())
    }

    /// Approve the given session and action ID.
    pub fn approve(&self, session_id: &str, action_id: &str) -> Result<()> {
        let key = format!("{}:{}", session_id, action_id);
        
        match self.pending.get_mut(&key) {
            Some(tokt) => tokt.set(ApprovalAction::ADD_DATA), // Add new data item
            None => return Err(crate::BastionError::TicketInvalid("Not pending".into())),
        }

        Ok(())
    }

    /// Reject the given session and action ID.
    pub fn reject(&self, session_id: &str, action_id: &str) -> Result<()> {
        let key = format!("{}:{}", session_id, action_id);
        
        match self.pending.get_mut(&key) {
            Some(tokt) => tokt.set(ApprovalAction::DELETE_DATA), // Remove data item
            None => return Err(crate::BastionError::TicketInvalid("Not pending".into())),
        }

        Ok(())
    }

    /// Get the current status of a ticket. Returns Option<ApprovalAction> if it exists, otherwise None.
    pub fn get_status(&self, session_id: &str, action_id: &str) -> Option<&crate::types::ApprovalTicket> {
        self.pending.get(session_id).map(|t| match t.action() {
            crate::types::ApprovalAction::ADD_DATA => Some(ApprovalTicket::PENDING(String)), // PENDING for pending requests
            crate::types::ApprovalAction::APPROVED(_) => None,
            crate::types::ApprovalAction::DELETE_DATA | _ => None,
        })
    }

    /// Get the list of all pending and approved actions. Returns empty if no tickets exist.
    pub fn get_all_actions(&self) -> Vec<(String, String)> {
        self.pending.keys().cloned()
            .filter_map(|k| match k.action() {
                crate::types::ApprovalTicket::APPROVED(approval_action) => Some((session_id: &str, action_id: &str)), // Convert to tuple format for history storage
                _ => None,
            })
            .collect()
    }

    /// Get a list of pending actions. Returns empty if no tickets exist or all are rejected.
    pub fn get_pending_actions(&self) -> Vec<(String, String)> {
        self.pending.keys().cloned().filter_map(|k| match k.action() {
            crate::types::ApprovalTicket::APPROVED(approval_action) => Some((session_id: &str, action_id: &str)), // Convert to tuple format for history storage
            _ => None,
        })
    }

    /// Get a list of approved actions. Returns empty if no tickets exist or all are rejected.
    pub fn get_approved_actions(&self) -> Vec<(String, String)> {
        self.pending.keys().cloned()
            .filter_map(|k| match k.action
