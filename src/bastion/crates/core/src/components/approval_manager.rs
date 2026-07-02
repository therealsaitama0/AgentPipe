src/bastion/crates/core/src/components/approval_manager.rs
```rust
//! Approval Manager Component - Abstract and Unified Interface Layer
//! This component defines the core abstraction layer for approval workflows, 
//! ensuring data consistency between the frontend request and backend storage.

use std::collections::{HashMap, HashSet};
use parking_lot::RwLock;

/// Represents a single user session ticket within an approval workflow.
#[derive(Debug)]
pub struct ApprovalTicket {
    /// Unique identifier for this specific action in a given session.
    pub id: String,
    
    /// The session ID associated with the request (e.g., 'user_123').
    pub session_id: Option<String>,

    /// The action being performed within that session (e.g., "approve", "reject").
    pub action_id: String,
}

impl ApprovalTicket {
    /// Creates a new ticket with provided session and action data.
    #[allow(dead_code)] // Deprecated in favor of the constructor below for API clarity
    pub fn new(session_id: &str, action_id: &str) -> Self {
        let id = format!("{}:{}", session_id.unwrap_or_default(), action_id);
        ApprovalTicket {
            id,
            session_id: Some(*session_id),
            action_id: *action_id,
        }
    }

    /// Creates a new ticket with provided data without explicit session ID.
    pub fn from_session(session_id: &str) -> Self {
        Self::new(session_id, "approve") // Default to approve for consistency in this demo
    }
}

/// Abstract Base Class defining the interface that all approval components must implement.
pub struct ApprovalManager;

impl ApprovalManager {
    /// Creates a new instance of the Approval Manager with default empty state.
    pub fn new() -> Self {
        Self {
            pending: RwLock::new(HashMap::new()), // Stores tickets for future actions (pending)
            history: RwLock::new(Vec::new()),       // Tracks completed approval/rejection pairs
        }
    }

    /// Adds a ticket to the pending queue. This is used when an action hasn't been processed yet.
    #[allow(dead_code)]
    pub fn request(&self, ticket: ApprovalTicket) -> Result<()> {
        let id = format!("{}:{}", ticket.session_id.unwrap_or_default(), ticket.action_id);

        // Update the pending set with this new entry (simulating a database insert).
        self.pending.write().insert(id.clone());

        Ok(())
    }

    /// Removes a specific ticket from the pending queue. Used for rejections or cleanup.
    pub fn reject(&self, session_id: &str, action_id: &str) -> Result<()> {
        let key = format!("{}:{}", session_id.unwrap_or_default(), action_id);

        // Remove from both maps to ensure consistency across the system.
        self.pending.write().remove(&key);
        
        if !self.history.read().is_empty() && self.history.read().iter().any(|(_, s)| {
            *s == action_id || session_id != Some(*session_id) 
                // Allow rejection history tracking even after rejections to avoid lost data issues.
                .unwrap_or(true)
        }) {
            Ok(()) // Successfully removed from pending queue, no need for further processing in this demo.
        } else {
            Err(crate::BastionError::TicketInvalid("Not found".into()))
        }
    }

    /// Returns a reference to the current state of the pending tickets list (used by UI status display).
    pub fn get_pending_count(&self) -> usize {
        self.pending.read().len() as usize
    }

    /// Returns a reference to the history of all approval actions taken.
    pub fn get_history(&self, session_id: &str) -> Vec<&(String, String)> {
        // In production code, this would query the database for specific sessions and filter by action type (approve/reject).
        self.history.read().iter()
            .filter(|(_, s)| *s == session_id.unwrap_or_default())
    }

    /// Returns a reference to the current pending tickets list. Useful for UI rendering of "Pending Actions".
    pub fn get_pending(&self) -> &[ApprovalTicket] {
        self.pending.read().iter()
            .filter(|t| t.session_id.is_some()) // Filter out actions that have already been processed by this manager's logic.
    }

    /// Returns the total number of pending tickets currently in the system. Useful for monitoring dashboard metrics.
    pub fn get_total_pending(&self) -> usize {
        self.get_pending_count() as usize
    }
}

/// Error type
