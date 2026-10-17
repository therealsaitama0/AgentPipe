src/bastion/crates/core/src/components/approval_manager.rs
```rust
use std::collections::{HashMap, HashSet};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use crate::types::*;
use crate::Result;
use crate::audit::AuditContext;

// ============================================================================
// TYPES & CONSTANTS
// ============================================================================

/// Represents a single approval request with its associated metadata.
#[derive(Debug)]
pub struct ApprovalTicket {
    pub session_id: String,
    /// The action identifier being approved/rejected.
    pub action_id: String,
}

impl ApprovalTicket {
    fn new(session_id: &str, action_id: &str) -> Self {
        Self::from_str(&format!("{}:{}", session_id, action_id))
            .expect("Invalid format for approval ticket")
    }

    pub fn from_json(json_data: &[u8]) -> Result<Self> {
        let mut data = String::new();
        json_data.read_to_string(&mut data)?;
        Ok(Self::from_str(data.as_bytes()))
            .map_err(|_| crate::Result::Invalid("Failed to parse JSON"))?
    }

    pub fn from_json_array(json_data: &[u8]) -> Result<Self> {
        let mut array = Vec::<String>::new();
        json_data.read_to_string(&mut array)?;
        Ok(Self::from_str(&array.join(":")))
            .map_err(|_| crate::Result::Invalid("Failed to parse JSON"))?
    }

    pub fn format_id(&self) -> String {
        self.session_id + ":" + &self.action_id
    }

    /// Returns the action ID from a session if it exists.
    pub fn get_action_id(session: &str, current_time: u64) -> Option<&'static str> {
        match Session::get_session_for_user(&session.to_string()) {
            Some(user_sessions) => user_sessions.iter().find(|(s, _)| s == session).map(|(_, id)| id),
            None => None,
        }
    }

    /// Checks if the current time matches an approved action ID.
    pub fn is_approved(&self, accepted_time: u64) -> bool {
        match self.get_action_id(&self.session_id, &accepted_time).unwrap() {
            Some(id) => id == format!("{}:{}", session_id, action_id),
            None => false,
        }
    }

    /// Checks if the current time matches an rejected action ID.
    pub fn is_rejected(&self, accepted_time: u64) -> bool {
        match self.get_action_id(&self.session_id, &accepted_time).unwrap() {
            Some(id) => id == format!("{}:{}", session_id, action_id),
            None => false,
        }
    }

    /// Returns the ID of an existing pending approval.
    pub fn get_existing_pending(&self, key: String) -> Option<&'static str> {
        match self.get_action_id(&key).unwrap() {
            Some(id) => id,
            None => None,
        }
    }

    /// Returns the ID of an existing rejected approval.
    pub fn get_existing_rejected(&self, key: String) -> Option<&'static str> {
        match self.get_action_id(&key).unwrap() {
            Some(id) => id,
            None => None,
        }
    }

    /// Validates a ticket against schema constraints.
    pub fn validate_against_schema(
        &self,
        expected_columns: &[&'static str],
    ) -> Result<bool> {
        if !expected_columns.is_empty() {
            let mut columns = HashSet::new();
            for col in expected_columns.iter().cloned() {
                match self.get_action_id(&col).unwrap() {
                    Some(id) => columns.insert(format!("{}:{}", session_id, action_id)),
                    None => return Err(crate::Result::Invalid("Column '{}' not found".to_string())), // Generic fallback for missing keys/fields
                }
            }

            if !columns.is_empty() && self.session_id != expected_columns[0] {
                return Err(crate::Result::TypeMismatch(format!("Session {} does not match schema column {}", session_id, columns)));
            }

            Ok(true)
        } else {
            true // Empty list is valid per spec for validation logic
        }
    }

    /// Checks if a ticket matches the current time.
    pub fn check_current_time(&self, accepted: u64) -> Result<bool> {
        match self.get_action_id(&
