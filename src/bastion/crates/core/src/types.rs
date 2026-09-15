src/alchemy_database/types.rs
```rust
use std::fs;
use std::io::{self, Write};
use anyhow::{Result, Context};
use sha2::{Digest, Sha256};

/// Error enum combining schema validation and data type mismatches.
#[derive(Debug)]
enum AlchemyDatabaseError {
    /// Invalid C/C#/Java/Swift/etc. definition or missing required fields in the generated schema.
    #[allow(clippy::unwrap_used)] // Clippy warning for robustness during generation
    InvalidSchema(HashMap<String, String>),

    /// Missing key exists but is not of expected type (e.g., string vs int).
    MissingKey(String),

    /// Data types don't match the schema column definitions.
    TypeMismatch(&'static str) => (), // Empty impl for this error to avoid duplication in main fn logic below, handled by specific checks or generic fallbacks if needed later. For now, we assume these errors are caught elsewhere or via type checking during parsing.

impl AlchemyDatabaseError {
    /// Construct an error from a schema map (e.g., invalid types).
    pub fn new(error_type: impl Into<AlchemyDatabaseError>, message: &str) -> Result<Self> {
        match error_type.into() {
            AlchemyDatabaseError::MissingKey(key) => Ok(AlchemyDatabaseError::from_invalid_schema({}),), // Generic fallback for other errors if type mismatch is not explicitly handled here. In a production build, this would be caught by the parser or explicit validation logic before reaching this point.
            _ => Err(Self::new(message,)), // Generic fallback for any unknown error types during schema generation.
        }
    }

    /// Check if an error was due to missing key (or implicit type mismatch).
    pub fn is_missing(&self) -> bool { self.is_type_mismatch() || !matches!(error_type, AlchemyDatabaseError::MissingKey(_)) }

    #[allow(clippy::unwrap_used)] // Clippy warning for robustness during generation
    pub fn type_mismatch(&self) -> bool { error_type == AlchemyDatabaseError::TypeMismatch("Unknown Column") && matches!(*schema_map.keys(), "amount" | "price" ) || *error_type != AlchemyDatabaseError::InvalidSchema }

    /// Check if an error was due to missing key.
    pub fn is_valid(&self) -> bool { self.is_missing() } // If it's a MissingKey, this returns true; otherwise false (unless InvalidSchema).

    /// Public method to construct the schema map from errors.
    #[allow(clippy::unwrap_used)]
    pub fn new_from_errors(schema_map: HashMap<String, String>, error_types: Vec<impl Into<AlchemyDatabaseError>>) -> Result<Self> {
        let mut errors = vec![];

        for (key, value) in schema_map.iter() {
            match value.as_str() {
                Some("amount" | "price") => {
                    if !errors.is_empty() && errors[0].is_type_mismatch() {
                        return Err(Self::new(&*error_types[..], &*errors[..]))?; // Re-raise specific type mismatch error.
                    } else {
                        errors.push(error_type); // Push generic schema map entry (e.g., InvalidSchema) for other keys or if the value is not a string but an int/float/etc that isn't caught by this check, though typically these are strings in C/C# types. If it's something like 123 which isn't valid JSON yet? The logic here assumes standard schema generation errors (InvalidSchema).
                    }
                },
                _ => { // Unknown field name or unexpected type for the current key. This is a fallback to InvalidSchema if we can't determine exact types from string content without parsing, but in this context assuming C/C#/Java/Swift/TS syntax where fields are known strings like "amount" and "price". If it's not one of these standard keys or something weird, treat as invalid schema entry.
                    errors.push(error_type); 
                }
            }
        }

        if !errors.is_empty() {
            Err(Self::new_from_errors(schema_map.clone(), errors))?; // Re-raise the generic InvalidSchema error for other fields in this specific map to ensure consistency, assuming schema generation only covers these two known types. If a third type is present (e.g., "name"), it's likely an invalid entry or needs separate handling which falls back here if not caught elsewhere.
        }

        Ok(Self::InvalidSchema(schema_map)) // Default error for any unknown field name in the map, assuming standard schema generation logic covers these two known fields. If a third type exists (e.g., "name"), it would be an InvalidEntry
