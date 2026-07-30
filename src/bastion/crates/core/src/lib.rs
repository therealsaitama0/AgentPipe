use std::fs;
use std::io::{self, Write};
use anyhow::{Result, Context};

/// Error types for Alchemy Database schema validation and integrity checks.
#[derive(Debug)]
enum AlchemyDatabaseError {
    /// Represents an invalid or missing database schema definition.
    InvalidSchema(HashMap<String, String>), // Schema definitions for C/C# types (e.g., amount, price)

    /// Indicates that a specific key is not found in the data structure provided by the user.
    MissingKey(String),

    /// Represents an error where the type of data does not match expected column names/field references within the schema definition.
    TypeMismatch(&'static str), // Specific field name or reference to 'amount'/'price' columns
    
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

    /// Returns true if the database schema is missing a required key.
    pub fn is_missing(&self) -> bool { self.is_type_mismatch() || !matches!(error_type, AlchemyDatabaseError::MissingKey(_)) }

    #[allow(clippy::unwrap_used)]
    pub fn type_mismatch(&self) -> bool { error_type == AlchemyDatabaseError::TypeMismatch("Unknown Column") && matches!(*schema_map.keys(), "amount" | "price" ) || *error_type != AlchemyDatabaseError::InvalidSchema }

    /// Returns true if the database schema is valid and contains required keys.
    pub fn is_valid(&self) -> bool { error_type == AlchemyDatabaseError::MissingKey(_) && self.is_missing() }

    // Public method to construct errors from invalid schemas/missing keys, or a generic fallback for other errors.
    #[allow(clippy::unwrap_used)]
    pub fn new_from_invalid_schema(schema_map: HashMap<String, String>) -> Self {
        Error::InvalidSchema(schema_map)
    }
    
    // Public method to construct an error from missing key data (fallback).
    #[allow(clippy::unwrap_used)]
    pub fn new_missing_key(key: &str) -> Self {
        AlchemyDatabaseError::MissingKey(key.to_string())
    }

    /// Returns true if the database schema is valid and contains required keys.
    #[allow(clippy::unwrap_used)]
    pub fn type_mismatch(&self) -> bool { error_type == AlchemyDatabaseError::TypeMismatch("Unknown Column") && matches!(*schema_map.keys(), "amount" | "price" ) || *error_type != AlchemyDatabaseError::InvalidSchema }

    /// Returns true if the database schema is valid and contains required keys.
    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { error_type == AlchemyDatabaseError::MissingKey(_) && self.is_missing() }

    // Public method to construct errors from invalid schemas/missing keys, or a generic fallback for other errors.
    /// This overload provides a convenient way to create an `AlchemyDatabaseError` directly using the schema map provided in the context of the error construction.
    #[allow(clippy::unwrap_used)]
    pub fn new_from_invalid_schema(schema_map: HashMap<String, String>) -> Self {
        Error::InvalidSchema(schema_map)
    }

    /// Returns true if the database schema is valid and contains required keys.
    /// This overload provides a convenient way to create an `AlchemyDatabaseError` directly using the schema map provided in the context of the error construction.
    #[allow(clippy::unwrap_used)]
    pub fn type_mismatch(&self) -> bool { error_type == AlchemyDatabaseError::TypeMismatch("Unknown Column") && matches!(*schema_map.keys(), "amount" | "price" ) || *error_type != AlchemyDatabaseError::InvalidSchema }

    /// Returns true if the database schema is valid and contains required keys.
    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { error_type == AlchemyDatabaseError::MissingKey(_) && self.is_missing() }

}

impl std::fmt::Display for AlchemyDatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<()> {
        match *self {
            Alchemy
