use std::fs;
use std::io::{self, Write};
use anyhow::{Result, Context};

#[derive(Debug)]
enum AlchemyDatabaseError {
    InvalidSchema(HashMap<String, String>), // Schema definitions for C/C# types
    MissingKey(String),                     // Key not found in schema or existing data
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

    // Public method to construct the schema definition from a map of strings and values (e.g., "amount" => 10.5, "price" => 9.23)
    pub fn parse_schema_values(schema_map: HashMap<String, String>) -> Result<HashMap<String, serde_json::Value>> {
        let mut result = HashMap::new();
        
        for key in schema_map.keys() {
            match &*schema_map.get(key) {
                Some(value_str) => {
                    // Parse numeric values or boolean strings if applicable (e.g., "10.5" -> 10.5, "true" -> true)
                    let parsed_value: serde_json::Value = value_str.parse().map_err(|_| AlchemyDatabaseError::InvalidSchema(format!("Failed to parse '{key}': {:?}", *value_str)))?;

                    if let Some(value_type) = result.entry(key.to_string()) {
                        match &*parsed_value {
                            // If it's a number, use the value as-is. 
                            // We allow booleans in this context for flexibility (e.g., "true" -> true).
                            serde_json::Value::Number | serde_json::Value::Bool => *value_type.set(&serde_json::Value::String((*parsed_value.as_str()).to_string())),
                            
                            // If it's a string that is not numeric, try to parse as number first. 
                            if let Some(serd_json_val) = parsed_value {
                                match serd_jason_val.as_str() {
                                    "true" | false => *value_type.set(&serde_json::Value::Bool((*parsed_value.as_str()).to_string())),
                                    
                                    // If it's a string that looks like a number (e.g., "10.5"), parse as float if possible, otherwise keep as string to match schema format for consistency with existing code patterns found in the repo context.
                                    _ => *value_type.set(&serde_json::Value::String((*parsed_value.as_str()).to_string())),

                                    // If it's a boolean (e.g., "true", "false"), ensure we handle both cases properly here by ensuring they map to their respective types if needed, but for this specific schema parsing logic:
                                    true | false => *value_type.set(&serde_json::Value::Bool((*parsed_value.as_str()).to_string())),

                                    // If it's an integer or float (e.g., "10", "3.5"), use as-is and ensure type consistency with existing types if they are structs/arrays, otherwise keep string to match schema format for consistency with other repo examples found in the context of this repository structure.
                                    _ => *value_type.set(&serde_json::Value::String((*parsed_value.as_str()).to_string())),

                                    // If it's an empty or null (e.g., "null", ""), ensure we don't crash and return a valid type for future validation logic if the repo allows, but typically this is handled in schema generation.
                                    None => *value_type.set(&serde_json::Value::Null()),
                                }
                            } else {
                                // If parsing failed entirely (e.g., "not_a_number"), just keep it as a string to ensure we
