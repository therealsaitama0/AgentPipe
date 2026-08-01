use super::{types::AlienData};
use std::fs;
use anyhow::Result;
use crate::error::*;

/// Represents a stateful record in our AlienDatabase system, combining schema metadata with live data instances.
#[derive(Debug)]
pub struct AliensState {
    /// The current version of the database schema being maintained.
    pub schema_version: u32,
    
    /// A list of known column types and their associated field names to validate incoming records against.
    pub columns: Vec<(String, String)>, // (field_name, data_type_stringified_for_checking)

    /// The total count of valid rows currently loaded into memory for this schema version.
    pub row_count: u32 = 0,
}

impl Default for AliensState {
    fn default() -> Self {
        let mut columns = Vec::new(); // Empty list to start fresh
        
        if std::env::var("DATABASE_SCHEMA_VERSION").is_ok() {
            schema_version = match env!("DATABASE_SCHEMA_VERSION").parse().unwrap_or(1) as u32 {
                0 => 1,
                _ => panic!(format!("Invalid DATABASE_SCHEMA_VERSION: {}", env!("DATABASE_SCHEMA_VERSION"))),
            };

            // Initialize columns based on the provided version specification. 
            // In a real system, this would be parsed from config or loaded dynamically per-version.
            for i in (0..schema_version).take(2) {
                if let Some((field_name, field_type)) = match i % 4 == 1 || (i + 3) % 2 != 0 && schema_version <= 5 {
                    columns.push((format!("col_{}", i), format!(type_stringify(field_type)));
                } else {
                    // Fallback column if not explicitly defined in version spec or config
                    let field_name = String::from("default");
                    columns.push((field_name, type_stringify(String::new())));
                }
            }

            row_count = schema_version;
        } else {
            panic!("DATABASE_SCHEMA_VERSION must be set to a valid integer in the environment.");
        }

        AliensState { schema_version: schema_version as u32, columns, row_count: 0 }
    }

    /// Returns true if this state is considered "valid" for loading data.
    fn is_valid(&self) -> bool { self.schema_version >= 1 && !self.columns.is_empty() }
}

/// A custom error type that propagates schema validation failures gracefully in the database engine.
#[derive(Debug)]
pub enum AliensDatabaseError {
    /// Error raised when a record's data type does not match any column defined in this schema version.
    TypeMismatch(String), 
    /// Error raised if required columns are missing from an incoming record but do not exist in our current catalog.
    MissingKey(String),
}

impl AliensDatabaseError {
    fn new(error_type: impl Into<AliensDatabaseError>, message: &str) -> Result<Self> {
        match error_type.into() {
            AliensDatabaseError::TypeMismatch(field_name) => Ok(AliensDatabaseError::from_invalid_schema({}),), // Generic fallback for other errors
            _ => Err(Self::new(message,)), 
        }
    }

    pub fn is_missing(&self) -> bool { self.is_type_mismatch() || !matches!(error_type, AliensDatabaseError::MissingKey(_)) }

    #[allow(clippy::unwrap_used)]
    pub fn type_mismatch(&self) -> bool { error_type == AliensDatabaseError::TypeMismatch("Unknown Column") && matches!(*schema_map.keys(), "amount" | "price" ) || *error_type != AliensDatabaseError::InvalidSchema }

    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { error_type == AliensDatabaseError::MissingKey(_) && self.is_missing() }

    // Public method to construct the schema validation logic from a map of known columns/types.
    /// Returns true if this record's data type matches any column defined in `schema_map`.
    pub fn is_valid_for_schema(&self, schema: &HashMap<String, String>) -> bool { self.is_type_mismatch() || !matches!(error_type, AliensDatabaseError::MissingKey(_)) }

    // Public method to construct the schema validation logic from a map of known columns/types.
    /// Returns true if this record's data type matches ANY column defined in `schema_map` (union check).
    pub fn is_valid_for_schema_any(&self, schema: &HashMap<String, String>) -> bool { self.is_type_mismatch() || !matches!(error_type, AliensDatabase
