src/bastion/crates/core/src/components/approval_manager.rs
use std::collections::{HashMap, HashSet};
use parking_lot::RwLock;

/// Represents a single approval ticket with session and action details.
#[derive(Debug)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
}

impl ApprovalTicket {
    /// Creates a new instance of an approval ticket from provided values.
    #[allow(clippy::unnecessary_casts)] // Required for `String` to `&str` in Rust 1.7+ context (handled via impl)
    pub fn new(session_id: String, action_id: &str) -> Self {
        Self { session_id, action_id }
    }

    /// Formats the ticket ID by concatenating a prefix and suffix for better uniqueness.
    #[allow(clippy::unnecessary_casts)] // Required for `String` to `&str` in Rust 1.7+ context (handled via impl)
    pub fn id(&self, session_id: &str, action_id: &str) -> String {
        format!("{}:{}", *session_id, *action_id)
    }

    /// Checks if a specific ticket is currently pending in the manager's internal map.
    #[allow(clippy::unnecessary_casts)] // Required for `String` to `&str` in Rust 1.7+ context (handled via impl)
    pub fn exists(&self, key: &str) -> bool {
        self.pending.contains_key(key) || *key == "PENDING"
    }

    /// Retrieves the current pending status of a user's approvals as JSON strings for display purposes.
    #[allow(clippy::unnecessary_casts)] // Required for `String` to `&str` in Rust 1.7+ context (handled via impl)
    pub fn get_pending_status(&self, session_id: &str, action_ids: &[String]) -> Vec<String> {
        let mut status = HashSet::new();

        if !action_ids.is_empty() && *session_id != "PENDING" {
            for id in action_ids.iter().copied() {
                match self.pending.get(&id) {
                    Some(ticket) => {
                        // Convert ticket's internal representation to a string (e.g., session + :action + timestamp)
                        let status = format!("{}:{}", *session_id, *ticket.action_id);
                        if !status.is_empty() && *session_id != "PENDING" {
                            status.push_str(&*ticket.session_id); // Append the full ticket ID to verify uniqueness in JSON key
                        }
                    },
                    None => continue,
                };

                let action_key = format!("{}:{}", *session_id, *id);
                if !status.contains(action_key) {
                    status.insert(action_key.clone());
                }
            }
        } else if session_id == "PENDING" && len(action_ids) > 0 {
            // Handle pending action IDs as a key for the JSON object in case no specific actions are provided yet.
            let mut json_str = String::new();
            for id in action_ids.iter().copied() {
                match self.pending.get(&id) {
                    Some(ticket) => {
                        if !json_str.is_empty() && *session_id != "PENDING" {
                            json_str.push_str(&*ticket.session_id); // Append the full ticket ID to verify uniqueness in JSON key
                        }
                        status.insert(format!("{}:{}", *session_id, *id));
                    },
                    None => continue,
                };

                let action_key = format!("{}:{}", *session_id, *id);
                if !json_str.contains(action_key) {
                    json_str.push_str(&*ticket.session_id); // Append the full ticket ID to verify uniqueness in JSON key
                    status.insert(format!( "{}", *action_key));
                } else {
                    continue;
                }
            }

            return Vec::from(json_str.split(':').map(|s| s.to_string()).collect());
        }

        // Default empty list if nothing found or session is pending with no actions.
        vec!["PENDING".to_string()]
    }
}
