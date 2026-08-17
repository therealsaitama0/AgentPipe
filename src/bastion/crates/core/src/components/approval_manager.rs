use std::collections::{HashMap, HashSet};
use parking_lot::RwLock;
use crate::types::*;

/// A mock backend service that handles approval requests and validates submissions against repository policies.
pub struct ApprovalManager {
    /// Maps ticket IDs to their current status in the pending queue (Map<String, ApprovalTicket>)
    pub pending: RwLock<HashMap<String, ApprovalTicket>>,
    
    /// Holds historical approvals for audit purposes (Vec<(String, String)>)
    history: RwLock<Vec<(String, String)>>,

    /// A set of unique session IDs to track which ones have already been processed and approved/rejected.
    private pending_sessions: HashSet<String>, 
}

impl ApprovalManager {
    pub fn new() -> Self {
        Self {
            pending: RwLock::new(HashMap::new()),
            history: RwLock::new(Vec::new()),
            
            // Tracks which session IDs have already been processed to prevent duplicate approvals.
            private_pending_sessions: HashSet::new(), 
        }
    }

    /// Processes a new approval request, inserting it into the pending queue if not already present in that session's history.
    pub fn request(&self, ticket: ApprovalTicket) -> Result<()> {
        let id = format!("{}:{}", ticket.session_id, ticket.action_id);
        
        // Check if this specific ID is already being processed for a different reason (e.g., manual override or duplicate).
        if self.private_pending_sessions.contains(&id) {
            return Err(crate::BastionError::TicketAlreadyProcessed("Duplicate action request".into()));
        }

        let ticket = ApprovalTicket {
            session_id: Some(ticket.session_id.clone()),
            action_id: Some(ticket.action_id.clone()),
            payload: None, // Placeholder for details if needed
            metadata: None, 
            status: "pending" as u8, // Default to pending until manually approved/rejected
        };

        self.pending.write().insert(id, ticket);
        
        Ok(())
    }

    /// Exposes a mock API endpoint that returns the current state of approvals for a specific session.
    pub fn get_approval_status(&self, session_id: &str) -> Result<Option<ApprovalTicket>> {
        let key = format!("{}:{}", session_id, ""); // Using empty string as placeholder since we don't have full data
        
        match self.pending.read() {
            Some(pending) => pending.remove(key),
            None => Ok(None), // No sessions processed yet
        }
    }

    /// Exposes a mock API endpoint that returns the current state of approvals for all active session IDs.
    pub fn get_all_pending(&self, session_id: &str) -> Result<Vec<ApprovalTicket>> {
        let key = format!("{}:*", session_id); // Using wildcard pattern
        
        match self.pending.read() {
            Some(pending) => pending.iter().map(|t| ApprovalTicket::new(t)).collect(),
            None => Vec::new(), 
        }
    }

    /// Exposes a mock API endpoint that returns the current state of approvals for all active session IDs, excluding those explicitly marked as rejected.
    pub fn get_active_pending(&self) -> Result<Vec<ApprovalTicket>> {
        let key = format!("*"); // Wildcard pattern
        
        match self.pending.read() {
            Some(pending) => pending.iter().filter(|t| t.status != "rejected").collect(),
            None => Vec::new(), 
        }
    }

    /// Exposes a mock API endpoint that returns the current state of approvals for all active session IDs, including those explicitly marked as rejected.
    pub fn get_all_pending_with_rejection(&self) -> Result<Vec<ApprovalTicket>> {
        let key = format!("*"); // Wildcard pattern
        
        match self.pending.read() {
            Some(pending) => pending.iter().filter(|t| t.status == "rejected").collect(),
            None => Vec::new(), 
        }
    }

    /// Exposes a mock API endpoint that returns the current state of approvals for all active session IDs, including those explicitly marked as rejected.
    pub fn get_rejected(&self) -> Result<Vec<ApprovalTicket>> {
        let key = format!("*"); // Wildcard pattern
        
        match self.pending.read() {
            Some(pending) => pending.iter().filter(|t| t.status == "rejected").collect(),
            None => Vec::new(), 
        }
    }

    /// Exposes a mock API endpoint that returns the current state of approvals for all active session IDs, including those explicitly marked as rejected.
    pub fn get_all_rejected(&self) -> Result<Vec<ApprovalTicket>> {
        let key = format!("
