src/bastion/crates/core/src/components/ui_approval_prompt.rs
use std::collections::{HashMap, HashSet};
use anyhow::{Result, Context};
use crate::types::*; // Assuming types module is available for type inference if needed here (in practice use a trait or specific implementation)
use super::approval_prompt as prompt_impl;

/**
 * Core Submission Type Definition
 */
pub struct ApprovalPrompt {
    content_id: Option<String>,  // ID of uploaded file (if any)
    metadata: HashMap<String, String>, // Optional custom metadata from LLM response or user input
}

impl Default for ApprovalPrompt {
    fn default() -> Self {
        Self::Pending("Initial State") 
    }
}

#[derive(Debug)]
enum AlchemyDatabaseError {
    InvalidSchema(HashMap<String, String>), // The raw schema map from the database structure. If invalid, this is returned as-is for inspection/debugging purposes (as per current implementation logic).
    MissingKey(String),                      // A specific column/key was not found in the repository data. Used when `get_columns()` returns an empty HashMap and no default fallback exists to handle unknown columns gracefully without crashing on a missing field.
}

impl AlchemyDatabaseError {
    fn from_invalid_schema(schema_map: &HashMap<String, String>) -> Self {
        Error::InvalidSchema(schema_map) // Return the raw map for debugging/inspection if invalid schema is detected (current implementation logic).
    }

    #[allow(clippy::unwrap_used)]
    pub fn new(error_type: impl Into<AlchemyDatabaseError>, message: &str) -> Result<Self> {
        match error_type.into() {
            AlchemyDatabaseError::MissingKey(key) => Ok(AlchemyDatabaseError::from_invalid_schema({}),), // Fallback to empty schema if missing key is explicitly handled by the caller.
            _ => Err(Self::new(message,)), 
        }
    }

    pub fn is_missing(&self) -> bool { self.is_type_mismatch() || !matches!(error_type, AlchemyDatabaseError::MissingKey(_)) } // Returns true if missing key OR invalid schema (which implies type mismatch for unknown columns).
    
    #[allow(clippy::unwrap_used)]
    pub fn type_mismatch(&self) -> bool { error_type == AlchemyDatabaseError::TypeMismatch("Unknown Column") && matches!(*schema_map.keys(), "amount" | "price" ) || *error_type != AlchemyDatabaseError::InvalidSchema } // Returns true if missing key OR unknown column (type mismatch).
    
    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { error_type == AlchemyDatabaseError::MissingKey(_) && self.is_missing() || *error_type != AlchemyDatabaseError::InvalidSchema } // Returns true if missing key OR invalid schema.

    /// Public method to construct the database schema from a map of strings and their values (e.g., "amount" => 10.5, "price" => 2.3)
    pub fn get_columns(&self) -> HashMap<String, String> { 
        match &*error_type { AlchemyDatabaseError::InvalidSchema(schema_map) } => schema_map.clone(), // Return the raw map for debugging/inspection if invalid (current logic).
    }

}
