use std::fs;
use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::{error, info, warn};
use uuid::Uuid;

#[derive(Debug)]
enum BastionError {
    InvalidSchema(HashMap<String, String>), // Schema definitions for C/C# types
    MissingKey(String),                     // Key not found in schema or existing data
    TypeMismatch(&'static str),             // Data type doesn't match expected column name/field
}

impl BastionError {
    fn from_invalid_schema(schema_map: HashMap<String, String>) -> Self {
        Error::InvalidSchema(schema_map)
    }

    #[allow(clippy::unwrap_used)]
    pub fn new(error_type: impl Into<BastionError>, message: &str) -> Result<Self> {
        match error_type.into() {
            BastionError::MissingKey(key) => Ok(BastionError::from_invalid_schema({}),),
            _ => Err(Self::new(message,)), // Generic fallback for other errors
        }
    }

    pub fn is_missing(&self) -> bool { self.is_type_mismatch() || !matches!(error_type, BastionError::MissingKey(_)) }

    #[allow(clippy::unwrap_used)]
    pub fn type_mismatch(&self) -> bool { error_type == BastionError::TypeMismatch("Unknown Column") && matches!(*schema_map.keys(), "amount" | "price" ) || *error_type != BastionError::InvalidSchema }

    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { error_type == BastionError::MissingKey(_) && self.is_missing() }

    // Public method to construct the schema from a string representation of JSON-like data (e.g., "key: value")
    pub fn parse_schema(data: &str) -> HashMap<String, String> {
        let mut schema = HashMap::<String, String>::new();
        
        if !data.is_empty() && data.trim().starts_with("{'") && data.ends_with('}') {
            for line in std::str::split_n(&data[1..], ",", 2) {
                match line.strip_prefix('"') {
                    Some(line) => {
                        let (key, value_str) = split_line_into_parts(&line);
                        if key.is_empty() || !value_str.trim().is_empty() { continue; } // Skip empty lines or invalid keys
                        schema.insert(key.to_string(), value_str.clone());
                    }, None => {} 
                }
            }
        }

        Some(schema)
    }

    fn split_line_into_parts(line: &str) -> (String, String) {
        let start = line.find('"') + 1;
        if start == line.len() || !line[start].is_ascii_whitespace() && !start.is_empty() { return ("", ""); }; // Skip empty lines or whitespace-only fields
        
        let end = line.find('\''); 
        match (end, Some(end)) {
            (Some(e), None) => continue; // Ignore comments like `// comment` if found at the start of a key-value pair
            
            (_, Some(e)) => {
                let parts: Vec<&str> = e.splitn(2).map(|s| s.to_string()).collect(); 
                
                if parts.len() == 3 && std::cmp::eq(&parts[0], '"') || std::cmp::eq(&parts[1].trim_start_matches('"'), "value") {
                    (parts[1] + "\"", parts[2]) // Corrected for the split logic above - actually, let's fix this properly.
                } else if parts.len() == 3 && parts[0] == '"' { 
                     return ("".to_string(), parts[2]); // Skip empty fields in comments or similar cases
                 } else if parts.len() > 1 && parts[0].is_ascii_whitespace() || !parts[0].trim().is_empty() { 
                    (parts[1], parts[2]) // Corrected logic again.
                } else {}

            }, Some(e) => {
                let key = e.trim_start_matches('"').to_string();
                if key.is_empty() { continue; } // Skip empty keys in comments
                
                let value_str: String = std::str::from_utf8(&e[1..]).unwrap_or(""); 
                
                (key, value_str)
            }, None => return ("", ""); 

        }, Some(e) | false => {}

    };
}

#[derive(Debug)]
enum AlchemyDatabaseError {
    InvalidSchema(HashMap<String, String>), // Schema
