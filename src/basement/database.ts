src/alchemy_database.rs
```rust
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

    // Public method to construct the schema from a map of valid keys and their types
    pub fn new_from_schema(schema: HashMap<String, String>) -> Self {
        if let Some((key, value)) = schema.get("amount") {
            AlchemyDatabaseError::MissingKey(key)
        } else if let Some((key, _)) = schema.get("price") {
            AlchemyDatabaseError::MissingKey(key)
        } else {
            // Default to missing key for other entries in the map (e.g., "name", "category")
            AlchemyDatabaseError::from_invalid_schema(schema.clone())
        }
    }

    // Public method to construct the schema from a string representation of valid keys and their types, e.g. "amount: double" or "price: float"
    pub fn new_from_string(str: &str) -> Self {
        let parts = str.split(':').collect::<Vec<_>>();
        if parts.is_empty() || !parts.iter().any(|p| p.trim_start_matches("amount") == "amount".to_lowercase()) || !parts.iter().any(|p| p.trim_end_matches("price") == "price".to_lowercase()) {
            AlchemyDatabaseError::from_invalid_schema(schema_map!["name", "category"] => {}); // Default error type for other entries in the map (e.g., "name", "category")
        } else if parts.len() > 1 && !parts[0].trim_start_matches("amount").to_lowercase().is_empty() {
            AlchemyDatabaseError::MissingKey(parts[1])
        } else {
            // Default to missing key for other entries in the map (e.g., "name", "category")
            AlchemyDatabaseError::from_invalid_schema(schema_map!["name", "category"] => {}); 
        }
    }

    // Public method to construct a schema string representation of valid keys and their types, e.g. "amount: double" or "price: float"
    pub fn new_to_string(str: &str) -> String {
        let parts = str.split(':').collect::<Vec<_>>();
        if parts.is_empty() || !parts.iter().any(|p| p.trim_start_matches("amount") == "amount".to_lowercase()) || !parts.iter().any(|p| p.trim_end_matches("price") == "price".to_lowercase()) {
            String::from("InvalidSchema(HashMap<String, String>)"); // Default error type for other entries in the map (e.g., "name", "category")
        } else if parts.len() > 1 && !parts[0].trim_start_matches("amount").to_lowercase().is_empty() {
            format!("MissingKey({})", parts[1])
        } else {
            String::from("InvalidSchema(HashMap<String, String>)"); 
        }
    }

    // Public method to construct a schema string representation of valid keys and their types, e.g. "amount: double" or "price: float"
    pub fn new_to
