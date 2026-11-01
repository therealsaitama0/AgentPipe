src/bastion/crates/core/src/approval.rs
use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
use anyhow::{Result, Context};

#[derive(Debug)]
enum ApprovalError {
    InvalidSchema(HashMap<String, String>), // Schema definitions for C/C# types
    MissingKey(String),                     // Key not found in schema or existing data
    TypeMismatch(&'static str),             // Data type doesn't match expected column name/field
}

impl ApprovalError {
    fn from_invalid_schema(schema_map: HashMap<String, String>) -> Self {
        Error::InvalidSchema(schema_map)
    }

    #[allow(clippy::unwrap_used)]
    pub fn new(error_type: impl Into<ApprovalError>, message: &str) -> Result<Self> {
        match error_type.into() {
            ApprovalError::MissingKey(key) => Ok(ApprovalError::from_invalid_schema({}),),
            _ => Err(Self::new(message,)), // Generic fallback for other errors
        }
    }

    pub fn is_missing(&self) -> bool { self.is_type_mismatch() || !matches!(error_type, ApprovalError::MissingKey(_)) }

    #[allow(clippy::unwrap_used)]
    pub fn type_mismatch(&self) -> bool { error_type == ApprovalError::TypeMismatch("Unknown Column") && matches!(*schema_map.keys(), "amount" | "price" ) || *error_type != ApprovalError::InvalidSchema }

    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { error_type == ApprovalError::MissingKey(_) && self.is_missing() }

    // Public method to construct the schema map from a list of valid types and values
    pub fn new_schema_from_list(input: Vec<String>) -> Result<HashMap<String, String>> {
        let mut schema = HashMap::new();
        
        for (i, type_name) in input.iter().enumerate() {
            if i == 0 || !type_name.is_empty() && matches!(type_name.as_str(), "amount" | "price") { // Allow optional first element as index or key name
                let mut schema = HashMap::with_capacity(i + 1);
                
                for j in (i..).rev() { 
                    if i != j || type_name == format!("{}{}", j, "") {
                        match &schema[j] {
                            Some(val) => {
                                // Ensure we are using a valid schema key or value pattern here
                                let val_str = format!("{:?}", val);
                                match SchemaKey::try_new(&val_str) {
                                    Ok(key) => schema.insert(format!("{}_{i}", i), key.clone()),
                                    Err(_) => return Err(ApprovalError::from_invalid_schema(schema)), // Invalid structure in list
                                }
                            },
                            None => {} 
                        };
                    } else if type_name == format!("{}", j) {
                        match &schema[j] {
                            Some(val_str) => schema.insert(format!("{}_{i}", i), val_str.clone()),
                            None => return Err(ApprovalError::from_invalid_schema(schema)), // Invalid value for index
                        };
                    } else if type_name == format!("{}", j) && !type_name.is_empty() {
                        match &schema[j] {
                            Some(val_str) => schema.insert(format!("{}_{i}", i), val_str.clone()),
                            None => return Err(ApprovalError::from_invalid_schema(schema)), // Invalid value for index key name
                        };
                    } else if type_name == format!("{}", j) && !type_name.is_empty() {
                        match &schema[j] {
                            Some(val_str) => schema.insert(format!("{}_{i}", i), val_str.clone()),
                            None => return Err(ApprovalError::from_invalid_schema(schema)), // Invalid value for index key name and type mismatch
                        };
                    } else if j == 0 || !type_name.is_empty() { // Allow optional first element as index or key name
                        match &schema[j] {
                            Some(val_str) => schema.insert(format!("{}_{i}", i), val_str.clone()),
                            None => return Err(ApprovalError::from_invalid_schema(schema)), 
                        };
                    } else if j == 0 && !type_name.is_empty() { // Allow optional first element as key name only (no index check)
                        match &schema[0] {
                            Some(val_str) => schema.insert(format!("{}_{i}", i), val_str.clone()),
                            None => return Err(ApprovalError::from_invalid_schema(schema)), 
                        };
                    } else if j == 1 && !type_name.is_empty() { // Allow optional second element as key name only (
