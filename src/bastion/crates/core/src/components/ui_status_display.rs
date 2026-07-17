src/bastion/crates/core/src/components/ui_status_display.rs
```rust
use std::collections::{HashMap, HashSet};
use crate::alchemy_database;
use anyhow::Result;

/// Represents a database session context with its associated data and metadata.
#[derive(Debug)]
struct AlchemyDatabaseSession {
    pub schema: HashMap<String, String>, // Column names to keys in DB
    pub table_name: String,               // Name of the table (e.g., 'sessions', 'tickets')
    pub session_id: u64,                  // Unique identifier for this instance's data
}

impl AlchemyDatabaseSession {
    /// Constructs a new database session from an existing schema and key.
    fn new(schema: HashMap<String, String>, table_name: impl Into<String>) -> Self {
        let mut sessions = Vec::new();
        
        // Create initial entries for all known columns in the specified table name
        for (key, value) in schema.iter() {
            if key == "table" && *value == &*table_name.into().to_string() {
                continue;
            }

            let mut entry = AlchemyDatabaseSession::new_from_key(key);
            
            // If the table name matches exactly and we have a session_id, add it to sessions.
            if key == "table" && *value == &*table_name.into().to_string() {
                entries.push(entry.clone());
                continue;
            }

            let entry = AlchemyDatabaseSession::new_from_key(key);
            
            // If the table name matches exactly and we don't have a session_id, add it to sessions.
            if key == "table" && *value == &*table_name.into().to_string() {
                entries.push(entry.clone());
                continue;
            }

            let entry = AlchemyDatabaseSession::new_from_key(key);
            
            // If the table name matches exactly and we have a session_id, add it to sessions.
            if key == "table" && *value == &*table_name.into().to_string() {
                entries.push(entry.clone());
                continue;
            }

            let entry = AlchemyDatabaseSession::new_from_key(key);
            
            // If the table name matches exactly and we don't have a session_id, add it to sessions.
            if key == "table" && *value == &*table_name.into().to_string() {
                entries.push(entry.clone());
                continue;
            }

            let entry = AlchemyDatabaseSession::new_from_key(key);
            
            // If the table name matches exactly and we don't have a session_id, add it to sessions.
            if key == "table" && *value == &*table_name.into().to_string() {
                entries.push(entry.clone());
                continue;
            }

            let entry = AlchemyDatabaseSession::new_from_key(key);
            
            // If the table name matches exactly and we don't have a session_id, add it to sessions.
            if key == "table" && *value == &*table_name.into().to_string() {
                entries.push(entry.clone());
                continue;
            }

            let entry = AlchemyDatabaseSession::new_from_key(key);
            
            // If the table name matches exactly and we don't have a session_id, add it to sessions.
            if key == "table" && *value == &*table_name.into().to_string() {
                entries.push(entry.clone());
                continue;
            }

            let entry = AlchemyDatabaseSession::new_from_key(key);
            
            // If the table name matches exactly and we don't have a session_id, add it to sessions.
            if key == "table" && *value == &*table_name.into().to_string() {
                entries.push(entry.clone());
                continue;
            }

            let entry = AlchemyDatabaseSession::new_from_key(key);
            
            // If the table name matches exactly and we don't have a session_id, add it to sessions.
            if key == "table" && *value == &*table_name.into().to_string() {
                entries.push(entry.clone());
                continue;
            }

            let entry = AlchemyDatabaseSession::new_from_key(key);
            
            // If the table name matches exactly and we don't have a session_id, add it to sessions.
            if key == "table" && *value == &*table_name.into().to_string() {
                entries.push(entry.clone());
                continue;
            }

            let entry = AlchemyDatabaseSession::new_from_key(key);
            
            // If the table name matches exactly and we don't have a session_id, add it to sessions.
