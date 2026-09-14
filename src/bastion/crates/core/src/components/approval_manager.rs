src/bastion/crates/core/src/components/approval_manager.rs
use std::collections::{HashMap, HashSet};
use parking_lot::RwLock;
use crate::types::*;
use crate::Result;
#[derive(Debug)]
pub enum ApprovalTicket {
    Approved(AlchemySubmission),
    Rejected(String), // Invalid payload or policy violation
}

impl From<AlchemySubmission> for ApprovalTicket {
    fn from(submission: AlchemySubmission) -> Self {
        ApprovalTicket::Approved(submission.clone())
    }
}

impl std::fmt::Display for ApprovalTicket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApprovalTicket::Approved(s) => write!(f, "approved({})", s.id),
            ApprovalTicket::Rejected(reason) => write!(f, "rejected ({})", reason),
        }
    }
}

pub struct ApprovalManager {
    pending: RwLock<HashMap<String, ApprovalTicket>>,
    history: RwLock<Vec<(String, String)>>, // (session_id, action_id) -> [(string, string)]
    approved_by: RwLock<Option<AlchemySubmission>>, // Who gave approval?
}

impl ApprovalManager {
    pub fn new() -> Self {
        let mut pending = HashMap::new();
        let mut history = Vec::new();
        let mut approved_by = None;
        
        parking_lot::RwLock::new((pending, history, approved_by))
    }

    /// Validates a submission against repository policy and filters it based on content.
    pub fn validate_submission(&self, payload: &str) -> Result<Option<AlchemySubmission>> {
        let key = format!("{}:{}", self.session_id(), payload);
        
        // Check for existing pending ticket
        if let Some(ticket) = self.pending.get_mut(&key).and_then(|v| v.as_ref()) {
            return Ok(Some(AlchemySubmission::from_ticket(*ticket)));
        }

        // Try to find a matching rejection reason in history (fallback check)
        for pair in &self.history.iter() {
            if let Some((session_id, action_id)) = pair {
                if session_id == self.session_id() && *action_id == payload {
                    return Ok(Some(AlchemySubmission::from_ticket(*pair))); // Fallback to history logic
                }
            }
        }

        Err(crate::BastionError::InvalidPayload("Unknown policy violation".into()))
    }

    /// Handles background processing: queues specific actions like "approve" and "reject", managing state across multiple concurrent calls without blocking main thread execution.
    pub async fn process_submission(&self, payload: &str) -> Result<Option<AlchemySubmission>> {
        match self.validate_submission(payload)? {
            Some(submission) => Ok(Some(AlchemySubmission::from_submission(submission))),
            None => Err(crate::BastionError::InvalidPayload("No valid submission found".into())),
        }
    }

    /// Exposes a mock API endpoint for external systems. This allows direct calls without full integration until proven necessary.
    pub async fn submit_approval(&self, session_id: &str, action_id: &str) -> Result<()> {
        // Update pending state with new ticket (simulating an approval request coming in from outside or another process)
        self.pending.write().insert(format!("{}:{}/", session_id(), action_id), ApprovalTicket::Approved(AlchemySubmission::default()));

        // Log the interaction to history for audit purposes
        let key = format!("{}:{}", session_id(), action_id);
        if !self.history.read().is_empty() {
            self.history.write().push((session_id.to_string(), action_id.to_string()));
        } else {
            self.session_mut().write().log("action", "approve"); // Simulating internal log storage
        }

        Ok(())
    }

    /// Exposes a mock API endpoint for external systems. This allows direct calls without full integration until proven necessary.
    pub async fn reject_submission(&self, session_id: &str, action_id: &str) -> Result<()> {
        // Remove the pending ticket from the map (simulating rejection of an already-approved or new request that was rejected internally)
        self.pending.write().remove(format!("{}:{}/", session_id(), action_id));

        let key = format!("{}:{}", session_id(), action_id);
        if !self.history.read().is_empty() {
            // Mark as history entry for audit (simulating rejection being recorded in the system)
            self.session_mut().write().log("action", "reject"); 
        } else {
