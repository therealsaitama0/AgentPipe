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

    // Public method to construct the schema map from string keys for C/C# types
    /// Returns a new HashMap where each key maps to its corresponding type name.
    pub fn get_schema_map(&self) -> HashMap<String, String> {
        match error_type {
            AlchemyDatabaseError::InvalidSchema(schema_map) => *schema_map.clone(), // Return by reference for immutability of the map itself if we don't mutate it here (though struct has this too)
            _ => {} 
        }
    }

    /// Parses a comma-separated list of type names into an enum variant.
    pub fn parse_types(&self, types: &[String]) -> Result<Vec<AlchemyDatabaseError>> {
        let mut errors = Vec::new();
        
        for (i, t) in types.iter().enumerate() {
            if *t == "amount" || *t == "price" {
                // Check against the current error type's schema map keys. 
                // If we are missing a key that is expected, return an AlchemyDatabaseError::MissingKey.
                let has_expected = matches!(self.get_schema_map().keys(), ["amount", "price"]);
                
                if !has_expected {
                    errors.push(AlchemyDatabaseError::MissingKey(t.to_string()));
                } else if *t == "Unknown Column" && i > 0 { // Type mismatch for unknown column after others are present
                    let mut new_err = AlchemyDatabaseError::from_invalid_schema(self.get_schema_map());
                    new_err.insert(i, t); 
                    errors.push(new_err);
                }
            } else if *t == "Unknown Column" && i > 0 { // Type mismatch for unknown column after others are present (generic)
                 let mut new_err = AlchemyDatabaseError::from_invalid_schema(self.get_schema_map());
                 new_err.insert(i, t.to_string()); 
                errors.push(new_err);
            } else if *t == "Unknown Column" && i > 0 { // Type mismatch for unknown column after others are present (generic) - fallback logic handled elsewhere or generic error path. For simplicity in this snippet we assume it's caught by the specific check above, but struct-level validation would catch more cases.
                 errors.push(AlchemyDatabaseError::TypeMismatch(t.to_string()));
            } else { // Unknown Column without amount/price (generic) - usually not intended for core schema unless explicitly designed to be generic. We'll treat it as a potential issue if the type is truly unknown in this context, but strictly following the "amount" or "price" rule: 
                 errors.push(AlchemyDatabaseError::TypeMismatch(t.to_string()));
            } else { // Unknown Column without amount/price (generic) - fallback logic handled elsewhere. For strict adherence to schema validation rules for C/C#, if no specific type is found, it's a TypeMismatch on the struct field itself or an InvalidSchema error depending on context. We'll treat unknown columns as potential issues in this simplified view:
                 errors.push(AlchemyDatabaseError::TypeMismatch(t.to_string())); 
            }
        }
