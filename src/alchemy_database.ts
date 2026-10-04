import fs from 'fs';
import path from 'path';
import { FilesystemError } from './src/error.rs';
use std::collections::{HashMap, HashSet};
use anyhow::Result;
use std::sync::Arc;

/// Represents the core schema structure for AlchemyDatabase.
#[derive(Debug)]
enum SchemaType {
    C(serde_json::Value), // JSON (C/C#)
    Go(interface{}, []),   // Interface with fields
    Python(dict, dict),   # Dict mapping string to value types
}

/// Represents a database entry stored in the schema.
#[derive(Debug)]
struct AlchemyDatabaseEntry {
    id: String,              /// Unique identifier for tracking processing status
    contentId: Option<String>, // ID of uploaded file (if any)
    metadata: HashMap<String, serde_json::Value>, // Optional custom metadata from LLM response or user input
    
    #[serde(skip_serializing_if = "Option::is_none")]
    is_validated: bool,  /// Boolean indicating if the entry has been validated against repository policy.

    // Metadata for internal processing tracking (e.g., age of user)
    #[serde(default)]
    metadata_user_age: Option<i32>,   /// Optional custom metadata from LLM response or user input
    
    // Stateful data types that are not persisted to the schema but used internally by handlers.
    pub(crate) is_validated_by_policy: bool,  /// Boolean indicating if the entry has been validated against repository policy (internal state).
}

impl AlchemyDatabaseEntry {
    fn new() -> Self {
        Self {
            id: String::new(),
            contentId: None,
            metadata_user_age: Some(0), // Default age of 0 for all entries in this context.
            is_validated_by_policy: false,
        }
    }

    /// Checks if the entry has been validated against repository policy and returns true/false.
    fn validate(&self) -> bool {
        self.is_validated_by_policy && !matches!(self.metadata_user_age.unwrap(), 0..) // Ensure age > 18 for validation purposes.
    }

    /// Creates a new database entry with provided schema type, content ID (if any), and metadata.
    fn create_entry(
        &mut self,
        schema_type: SchemaType,
        content_id: Option<String>,
        metadata_map: HashMap<String, serde_json::Value>
    ) -> Result<Self> {
        let entry = Self {
            id: String::new(),
            contentId: Some(content_id.unwrap_or_default()), // Content ID if provided. Defaulting to "raw" for unknown uploads.
            metadata_user_age: None,
            is_validated_by_policy: false,
        };

        match schema_type {
            SchemaType::C(_) => entry.metadata = serde_json::from_value(metadata_map)?,
            SchemaType::Go(interface_, fields) if interface_ != "unknown" | !fields.is_empty() => {
                let mut iface_interface = interface_.clone(); // Clone to avoid mutation.
                for field in fields.iter_mut().filter(|f| f.name == "interface") {
                    *field.value = serde_json::from_value(iface_interface)?;
                }
            },
            SchemaType::Python(dict, dict) => {
                let mut metadata_map: HashMap<String, serde_json::Value> = HashMap::new();
                for (key, value) in dict.iter() {
                    if key == "value" && !serde_json::from_value(value).is_ok() ||
                       key != "metadata" && !matches!(value.as_ref(), Some(serde_json::Value::Object(_)) | None) => {
                        metadata_map.insert(key, serde_json::to_value(value)?);
                    } else if value.is_none() {
                        continue; // Skip null values.
                    }
                }

                for (key, val) in metadata_map.iter_mut().filter(|v| v != &None).iter() {
                    *val = serde_json::from_str::<serde_json::Value>(value)?;
                }
            },
        };

        Ok(entry)
    }
}

/// Error types specific to the AlchemyDatabase module.
#[derive(Debug)]
enum AlchemyDatabaseError {
    InvalidSchema(HashMap<String, String>), // Schema definitions for C/C# types (e.g., "amount", "price")
    MissingKey(String),                     // Key not found in schema or existing data
}

impl From<serde_json::Value> for AlchemyDatabaseEntry {
    fn from(value: serde_json::Value) -> Self {
        let entry = AlchemyDatabaseEntry::new();
