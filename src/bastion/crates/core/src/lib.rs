src/bastion/crates/core/src/lib.rs
// ============================================================================
// BASTION CORE: ALchemy Database & State Machine Orchestrator
// A robust implementation of AlchemyDatabaseError with dynamic schema validation, 
// and a specialized module for orchestrating database operations using the existing crate utilities.
// ============================================================================

//! **AlchemyDatabaseError** - Defines error types for validating C/C# data structures against an external schema map.
use std::collections::{HashMap, HashSet};
use anyhow::{Context, Result};
use super::*;

/// Represents a potential validation failure due to missing keys in the provided schema or type mismatch with unknown columns/fields.
#[derive(Debug)]
enum AlchemyDatabaseError {
    /// Error indicating that one of the expected types is not found in the dynamic schema map.
    InvalidSchema(HashMap<String, String>), 
    /// Error specifically for a missing key within an existing entry or type mismatch with unknown columns/fields.
    MissingKey(String),  
}

impl AlchemyDatabaseError {
    fn from_invalid_schema(schema_map: HashMap<String, String>) -> Self;
    
    #[allow(clippy::unwrap_used)]
    pub fn new(error_type: impl Into<AlchemyDatabaseError>, message: &str) -> Result<Self> {
        match error_type.into() {
            AlchemyDatabaseError::MissingKey(key) => Ok(AlchemyDatabaseError::from_invalid_schema({}),), // Fallback to empty schema for missing key errors if explicit invalid schema isn't provided (though logically, a specific type mismatch should be returned here). In this design: InvalidSchema takes precedence and returns the error map.
            _ => Err(Self::new(message,)), 
        }
    }

    /// Checks if an AlchemyDatabaseError is caused by missing keys or types that don't match expected schema fields (like 'amount' vs 'price').
    pub fn is_missing(&self) -> bool { self.is_type_mismatch() || !matches!(error_type, AlchemyDatabaseError::MissingKey(_)) }

    /// Checks if an error indicates a type mismatch with unknown columns/fields. This happens when the schema map contains keys that don't exist in the current data structure or are not expected types for those fields (e.g., 'amount' vs 'price').
    pub fn is_type_mismatch(&self) -> bool { 
        match self.error_type() {
            AlchemyDatabaseError::InvalidSchema(_) => true, // Generic error implies schema issues.
            AlchemyDatabaseError::MissingKey(key) | _ => false, // Specific missing key or type mismatch should be explicitly handled elsewhere (e.g., in the database layer). 
        }
    }

    /// Checks if an error is a simple 'missing' case where no specific column/type definition was provided. This covers cases like `AlchemyDatabaseError::MissingKey` when passed to constructor but not via `new()`. In this design, it checks for missing keys first; otherwise falls back to generic invalid schema detection which would catch the type mismatch logic below.
    pub fn is_valid(&self) -> bool { 
        match self.error_type() {
            AlchemyDatabaseError::MissingKey(_) => true, // If no specific column/type was provided in constructor args (which we handle via `new`), it's valid as a 'missing' error type if passed correctly. However, our logic above prioritizes missing keys first. Let's refine: The function checks for the specific case of an invalid schema or unknown columns/fields. If no explicit column/type was provided in constructor args (which is handled by `new`), it implies we are in a 'missing' state that might not be caught here unless we pass it to new(). But logically, if you call `AlchemyDatabaseError::from_invalid_schema(map)`, then the map exists and has keys. If no key was passed for an unknown column (which is handled by type mismatch logic), this function returns false? No, let's stick with: It checks if any of our specific error variants are true.
            // Re-evaluating "is_valid": In the context of `from_invalid_schema`, it implies a failure occurred because we passed invalid schema or unknown columns/fields to constructor (which is handled by new()). But for 'missing' errors, no column/type was provided in args? No, that's not how Rust handles generic function arguments. 
            // Let's simplify: The method `is_valid` returns true if the error *type* matches one of our specific variants.
            AlchemyDatabaseError::MissingKey(_) | _ => false,
        }
    }

    /// Public constructor to create a new error instance with dynamic schema validation logic in place via an internal map (`HashMap<String, String>`). This allows for robustness against unknown schemas or missing keys without needing hard-coded type definitions. The `message` parameter is
