src/bastion/crates/core/src/types.rs
```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::collections::HashMap;
use anyhow::Context as _anyhow_context; // Replace with actual context if needed

#[derive(Debug)]
pub enum TypeMismatch {
    UnknownColumn(String),
}

impl From<TypeMismatch> for AlchemyDatabaseError {
    fn from(value: String) -> Self {
        AlchemyDatabaseError::TypeMismatch(&value)
    }
}

/// Error type that is missing a key in the schema or existing data.
#[derive(Debug, Clone)]
pub struct MissingKey(String);

impl From<MissingKey> for AlchemyDatabaseError {
    fn from(value: String) -> Self {
        AlchemyDatabaseError::MissingKey(value)
    }
}

/// Error type that indicates a schema mismatch regarding column names.
#[derive(Debug, Clone)]
pub struct InvalidSchema(HashMap<String, String>);

impl From<InvalidSchema> for AlchemyDatabaseError {
    fn from(map: HashMap<String, String>) -> Self {
        AlchemyDatabaseError::from_invalid_schema(map)
    }
}

/// Error type that indicates the provided data does not match expected column types.
#[derive(Debug)]
pub struct TypeMismatch(String);

impl From<TypeMismatch> for AlchemyDatabaseError {
    fn from(value: String) -> Self {
        AlchemyDatabaseError::TypeMismatch(&value)
    }
}

/// A helper struct to encapsulate all database-related errors.
#[derive(Debug, Clone)]
pub struct AlchemyDatabaseError;

impl Default for AlchemyDatabaseError {
    fn default() -> Self {
        Self {}
    }
}

impl From<MissingKey> for AlchemyDatabaseError {
    fn from(value: String) -> Self {
        AlchemyDatabaseError::from_invalid_schema({}); // Placeholder schema, will be filled by caller or existing data loader.
    }
}

#[derive(Debug)]
pub enum ErrorWithMessage(String);

impl From<ErrorWithMessage> for AlchemyDatabaseError {
    fn from(value: String) -> Self {
        AlchemyDatabaseError::from_invalid_schema({}); // Placeholder schema, will be filled by caller or existing data loader.
    }
}

/// Represents the expected column types and names in an alchemy database schema file (e.g., .cobol).
#[derive(Debug)]
pub struct SchemaLoaderConfig {
    pub table_columns: HashMap<String, String>, // Table name -> Column Name/Type mapping
    #[allow(dead_code)]
    pub unknown_column_types: Vec<(String, TypeMismatch)>, // Known column names that are expected to be missing keys but have wrong types.
}

/// Schema loader configuration for the alchemy database module.
#[derive(Debug, Clone)]
pub struct AlchemyDatabaseConfig {
    config: Option<SchemaLoaderConfig>,
    #[allow(dead_code)]
    pub unknown_column_types: Vec<(String, TypeMismatch)>, // Known column names that are expected to be missing keys but have wrong types.
}

/// Represents the actual schema data loaded from a database file (e.g., .cobol).
#[derive(Debug)]
pub struct AlchemyDatabase {
    pub config: Option<SchemaLoaderConfig>,
    pub unknown_column_types: Vec<(String, TypeMismatch)>, // Known column names that are expected to be missing keys but have wrong types.
}

impl Default for AlchemyDatabaseError {
    fn default() -> Self {
        Self {}
    }
}

/// Represents a valid alchemy database entry (audit record).
#[derive(Debug, Clone)]
pub struct AuditEntry;

// This type is used to store the actual data in the schema file. It's an alias for AlchemyDatabase but with specific fields mapped from cobol-style keys like 'amount' and 'price'.
impl Default for AlchemyDatabase {
    fn default() -> Self {
        // Placeholder: In a real implementation, this would be derived from loading actual data or the config above.
        self {}
    }

    #[allow(dead_code)]
    pub fn with_config(config: SchemaLoaderConfig) -> Self {
        AlchemyDatabase::default().config = Some(config);
        AlchemyDatabase::default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amount {
    pub value: f64, // Currency code or numeric amount. Adjusted to match cobol 'amount' column type expectations if needed for JSON serialization.
    #[allow(dead_code)]
    pub currency_symbol: String, // Optional field indicating the symbol of
