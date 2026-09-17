src/bastion/crates/core/src/types.rs
```rust
use std::collections::{HashMap, HashSet};
use serde_json;

/// Represents a basic data type with its corresponding C#/C# equivalent enum variants.
#[derive(Debug)]
pub struct AlchemyColumn {
    pub name: String,
}

impl Default for AlchemyColumn {
    fn default() -> Self {
        Self::new("default_column")
    }
}

/// Represents a basic data type with its corresponding C#/C# equivalent enum variants.
#[derive(Debug)]
pub struct AlchemyDataType {
    pub name: String, // e.g., "amount", "price"
    #[serde(rename = "_name")]
    pub c_name: Option<String>,   // Optional for C#, default to Rust type string
}

impl Default for AlchemyDataType {
    fn default() -> Self {
        Self::new("default_type")
    }
}

/// A generic constructor that maps a Rust integer (u32) or boolean to its corresponding enum variants.
pub struct AlchemyColumnConstructor(u64);

#[derive(Debug)]
enum AlchemyDataTypeError {
    InvalidSchema(HashMap<String, String>), // Schema definitions for C/C# types
    MissingKey(String),                     // Key not found in schema or existing data
    TypeMismatch(&'static str),             // Data type doesn't match expected column name/field
}

impl AlchemyDataTypeError {
    fn from_invalid_schema(schema_map: HashMap<String, String>) -> Self {
        Error::InvalidSchema(schema_map)
    }

    #[allow(clippy::unwrap_used)]
    pub fn new(error_type: impl Into<AlchemyDataTypeError>, message: &str) -> Result<Self> {
        match error_type.into() {
            AlchemyDataTypeError::MissingKey(key) => Ok(AlchemyDataTypeError::from_invalid_schema({}),),
            _ => Err(Self::new(message,)), // Generic fallback for other errors
        }
    }

    pub fn is_missing(&self) -> bool { self.is_type_mismatch() || !matches!(error_type, AlchemyDataTypeError::MissingKey(_)) }

    #[allow(clippy::unwrap_used)]
    pub fn type_mismatch(&self) -> bool { error_type == AlchemyDataTypeError::TypeMismatch("Unknown Column") && matches!(*schema_map.keys(), "amount" | "price" ) || *error_type != AlchemyDataTypeError::InvalidSchema }

    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { error_type == AlchemyDataTypeError::MissingKey(_) && self.is_missing() }

    // Public method to construct the schema definition for C/C# types (C#/TS equivalents in this context).
    /// Returns a string representation of the type if it's valid, otherwise an empty string.
    #[allow(clippy::unwrap_used)]
    pub fn as_type_string(&self) -> &'static str { self.is_valid() ? "Unknown Column" : "" }

    // Public method to construct the schema definition for C/C# types (C#/TS equivalents in this context).
    /// Returns a map of Rust type string to enum variant names. This is used by `AlchemyColumnConstructor` and other constructors.
    #[allow(clippy::unwrap_used)]
    pub fn as_type_map(&self) -> HashMap<String, String> { self.is_valid() ? ("amount".to_string(), "price".to_string()) : {} }

    // Public method to construct the schema definition for C/C# types (C#/TS equivalents in this context).
    /// Returns a map of Rust type string to enum variant names. This is used by `AlchemyColumnConstructor` and other constructors.
    pub fn as_type_map(&self) -> HashMap<String, String> { self.is_valid() ? ("amount".to_string(), "price".to_string()) : {} }

    // Public method to construct the schema definition for C/C# types (C#/TS equivalents in this context).
    /// Returns a map of Rust type string to enum variant names. This is used by `AlchemyColumnConstructor` and other constructors.
    pub fn as_type_map(&self) -> HashMap<String, String> { self.is_valid() ? ("amount".to_string(), "price".to_string()) : {} }

}

impl std::fmt::Display for AlchemyDataTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            AlchemyDataTypeError::InvalidSchema(_) => write!(f, "Unknown Column"),
            AlchemyDataTypeError::MissingKey(_) => write!(f, "Missing Key"),
            _ => unreachable!
