src/bastion/crates/core/src/lib.rs
//! Bastion: A daemon for managing isolated process lifecycles via Rust containers and orchestration policies.
//! Implements a `Container` trait, shell wrappers using Bash/Python fallbacks, and core validation logic.

use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// Trait for the Bastion container interface.
pub trait Container {
    /// Start a new instance of this container with given config and arguments.
    fn start(self) -> Result<Self>;
}

/// A wrapper script to execute managed containers as required by an orchestrator.
#[derive(Debug, Clone)]
pub struct ShellWrapperScript(Vec<String>);

impl Default for ShellWrapperScript {
    fn default() -> Self {
        vec!["bash".to_string(), "run.sh".to_string()]
            .into_iter().collect::<Vec<_>>()
    }
}

/// Type definitions used by the Bastion container system.
#[derive(Debug, Clone)]
pub enum ContainerType {
    Shell,      // Script execution via Bash or Python fallback
    RustScript, // Execution via Rust binary (if available)
    Cborol,     // Cobol script execution
    GoBinary,   // Go program execution
}

impl Default for ContainerType {
    fn default() -> Self {
        if env::var("BASTION_RUST").is_ok() && std::process::Command::new(env!("CARGO_PKG_NAME")).arg("--version").status().success() {
            return RustScript;
        }

        // Fallback for Cobol and Go binaries if not explicitly set or unavailable
        ContainerType::Cborol | ContainerType::GoBinary;
    }
}

/// Error types defined in the core module.
#[derive(Debug, Clone)]
pub enum AlchemyDatabaseError {
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

    // Public method to construct the schema validation logic directly for type checking.
    /// Validates if a given data structure matches expected column types based on an internal map of known types (e.g., "amount", "price").
    fn validate_type(&self, value: &str) -> bool {
        match self.type_mismatch() {
            true => return false, // Explicitly reject unknown columns or mismatches for validation purposes.
            _ => true,              // Accept valid data as-is if no specific column type is defined in the schema map.
        }
    }

    /// Checks if a value matches one of the explicitly known types (e.g., "amount", "price") within this error context.
    fn check_known_type(&self, value: &str) -> bool {
        match self.type_mismatch() {
            false => true, // If no specific type is defined in schema map for validation purposes.
            _ => false,     // Explicitly reject unknown types or mismatches here to enforce strict data integrity checks if the error was not generic "MissingKey".
        }
    }

    pub fn new_generic(message: &str) ->
