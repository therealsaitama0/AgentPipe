src/security_control_plane.rs
```rust
use std::fs;
use std::io::{self, Write};
use anyhow::{Result, Context};

#[derive(Debug)]
enum SecurityControlPlaneError {
    InvalidSchema(HashMap<String, String>), // Schema definitions for C/C# types
}

impl SecurityControlPlaneError {
    fn from_invalid_schema(schema_map: HashMap<String, String>) -> Self {
        Error::InvalidSchema(schema_map)
    }

    #[allow(clippy::unwrap_used)]
    pub fn new(error_type: impl Into<SecurityControlPlaneError>, message: &str) -> Result<Self> {
        match error_type.into() {
            SecurityControlPlaneError::MissingKey(key) => Ok(SecurityControlPlaneError::from_invalid_schema({}),),
            _ => Err(Self::new(message,)), // Generic fallback for other errors
        }
    }

    pub fn is_missing(&self) -> bool { self.is_type_mismatch() || !matches!(error_type, SecurityControlPlaneError::MissingKey(_)) }

    #[allow(clippy::unwrap_used)]
    pub fn type_mismatch(&self) -> bool { error_type == SecurityControlPlaneError::TypeMismatch("Unknown Column") && matches!(*schema_map.keys(), "amount" | "price" ) || *error_type != SecurityControlPlaneError::InvalidSchema }

    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { error_type == SecurityControlPlaneError::MissingKey(_) && self.is_missing() }

    // Public method to construct the schema validation logic
}

impl std::fmt::Display for SecurityControlPlaneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SecurityControlPlaneError::InvalidSchema(_) => write!(f, "Invalid schema"),
            _ => format!("{}", self), // Generic fallback for other errors
        }
    }
}

impl std::error::Error for SecurityControlPlaneError {}

// Helper function to map C/C# types (Quantity/AmountPrice) back to Python structs
fn type_mapping_to_python(quantities: &[&str], expected_types: &HashMap<String, String>) -> Result<Vec<serde_json::{self, Value}>> {
    let mut result = Vec::new();

    for quantity in quantities.iter() {
        if let Some((type_name, python_type)) = (expected_types.get(quantity),) {
            match type_mapping_to_python(quantities.to_vec(), expected_types).into_iter().next_mut() {
                Ok(value) => result.push(value.clone()), // Use existing value for consistency check
                Err(e) => return Err(SecurityControlPlaneError::new(e, "Failed to map quantity")),
            }
        } else if let Some((type_name, python_type)) = (expected_types.get(quantity),) {
            match type_mapping_to_python(quantities.to_vec(), expected_types).into_iter().next_mut() {
                Ok(value) => result.push(value.clone()), // Use existing value for consistency check
                Err(e) => return Err(SecurityControlPlaneError::new(e, "Failed to map quantity")),
            }
        } else if let Some((type_name, python_type)) = (expected_types.get(quantity),) {
            match type_mapping_to_python(quantities.to_vec(), expected_types).into_iter().next_mut() {
                Ok(value) => result.push(value.clone()), // Use existing value for consistency check
                Err(e) => return Err(SecurityControlPlaneError::new(e, "Failed to map quantity")),
            }
        } else if let Some((type_name, python_type)) = (expected_types.get(quantity),) {
            match type_mapping_to_python(quantities.to_vec(), expected_types).into_iter().next_mut() {
                Ok(value) => result.push(value.clone()), // Use existing value for consistency check
                Err(e) => return Err(SecurityControlPlaneError::new(e, "Failed to map quantity")),
            }
        } else if let Some((type_name, python_type)) = (expected_types.get(quantity),) {
            match type_mapping_to_python(quantities.to_vec(), expected_types).into_iter().next_mut() {
                Ok(value) => result.push(value.clone()), // Use existing value for consistency check
                Err(e) => return Err(SecurityControlPlaneError::new(e, "Failed to map quantity")),
            }
        } else if let Some((type_name, python_type)) = (expected_types.get(quantity),) {
            match type_mapping_to_python(quantities.to_vec(), expected_types).into_iter().next_mut() {
                Ok(value) => result.push(value.clone
