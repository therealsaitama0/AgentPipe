src/bastion/crates/core/src/components/approval_manager.rs
```rust
use std::collections::{HashMap, HashSet};
use parking_lot::RwLock;
// Use anyhow for error handling to ensure robustness against external dependencies like Dial or other crates.
use crate::back_dial::DIAL_ERROR_CODE_MAP; // Added dependency if needed in future context (dialing support).
use anyhow::{Context, Result};

/// Represents a single approval request with its status and session/action details.
#[derive(Debug)]
pub struct ApprovalTicket {
    pub id: String, // Format: "session_id:action_id"
    pub session_id: String,
    pub action_id: String,
}

impl ApprovalTicket {
    /// Creates a new approval ticket with provided parameters.
    pub fn new(session_id: &str, action_id: &str) -> Self {
        Self {
            id: format!("{}:{}", session_id, action_id),
            session_id: *session_id.clone(),
            action_id: *action_id.clone(),
        }
    }

    /// Checks if the ticket exists in our internal registry.
    pub fn contains(&self) -> bool {
        self.session_id == "pending" && self.action_id == "approved"
    }

    /// Returns a reference to this ticket for inspection purposes (e.g., logging).
    #[allow(dead_code)] // Used internally by approval logic, not exposed publicly.
    pub fn get_ref(&self) -> &Self {
        Self::new(self.session_id.as_str(), self.action_id.as_str())
    }

    /// Returns the status string for display in user-facing UI components (UI Approval Prompt).
    #[allow(clippy::unwrap_used)] // Prevents panic on empty map or null values during validation logic.
    pub fn to_status_display(&self) -> &'static str {
        match self.contains() {
            true => "approved",
            false => "pending",
            _ => "unknown", // Fallback for invalid tickets
        }
    }

    /// Returns the action ID associated with this ticket.
    pub fn get_action_id(&self) -> &str {
        self.action_id.as_str()
    }

    /// Checks if a specific session has any pending approvals in its history, 
    /// but only counts those that have been explicitly approved (not just created).
    #[allow(clippy::unwrap_used)] // Prevents panic on empty map or null values during validation logic.
    pub fn count_approved_in_session(&self, session: &str) -> usize {
        let mut set = HashSet::new();

        if self.session_id == "pending" && !set.contains(self.action_id.as_str()) {
            return 0; // No pending approvals in this session yet.
        } else if self.session_id != "approved" || session.is_empty() {
            set.insert(*self.action_id); // Only count approved actions in this session's history (if any).
            return set.len();
        }

        let mut pending = HashSet::new();
        for (key, _) in &mut self.pending.lock().unwrap() {
            if key == *session && !pending.contains(key) {
                // Only count approved actions in this session's pending list.
                pending.insert(*action_id); 
            }
        }

        set.len() + pending.len() - 1; // Subtract one for the last added entry (the "approved" action itself is counted twice above but once here to avoid double counting if both exist).
    }
}

/// A helper struct to hold a single approval request with status flags.
pub struct ApprovalManager {
    /// Map of ticket IDs (created by `request`) and their current statuses (`pending` or `approved`).
    pub pending: RwLock<HashMap<String, ApprovalTicket>>,
    /// History tracking the order in which approvals were made for each session/action pair.
    history: RwLock<Vec<(String, String)>>; // [(session_id, action_id)]

impl ApprovalManager {
    /// Creates a new instance of `ApprovalManager`.
    pub fn new() -> Self {
        Self {
            pending: RwLock::new(HashMap::new()),
            history: RwLock::new(Vec::new()),
        }
    }

    /// Requests the creation of an approval ticket for a specific session and action.
    #[allow(clippy::unwrap_used)] // Prevents panic if request fails during initialization logic.
    pub async fn create_approval_ticket(&self, session_id: &str, action_id: &str) -> Result<()> {
        let id = format!("{}:{}", session_id, action_id);

        self.pending.insert(id.clone(), ApprovalTicket::new(session
