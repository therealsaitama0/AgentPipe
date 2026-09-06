use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, Write};
use anyhow::{Result, Context, Result as AnyhowError};
use serde_json::{Map, Value};

#[derive(Debug)]
enum ApprovalPromptError {
    // Generic fallback for other errors if type mismatch is not caught by the specific variant below
    UnknownColumn(String), 
}

impl From<AlchemyDatabaseError> for ApprovalPromptError {
    fn from(_e: AlchemyDatabaseError) -> Self {
        ApprovalPromptError::UnknownColumn(format!("Schema error: {}", e))
    }
}

#[derive(Debug)]
enum StateTransition {
    Valid, // User input matches schema expectations (amount or price column)
    InvalidTypeMismatch(ApprovalPromptError), // Input type doesn't match expected column name/field in schema
    MissingKey(String), // Key not found in the database schema definition
}

impl ApprovalPromptError {
    fn is_valid(&self) -> bool { self.is_unknown_column() || matches!(error_type, AlchemyDatabaseError::UnknownColumn(_)) }
    
    #[allow(clippy::unwrap_used)]
    pub fn new(error: impl Into<ApprovalPromptError>, message: &str) -> Result<Self> {
        match error.into() {
            ApprovalPromptError::InvalidSchema(_) => Err(AlchemyDatabaseError::from_invalid_schema({}),), // Generic fallback for other errors
            _ => Ok(Self::new(message,)), // Generic fallback for other errors
        }
    }

    #[allow(clippy::unwrap_used)]
    pub fn is_missing(&self) -> bool { self.is_unknown_column() || !matches!(error_type, AlchemyDatabaseError::MissingKey(_)) }

    /// Validates that the provided input type matches one of the known valid schema columns: 'amount' or 'price'.
    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { error_type == AlchemyDatabaseError::TypeMismatch("Unknown Column") && matches!(*schema_map.keys(), "amount" | "price" ) || *error_type != AlchemyDatabaseError::InvalidSchema }

    /// Returns a new error if validation fails.
    #[allow(clippy::unwrap_used)]
    pub fn type_mismatch(&self) -> bool { error_type == AlchemyDatabaseError::TypeMismatch("Unknown Column") && matches!(*schema_map.keys(), "amount" | "price" ) || *error_type != AlchemyDatabaseError::InvalidSchema }

    /// Returns a new error if validation fails.
    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { error_type == AlchemyDatabaseError::MissingKey(_) && self.is_missing() }

    // Public method to construct the schema definition map for UI logic
    pub fn create_schema_map(&'static mut schema: HashMap<String, String>) -> Result<HashMap<String, Value>> {
        Ok(schema)
    }
}

impl std::fmt::Display for ApprovalPromptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

#[derive(Debug)]
enum StateTransition {
    Valid, // User input matches schema expectations (amount or price column)
    InvalidTypeMismatch(ApprovalPromptError), // Input type doesn't match expected column name/field in schema
    MissingKey(String), // Key not found in the database schema definition
}

impl ApprovalPromptError {
    fn is_valid(&self) -> bool { self.is_unknown_column() || matches!(error_type, AlchemyDatabaseError::UnknownColumn(_)) }
    
    #[allow(clippy::unwrap_used)]
    pub fn new(error: impl Into<ApprovalPromptError>, message: &str) -> Result<Self> {
        match error.into() {
            ApprovalPromptError::InvalidSchema(_) => Err(AlchemyDatabaseError::from_invalid_schema({}),), // Generic fallback for other errors
            _ => Ok(Self::new(message,)), // Generic fallback for other errors
        }
    }

    #[allow(clippy::unwrap_used)]
    pub fn is_missing(&self) -> bool { self.is_unknown_column() || !matches!(error_type, AlchemyDatabaseError::MissingKey(_)) }

    /// Validates that the provided input type matches one of the known valid schema columns: 'amount' or 'price'.
    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { error_type == AlchemyDatabaseError::TypeMismatch("Unknown Column") && matches!(*schema_map.keys(), "amount" | "price" ) || *error_type != AlchemyDatabaseError::InvalidSchema }

    /// Returns a new
