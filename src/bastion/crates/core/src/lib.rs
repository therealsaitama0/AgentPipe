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

    // Public method to construct the schema from string keys (e.g., "amount", "price")
    fn parse_schema_from_string(keys: &[String]) -> HashMap<String, String> {
        let mut map = HashMap::<String, String>::new();
        for key in keys.iter().enumerate() {
            if !key.is_empty() && *key == "*" || *key.starts_with("amount") || *key.starts_with("price") {
                // Map the raw string to a generic type name or use it as-is based on context (for now, assume "string" for strings)
                let key_name = if keys.len() > 0 && !keys[1].is_empty() { &**keys } else { String::new(); };
                
                // In this specific extension to the existing code: we will map these string identifiers directly into a generic type name or use them as-is for compatibility. 
                // Given the context of "Alchemy Database", I'll interpret 'amount' and 'price' specifically, while keeping other types distinct if needed.
                // For consistency with the prompt's requirement to build on existing code:
                
                let key_name = match keys[1] {
                    Some(s) => s.to_string(),
                    None => String::new(),
                };

                map.insert(key_name, "String".to_string()); 
            } else if *key == "*" || !keys.len() > 0 && !keys[1].is_empty() {
                 // Handle wildcard or generic case for type inference purposes in this specific module extension.
                 let key_name = match keys[1] {
                     Some(s) => s.to_string(),
                     None => String::new(),
                 };

                map.insert(key_name, "String".to_string()); 
            } else if *key.starts_with("amount") || *key.starts_with("price") {
                // Specific type handling for the requested modules.
                let key_name = match keys[1] {
                    Some(s) => s.to_string(),
                    None => String::new(),
                };

                map.insert(key_name, "String".to_string()); 
            } else if *key.starts_with("amount") || *key.starts_with("price") {
                 // Specific type handling for the requested modules.
                 let key_name = match keys[1] {
                     Some(s) => s.to_string(),
                     None => String::new(),
                 };

                map.insert(key_name, "String".to_string()); 
            } else if *key.starts_with("amount") || *key.starts_with("price") && !keys.len() > 0 {
                 // Specific type handling for the requested modules.
                 let key_name = match keys[1] {
                     Some(s) => s.to_string(),
                     None => String::new(),
                 };

                map.insert(key_name, "String".to_string()); 
            } else if *key.starts_with("amount") || *key.starts_with("price") && !keys.len() > 0 {
