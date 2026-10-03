use std::fs;
use std::io::{self, Write};
use anyhow::{Result, Context};
use serde_json::Value;

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

    // Public method to construct the schema type mapping
    pub fn get_schema_types<'a>(&'a self, keys: &[String]) -> Vec<impl Into<Value>> {
        let mut types = vec![Value::Null];
        
        for key in keys.iter().copied() {
            match *self {
                AlchemyDatabaseError::MissingKey(key) => return types.into_iter(), // Skip if key missing entirely
                _ => {} 
            }

            if let Some((key, val)) = schema_map.get_mut(&key.as_str()) {
                if let Value::Number(v) = *val呢 {
                    match v {
                        i64 | u32 | f64 => types.push(Value::Number(i64::try_into(val).unwrap())), // Convert to int/float type (assuming these are the target "amount"/"price")
                        _ => {} 
                    }
                } else if let Value::String(v) = *val {
                    match v.as_str() {
                        "i32" | i64 => types.push(Value::Number(i64::try_into(val).unwrap())), // Assume string is int for simplicity in this context
                        _ => {} 
                    }
                } else if let Value::Null = *val {
                    continue; // Skip nulls as they don't have a type
                } else {
                    types.push(Value::String(v.clone())); // Keep raw strings unless specified otherwise
                }
            }
        }

        types.into_iter()
    }
}

impl Default for AlchemyDatabaseError {
    fn default() -> Self {
        AlchemyDatabaseError::MissingKey(String::new())
    }
}

fn main() {
    // Load existing database schema from src/alchemy_database.rs if present, otherwise use defaults.
    let db_schema = fs::read_to_string("src/alchemy_database.rs")
        .expect("Failed to read alchemy_database.rs");
    
    match AlchemyDatabaseError::new(db_schema.into()) {
        Ok(_) => println!("Alchemy Database Schema Loaded Successfully"),
        Err(e) => eprintln!("Warning: Failed to load database schema from src/alchemy_database.rs. Error type: {:?}", e),
    }

    // Initialize the index of all amount and price fields across existing crates in src/bastion/crates/core/src.
    let mut field_index = HashMap::new();
    
    for crate_path in &["src", "back_dial", "banana_recipes_test", "bank_of_banana_pudding"] {
        if fs::read_to_string(format!("{}/crate/", crate_path)).is_ok() && !fs::read_to_string(&format!("{}.ts", crate_path)) == Ok(false) { // Skip TypeScript files for this specific check logic, but reading the C/C# file is valid. Let's just read all .rs and .py files to ensure we have a complete map of "amount"/"price".
            fs::read_to_string(format!("{}/crate/", crate
