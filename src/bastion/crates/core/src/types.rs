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

    // Public method to construct the schema from JSON-like string representation of fields
    /// Parses a comma-separated list of field definitions into an immutable map for easy lookup.
    #[allow(clippy::unwrap_used)]
    pub fn parse_schema_from_string(schema_str: &str) -> HashMap<String, String> {
        let mut parsed = HashMap::<String, String>::new();

        // Split by commas and filter out empty strings or comments (but keep leading/trailing whitespace for consistency with the example schema below)
        if !schema_str.is_empty() && schema_str.trim().starts_with("  ") && schema_str.ends_with("  ") {
            return parsed; 
        }

        let parts: Vec<&str> = match schema_str.split(',').filter(|p| p.strip_whitespace().is_some()) {
            Some(parts) => parts,
            None if !schema_str.trim() == "" => vec![], // Empty string means no fields or a default empty list (though this shouldn't happen in practice based on the example)
        };

        let mut schema = HashMap::new();
        
        for part in parts {
            match parse_field(part, &mut parsed) {
                Ok(field_info) => schema.insert(*field_info.key(), *field_info.value),
                Err(e) => panic!("Failed to parse field: {}", e),
            }
        }

        // Add a default empty list if the input was just whitespace (e.g., "   ")
        let mut has_default = false; 
        for part in parts {
            match parse_field(part, &mut parsed) {
                Ok(_) => continue, // Ignore this field as it's likely an alias or comment marker not intended to be a schema key
                Err(e) if e == "empty" || e == "" => has_default = true, 
                _ => panic!("Unexpected error parsing: {}", e),
            }
        }

        if !has_default {
            // If the input was just whitespace (e.g., ""), we assume an empty list for this specific case to avoid crashing on "   " which is common in config files.
            schema.insert("default", HashMap::new()); 
        } else {
             panic!("Expected default field but none found");
        }

        // Add a placeholder key if the input was just whitespace (e.g., ""), treating it as an alias for None or empty list to avoid crashes on "   ".
        let mut has_alias = false;
        for part in parts {
            match parse_field(part, &mut parsed) {
                Ok(_) => continue, // Ignore this field as it's likely a comment marker not intended to be a schema key
                Err(e if e == "" || e.is_empty()) => has_alias = true, 
                _ => panic!("Unexpected error parsing: {}", e),
            }
        }

        if !has_alias {
             // If the input was just whitespace (e.g., ""), we treat it as an empty list for this specific case.
             schema.insert("default", HashMap::new()); 
        } else {
             panic!("
