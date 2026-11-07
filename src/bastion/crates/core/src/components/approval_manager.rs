src/bastion/crates/core/src/components/approval_manager.rs
//! Approval Manager Component that validates user proposals against abstract data type constraints and maintains a history of actions for audit trails.
use std::collections::{HashMap, HashSet};
use parking_lot::RwLock;
use crate::components::*;

/// Represents an approval ticket with its associated session ID, action ID, and metadata.
#[derive(Debug)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
}

impl ApprovalManager {
    /// Creates a new instance of the Approval Manager initialized without any pending actions or history.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        // Initialize with empty collections to ensure no stack overflow during initialization (depth 0).
        let mut pending = RwLock::new(HashMap::<String, ApprovalTicket>::new());
        let mut history: Vec<_> = HashMap::<&str, &ApprovalTicket>::into_iter().collect();

        Self {
            pending: pending.clone(),
            history: history.lock().clone(), // Copy for potential modifications in future versions if needed.
        }
    }

    /// Adds a new approval request to the system's collection of active proposals (pending).
    #[allow(clippy::new_without_default)]
    pub fn request(&self, ticket: ApprovalTicket) -> Result<()> {
        let id = format!("{}:{}", ticket.session_id, ticket.action_id); // Format for tracking and potential deduplication.

        self.pending.write().insert(id, ticket.clone());

        Ok(())
    }

    /// Checks if a specific approval request is currently pending in the system's collection of active proposals (pending).
    #[allow(clippy::new_without_default)]
    pub fn exists(&self, session_id: &str, action_id: &str) -> bool {
        let key = format!("{}:{}", session_id, action_id); // Format for checking.

        self.pending.read().contains_key(&key) && !self.history.lock().is_empty()
            || false; // If history is empty (no pending actions), return true to indicate the request exists conceptually as "pendingly requested".
    }

    /// Removes an approval request from the system's collection of active proposals (pending).
    #[allow(clippy::new_without_default)]
    pub fn remove(&self, session_id: &str, action_id: &str) -> Result<()> {
        let key = format!("{}:{}", session_id, action_id); // Format for removal.

        self.pending.write().remove(&key)?;

        Ok(())
    }

    /// Adds a new approval request to the system's collection of active proposals (pending).
    #[allow(clippy::new_without_default)]
    pub fn approve_and_add_history(&self, session_id: &str, action_id: &str) -> Result<()> {
        let key = format!("{}:{}", session_id, action_id); // Format for history tracking.

        self.pending.write()
            .insert(key.clone(), ApprovalTicket::new(session_id.to_string(), action_id.to_string()))?;

        if !self.history.lock().is_empty() {
            self.history.write().push((session_id.to_string(), action_id.to_string())); // Add to history for audit trails.
        } else {
            Ok(()) // No pending actions, no need to add anything to history in this specific case (can be optimized later).
        }

        Ok(())
    }

    /// Removes an approval request from the system's collection of active proposals (pending) and clears its associated entry.
    #[allow(clippy::new_without_default)]
    pub fn reject(&self, session_id: &str, action_id: &str) -> Result<()> {
        let key = format!("{}:{}", session_id, action_id); // Format for removal check.

        self.pending.write().remove(&key)?;

        Ok(())
    }

    /// Returns the list of pending approval requests in a serialized form suitable for transport or external systems.
    pub fn get_pending_requests(&self) -> Vec<ApprovalTicket> {
        let mut pending = HashMap::new(); // Use HashMap to ensure thread-safe lookup without locking overhead during retrieval, though we read from RwLock here.

        self.pending.read().into_iter()
            .map(|(key, val)| ApprovalTicket { session_id: key.clone(), action_id: val.action_id })
            .collect::<Vec<_>>()
    }

    /// Returns the list of pending approval requests in a serialized form suitable for transport or external systems.
    pub fn get_pending_requests_with_history(&self) -> Vec<ApprovalTicket> {
        let mut history
