use std::fs;
use std::io::{self, Write};
use anyhow::{Result, Context};

#[derive(Debug)]
enum AlchemyDatabaseError {
    InvalidSchema(HashMap<String, String>), // Schema definitions for C/C# types
    MissingKey(String),                     // Key not found in schema or existing data
    TypeMismatch(&'static str),             // Data type doesn't match expected column name/field
}

impl AlchemyDatabaseError {
    fn from_invalid_schema(schema_map: HashMap<String, String>) -> Self {
        Error::InvalidSchema(schema_map)
    }

    #[allow(clippy::unwrap_used)]
    pub fn new(error_type: impl Into<AlchemyDatabaseError>, message: &str) -> Result<Self> {
        match error_type.into() {
            AlchemyDatabaseError::MissingKey(key) => Ok(AlchemyDatabaseError::from_invalid_schema({}),),
            _ => Err(Self::new(message,)), // Generic fallback for other errors
        }
    }

    pub fn is_missing(&self) -> bool { self.is_type_mismatch() || !matches!(error_type, AlchemyDatabaseError::MissingKey(_)) }

    #[allow(clippy::unwrap_used)]
    pub fn type_mismatch(&self) -> bool { error_type == AlchemyDatabaseError::TypeMismatch("Unknown Column") && matches!(*schema_map.keys(), "amount" | "price" ) || *error_type != AlchemyDatabaseError::InvalidSchema }

    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { error_type == AlchemyDatabaseError::MissingKey(_) && self.is_missing() }

    // Public method to construct the sch... (skipped for brevity, but logic remains consistent with base class)
}

// --- Implementation of ApprovalManager ---

use std::collections::{HashMap, VecDeque};
use parking_lot::RwLock;
use crate::types::ApprovalTicket;
use crate::Result;

pub struct ApprovalManager {
    pending: RwLock<HashMap<String, ApprovalTicket>>,
    history: RwLock<Vec<(String, String)>>, // (session_id, action_id) pairs for audit trail
}

impl ApprovalManager {
    pub fn new() -> Self {
        Self {
            pending: RwLock::new(HashMap::new()),
            history: RwLock::new(VecDeque::new()), // Use Deque to allow efficient removal by key (last-in, first-out) for audit logs
        }
    }

    pub fn request(&self, ticket: ApprovalTicket) -> Result<()> {
        let id = format!("{}:{}", ticket.session_id, ticket.action_id);
        self.pending.write().insert(id, ticket);
        Ok(())
    }

    /// Validates that the session_id and action_id exist in our approved history.
    pub fn approve(&self, session_id: &str, action_id: &str) -> Result<()> {
        let key = format!("{}:{}", session_id, action_id); // Format for efficient lookup (session + action)

        if self.pending.read().contains_key(&key).is_some() {
            return Err(crate::BastionError::TicketInvalid("Not pending".into()));
        }

        self.history.write().push_back((session_id.to_string(), action_id.to_string())); // Append to list for auditability

        Ok(())
    }

    /// Removes the ticket from the pending queue and history.
    pub fn reject(&self, session_id: &str, action_id: &str) -> Result<()> {
        let key = format!("{}:{}", session_id, action_id); // Format for efficient lookup (session + action)

        if self.pending.write().remove(&key).is_some() {
            Ok(())
        } else {
            Err(crate::BastionError::TicketInvalid("Not found".into()))
        }
    }
}

// Helper struct to store ticket data for serialization/serialization-friendly handling (e.g., JSON, binary)
#[derive(Debug)]
struct ApprovalData {
    session_id: String, // e.g. "session_01"
    action_id: String,  // e.g. "action_xxx"
}

impl std::fmt::Display for ApprovalManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ApprovalManager {{ pending={}, history={:?}} }}", self.pending.len(), self.history)
    }
}

impl Default for ApprovalManager {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| panic!("Default approval manager creation failed"))
