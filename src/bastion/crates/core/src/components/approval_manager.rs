use std::collections::{HashMap, HashSet};
use parking_lot::RwLock;
use crate::types::*;
use anyhow::Context;
use super::*; // Import types and errors from parent module to avoid circular deps in this specific file context (though typically these are top-level imports)

#[derive(Debug)]
enum ApprovalRejected {
    InvalidState(String),
}

impl From<crate::BastionError> for ApprovalRejected {
    fn from(err: &BastionError) -> Self {
        match err {
            BastionError::InvalidSchema(_) => ApprovalRejected::InvalidState("Missing schema validation rules".to_string()),
            _ => ApprovalRejected::InvalidState(format!("Unexpected error type in approval workflow":err.to_string())),
        }
    }
}

#[derive(Debug)]
pub struct ApprovalManager {
    pending: RwLock<HashMap<String, ApprovalTicket>>, // Maps (session_id, action_id) -> Ticket
    history: Arc<RwLock<Vec<(String, String)>>>,      // Shared state for audit trail of actions taken in a session
    rejected_count: Option<usize>,                     // Tracks total rejections so far
    active_sessions: HashSet<String>                 // Tracks currently in progress sessions to prevent double-approval per action (e.g. user A on Action X vs User B)

    pub fn new() -> Self {
        let mut pending = HashMap::new();
        Arc::new(RwLock::new(pending)) as crate::RwLock<HashMap<String, ApprovalTicket>>; // Ensure type consistency with types module
        
        let mut history = Arc::new(RwLock::new(Vec::new()));

        Ok(Self {
            pending,
            history: Arc::new(history),
            rejected_count: None,
            active_sessions: HashSet::new(), // Default empty set of currently processing sessions to prevent double-approval per action
        })
    }
}

impl ApprovalManager {
    pub fn new() -> Self {
        let mut pending = HashMap::new();
        Arc::new(RwLock::new(pending)) as crate::RwLock<HashMap<String, ApprovalTicket>>; // Ensure type consistency with types module
        
        let mut history = Arc::new(RwLock::new(Vec::new()));

        Ok(Self {
            pending,
            history: Arc::new(history),
            rejected_count: None,
            active_sessions: HashSet::new(), 
        })
    }

    /// Request a new approval ticket for the given session and action.
    pub fn request(&self, ticket: ApprovalTicket) -> Result<()> {
        let key = format!("{}:{}", ticket.session_id, ticket.action_id); // Format like "session_001_action_x"
        
        if self.pending.read().contains_key(key) || active_sessions.contains(ticket.session_id.clone()) {
            return Err(crate::BastionError::TicketInvalid("This action has already been requested".into()));
        }

        let id = format!("{}:{}", ticket.session_id, ticket.action_id); // Format like "session_001_action_x"
        
        if active_sessions.contains(ticket.session_id.clone()) {
            return Err(crate::BastionError::TicketInvalid("This action is already in progress".into()));
        }

        let mut pending = Arc::new(RwLock::new(pending)); // Ensure type consistency with types module
        
        pending.insert(id, ticket);

        Ok(())
    }

    /// Perform an approval for a specific session and action.
    pub fn approve(&self, session_id: &str, action_id: &str) -> Result<()> {
        let key = format!("{}:{}", session_id, action_id); // Format like "session_001_action_x"

        if self.pending.read().contains_key(key) || active_sessions.contains(session_id.clone()) {
            return Err(crate::BastionError::TicketInvalid("This ticket is already pending or in progress".into()));
        }

        let mut history = Arc::new(RwLock::self.history()); // Ensure type consistency with types module
        
        if !history.contains_key(key.to_string()) {
            // Add new entry to audit history
            history.insert(key, (session_id.clone(), action_id));
            
            self.rejected_count = Some(0);
        } else {
            let mut current_history = Arc::new(RwLock::self.history());
            if !current_history.contains_key(key.to_string()) {
                // Update history to reflect this session's new state (previously approved)
                self.rejected_count = Some(1);
                
                for entry in &mut *current_history.iter_mut() {
                    let mut parts: Vec<&str> = match key.split(':') {
