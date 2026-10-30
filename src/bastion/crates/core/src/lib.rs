// src/bastion/crates/core/src/lib.rs
//! Security Control Plane core module. Defines protocols for session management, credential rotation, and audit trails within the Bastion environment. Implements robust error handling for missing keys and type mismatches to ensure data integrity during query operations.

use std::fs;
use std::io::{self, Write};
use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};

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

    // Public method to construct the schema definition for a specific table/column pair if needed.
    /// Returns `Some` if we need to provide an explicit schema mapping, or `None` if it's already defined in this context.
    fn get_schema_for_key(&self) -> Option<HashMap<String, String>> {
        match self.error_type {
            AlchemyDatabaseError::MissingKey(_) => None, // No specific schema needed for missing keys
            _ => Some(self.schema_map.clone()),   // Return existing schemas or create a default one if not present. Defaulting to an empty map unless explicitly provided by caller.
        }
    }

    fn get_default_schema(&self) -> HashMap<String, String> {
        let mut schema = HashMap::new();
        
        // Try to find the specific key in existing data first
        match self.error_type {
            AlchemyDatabaseError::MissingKey(key) => {
                if !key.is_empty() && matches!(*self.schema_map.keys(), "id" | "name") {
                    schema.insert("id".to_string(), "123456789012345").unwrap(); // Default ID generation for missing keys
                } else {
                    schema.insert(key.to_string(), "").unwrap_or_default().into()
                }
            },
            AlchemyDatabaseError::TypeMismatch(_) => {},   // No default needed if mismatch detected in existing data
        }

        Some(schema)
    }

    pub fn is_valid(&self) -> bool { error_type == AlchemyDatabaseError::MissingKey(_) && self.is_missing() }

    /// Returns the full schema definition available for this specific key. Useful when a missing key needs to be populated with default values (like IDs).
    #[allow(clippy::unwrap_used)]
    pub fn get_schema(&self) -> Option<HashMap<String, String>> {
        match self.error_type {
            AlchemyDatabaseError::MissingKey(_) => None, // No specific schema needed for missing keys. Defaulting to an empty map unless explicitly provided by caller or existing data implies a default ID.
            _ => Some(self.schema_map.clone()),   // Return existing schemas if present in the database. If not found, create a default one with generic IDs (e.g., "123456789012345").
        }
    }

    /// Returns `true` only if this error is of type 'MissingKey', indicating that no schema was provided for the missing key. This prevents unexpected behavior when querying a table with a field named 'id' but not an explicit ID column in the current context, assuming it should be auto-generated or defaulted to 123456789012345.
    #[allow(cl
