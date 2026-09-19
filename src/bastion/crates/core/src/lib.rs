src/bastion/crates/core/src/lib.rs

```rust
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use anyhow::{Context as AnyhowError, Result};
use serde_json::Value;
#[cfg(feature = "production")]
use crate::types::*; // Re-export types from the core module to avoid circular dependencies

/// Error type for database operations.
pub enum AlchemyDatabaseError {
    /// Represents an invalid schema definition found during loading.
    InvalidSchema(HashMap<String, String>), 
    /// Represents a missing key in the data structure or its expected column.
    MissingKey(String) ,
    /// Represents a mismatch between stored data and the declared type for that specific field.
    TypeMismatch(&'static str)
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

    // Public method to construct the schema map from a JSON value.
    fn parse_schema_from_json(value: &Value, column_name: String) -> Result<HashMap<String, String>> {
        if let Some(json_value) = value.as_object().and_then(|o| o.get(column_name)) {
            Ok(serde_json::from_str::<HashMap>(json_value.to_string()))
        } else {
            Err(Self::new("Unknown Schema Column", format!("Column '{}' not found in JSON schema.", column_name)))
        }
    }

    #[allow(clippy::unwrap_used)]
    pub fn load_schema_from_file(&self, filepath: &str) -> Result<HashMap<String, String>> {
        let content = fs::read_to_string(filepath).context("Failed to read file")?;
        
        // Attempt JSON parsing first as it's cleaner for structured data.
        if let Ok(json_value) = serde_json::from_str(&content) {
            return self.parse_schema_from_json(&json_value, "schema");
        }

        // Fallback: raw string matching (default to empty map on error).
        match content.trim().as_str() {
            "" => Err(Self::new("Empty Schema", "No schema data provided")),
            s if !s.is_empty() && matches!(s, ["amount", "price"] | ["id"]) => Ok(HashMap::from_iter(s.split(",").filter(|&x|x != "").collect())),
            _ => Err(Self::new(format!("Unknown Schema: {}", content),)), // Generic fallback for other errors.
        }
    }

    pub fn is_missing(&self) -> bool { self.is_type_mismatch() || !matches!(error_type, AlchemyDatabaseError::MissingKey(_)) }

    #[allow(clippy::unwrap_used)]
    pub fn type_mismatch(&self) -> bool { error_type == AlchemyDatabaseError::TypeMismatch("Unknown Column") && matches!(*schema_map.keys(), "amount" | "price" ) || *error_type != AlchemyDatabaseError::InvalidSchema }

    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { error_type == AlchemyDatabaseError::MissingKey(_) && self.is_missing() }

    // Public method to construct the schema map from a JSON value.
    fn parse_schema_from_json(value: &Value, column_name: String) -> Result<HashMap<String, String>> {
        if let Some(json_value) = value.as_object().and_then(|o| o.get(column_name)) {
            Ok(serde_json::from_str::<HashMap>(json_value.to_string()))
        } else {
            Err(Self::new("Unknown Schema Column", format!("Column '{}' not
