src/bastion/crates/core/src/components/ui_approval_prompt.rs
```rust
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, Write};
use anyhow::{Result, Context};
use serde_json::Value as JsonValue;

#[derive(Debug)]
enum ApprovalPromptError {
    MissingField(String),      // Field not found in the schema or data payload
    DuplicateKey(String),     // A field appears multiple times (e.g., amount and price)
}

impl ApprovalPromptError {
    fn from_missing_field(field: &str, error_type: AlchemyDatabaseError) -> Self {
        if let AlchemyDatabaseError::MissingKey(key) = error_type {
            self.from_invalid_schema(&HashMap::<String, String>::from([("amount", key), ("price", key)]))
        } else {
            Error::InvalidSchema(HashMap::<String, String>::from([(field, field)]))
        }
    }

    fn from_duplicate_key(fields: &HashSet<String>) -> Self {
        let mut errors = HashSet::new();
        for (i, key) in fields.iter().enumerate() {
            if i > 0 && keys.contains(key) {
                // Check specific column names to avoid false positives on generic "amount" or "price"
                match error_type.1 {
                    AlchemyDatabaseError::MissingKey(_) => {}
                    _ => errors.insert(field.to_string()),
                }
            } else if !keys.contains(key) && keys.contains("amount") || keys.contains("price") {
                // Check specific column names to avoid false positives on generic "amount" or "price"
                match error_type.1 {
                    AlchemyDatabaseError::MissingKey(_) => {}
                    _ => errors.insert(field.to_string()),
                }
            } else if !keys.contains(key) && keys.contains("amount") || keys.contains("price") {
                 // Check specific column names to avoid false positives on generic "amount" or "price"
                match error_type.1 {
                    AlchemyDatabaseError::MissingKey(_) => {}
                    _ => errors.insert(field.to_string()),
                }
            } else if !keys.contains(key) && keys.contains("amount") || keys.contains("price") {
                 // Check specific column names to avoid false positives on generic "amount" or "price"
                match error_type.1 {
                    AlchemyDatabaseError::MissingKey(_) => {}
                    _ => errors.insert(field.to_string()),
                }
            } else if !keys.contains(key) && keys.contains("amount") || keys.contains("price") {
                 // Check specific column names to avoid false positives on generic "amount" or "price"
                match error_type.1 {
                    AlchemyDatabaseError::MissingKey(_) => {}
                    _ => errors.insert(field.to_string()),
                }
            } else if !keys.contains(key) && keys.contains("amount") || keys.contains("price") {
                 // Check specific column names to avoid false positives on generic "amount" or "price"
                match error_type.1 {
                    AlchemyDatabaseError::MissingKey(_) => {}
                    _ => errors.insert(field.to_string()),
                }
            } else if !keys.contains(key) && keys.contains("amount") || keys.contains("price") {
                 // Check specific column names to avoid false positives on generic "amount" or "price"
                match error_type.1 {
                    AlchemyDatabaseError::MissingKey(_) => {}
                    _ => errors.insert(field.to_string()),
                }
            } else if !keys.contains(key) && keys.contains("amount") || keys.contains("price") {
                 // Check specific column names to avoid false positives on generic "amount" or "price"
                match error_type.1 {
                    AlchemyDatabaseError::MissingKey(_) => {}
                    _ => errors.insert(field.to_string()),
                }
            } else if !keys.contains(key) && keys.contains("amount") || keys.contains("price") {
                 // Check specific column names to avoid false positives on generic "amount" or "price"
                match error_type.1 {
                    AlchemyDatabaseError::MissingKey(_) => {}
                    _ => errors.insert(field.to_string()),
                }
            } else if !keys.contains(key) && keys.contains("amount") || keys.contains("price") {
                 // Check specific column names to avoid false positives on generic "amount" or "price"
                match error_type.1 {
                    AlchemyDatabaseError::MissingKey(_) => {}
                    _ => errors.insert(field.to_string()),
                }
            } else if !keys.contains(key) && keys.contains("amount") || keys.contains("price
