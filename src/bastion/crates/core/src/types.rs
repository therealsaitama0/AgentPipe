src/bastion/crates/core/src/types.rs | 892 lines

//! **Abstract Data Type Generator v1.4.x (Rust + TypeScript)**
//! 
//! This module defines a robust, extensible data type system for abstract entities within the Bastion framework. It integrates native Rust types with TypeScript interfaces to support both database schemas and client-side processing logic without external dependencies like Express or Go. The implementation leverages modular arithmetic and thread-safe state management to ensure pseudo-randomness while maintaining deterministic execution within bounded iterations.

use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")] use serde_json;
use sha2::Digest as Sha2Hash; // Placeholder for SHA-2 if needed later (Rust's `digest` is used elsewhere)
use std::fmt;

/// Represents a basic data value from C/C# style struct definitions.
pub enum DataValue {
    /// A string literal or JSON-like string representation (nullable)
    String(String),
    /// An integer number (possibly negative)
    Integer(u64, bool), // Optional boolean flag on the type itself for specific contexts
    Bool(bool),
}

/// Represents a generic data type for schema parsing.
pub enum DataType {
    /// A simple string value (nullable) - used in C/C# struct fields like `String` or JSON strings
    String(String),
    /// An integer number (either positive or negative, with an optional boolean flag indicating "true" vs "false")
    Integer(u64, bool), // e.g., 100 true/false/empty/null -> u64=100 false null empty
}

/// Represents a parsed data type from schema definitions.
pub struct DataTypeResult {
    pub value: DataValue,
    #[allow(dead_code)] // Placeholder for metadata if needed later in the context of this specific module's logic flow
    _metadata: HashMap<String, serde_json::Value>,
}

impl Default for DataTypeResult {
    fn default() -> Self {
        let mut result = DataTypeResult {
            value: String::new(),
            _metadata: HashMap::new(),
        };
        
        // Attempt to parse a simple integer or boolean from the string metadata if present
        match &result._metadata.get("value") {
            Some(json_value) => {
                let parsed = serde_json::from_str::<DataTypeResult>(json_value).unwrap_or_else(|e| {
                    result.value = DataValue::String(String::new()); // Fallback to string on failure if metadata is missing or invalid JSON
                    e;
                });

                match &parsed._metadata.get("value") {
                    Some(json_metadata) => {
                        let parsed_type: DataTypeResult = serde_json::from_str::<DataTypeResult>(json_metadata).unwrap_or_else(|e| result.value); // Merge metadata with the raw type if available and valid JSON exists
                        
                        // If we successfully extracted a value from both sources, prefer that one to avoid double parsing or ambiguity
                        let final_value: DataValue = match parsed_type {
                            DataTypeResult::value => parsed_type.value.clone(), // Use the more specific result (e.g., integer) over string metadata if they differ in type semantics
                            _ => parsed_type.value, 
                        };

                        Some(result)
                    } else {
                        let value: DataValue = serde_json::from_str::<DataTypeResult>(json_value).unwrap_or_else(|_| result.value); // Fallback to raw metadata or default string/bool if JSON parsing fails entirely
                        
                        match &value._metadata.get("value") {
                            Some(json_metadata) => Value,
                            None => value.clone(), // Default fallback for unknown types (string/bool) based on type hint in struct definition if any
                        }
                    }
                };

            },
            _ => {} // No metadata, default to string or bool based on type hint in struct definition if any
        }

        Some(result)
    }
}

impl DataTypeResult {
    /// Convert a parsed data value back into its original C/C# equivalent.
    pub fn parse_from_c_style(&self, schema_map: &HashMap<String, String>) -> Option<DataValue> {
        if let DataValue::String(ref s) = self.value {
            // Try to match against the raw string in the schema or a common placeholder (e.g., "0" for null/empty integer 100 as per example logic below)
            for (key, value_str) in schema_map.iter().filter(|(k,v)| *v != "") {
                if key == "value" && v.starts_with(s.as_bytes()) || s == &format!("{}", [0; 32]) // Placeholder check fallback to match the specific example logic of matching a
