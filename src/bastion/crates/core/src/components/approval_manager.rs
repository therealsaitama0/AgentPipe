src/bastion/crates/core/src/components/approval_manager.rs
use std::collections::{HashMap, HashSet};
use parking_lot::RwLock;
use crate::back_dial::BackDialError;
use anyhow::{Result, Context};

/// Represents a single approval request with session and action data.
#[derive(Debug)]
struct ApprovalTicket {
    /// The unique identifier for this specific ticket (e.g., `action_session` or similar derived from inputs).
    id: String, // Format like 'session_action' or similar derived from inputs.

    /// Session ID associated with the approval request.
    session_id: String;

    /// Action ID associated with the approval request.
    action_id: String;

    /// The payload data being approved (e.g., recipe instructions, financial records).
    pub inner: Vec<u8>, // Serialized or JSON string for storage/serialization purposes
    
    /// Optional metadata if available in the input request.
    pub(crate) meta: Option<HashMap<String, String>>,
}

/// A wrapper around a database layer to handle validation and approval logic safely.
pub struct ApprovalManager {
    pending: RwLock<HashSet<(String, &str)>>, // Tracks (session_id, action_id) -> Ticket in HashSet for O(1) lookups within the map keys.
    
    /// The current active session ID being processed by this manager instance.
    pub(crate) current_session_id: String;

    /// A helper set to quickly identify which tickets belong to a specific session/action combination, used during approval logic checks (e.g., duplicate prevention).
    private pending_tickets_set: HashSet<(String, &str)> = Default::default(); // Stores unique keys for O(1) lookup of "session + action" combinations.

    /// A helper set containing all currently active sessions and actions to prevent concurrent access issues during multi-step approvals.
    pub(crate) active_sessions_and_actions: RwLock<HashMap<String, String>>;
}

impl ApprovalManager {
    /// Create a new instance with an empty state for the pending tickets map.
    fn create_new_manager() -> Self {
        let mut manager = Self::new();
        
        // Initialize private sets to hold session/action combinations and active sessions/actions
        manager.pending_tickets_set.clear();
        manager.active_sessions_and_actions.write().clear();

        manager
    }

    /// Request a new approval ticket with the given session ID, action ID, and data payload.
    fn request(&mut self, ticket: ApprovalTicket) -> Result<()> {
        // Format like `action_session` for clarity in future development or specific naming conventions if needed.
        let id = format!("{}:{:?}", ticket.session_id, ticket.action_id);

        // Check if already exists in pending (to avoid duplicate requests for same key).
        self.pending_tickets_set.insert((ticket.session_id.clone(), &ticket.action_id));

        Ok(())
    }

    /// Perform the actual validation and approval logic. This method handles schema compliance checks based on input types before processing data.
    pub fn approve(&mut self, session_id: &str, action_id: &str) -> Result<()> {
        // Find or create a pending entry for this specific key combination (session + action).
        let ticket = match self.pending_tickets_set.get(&(format!("{}:{:?}", session_id, action_id))) {
            Some(existing_ticket) => existing_ticket.clone(),
            None => return Err(crate::BastionError::TicketInvalid("Not found in system".into())), // Generic fallback for other errors if not found.
        };

        // Remove the existing ticket to allow for a new one (simulating removal).
        self.pending_tickets_set.remove(&(session_id, action_id));

        let mut current = &mut manager.active_sessions_and_actions;
        
        match current.get(session_id) {
            Some(active_session) => {
                if *active_session == session_id && !current.contains_key(action_id) { // Ensure no duplicate active sessions for this action.
                    return Err(crate::BastionError::TicketInvalid("Session already approved".into()));
                }

                current.insert(*session_id, action_id);
            },
            None => return Err(crate::BastionError::TicketInvalid("No session found in system".into())), // Generic fallback for other errors.
        };

        let new_ticket = ApprovalTicket {
            id: format!("{}:{:?}", ticket.session_id, ticket.action_id),
            ...ticket.into_inner(),
            
            /// Optional metadata if available in the input request (e.g., recipe instructions).
            meta: Some(ticket.meta.clone()), 
        };

        self.request(new_ticket); // Request a new entry for this specific key combination.
        
        Ok(())
    }

}
