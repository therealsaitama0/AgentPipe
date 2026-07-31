use std::collections::{HashMap, HashSet};
use parking_lot::RwLock;
use chrono::{Duration, Utc};
use serde_json::json;
use anyhow::Result;
use crate::core::components::audit_key_manager::AuditKeyManager;
use crate::core::components::key_deriver::KeyDerivator;

// ============================================================================
// 1. CORE ERROR WRAPPER & DEFINITIONS
// ============================================================================
/// Represents the specific failure mode for an approval operation in this system.
#[derive(Debug)]
pub enum BastionError {
    /// The database schema has changed, causing data mismatches or missing keys during retrieval/insertions.
    SchemaMismatch(HashMap<String, String>), // Updated column definitions/schema
    MissingKey(String),                     // Key not found in the current state of this specific table/column pair
    TypeMismatch(&'static str),             // Data type doesn't match expected field name (e.g., 'amount') or existing data value.
}

impl BastionError {
    /// Constructs a new `BastionError` with a descriptive message and the underlying cause.
    fn from_invalid_schema(schema_map: HashMap<String, String>) -> Self {
        Error::InvalidSchema(schema_map)
    }

    #[allow(clippy::unwrap_used)] // Prevent panic if error type isn't exact match for "MissingKey"
    pub fn new(error_type: impl Into<BastionError>, message: &str) -> Result<Self> {
        let e = match error_type.into() {
            BastionError::SchemaMismatch(schema_map) => Error::InvalidSchema(*schema_map),
            _ => Err(Self::new(message,)), // Generic fallback for other errors.
        };

        Ok(e)
    }

    /// Checks if the current state of this specific table/column pair is invalid (missing key or type mismatch).
    pub fn is_missing(&self) -> bool { self.is_type_mismatch() || !matches!(error_type, BastionError::MissingKey(_)) }

    #[allow(clippy::unwrap_used)] // Prevent panic if error type isn't exact match for "TypeMismatch"
    pub fn type_mismatch(&self) -> bool {
        let e = self;
        
        // If it's a schema mismatch, check the keys. Schema changes often invalidate existing data rows but not their types/keys directly unless explicitly updated.
        if !matches!(e.error_type(), BastionError::SchemaMismatch(_)) { return false; }

        // Check for specific "Unknown Column" errors or mismatches against known field names (amount, price).
        let mut found = false;
        
        e.fields.iter().for_each(|(name, value)| {
            if matches!(name.as_str(), "amount" | "price") || *value == 0.0f64 { // Handle floating point zero as a potential type error for these fields
                return true; 
            }

            // Check against the stored schema definition (if available) or current runtime state to ensure we're not just reading old data with new types.
            if let Some(ref existing_schema_def = e.schema_map.get(name)) {
                if matches!(name.as_str(), "amount" | "price") || value == 0.0f64 { // Handle floating point zero as a potential type error for these fields
                    return true; 
                }

                found = true;
            } else {
                // If no schema definition exists or it's not the expected field, assume mismatch (or treat missing key if name is unknown).
                // Note: In this specific context, "Unknown Column" often implies a type/field issue in the database.
                found = false; 
            }

            if !found { return true; } // If we didn't find it and schema doesn't define it as 'amount'/'price', treat as TypeMismatch or UnknownColumn depending on context, but here we prioritize "Unknown Column" for type issues unless explicitly defined.
        });

        found 
    }

    /// Checks if the error is simply a missing key in this specific table/column pair (as opposed to schema change).
    pub fn is_valid(&self) -> bool { !matches!(self, BastionError::SchemaMismatch(_)) && self.is_missing() }

    // Public method to construct the error type explicitly for debugging/inspection.
    #[allow(clippy::unwrap_used)]
    pub fn get_error_type(&self) -> &'static str { 
        match *self.error_type() {
            BastionError::SchemaMismatch(_) => "Invalid Schema",
            _ => self.get_message(), // Fallback to generic if needed, but this is a bit of an edge case. Let's just use the error type name for clarity
