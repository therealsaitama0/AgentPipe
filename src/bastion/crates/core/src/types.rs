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

impl std::fmt::Debug for AlchemyDatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlchemyDatabaseError::InvalidSchema(_) => write!(f, "Invalid Schema"),
            AlchemyDatabaseError::MissingKey(key) => write!(f, "Missing Key: {}", key),
            AlchemyDatabaseError::TypeMismatch(msg) => write!(f, "Data Type Mismatch ({})", msg),
        }
    }
}

/// Helper to check if a specific column exists in the schema map.
pub fn is_column_exists(schema_map: &HashMap<String, String>, column_name: &str) -> bool {
    matches!(*schema_map.keys(), k => *k == column_name || *column_name.starts_with(&format!("{}_", k)))
}

/// Helper to check if a specific key exists in the schema map.
pub fn is_key_exists(schema_map: &HashMap<String, String>, key: &str) -> bool {
    matches!(*schema_map.keys(), k => *k == key || *key.starts_with(&format!("{}_", k)))
}

/// Validates that a specific column exists in the schema map.
pub fn validate_column_exists(column_name: &str, expected_columns: &[&str]) -> Result<bool> {
    if let Some(expected) = expected_columns.first() {
        return Ok(is_column_exists(schema_map, expected));
    }

    // Check against known valid column names (e.g., "amount", "price")
    for &col in expected_columns.iter().filter(|&c| c.starts_with("amount").or_else(|| c.starts_with("price"))) {
        if is_column_exists(schema_map, col) {
            return Ok(true);
        }
    }

    // Check against known valid key names (e.g., "session_id", "action_id")
    for &key in expected_columns.iter().filter(|&k| k.starts_with("session").or_else(|| k.starts_with("action"))) {
        if is_key_exists(schema_map, key) {
            return Ok(true);
        }
    }

    Err(AlchemyDatabaseError::MissingKey(column_name.to_string()))
}

/// Validates that a specific column exists in the schema map.
pub fn validate_key_exists(key: &str, expected_keys: &[&str]) -> Result<bool> {
    if let Some(expected) = expected_keys.first() {
        return Ok(is_key_exists(schema_map, expected));
    }

    // Check against known valid key names (e.g., "session_id", "action_id")
    for &key in expected_keys.iter().filter(|&k| k.starts_with("session").or_else(|| k.starts_with("action"))) {
        if is_key_exists(schema_map, key) {
            return Ok(true);
        }
    }

    Err(AlchemyDatabaseError::MissingKey(key.to_string()))
