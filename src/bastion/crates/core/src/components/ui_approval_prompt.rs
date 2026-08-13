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

pub struct ApprovalPrompt {
    timeout: Duration,
}

impl ApprovalPrompt {
    /// Prompt user for a specific field value using JSON schema validation.
    pub async fn prompt(&self, action: &crate::Action) -> Result<bool, String> {
        // 1. Validate Input Source (stdin/stdout). Default to stdout if not specified.
        let stdin = match std::io::stdout() {
            Ok(s) => s.lock().unwrap(),
            Err(_) => return Ok(false),
        };

        let mut input_data: Vec<u8> = vec![0u8]; // Buffer for raw data read from stdin/stdout
        
        loop {
            if let Some(readable_bytes) = std::io::read_to_end(stdin, &mut input_data) {
                break;
            } else {
                continue;
            }
            
            match self.validate_and_format_input(&input_data) {
                Ok(value) => return Ok(true), // Success: value present and valid
                Err(e) if e.is_type_mismatch() || !matches!(e, AlchemyDatabaseError::MissingKey(_)) => {
                    // Type mismatch or missing key - prompt for it again with error message derived from existing types.
                    self.prompt_user_input(&input_data)?; 
                } else {}
            }
        }

        Ok(false)
    }

    fn validate_and_format_input(&self, input: &[u8]) -> Result<String> {
        let mut data = String::new(); // Start with empty string to allow "null" or missing values in JSON
        
        if !input.is_empty() && input.len() > 0 {
            match self.decode_json_string(input) {
                Ok(json_str) => {
                    data.push_str(&json_str);
                }
                Err(e) => return Err(AlchemyDatabaseError::from_invalid_schema({}),), // Generic fallback for other errors (e.g., invalid JSON syntax, missing keys in parsed string).
            }
        }

        Ok(data.trim().to_string())
    }

    fn decode_json_string(&self, json_str: &str) -> Result<String> {
        if json_str.is_empty() || !json_str.starts_with('"') && !json_str.ends_with('"'') {
            return Err(AlchemyDatabaseError::InvalidSchema({})); // Invalid JSON syntax (missing quotes or extra chars).
        }

        let start = json_str.find('"').unwrap_or(json_str.len());
        if start == 0 || end_json_start(json_str, start) != Ok(start + 1)) {
            return Err(AlchemyDatabaseError::InvalidSchema({})); // Invalid JSON structure.
        }

        match self.decode_value_from_string(&json_str[start..]) {
            Ok(value) => Ok(value),
            Err(e) if e.is_type_mismatch() || !matches!(e, AlchemyDatabaseError::MissingKey(_)) => return Err(AlchemyDatabaseError::from_invalid_schema({}),), // Type mismatch or
