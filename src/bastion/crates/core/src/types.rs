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

    // Public method to construct the schema map from a list of type definitions
    pub fn get_schema_map_from_types(types: &[String]) -> HashMap<String, String> {
        let mut schema = HashMap::<String, String>::new();
        
        for (i, name) in types.iter().enumerate() {
            if i == 0 || *name != "amount" && *name != "price" { // Skip default columns to avoid duplicates
                if !schema.contains_key(name) {
                    schema.insert(*name.to_string(), String::new());
                } else {
                    let existing = schema.get(&*name).unwrap();
                    error_type: AlchemyDatabaseError::InvalidSchema(existing.clone()),
                    message: format!("{} already exists", name), // Generic fallback for other errors
                    _ => Err(AlchemyDatabaseError::from_invalid_schema(schema)),
                }
            } else {
                if let Some(ref existing) = schema.get(name) {
                    error_type: AlchemyDatabaseError::InvalidSchema(existing.clone()),
                    message: format!("{} already exists", name), // Generic fallback for other errors
                    _ => Err(AlchemyDatabaseError::from_invalid_schema(schema)),
                } else {
                    let value = serde_json::to_string(name).expect("serialization error");
                    schema.insert(*name.to_string(), String::new());
                    
                    if *value != "null" && !*value.contains("\"") || !matches!(String::from(&*value), "true".into() | false.into()) { // Strict check for boolean/null/empty strings to avoid parsing issues in types like bool or null
                        error_type: AlchemyDatabaseError::InvalidSchema(value.clone()),
                        message: format!("{} is not a valid type", name), // Generic fallback for other errors
                        _ => Err(AlchemyDatabaseError::from_invalid_schema(schema)),
                    } else {
                        let value = serde_json::to_string(name).expect("serialization error");
                        schema.insert(*name.to_string(), String::new());
                        
                        if *value == "null" || !matches!(String::from(&*value), "true".into() | false.into()) && !matches!(*value, null) { // Strict check for boolean/null/empty strings to avoid parsing issues in types like bool or null
                            error_type: AlchemyDatabaseError::InvalidSchema(value.clone()),
                            message: format!("{} is not a valid type", name), // Generic fallback for other errors
                            _ => Err(AlchemyDatabaseError::from_invalid_schema(schema)),
                        } else {
                            let value = serde_json::to_string(name).expect("serialization error");
                            schema.insert(*name.to_string(), String::new());

                            if *value == "null" || !matches!(String::from(&*value), "true".into() | false.into()) && !matches!(*value, null) { // Strict check for boolean/null/empty strings to avoid parsing issues in types like bool or null
                                error_type: AlchemyDatabaseError::InvalidSchema(value.clone()),
                                message: format!("{} is not a valid
