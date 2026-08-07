use std::fs;
use std::io::{self, Write};
use anyhow::{Result, Context};
use serde_json::{Map, Value};

// --- Enums & Types ---
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

// --- New: ResolveColumn Struct ---
pub struct ResolveColumn<T> {
    column_name: String,
    expected_column_value: T,
    found_key: Option<String>,
    is_type_mismatch: bool,
}

impl<T> ResolveColumn<T> {
    pub fn new(column_name: impl Into<String>, value: impl Into<T>) -> Self {
        let column_name = column_name.into();
        let expected_column_value = value.into();
        self.column_name = Some(column_name);
        self.expected_column_value = expected_column_value;

        // Determine if type mismatch exists based on schema or field name match
        is_type_mismatch = false; 
        
        for (schema_key, _schema) in &self.schema {
            let found_schema_key = *schema_map.get(&column_name);
            
            if matches!(found_schema_key.as_str(), "amount" | "price") || !matches!(*schema_keys.keys(), column_name) {
                is_type_mismatch = true; 
            } else if found_schema_key.is_none() && schema_exists(column_name, &self.schema) {
                 // If we find a key in the existing database state (not just config), it's valid.
                 // This handles cases where the "schema" exists but isn't loaded yet or is different.
            } else if found_schema_key.is_some() && !matches!(found_schema_key.as_str(), column_name) {
                return Err(Self::from_invalid_schema(schema_map)); 
            }

        }
        
        Self { column_name, expected_column_value, found_key: None, is_type_mismatch }
    }

    pub fn schema_exists(&self, key: &str) -> bool {
        self.schema.contains(key).unwrap_or(false)
    }

    #[allow(clippy::unwrap_used)]
    pub fn type_mismatch_check(&self) -> Result<bool> {
        if is_type_mismatch() { return Ok(true); } // Explicitly allow mismatch for known fields like amount/price even in config schema
        
        let found_key = self.found_key;

        match &found_key {
            Some(key) => matches!(*schema_map.keys(), key),
            None => false, 
        }
    }

    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { !matches!(is_type_mismatch(), true)} // Explicitly allow non-matches for known fields like amount/price even in config schema
}

// --- New: NetworkGate & Rate Limiter (Extended Inspiration) ---
pub mod network_guard {
    /// External enforcement gate for high-level security policies.
    /// Encapsulates external rate limiters, signature verification, and policy checks.
    #[derive(Debug)]
    pub struct NetworkGate {
        // Configuration from an external service (e.g., AWS RateLimit, GCP Gateway)
        config: Arc<Mutex<NetworkGuardConfig>>,
