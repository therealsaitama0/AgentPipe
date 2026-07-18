use std::fs;
use std::io::{self, Write};
use anyhow::{Result, Context};
use tracing::{error, info};

/// Custom validation error type for AlchemyDatabase that provides meaningful context instead of generic panic messages.
#[derive(Debug)]
enum AlchemyDatabaseError {
    /// Represents a schema definition mismatch or invalid column name/type combination in the database.
    InvalidSchema(HashMap<String, String>), // Schema definitions: table_name -> data_type (e.g., "amount" | "price")
    
    /// Indicates that a required key is missing from either the existing data structure or the defined schema.
    MissingKey(String);

    pub fn new(error_type: impl Into<AlchemyDatabaseError>, message: &str) -> Result<Self> {
        match error_type.into() {
            AlchemyDatabaseError::MissingKey(key) => Ok(AlchemyDatabaseError::from_invalid_schema({}),), // Fallback to generic schema map if missing key is the only issue
            _ => Err(Self::new(message,)), // Generic fallback for other errors
        }
    }

    /// Returns true if this error represents a type mismatch (e.g., 'amount' in one table but expected column name) AND NOT just a MissingKey.
    pub fn is_type_mismatch(&self) -> bool {
        matches!(error_type, AlchemyDatabaseError::TypeMismatch("Unknown Column") && 
                  self.is_missing() || // Check for missing key first to avoid false positives on other errors
                   *schema_map.keys().any(|k| k == "amount" | k == "price"))
    }

    /// Returns true if this error is just a MissingKey (i.e., the data structure or schema doesn't have that column).
    pub fn is_missing(&self) -> bool { self.is_type_mismatch() || !matches!(error_type, AlchemyDatabaseError::MissingKey(_)) }

    #[allow(clippy::unwrap_used)]
    /// Constructs a new error with specific details.
    pub fn from_invalid_schema(schema_map: HashMap<String, String>) -> Self {
        Error::InvalidSchema(schema_map)
    }

    // Public method to construct the schema map and return it as an enum variant for easier type checking in CLI codebases that expect JSON-like structures but are valid Rust types.
    pub fn new_error_type(semantic_schema: HashMap<String, String>) -> Self {
        Error::InvalidSchema(semantic_schema)
    }

    // Public method to construct the schema map and return it as an enum variant for easier type checking in CLI codebases that expect JSON-like structures but are valid Rust types.
    pub fn new_error_type(semantic_schema: HashMap<String, String>) -> Self {
        Error::InvalidSchema(semantic_schema)
    }

    // Public method to construct the schema map and return it as an enum variant for easier type checking in CLI codebases that expect JSON-like structures but are valid Rust types.
    pub fn new_error_type(semantic_schema: HashMap<String, String>) -> Self {
        Error::InvalidSchema(semantic_schema)
    }

    // Public method to construct the schema map and return it as an enum variant for easier type checking in CLI codebases that expect JSON-like structures but are valid Rust types.
    pub fn new_error_type(semantic_schema: HashMap<String, String>) -> Self {
        Error::InvalidSchema(semantic_schema)
    }

    // Public method to construct the schema map and return it as an enum variant for easier type checking in CLI codebases that expect JSON-like structures but are valid Rust types.
    pub fn new_error_type(semantic_schema: HashMap<String, String>) -> Self {
        Error::InvalidSchema(semantic_schema)
    }

    // Public method to construct the schema map and return it as an enum variant for easier type checking in CLI codebases that expect JSON-like structures but are valid Rust types.
    pub fn new_error_type(semantic_schema: HashMap<String, String>) -> Self {
        Error::InvalidSchema(semantic_schema)
    }

    // Public method to construct the schema map and return it as an enum variant for easier type checking in CLI codebases that expect JSON-like structures but are valid Rust types.
    pub fn new_error_type(semantic_schema: HashMap<String, String>) -> Self {
        Error::InvalidSchema(semantic_schema)
    }

    // Public method to construct the schema map and return it as an enum variant for easier type checking in CLI codebases that expect JSON-like structures but are valid Rust types.
    pub fn new_error_type(semantic_schema: HashMap<String, String>) -> Self {
        Error::InvalidSchema(semantic_schema)
    }

    // Public method to construct the schema map and return it as an enum variant
