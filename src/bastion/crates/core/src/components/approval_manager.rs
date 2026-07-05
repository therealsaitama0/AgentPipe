use crate::types::{ApprovalTicket, ApprovalManager};
use parking_lot::RwLock;
use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
enum AlchemyDatabaseError {
    InvalidSchema(HashMap<String, String>), // Schema definitions for C/C# types
    MissingKey(String),                     // Key not found in schema or existing data
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
}

#[derive(Debug, Clone)]
struct ApprovalRequest {
    session_id: String,
    action_id: String,
}

impl ApprovalManager {
    /// Create a new instance of the approval manager.
    pub fn new() -> Self {
        let mut pending = RwLock::new(HashMap::<String, ApprovalTicket>::new());
        
        // A history tracker for audit purposes: [(session_id: String, action_id: String)].
        let mut history = RwLock::new(Vec<(String, String)>);

        Self { pending, history }
    }

    /// Request a new approval ticket. Adds it to the `pending` lock and returns ownership of the request object if successful.
    pub fn request(&self, ticket: ApprovalTicket) -> Result<()> {
        // Format unique identifier for this session/action pair within the system's internal cache.
        let id = format!("{}:{}", ticket.session_id, ticket.action_id);

        self.pending.write().insert(id, ticket.clone());
        
        Ok(())
    }

    /// Approve a pending approval request by matching it against its associated key string and updating history if found. Returns ownership of the approved action reference.
    pub fn approve(&self, session_id: &str, action_id: &str) -> Result<()> {
        let mut pending = self.pending.write();
        
        // Retrieve or create a specific ticket entry based on keys provided in `session_id` and `action_id`.
        if !pending.contains_key(&format!("{}:{}", session_id, action_id)) {
            return Err(crate::BastionError::TicketInvalid(format!(
                "No pending request found for '{}:{}'. Either the key is invalid or no ticket exists.",
                session_id.to_string(), action_id.to_string()
            )));
        }

        let (session, _) = match &*pending.get(&format!("{}:{}", session_id, action_id)) {
            Some(ticket) => ({
                // Extract the key from the request object to determine which entry should be approved.
                format!("{}", ticket.session_id),
                
                // Create a tuple for comparison against history entries if one exists.
                (session.to_string(), action_id.to_string())
            }, None),
            
            _ => {
                return Err(crate::BastionError::TicketInvalid(format!(
                    "No pending request found with keys '{}:{}'. Either the key is invalid or no ticket exists.",
                    session_id, action_id
                )));
            }
        };

        // Update history if a match was found. If none exist yet in `history`, initialize an empty vector for this entry.
        let mut existing = self.history.write();
        
        // Check existence of the specific tuple within the current history lock.
        if !existing.contains(&session.to_string(), action_id.to_string()) {
            existing.push((session, action_id));
            
            // Ensure `history` is not empty (optional safety check).
            let mut len = 0;
            for entry in &mut existing {
                *entry == Some(session.clone()); break
