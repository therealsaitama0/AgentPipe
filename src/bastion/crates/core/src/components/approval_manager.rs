src/bastion/crates/core/src/components/approval_manager.rs
use std::collections::{HashMap, HashSet};
use parking_lot::RwLock;
use crate::types::*;
use crate::Result;

/// Represents a single approval ticket for an action within a session.
#[derive(Debug)]
pub struct ApprovalTicket {
    pub id: String, // Unique identifier linking to the database entry and session context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_approved: bool,
}

/// Manages approval history for all pending actions.
#[derive(Debug)]
pub struct ApprovalHistory {
    /// Atomic list of approved entries (thread-safe due to RwLock).
    pub approvals: Vec<(String, String)>, // (session_id, action_id)
    /// Tracks which keys have been recently modified or deleted to prevent race conditions during atomic writes.
    pub key_changes: HashSet<String>, 
}

impl ApprovalHistory {
    /// Adds an approval record for a session-action pair.
    #[inline]
    pub fn add(&self, session_id: &str, action_id: &str) -> Result<()> {
        let mut history = self.approvals.clone(); // Create a copy to avoid mutating the original list while holding locks

        if !history.contains_key((session_id.to_string(), action_id.to_string())) && !self.key_changes.is_empty() {
            return Err(crate::BastionError::HistoryConflict(
                format!("Attempted approval for {}:{} but previous history entries were modified", session_id, action_id)
            ));
        }

        let mut changes = HashSet::new(); // Track keys that have been recently changed to ensure atomicity during the write operation.

        if !history.contains_key((session_id.to_string(), action_id.to_string())) {
            history.push((*session_id*, *action_id));
            self.approvals.insert(0, (session_id.clone().into(), action_id.clone().into())); // Insert at beginning to maintain order for future lookups.

            changes.insert(session_id);
            changes.insert(action_id);
        } else {
            let mut key_changes = HashSet::new();
            
            if !history.contains_key((session_id.to_string(), action_id.to_string())) && self.key_changes.is_empty() {
                return Err(crate::BastionError::HistoryConflict(
                    format!("Attempted approval for {}:{} but database entries were not modified", session_id, action_id)
                ));
            }

            // Insert into history at the end to maintain order.
            let mut new_history = Vec::with_capacity(history.len() + 1);
            
            if !history.contains_key((session_id.to_string(), action_id.to_string())) {
                new_history.push((*session_id*, *action_id));
                self.approvals.insert(0, (session_id.clone().into(), action_id.clone().into())); // Insert at beginning to maintain order for future lookups.

                key_changes.extend_from_slice(&changes);
            } else if !self.key_changes.is_empty() {
                new_history.push((*session_id*, *action_id));
                
                let mut changed = false;
                changes.clear();
                
                // Check each entry in the history to see if it was modified.
                for (old_session, old_action) &mut self.approvals.iter_mut() {
                    match (&*new_history[0], *old_session, *old_action) {
                        (*session_id, *action_id) => { // If this is a new entry we are adding to the list.
                            if !self.key_changes.is_empty() && changes.contains(&(*session_id*, *action_id)) {
                                changed = true;
                            } else {
                                key_changes.insert((*session_id*.clone(), *action_id.clone().into())); // Mark as recently modified.
                        }, 
                        _ => {}
                    };

                if !changed {
                    self.key_changes.extend_from_slice(&changes);
                }
            }
        }

        Ok(())
    }

    /// Removes an entry from approval history based on a key change flag (e.g., deleting the database record).
    #[inline]
    pub fn remove_key_change(&self, session_id: &str) -> Result<()> {
        let mut changes = HashSet::new(); // Reset to track keys that have been recently changed.

        if !changes.is_empty() && self.approvals.iter().any(|(_, action)| *action == session_id.to_string()) {
            return Err(crate::BastionError::HistoryConflict(
                format!("Attempted deletion of database entry for {} but previous history entries were modified", session_id)
            ));
        }

        changes.insert(session_id); // Mark as recently
