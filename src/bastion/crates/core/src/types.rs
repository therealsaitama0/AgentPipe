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

    // Public method to construct the schema from a map of keys and values (e.g., 'amount' => 10.5, "price" => 9.23)
    pub fn build_schema(schema_map: HashMap<String, String>) -> Self {
        let mut errors = Vec::new();

        for key in &schema_map.keys() {
            if let Some(value_str) = schema_map.get(key) {
                match value_str.parse::<f64>() {
                    Ok(v) => errors.push(format!("Field '{}' must be a float between 0.1 and 1000.0", key)),
                    Err(e) => errors.push(format!("Invalid type for field '{}': {}", key, e)),
                }
            } else if let Some(value_str) = schema_map.get(key.as_ref()) {
                match value_str.parse::<f64>() {
                    Ok(v) => errors.push(format!("Field '{}' must be a float between 0.1 and 1000.0", key)),
                    Err(e) => errors.push(format!("Invalid type for field '{}': {}", key, e)),
                }
            } else if let Some(value_str) = schema_map.get(key.as_ref()) {
                match value_str.parse::<i32>() {
                    Ok(v) => errors.push(format!("Field '{}' must be an integer between 0 and 100", key)),
                    Err(e) => errors.push(format!("Invalid type for field '{}': {}", key, e)),
                }
            } else if let Some(value_str) = schema_map.get(key.as_ref()) {
                match value_str.parse::<i64>() {
                    Ok(v) => errors.push(format!("Field '{}' must be an integer between 0 and 100", key)),
                    Err(e) => errors.push(format!("Invalid type for field '{}': {}", key, e)),
                }
            } else if let Some(value_str) = schema_map.get(key.as_ref()) {
                match value_str.parse::<f64>() {
                    Ok(v) => errors.push(format!("Field '{}' must be a float between 0.1 and 1000.0", key)),
                    Err(e) => errors.push(format!("Invalid type for field '{}': {}", key, e)),
                }
            } else if let Some(value_str) = schema_map.get(key.as_ref()) {
                match value_str.parse::<f64>() {
                    Ok(v) => errors.push(format!("Field '{}' must be a float between 0.1 and 1000.0", key)),
                    Err(e) => errors.push(format!("Invalid type for field '{}': {}", key, e)),
                }
            } else if let Some(value_str) = schema_map.get(key.as_ref()) {
                match value_str.parse::<f64>() {
                    Ok(v) => errors.push(format!("Field '{}' must be a float between 0.1 and 1000.0", key)),
                    Err(e) => errors.push(format!("Invalid type for
