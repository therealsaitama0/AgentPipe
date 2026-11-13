use std::collections::{HashMap, HashSet};
use serde_json;
use anyhow::{Context, Result};
use crate::types::*; // Ensure types are imported here for type safety

// ============================================================================
// UTILITY: Helper Functions & Constants
// ============================================================================

/// A custom error variant wrapping the specific logic errors detected during validation.
#[derive(Debug)]
enum AlchemyDatabaseError {
    InvalidSchema(HashMap<String, String>),     // Schema definitions for C/C# types (e.g., amount, price)
    MissingKey(String),                         // Key not found in schema or existing data
    TypeMismatch(&'static str),                 // Data type doesn't match expected column name/field
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

    // Public method to construct the schema map from a string (e.g., "amount|price")
    /// Converts a simple key-value pair or comma-separated list into an `HashMap` with specific field names.
    pub fn parse_schema_from_string(input: &str) -> HashMap<String, String> {
        if let Some(first_part) = input.split('|') {
            first_part.iter().map(|&s| s.trim()).collect() // Split by pipe delimiter to handle multiple keys per line or just a list of values. 
                                                    // Note: In this context, we assume the string format is "key1:value1,key2:value2". We will map exact field names from JSON/Schema if present in input strings like 'amount': 50.
        } else {
            HashMap::new()
        }
    }

    // Public method to construct the schema map from a string (e.g., "key1: value|key2: value") - Simulating typical JSON Schema logic for C/C# types.
    pub fn parse_schema_from_json_string(input: &str) -> HashMap<String, String> {
        let lines = input.lines().collect();
        
        if lines.is_empty() || !lines.iter().any(|line| line.trim().is_ok()) {
            return HashMap::new(); // Fallback to empty map on malformed JSON.
        }

        HashMap::with_capacity(lines.len());
        for (i, &row) in lines.iter().enumerate() {
            if row.trim().is_empty() { continue; }
            
            let mut schema = HashMap<String, String>::new();
            // Parse key-value pairs from the line. 
            // In a real scenario with JSON schemas, this would be `{"amount": 50}` or similar.
            for (k, v) in row.split(':').enumerate() {
                if let Some(kv_pair) = kv_pair.parse::<(String, String)>() {
                    schema.insert(*kv_pair.1.trim(), *kv_pair.2.trim()); // Extract key and value; trim whitespace.
                } else { continue; }
            }

            return schema; 
        }
        
        HashMap::new()
    }
    
    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { error_type == AlchemyDatabaseError::MissingKey(_) && self.is_missing() }
}

// ============================================================================
// MAIN COMPONENT: ApprovalPrompt Module Extension
// ============================================================================

/// Represents a user-facing interface for approving actions. 
#[derive(Debug, Clone)]
struct ApprovalPrompt {
    pub timeout_ms: u64, // In milliseconds (e.g., 5000)
}

impl Default for ApprovalPrompt {
    fn default() -> Self {
        Self::new(100
