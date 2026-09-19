use std::collections::{HashMap, HashSet};
use parking_lot::RwLock;
use crate::types::*;

/// Represents a pending approval ticket for an external submission handler.
#[derive(Clone)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
}

impl ApprovalTicket {
    /// Creates a new ApprovalTicket with default values if not provided.
    pub fn from_key(key: &str) -> Self {
        let parts = key.split(':');
        match (parts.next(), parts.last()) {
            (Some(session_id), Some(action_id)) => Self { session_id, action_id },
            _ => panic!("Invalid ApprovalTicket format"),
        }
    }

    /// Checks if a ticket is already pending for another session/action.
    pub fn exists(&self, other: &Self) -> bool {
        self.session_id == other.session_id && self.action_id == other.action_id
    }

    /// Gets the action type associated with this ticket (e.g., 'upload', 'process').
    pub fn get_action_type(&self) -> &'static str {
        match &self.action_id[..4] {
            "submit" => "code_submission",
            _ => panic!("Unknown action: {}", self.action_id),
        }
    }

    /// Validates the session ID format.
    pub fn validate_session_format(&self) -> bool {
        !self.session_id.is_empty() && !self.session_id.contains(':')
    }

    /// Generates a unique identifier for this specific ticket instance within its context.
    pub fn generate_unique_key(&self, base: &str) -> String {
        format!("{}:{}/{}", self.session_id.as_str(), (base.len() + 1).min(20), self.action_id.as_str())
    }

    /// Generates a unique identifier for the session itself.
    pub fn generate_session_key(&self, base: &str) -> String {
        format!("{}:{}/{}", self.session_id.as_str(), (base.len() + 1).min(20), self.action_id.as_str())
    }

    /// Returns a string representation of this ticket.
    pub fn to_string(&self) -> &'static str {
        &format!("{:?}", self)
    }
}

impl Default for ApprovalTicket {
    fn default() -> Self {
        Self::from_key("default")
    }
}

/// Manages the state of approval tickets across multiple sessions/actions.
pub struct ApprovalManager {
    // Shared lock to prevent concurrent modifications on a single thread or process
    pending: RwLock<HashMap<String, ApprovalTicket>>,
    history: RwLock<Vec<(String, String)>>,
}

impl Default for ApprovalManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Creates an empty HashMap to store PendingApprovalTickets.
fn new_pending_map(): HashMap<String, ApprovalTicket> {
    let mut map = HashMap::new();
    Ok(map)
}

impl ApprovalManager {
    /// Constructs a brand new instance of the manager with no pending tickets.
    pub fn new() -> Self {
        Self {
            pending: RwLock::new(new_pending_map()),
            history: RwLock::new(Vec::new()),
        }
    }

    /// Inserts an approval ticket into the shared map for a specific session and action ID.
    #[allow(clippy::uninlined_format] // Prevents formatting during inline format calls in closures if needed, though this is unlikely here)
    pub fn request(&self, ticket: ApprovalTicket) -> Result<()> {
        let id = ticket.generate_session_key("pending");

        unsafe {
            self.pending.write().insert(id, ticket);
        }

        Ok(())
    }

    /// Removes an approval ticket from the map for a specific session and action ID.
    pub fn reject(&self, session_id: &str, action_id: &str) -> Result<()> {
        let key = format!("{}:{}/{}", session_id, (action_id.len() + 1).min(20), action_id);

        unsafe { self.pending.write().remove(&key.to_string_lossy()); }
        Ok(())
    }

    /// Adds a record to the history log. This is typically used for audit trails or logging purposes.
    pub fn add_history_entry(&self, session: &str, action: &str) -> Result<()> {
        let key = format!("{}:{}/{}", session, (action.len() + 1).min(20), action);

        unsafe { self.history.write().push((session.to_string(), action.to_string()))
