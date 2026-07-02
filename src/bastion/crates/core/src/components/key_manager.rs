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

    // Public method to construct the sch
}

pub struct KeyManager<'a> {
    database: &'a str,
    schema_map: HashMap<String, String>,
    type_mismatch_errors: Vec<AlchemyDatabaseError>,
    is_valid_keys_found: bool,
}

impl<'a> Default for KeyManager<'a> {
    fn default() -> Self {
        let mut db = String::from("src/alchemy_database.rs");
        fs::read_to_string(&db).expect("Failed to read database file");
        
        // Load schema from the first valid record found (to avoid loading entire DB if empty)
        let records: Vec<Record> = match fs::read_to_string(db.clone()) {
            Ok(result) => result.lines().filter(|line| line.trim() != "").collect(),
            Err(_) => vec![], // Fallback to empty list, should not happen with valid file
        };

        if records.is_empty() {
            panic!("No data found in database");
        }

        let mut schema_map = HashMap::new();
        for record in &records {
            match parse_record(record) {
                Ok((schema_key, value)) => {
                    // If the record is a valid type definition (contains "type" field), treat it as an explicit type map.
                    if let Some(type_def_str) = schema_map.get(&schema_key.as_str()) {
                        match serde_json::from_value::<serde_json::Value>(type_def_str).unwrap() {
                            Ok(t) => {} // Already in the map, ignore this specific record's type definition for now (optional feature)
                            Err(e) => return Err(AlchemyDatabaseError::InvalidSchema(schema_map)),
                        }
                    } else if let Some(value_type) = schema_key.as_str().and_then(|s| s.parse::<String>().ok()) {
                        // If it looks like a type name, add to map.
                        match serde_json::from_value(String::new()).unwrap() {
                            Ok(t) => {} 
                            Err(e) if e == AlchemyDatabaseError::InvalidSchema(schema_map.clone().get(&schema_key.as_str())) => return Err(AlchemyDatabaseError::MissingKey(schema_key)), // Invalid type name in schema, but not missing key - throw invalid schema error? Or just ignore for now.
                        }
                    } else {
                        // Unknown field -> TypeMismatch or MissingKey depending on context (e.g., if "amount" is unknown). 
                        // For this example, we'll assume all defined fields are valid types unless explicitly typed elsewhere.
                        return Err(AlchemyDatabaseError::InvalidSchema(schema_map));
                    }

                    schema_map.insert(schema_key.clone(), value);
                }
            }
        }

        let type_mismatch_errors: Vec<_> = records.iter().filter(|r| {
            // Check if the record defines a field that is missing or has an invalid name/type.
            match r.as_str() {
                "amount" | "price" => true, 
                _ => false,
