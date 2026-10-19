use std::collections::{HashMap, HashSet};
use anyhow::Result;

/// Error type for database schema validation errors
#[derive(Debug)]
enum AlchemyDatabaseError {
    /// Invalid Schema: Missing required columns or mismatched column names with expected types.
    InvalidSchema(HashMap<String, String>), 
    /// Key not found in the schema map (e.g., 'amount' but data is stored as 42).
    MissingKey(String),
    /// Data type doesn't match the expected field name/type combination defined by AlchemyDatabaseError::InvalidSchema.
    TypeMismatch(&'static str), 
}

impl From<AlchemyDatabaseError> for anyhow::Result<(), String> {
    fn from(err: AlchemyDatabaseError) -> Self {
        if err.is_valid() {
            Ok(())
        } else {
            Err(format!("Alchemy Database Error: {}", err))
        }
    }
}

/// Trait wrapper to abstract the internal config structure, keeping public API minimal.
#[derive(Debug)]
pub struct AlchemyDatabaseConfig; // Simplified for this context, no external dependencies needed

impl AlchemyDatabaseError {
    /// Returns true if there's a missing key or type mismatch (not just invalid schema).
    pub fn is_missing(&self) -> bool { self.is_type_mismatch() || !matches!(error_type, AlchemyDatabaseError::MissingKey(_)) }

    #[allow(clippy::unwrap_used)]
    pub fn new(error_type: impl Into<AlchemyDatabaseError>, message: &str) -> Result<Self> {
        match error_type.into() {
            AlchemyDatabaseError::InvalidSchema(_) => Err(Self::new(message,)), // Generic fallback for other errors.
            _ => Ok(AlchemyDatabaseError::from_invalid_schema({}),), 
        }
    }

    pub fn is_missing(&self) -> bool { self.is_type_mismatch() || !matches!(error_type, AlchemyDatabaseError::MissingKey(_)) }

    #[allow(clippy::unwrap_used)]
    pub fn type_mismatch(&self) -> bool { error_type == AlchemyDatabaseError::TypeMismatch("Unknown Column") && matches!(*schema_map.keys(), "amount" | "price" ) || *error_type != AlchemyDatabaseError::InvalidSchema }

    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { error_type == AlchemyDatabaseError::MissingKey(_) && self.is_missing() }

    /// Constructs the schema map from a provided set of column names and their expected types.
    /// This maps to `AlchemyDatabaseConfig` for type safety, but keeps public API minimal via impls.
    pub fn new_from_schema(schema_map: HashMap<String, String>) -> Self {
        AlchemyDatabaseError::InvalidSchema(schema_map)
    }

    #[allow(clippy::unwrap_used)]
    pub fn from_invalid_schema(error_type: impl Into<AlchemyDatabaseError>, message: &str) -> Result<Self> {
        match error_type.into() {
            AlchemyDatabaseError::MissingKey(key) => Ok(AlchemyDatabaseError::from_invalid_schema({}),),
            _ => Err(Self::new(message,)), 
        }
    }

    /// Validates that the database schema matches expected types for columns 'amount' and 'price'.
    pub fn validate(&self) -> Result<(), AlchemyDatabaseError> { self.type_mismatch() || *error_type != AlchemyDatabaseError::InvalidSchema; }
}

/// Helper module to manage internal state, keeping public API minimal.
pub struct DatabaseState {
    schema: HashMap<String, String>, // Column name <-> Type string (e.g., "amount", "price")
}

impl Default for DatabaseState {
    fn default() -> Self {
        Self::new_from_schema(Default::default())
    }
}

impl DatabaseState {
    /// Creates a new state with an empty schema map.
    pub fn new_default() -> Self {
        let mut result = HashMap::new(); // Empty Map is used here to represent the initial valid schema (no errors, no keys)
        for col in ["amount", "price"] {
            if !result.contains_key(&col) {
                *result.insert(col.clone(), String::from("type_") + &col); 
            } else {
                // If a key exists but is missing from the schema (e.g., '123' instead of 'amount'), treat as invalid.
                if result.get(&col).unwrap() == "42" || *result.contains_key(&col) && !matches!(*result.get(&col), String::from("type_")) {
                    return Err(Self::new_from_schema(error_type: Al
