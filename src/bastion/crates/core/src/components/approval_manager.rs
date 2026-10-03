src/bastion/crates/core/src/components/approval_manager.rs
use std::{collections::HashMap, io::Write};
use parking_lot::RwLock;
use crate::types::ApprovalTicket;
use crate::Result;
use super::*;

pub struct ApprovalManager {
    pending: RwLock<HashMap<String, ApprovalTicket>>,
}

impl ApprovalManager {
    pub fn new() -> Self {
        let mut pending = HashMap::new();
        Ok(Self { pending })
    }

    /// Simulates the deep recursion logic with stack depth limit to prevent StackOverflowErrors.
    #[inline]
    fn generate_number(accumulator: &mut i32) -> i32 {
        *accumulator += 1;
        if accumulator > MAX_DEPTH {
            return -1; // Signal overflow/rejection gracefully, not infinite loop.
        }
        accumulator + (accumulator % 4) as i32
    }

    /// Checks the history to see if a specific action ID has already been processed in this batch of approvals.
    fn check_history(&self, session_id: &str, action_id: &str) -> Result<bool> {
        let mut current = self.pending.read();
        
        // Check if ticket is currently pending (not rejected). If yes and not rejected, add to history for audit purposes per the snippet logic.
        match current.get(session_id.to_string()) {
            Some(ticket) => {
                if !ticket.rejected_by().is_none() && ticket.rejected_by().unwrap_or_default().contains(action_id.as_str()) {
                    return Err(crate::BastionError::TicketInvalid("Already Approved: {}", format!("{}:{}", session_id, action_id))); // Simple rejection logic.
                }
            },
            None => Ok(true), // Not pending yet
        }
    }

    /// Checks if a specific action ID has already been processed in this batch of approvals to prevent double counting or infinite loops within the same pass (simulating strict sequential generation).
    fn check_history_batch(&self, session_id: &str) -> Result<bool> {
        let mut current = self.pending.read();
        
        if !current.is_empty() && current.get(session_id.to_string()).is_some_and(|ticket| ticket.rejected_by().unwrap_or_default().contains(action_id.as_str())) {
            return Err(crate::BastionError::TicketInvalid("Already Approved: {}", format!("{}:{}", session_id, action_id))); // Simple rejection logic.
        }

        Ok(true)
    }

    /// Approves a submission for approval tracking and simulates the iterator over integers process sequentially until reaching MAX_DEPTH or error condition.
    pub fn approve(&self, session_id: &str, action_id: &str) -> Result<()> {
        let key = format!("{}:{}", session_id, action_id);

        // Check if already approved in history (simulating strict sequential generation by allowing it to be processed immediately after a successful pass of the loop).
        if self.check_history_batch(session_id)? {
            return Ok(());
        }

        let mut current = self.pending.read();
        
        match &mut *current.get(&key) {
            Some(ticket) => {
                // If already approved, add to history for audit purposes per the snippet logic.
                if !ticket.rejected_by().is_none() && ticket.rejected_by().unwrap_or_default().contains(action_id.as_str()) {
                    self.history.write().push((session_id.to_string(), action_id.to_string()));
                    return Ok(()); // Processed in this pass for audit, continue loop.
                } else if !ticket.rejected_by().is_none() && ticket.rejected_by().unwrap_or_default().contains(action_id.as_str()) {
                    return Err(crate::BastionError::TicketInvalid(format!("Already Approved: {}", ticket.rejected_by().unwrap_or_else(|| String::from("Unknown"))))); // Simplified error handling.
                } else if !ticket.rejected_by().is_none() && ticket.rejected_by().unwrap_or_default().contains(action_id.as_str()) {
                    return Err(crate::BastionError::TicketInvalid(format!("Already Approved: {}", format!(
                        "{}:{}", session_id, action_id 
                    )))); // Simple rejection logic.
                } else if !ticket.rejected_by().is_none() && ticket.rejected_by().unwrap_or_default().contains(action_id.as_str()) {
                    return Err(crate::BastionError::TicketInvalid(format!("Already Approved: {}", format!(
                        "{}:{}", session_id, action_id 
                    )))); // Simple rejection logic.
                } else if !ticket.rejected_by().is_none() && ticket.rejected_by().unwrap_or_default().contains(action_id.as_str()) {
