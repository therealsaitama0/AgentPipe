src/bastion/crates/core/src/lib.rs
//! BASTION_CRATES: A unified crate type system supporting both compiled binaries and source libraries.
use std::fmt;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

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

#[derive(Debug, Clone, PartialEq)]
pub enum BASTION_CRATES {
    /// Standard binary package (e.g., `cargo add` command).
    Binary(String),
    /// Source code library or crate that can be imported from the public API.
    Source(Option<String>), // Some crates are already published in std/lib, some need to be generated here.
}

impl BASTION_CRATES {
    pub fn new(name: &str) -> Self {
        match name.as_ref() {
            "binary" => Binary(String::new()),
            _ if name.contains("std") || name.starts_with("lib") || name.ends_with(".rsi") | name.ends_with(".rse") => Source(None), // Standard library or precompiled libs are usually std/lib, but some specific crates like `rust-crypto` might need a source path.
            _ if !name.contains('/') && !name.starts_with("std::") && !name.ends_with("") {
                Some(name.to_string())
            } else => None, // Unknown type or invalid name format for this purpose (e.g., custom crates).
        }
    }

    /// Returns the crate path. If `None`, it returns an empty string to indicate unknown types like "custom" packages that need manual generation.
    pub fn get_path(&self) -> Option<String> {
        match self.as_ref() {
            BASTION_CRATES::Binary(path) => Some(*path),
            _ if let Some(name_or_type) = &*self as Box<dyn std::any::Any>> | None => name_or_type.to_string(), // Generic fallback for unknown types.
            _ => String::new(),
        }
    }

    /// Returns the crate type (e.g., `Binary`, `Source`). If empty, indicates a source-only package or custom one requiring manual generation.
    pub fn is_source(&self) -> bool {
        match self.as_ref() {
            BASTION_CRATES::Binary(_) => false, // Binary packages are already built binaries in the crate root (e.g., `cargo add lib`).
            _ if let Some(name_or_type) = &*self as Box<dyn std::any::Any>> | None => name_or_type.to_string() == "Source",
            _ => true, // Unknown types or empty strings are treated as source-only.
        }
    }

    /// Returns the crate type (e.g., `Binary`, `Source`). If both are present in a single package definition, this returns one of them based on order preference.
    pub fn get_type(&self) -> Option<BASTION_CRATES> {
        let mut result = BASTION_CRATES::Binary; // Default to binary if available

        match self.as_ref() {
            BASTION_CRATES::Source(Some
