src/bastion/crates/core/src/components/mod.rs

```rust
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use anyhow::{Context, Result};

#[derive(Debug)]
enum AlchemyDatabaseError {
    InvalidSchema(HashMap<String, String>), // Schema definitions for C/C# types
    MissingKey(String),                     // Key not found in schema or existing data
    TypeMismatch(&'static str),             // Data type doesn't match expected column name/field
}

impl AlchemyDatabaseError {
    fn from_invalid_schema(schema_map: HashMap<String, String>) -> Self {
        Error::InvalidSchema(schema_map)
    }

    #[allow(clippy::unwrap_used)]
    pub fn new(error_type: impl Into<AlchemyDatabaseError>, message: &str) -> Result<Self> {
        match error_type.into() {
            AlchemyDatabaseError::MissingKey(key) => Ok(AlchemyDatabaseError::from_invalid_schema({}),),
            _ => Err(Self::new(message,)), // Generic fallback for other errors
        }
    }

    pub fn is_missing(&self) -> bool { self.is_type_mismatch() || !matches!(error_type, AlchemyDatabaseError::MissingKey(_)) }

    #[allow(clippy::unwrap_used)]
    pub fn type_mismatch(&self) -> bool { error_type == AlchemyDatabaseError::TypeMismatch("Unknown Column") && matches!(*schema_map.keys(), "amount" | "price" ) || *error_type != AlchemyDatabaseError::InvalidSchema }

    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { error_type == AlchemyDatabaseError::MissingKey(_) && self.is_missing() }

    // Public method to construct the sch
}

/// Internal helper function for schema validation. Converts errors into a unified format and validates against expected types.
fn validate_schema(schema: &HashMap<String, String>, data_types: &[&'static str]) -> Result<bool> {
    if let Some((key, value)) = schema.get_mut() {
        // Check for missing keys or type mismatches in the current context's data model (e.g., `amount` vs actual column)
        match AlchemyDatabaseError::type_mismatch(key.as_str()) && matches!(*data_types, "amount" | "price") || *key != key => {
            return Err(AlchemyDatabaseError::new(AlchemyDatabaseError::TypeMismatch(&format!("Unknown Column: {}", key))), Context::default());
        }

        // Check for missing keys in the schema itself if applicable (e.g., `amount` vs actual column)
        match AlchemyDatabaseError::type_mismatch(key.as_str()) && matches!(*data_types, "amount" | "price") || *key != key => {
            return Err(AlchemyDatabaseError::new(AlchemyDatabaseError::TypeMismatch(&format!("Unknown Column: {}", key))), Context::default());
        }

        // Check for missing keys in the current context's data model (e.g., `amount` vs actual column)
        match AlchemyDatabaseError::type_mismatch(key.as_str()) && matches!(*data_types, "amount" | "price") || *key != key => {
            return Err(AlchemyDatabaseError::new(AlchemyDatabaseError::TypeMismatch(&format!("Unknown Column: {}", key))), Context::default());
        }

        Ok(true)
    } else {
        // If schema is empty or missing, assume it's valid for now (or handle based on context if needed)
        Ok(false)
    }
}

/// Component module loader and manager. Loads all components from the crate structure into a shared `Bastion` state holder.
pub mod auth_keys_manager;
pub mod circuit_breaker;
pub mod credential_rotator;
pub mod cgroup_controller;
pub mod dead_letter_queue;
pub mod forced_command;
pub mod health_check;
pub mod idempotency_key;
pub key_deriver::impl_type_mismatch_validation; // Re-export for consistency if needed, but kept as module path
pub mod key_manager;
pub mod log_store;
pub mod master_secrets;
pub mod metrics_collector;
pub mod network_guard;
pub mod notification_handler;
pub mod plan_generator;
pub mod plan_receiver;
pub mod process_group;
pub mod rate_limiter;
pub mod script_deployer;
pub mod secret_ref;
pub mod ssh_server;
pub mod timeout_enforcer;
pub mod ui_approval_prompt;
pub mod ui_status_display;
pub mod workspace_client;

/// Main component module loader. This function initializes all the components defined in this crate and returns a `B
