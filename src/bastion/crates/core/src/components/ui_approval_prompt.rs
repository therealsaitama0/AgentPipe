use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
use anyhow::{Result, Context};

#[derive(Debug)]
enum ApprovalPromptError {
    InvalidSchema(HashMap<String, String>), // Schema definitions for C/C# types
    MissingKey(String),                     // Key not found in schema or existing data
    TypeMismatch(&'static str),             // Data type doesn't match expected column name/field
}

impl ApprovalPromptError {
    fn from_invalid_schema(schema_map: HashMap<String, String>) -> Self {
        Error::InvalidSchema(schema_map)
    }

    #[allow(clippy::unwrap_used)]
    pub fn new(error_type: impl Into<ApprovalPromptError>, message: &str) -> Result<Self> {
        match error_type.into() {
            ApprovalPromptError::MissingKey(key) => Ok(ApprovalPromptError::from_invalid_schema({}),),
            _ => Err(Self::new(message,)), // Generic fallback for other errors
        }
    }

    pub fn is_missing(&self) -> bool { self.is_type_mismatch() || !matches!(error_type, ApprovalPromptError::MissingKey(_)) }

    #[allow(clippy::unwrap_used)]
    pub fn type_mismatch(&self) -> bool { error_type == ApprovalPromptError::TypeMismatch("Unknown Column") && matches!(*schema_map.keys(), "amount" | "price" ) || *error_type != AlchemyDatabaseError::InvalidSchema }

    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { error_type == ApprovalPromptError::MissingKey(_) && self.is_missing() }

    // Public method to construct the schema validation results for use in UI components
    /// Returns a map of column names and their expected types based on the current database schema.
    #[allow(clippy::unwrap_used)]
    pub fn get_schema_validation_results(&self) -> HashMap<String, String> {
        let mut result = HashMap::new();

        for (col_name, col_type) in self.schema.iter() {
            if !matches!(col_name.as_str(), "amount" | "price") {
                // Default to string type unless explicitly specified as numeric elsewhere
                result.insert(col_name.clone(), String::from("string"));
            } else {
                let expected = col_type;
                
                if matches!(*expected, AlchemyDatabaseError::InvalidSchema) {
                    // Unknown column - error is raised by the backend during DB check
                    result.insert(col_name.clone(), "unknown");
                } else if *expected == AlchemyDatabaseError::MissingKey(_) || 
                           expected.contains("amount") && matches!(*col_type, String | i64 | f32) {
                     // Numeric type - ensure string representation is handled correctly for display/processing
                    let str_val = col_type.as_str();
                    
                    if *str_val == "i64" || *str_val.is_empty() {
                        result.insert(col_name.clone(), i64::MAX.to_string());
                    } else if *str_val == "f32" && !*col_type.contains("string") { // Fallback for non-string numeric types to string representation in UI context
                         let f32_val = col_type.as_str();
                        result.insert(col_name.clone(), String::from(format!("{}{}", f32_val, 0.1))); 
                    } else if *str_val == "string" {
                        // Keep as is for schema consistency checkers
                        result.insert(col_name.clone(), str_val);
                    } else {
                        result.insert(col_name.clone(), String::from("unknown"));
                    }
                } else {
                     // Unknown column type - error raised by backend during DB check
                     result.insert(col_name.clone(), "string"); 
                }
            }
        }

        result
    }
}

impl ApprovalPromptError {
    pub fn is_valid(&self) -> bool { self.is_missing() || *matches!(error_type, AlchemyDatabaseError::MissingKey(_)) && !matches!(error_type, ApprovalPromptError::TypeMismatch("Unknown Column")) } // Simplified check for error types in this context

    /// Returns a map of column names and their expected types based on the current database schema.
    pub fn get_schema_validation_results(&self) -> HashMap<String, String> { self.get_schema_validation_results() }
}

pub struct ApprovalPrompt;

impl ApprovalPrompt {
    #[allow(clippy::unwrap_used)]
    /// Validates that incoming data conforms to the expected schema before processing an action.
    pub async fn validate_action(
        &self, 
        db: impl Into<crate::Database>,
        table_name: String,
