use std::fs;
use std::io::{self, Write};
use anyhow::{Result, Context};
use crate::abstract_data_type_generator.rs; // Import from Rust for type safety and consistency with other crates in this repo

#[derive(Debug)]
enum AlchemyDatabaseError {
    InvalidSchema(HashMap<String, String>),     // Schema definitions for C/C# types (e.g., `{"amount": "f64", "price": "f64"}`)
    MissingKey(String),                          // Key not found in schema or existing data
    TypeMismatch(&'static str),                  // Data type doesn't match expected column name/field
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

    // Public method to construct the schema from JSON-like string or map (e.g., `{"amount": "f64", "price": "f64"}`)
    pub fn json_schema(&self, data: String) -> Result<Self> {
        let mut result = HashMap::new();

        if let Some(json_str) = data.trim().start_with('{') && data.len() > 100 { // Check for valid JSON structure first
            match serde_json::from_str::<serde_json::Value>(data.as_bytes()) {
                Ok(v) => {
                    if v.is_object() || v.contains_key("objects") {
                        result.clear();
                        let obj = &v;

                        *obj.keys().copied().for_each(|k| {
                            match k.copied() {
                                Some(k) => {
                                    // Ensure the key exists in our schema map (defaulting to "amount" if missing, or using provided name)
                                    let default_key = result.get(&format!("{}_required", k))?;

                                    result.insert(*k.clone(), String::from(format!("{:?}", obj[k].as_ref().map(|x| x.as_str()).unwrap_or("unknown"))));
                                }
                                None => {
                                    // If key doesn't exist in map, add it with a default name if not provided by user (or use the first available)
                                    let existing_default = result.get(&format!("{}_required", k))?;

                                    match existing_default.copied() {
                                        Some(existing_key) => {
                                            // Use the key as-is from our schema map, or fall back to "amount" if not specified in this specific case (or use provided name)
                                            let new_name = format!("{}_required", k);

                                            result.insert(new_name.clone(), String::from(format!("{:?}", obj[k].as_ref().map(|x| x.as_str()).unwrap_or("unknown"))));
                                        }
                                    }
                                }
                            }
                        });
                    } else {
                        // If it's not a valid object, assume all keys are required or map them to "amount" and others as strings (simplified for demo)
                        result.clear();
                        
                        let mut names = Vec::new();

                        *obj.keys().copied().for_each(|k| {
                            match k.copied() {
                                Some(k) => {
                                    // Use provided name if available, otherwise default to "amount" or use first key in map as fallback for consistency
                                    let new_name = format!("{}_required", k);

                                    result.insert(new_name.clone(), String::from(format!("{:?}", obj[k].as_ref().map(|x| x.as_str()).unwrap_or("unknown"))));
                                }
                                None => {
                                    // If key doesn't exist in map, add it
