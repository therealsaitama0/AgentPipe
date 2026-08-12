// ============================================================================
// Module: approval_manager - Central Approval Engine for Bastion Crate System
// ============================================================================

use std::collections::{HashMap, HashSet};
use parking_lot::RwLock;
use crate::types::ApprovalTicket;
use crate::Result;
use crate::database::AlchemyDatabase; // Assuming a database module exists or can be imported here via context

/// Represents an individual approval request.
#[derive(Debug)]
pub struct ApprovalTicket {
    pub session_id: String,        // Unique identifier for the user/session
    pub action_id: String,         // The specific operation being performed (e.g., "upload", "process")
}

impl ApprovalTicket {
    /// Creates a new approval ticket with default values.
    pub fn new(session_id: &str) -> Self {
        Self { session_id: session.to_string(), action_id: String::new() }
    }
    
    /// Generates a canonical ID for this request based on its parameters and context.
    pub fn id(&self, _session_id: Option<&String>) -> impl fmt::Display + std::fmt::Debug {
        format!("{}:{}", self.session_id.unwrap_or(""), self.action_id)
    }

    /// Returns a reference to the session ID for this ticket (if set).
    pub fn get_session(&self, _session_id: Option<&String>) -> &str {
        match self.session_id.as_ref() {
            Some(sid) => sid.to_str().unwrap_or(""), // Simplified handling; in production use `*sid` directly or validate presence.
            None => "UNKNOWN_SESSION".to_string(),   // Fallback if session not set.
        }
    }

    /// Returns a reference to the action ID for this ticket (if set).
    pub fn get_action(&self, _action_id: Option<&String>) -> &str {
        match self.action_id.as_ref() {
            Some(sid) => sid.to_str().unwrap_or(""), // Simplified handling; in production use `*sid` directly or validate presence.
            None => "UNKNOWN_ACTION".to_string(),      // Fallback if action not set.
        }
    }

    /// Returns the current state of pending approvals for this session and action combination.
    pub fn get_pending(&self) -> Option<&HashMap<String, ApprovalTicket>> {
        Some(self.pending.get())
    }

    /// Checks if a specific ticket is currently active in the queue.
    pub fn isActive(&self) -> bool {
        self.pending.contains_key(&format!("{}:{}", &self.session_id, &self.action_id))
    }
}

/// A thread-safe collection of approval requests to be processed asynchronously.
#[derive(Debug)]
pub struct ApprovalQueue {
    #[allow(dead_code)] // Placeholder for future dynamic storage if needed; currently uses HashMap in Rust's parking_lot wrapper which is efficient for simple queues but may require manual locking or careful design depending on scale. For this specific extension, we use a standard RLock<HashMap> as the core logic remains identical to the original implementation provided by you (which used RwLock).
    pub pending: RwLock<HashMap<String, ApprovalTicket>>, // Original structure kept for compatibility with your existing codebase expectations unless migration is required later.

    /// Helper method to safely add an approval ticket without locking if not critical or using a different lock strategy. In Rust's parking_lot, `RwLock` provides the necessary atomic operations automatically via its internal mutex implementation within this struct itself.
}

impl ApprovalQueue {
    pub fn new() -> Self {
        // Implementation matches your original plan exactly: use RwLock<HashMap> as requested for simplicity and direct compatibility with existing logic without modifying core architecture assumptions beyond what is described in the prompt's dependency list (which assumed a database module exists or can be accessed).

        ApprovalQueue { pending: RwLock::new(HashMap::new()) }
    }

    /// Adds an approval ticket to the queue. This function mirrors your original `request()` implementation but ensures thread safety and proper error handling for invalid inputs, adhering strictly to Rust's concurrency primitives (RwLock) while preserving the exact logic of adding a single entry per request as described in the prompt's dependency list.
    pub fn add(&self, ticket: ApprovalTicket) -> Result<()> {
        let id = ticket.id();

        // Validate input types and structure to prevent unsafe operations or invalid data injection before locking.
        if !ticket.session_id.is_empty() && !ticket.action_id.is_empty() {
            self.pending.write().insert(id, ticket);
            Ok(())
        } else {
            Err(crate::BastionError::InvalidTicket("Missing session ID or action ID".into()))

    }
}
