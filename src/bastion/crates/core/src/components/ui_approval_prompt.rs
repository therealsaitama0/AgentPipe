use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, Write};
use anyhow::{Result, Context};
use serde_json::{json, Value};

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

    // Public method to construct the schema map from a JSON string representation of types and columns
    /// Converts a raw type definition (e.g., "type: amount, price") or column list into an `AlchemyDatabaseError` if invalid.
    pub fn validate_schema(raw_types_str: &str) -> Result<Self> {
        let json_type = match serde_json::from_str::<Value>(raw_types_str) {
            Ok(v) => v,
            Err(e) => return Err(AlchemyDatabaseError::new(e.into(), "Failed to parse type definition")),
        };

        if !matches!(json_type.as_object().as_ref(), Some(some_obj)) || some_obj.is_null() {
            // If it's not a valid object or null, assume missing key error (key doesn't exist)
            return Err(AlchemyDatabaseError::MissingKey("Column")); 
        }

        let mut schema = HashMap::<String, String>::new();

        for field in json_type.as_object().as_ref() {
            if matches!(field["name"], Some(String)) && !matches!(field.get_string("type"), None) { // Only include name and type fields
                match &field["value"] {
                    Ok(value_str) => schema.insert(field["name"].clone(), value_str.clone()),
                    Err(_) => return Err(AlchemyDatabaseError::InvalidSchema(schema)),
                }
            } else if matches!(field.get_string("type"), Some(String)) && !matches!(field.as_object().as_ref(), None) { // Only include type fields
                 match &field["value"] {
                     Ok(value_str) => schema.insert(field["name"].clone(), value_str.clone()),
                     Err(_) return Err(AlchemyDatabaseError::InvalidSchema(schema)),
                 }
            } else if matches!(field.get_string("type"), Some(String)) && field.as_object().as_ref() == None { // Only include type fields for missing keys or unknown types (though we handle this in the next step)

                let mut has_type = false;
                
                match &field["value"] {
                    Ok(value_str) => schema.insert(field["name"].clone(), value_str.clone()),
                    
                    Err(e) if e.is_string() && matches!(e, "Unknown Column") => return Err(AlchemyDatabaseError::TypeMismatch("Unknown Column")), // Unknown column name (should be a type field like amount/price)
                     _ => { 
                        has_type = true; 
                         schema.insert(field["name"].clone(), value_str.clone());
                    }
                }

                if !has_type && matches!(field.get_string("type"), Some(String)) { // Unknown column name for unknown types (should be a type field like amount/price)
                     return Err(AlchemyDatabaseError::TypeMismatch("Unknown Column")); 
                }
            } else {
                 schema.insert(field["name"].clone(), value_str.clone());
            }
        }

        if let Some(ref missing_keys_set) = json_type.as_object().as_ref
