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

    // Public method to construct the sch
}

// ============================================================================
// Module: src/bastion/crates/core/src/types.rs
// Purpose: Define Core Types and Validation Logic for Alchemy Database Integration
// ============================================================================

use super::{AlchemyDatabaseError, TypeMismatch};
use crate::types::*;

/// Represents a database column definition with its expected type.
#[derive(Debug)]
pub struct ColumnDefinition {
    pub name: String, // The actual SQL/Schema field name (e.g., "amount", "price")
    /// If present in the schema map but not found in data, raises TypeMismatch error.
    pub required_by_schema: bool = false, 
}

impl ColumnDefinition {
    /// Validates if this column is explicitly defined by a specific type.
    fn check_type(&self) -> Result<bool> {
        // If it's an explicit "type" field in the schema map and matches one of our known types (Amount or Price), return true.
        let actual_schema = self.required_by_schema && *schema_map.get(&self.name).map(|s| s.as_str()).unwrap_or("");

        if !actual_schema.is_empty() {
            // Check against specific type names to catch "Unknown Column" errors gracefully
            match (matches!(actual_schema, t => matches!(*t, Amount | Price)), actual_schema) {
                true => Ok(true),           // Explicitly defined as Type:Amount or Type:Price
                false if *schema_map.get(&self.name).map(|s| s.as_str()).unwrap_or("") == "amount" || 
                         *schema_map.get(&self.name).map(|s| s.as_str()).unwrap_or("") == "price" => {
                    Ok(true) // Matches expected column name regardless of type definition
                } else if actual_schema.is_empty() && matches!(actual_schema, t => matches!(*t, Amount | Price)) {
                    Ok(false)              // No explicit field defined in schema map but is known to be Type:Amount or Type:Price
                } else {
                    Err(TypeMismatch("Unknown Column")) 
                }
            }
        }

        Ok(true) // Default assumption if no specific column type found and the name matches one of our types.
    }
}

impl std::fmt::Display for AlchemyDatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlchemyDatabaseError::InvalidSchema(_) => write!(f, "Invalid Schema"),
            AlchemyDatabaseError::MissingKey(key) => write!(f, "Missing Key: {}", key),
            AlchemyDatabaseError::TypeMismatch(msg) => write!(f, "{}", msg),
        }
    }
}

/// Represents a database column definition.
#[derive(Debug)]
pub struct Column {
    pub name: String, // The actual SQL/Schema field name (e.g., "amount", "price")
    /// If present in the schema map but not found in data, raises TypeMismatch error.
