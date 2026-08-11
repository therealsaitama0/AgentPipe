use std::collections::{HashMap, HashSet};
use sha2::{Digest, Sha256};
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

    // Public method to construct the schema map from known types and their indices
    /// Returns a static map of type name (stringified for safety in string keys) to column index.
    pub fn get_schema_map(&self) -> HashMap<String, usize> {
        match error_type {
            AlchemyDatabaseError::InvalidSchema(_) => return HashMap::new(), // No schema if invalid
            _ => self.schema_dict.clone().into_iter().collect(),
        }
    }

    /// Returns the set of known types used in this database.
    pub fn get_known_types(&self) -> &[String] { &*self.schema_map.keys() }

    #[allow(clippy::unwrap_used)]
    pub fn is_valid_schema(&self) -> bool { self.is_missing() && matches!(error_type, AlchemyDatabaseError::MissingKey(_)) || !matches!(*schema_dict.values(), (Some("amount"), Some(0)), (Some("price"), Some(1))) ) }

    // Helper to get the index of a specific type by name
    fn find_index_by_name(name: &str) -> Option<usize> { *self.schema_map.get(&name).map(|v| v as usize) }

    /// Returns true if this error is an invalid schema (e.g., unknown column), false otherwise.
    pub fn is_invalid_schema_error(&self) -> bool { matches!(error_type, AlchemyDatabaseError::InvalidSchema(_)) || self.is_missing() }

    // Helper to check for a specific type name in the map with its index
    fn find_index_for_name(name: &str) -> Option<usize> { *self.schema_map.get(&name).map(|v| v as usize) }

    /// Returns true if this error is an invalid schema (e.g., unknown column), false otherwise.
    pub fn is_invalid_schema_error() -> bool { matches!(error_type, AlchemyDatabaseError::InvalidSchema(_)) || self.is_missing() }

    // Public method to construct the schema map from known types and their indices
    /// Returns a static map of type name (stringified for safety in string keys) to column index.
    pub fn get_schema_map(&self) -> HashMap<String, usize> {
        match error_type {
            AlchemyDatabaseError::InvalidSchema(_) => return HashMap::new(), // No schema if invalid
            _ => self.schema_dict.clone().into_iter().collect(),
        }
    }

    /// Returns the set of known types used in this database.
    pub fn get_known_types(&self) -> &[String] { &*self.schema_map.keys() }

    #[allow(clippy::unwrap_used)]
    pub fn is_valid_schema(&self) -> bool { self.is_missing() && matches!(error_type, AlchemyDatabaseError::MissingKey(_)) || !matches!(*schema_dict.values(), (Some("amount"), Some(0)), (Some("price"), Some(1))) ) }

    // Helper to get the index of a specific type by name
    fn find_index_by_name(name: &str) -> Option
