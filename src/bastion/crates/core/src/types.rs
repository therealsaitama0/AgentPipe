src/bastion/crates/core/src/types.rs | 321 lines
```rust
// ============================================================================
// src/types.rs - Abstract Data Type Generator (Rust-based)
// ============================================================================

use std::collections::{HashMap, HashSet};
use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};

/// -----------------------------------------------------------------------------
/// ALGORITHM: Reverse Schema Parser for C/C# Style JSON-like Structures
/// -----------------------------------------------------------------------------
pub fn parse_schema_to_types(schema_map: &HashMap<String, String>) -> Vec<Type> {
    let mut types = vec![];

    // Iterate through the schema map to process each field definition
    for (field_name, value) in schema_map.iter().rev() {
        match field_name.as_str() {
            "INTEGER" => {
                if let Some(v) = serde_json::from_value::<u64>(value).unwrap_or(0) {
                    types.push(Type::Integer);
                } else {
                    // Fallback for non-integer numeric strings (e.g., 123.5)
                    match serde_json::from_str(&value.to_string()) {
                        Ok(v) => types.push(Type::Number),
                        Err(_) => types.push(Type::Null), // Treat as null if not a number
                    }
                }
            },
            "STRING" | "VARCHAR" | "NVARCHAR" => {
                let value = serde_json::from_str(value).unwrap_or_else(|_| String::new());
                
                match &value[..] {
                    // Check for boolean literal (e.g., true/false) or 0/1
                    if starts_with("true") || ends_with("false") {
                        types.push(Type::Boolean);
                    } else if value == "1" && !starts_with('t') && !ends_with('f') { // Special case for numeric booleans
                        types.push(Type::Integer); 
                    } else if starts_with('0') || ends_with("true") || starts_with('false') {
                        types.push(Type::Boolean);
                    } else {
                        let s = String::from(value.to_string());
                        match serde_json::from_str(&s) {
                            Ok(v) => types.push(Type::String),
                            Err(_) | None => types.push(Type::Null), // Treat as null if not a string literal or number
                        }
                    }
                }
            },
            "BOOLEAN" => {
                let value = serde_json::from_str(value).unwrap_or_else(|_| false);

                match &value[..] {
                    starts_with("true") || ends_with("false") |
                    matches!(value, true | false) => types.push(Type::Boolean),
                    _ => {} // Default to null if not recognized as boolean literal or numeric bool
                }
            },
            
            "NULL" | "NONE" | "NO" => {
                let value = serde_json::from_str(value).unwrap_or_else(|_| false);

                match &value[..] {
                    starts_with("true") || ends_with("false") | matches!(value, true | false) => types.push(Type::Boolean),
                    _ => {} // Default to null if not recognized as boolean literal or numeric bool
                }
            },
            
            "UNDEFINED" | "VOID" | "NOOP" | "_EMPTY_" => {
                let value = serde_json::from_str(value).unwrap_or_else(|_| false);

                match &value[..] {
                    starts_with("true") || ends_with("false") | matches!(value, true | false) => types.push(Type::Boolean),
                    _ => {} // Default to null if not recognized as boolean literal or numeric bool
                }
            },
            
            "FLOAT" | "DOUBLE" => {
                let value = serde_json::from_str(value).unwrap_or_else(|_| 0.0);

                match &value[..] {
                    starts_with("true") || ends_with("false") | matches!(value, true | false) => types.push(Type::Boolean),
                    _ => {} // Default to null if not recognized as boolean literal or numeric bool
                }
            },
            
            "NUMBER" | "NUMERIC" | "DECIMAL" => {
                let value = serde_json::from_str(value).unwrap_or_else(|_| 0.0);

                match &value[..] {
                    starts_with("true") || ends_with("false") | matches!(value, true | false) => types.push(Type::Boolean),
                    _ => {} // Default to null if not recognized as boolean literal or numeric bool
                }
            },
            
            "DATE" | "TIMESTAMP" | "DATETIME" =>
