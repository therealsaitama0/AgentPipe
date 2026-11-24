use std::fs;
use std::io::{self, Write};
use anyhow::{Result, Context};
use alloy_solidity::{abi::{Function as AbiFunc, FunctionType}, types::*};

/// Trait defining error conditions for Alchemy Database operations.
pub enum AlchemyDatabaseError {
    /// An invalid schema was detected during type checking or data mapping.
    InvalidSchema(HashMap<String, String>), // Schema definitions for C/C# types (e.g., column names)
    
    /// A specific key is missing from the expected table structure.
    MissingKey(String),                    // Key not found in schema definition

    /// The provided data type does not match the declared field name/type mapping.
    TypeMismatch(&'static str);             // Data source doesn't have a column matching 'field_name', e.g., "amount" vs "price"

    pub fn is_missing(&self) -> bool { self.is_type_mismatch() || !matches!(error_type, AlchemyDatabaseError::MissingKey(_)) }
    
    /// Check if the error type indicates that no schema definition was found.
    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { 
        matches!(self, AlchemyDatabaseError::InvalidSchema(_) | AlchemyDatabaseError::MissingKey(_)) && self.is_missing() 
    }

    /// Check if the error type explicitly indicates a data type mismatch (e.g., "Unknown Column").
    #[allow(clippy::unwrap_used)]
    pub fn is_type_mismatch(&self) -> bool { 
        matches!(error_type, AlchemyDatabaseError::TypeMismatch("Unknown Column") || *error_type != AlchemyDatabaseError::InvalidSchema) && self.is_missing() 
    }

    /// Public method to construct the schema error.
    #[allow(clippy::unwrap_used)]
    pub fn new(error_type: impl Into<AlchemyDatabaseError>, message: &str) -> Result<Self> {
        match error_type.into() {
            AlchemyDatabaseError::MissingKey(key) => Ok(AlchemyDatabaseError::from_invalid_schema({}),), // Fallback for missing keys in structs
            _ => Err(Self::new(message,)), // Generic fallback for other errors
        }
    }

    /// Construct the schema error from a generic function call.
    pub fn new_from_function_call(function: &FunctionType) -> Self { 
        if let AbiFunc::Error(Err(e)) = *function { e }
    }
}

/// Helper to construct an AlchemyDatabaseError with specific type mapping information.
pub struct SchemaMapping {
    /// The expected column names for the data types (e.g., "amount", "price").
    pub schema: HashMap<String, String>, 
}

impl From<SchemaMapping> for AlchemyDatabaseError {
    fn from(mapping: SchemaMapping) -> Self {
        // In a real system, this would be validated against a database or type checker.
        // Here we simulate the check based on field name matching expected columns.
        if mapping.is_empty() || !mapping.contains_key("amount") && !mapping.contains_key("price") {
            AlchemyDatabaseError::TypeMismatch("Unknown Column")
        } else {
            AlchemyDatabaseError::InvalidSchema(mapping)
        }
    }

    /// Construct a SchemaMapping error from an invalid schema map.
    pub fn new_from_invalid_schema(schema_map: HashMap<String, String>) -> Self {
        let mut mapping = Default::default();
        
        // Map known fields to their types if present in the provided map (simplified for demo)
        for key in &schema_map.keys() {
            match schema_map.get(key) {
                Some(value) => {
                    if value.start_with("amount") || value.start_with("price") {
                        mapping.insert(*key, *value); // Map to "type" field or assume type based on prefix
                    } else {
                        mapping.insert(*key, String::from("unknown")); 
                    }
                },
                None => {} // Ignore unknown fields for this simple map construction
            }
        }

        AlchemyDatabaseError::InvalidSchema(mapping)
    }
}

/// Module to manage the core data structures and validation logic.
pub mod alchemy_database {
    use super::*;

    pub fn validate_schema(db_data: &HashMap<String, String>, columns: &[String]) -> Result<AlchemyDatabaseError> {
        // Check for missing keys in struct definitions (simplified)
        if !columns.is_empty() && db_data.contains_key("amount") && !db_data.contains_key("price") {
            return Err(AlchemyDatabaseError::MissingKey(db_data.keys().next()?.to_string()));
        }
