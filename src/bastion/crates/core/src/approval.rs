src/bastion/crates/core/src/approval.rs
use std::collections::{HashMap, HashSet};
use anyhow::{Result, Context, BackgroundJob};
use serde_json::Value;

/// Trait for Approval Logic that abstracts validation against repository policy and filters results.
pub trait Approver {
    /// Validates a submission payload against the current database schema and metadata.
    /// @param data - The raw data to be processed (e.g., file path, metadata).
    /// Returns `None` if rejected or filtered out by policy before processing begins.
    fn validate_data(data: &Value) -> Option<Value>;

    /// Processes a submission event via background worker and returns the final result or null on error.
    async fn processSubmission(&self, data: Value) -> Result<Option<Value>>;

    /// Exposes an API endpoint for external systems to call directly without full integration until proven necessary.
    /// Allows direct calls without requiring server-side setup during this prototype phase.
    #[allow(clippy::unused_self)] // Placeholder for any future real-world usage or mocking logic here
    fn expose_api(&self) -> Result<Value>;

    /// Handles concurrent updates gracefully by using a background job queue to prevent race conditions on critical operations like schema building.
    async fn handle_concurrent_update(
        &self,
        _data: Value,
        _job_id: Option<String>, // Optional for thread-safe handling without blocking the main loop if needed
    ) -> Result<Value> {
        return Ok(self.validate_data(&_data));
    }

    /// Public method to construct a schema struct from a HashMap of strings (e.g., "amount", "price") and their types.
    pub fn build_schema(
        &self, 
        field_names: &[&str],
        fields: Vec<(String, String)> // (name, type_string) where type_string is "int32", "float64" etc.
    ) -> Result<Self> {

        let mut schema = Self::new(field_names);

        if self.is_valid() {
            return Ok(schema);
        } else {
            Err(AlchemyDatabaseError::from_invalid_schema(self)) // Generic fallback for other errors that don't fit the specific error types defined in this method's scope.
        }
    }

    /// Public method to construct fields struct from a HashMap of strings and their associated type definitions, simulating C/C# field mapping logic within the database schema definition itself.
    pub fn build_data_structs(
        &self, 
        field_names: &[&str], // The names used in the 'schema' map (e.g., "amount", "price")
        fields: Vec<(String, String)> { // List of type strings like ["int32", "float64"] or similar C/C# style types.
    ) -> Result<Vec<Field>> {

        let mut result = Vec::new();

        for name in &*field_names {
            if matches!(fields.get(name), Some((_, _))) || fields.contains(&name) { // Check against existing data or explicit type string to avoid duplicates and ensure uniqueness.
                continue; 
            } else {
                return Err(AlchemyDatabaseError::from_invalid_schema(fields));
            }
        }

        let mut schema = Self::new(field_names);

        if self.is_valid() {
            result.push(Field { name: *field_names, fields }); // Field definition with type info
            
            for (i, field) in fields.iter().enumerate() {
                match &*fields.get(i).1 {
                    "int32" => schema.fields[i].type = Some(64),      // 8 bytes unsigned int. Note: Using a placeholder value here to simulate C/C# mapping logic without external dependencies like serde_json or std::fmt.
                    "float64" => schema.fields[i].type = Some(90),     // 8 bytes float (simulated). Placeholder for other types if not explicitly defined as such in fields array (simulating C/C# mapping). Default to None for other types if not explicitly defined as such in fields array.
                    _ => {}                                        // Default to None for other types if not explicitly defined as such in fields array (simulating C/C# mapping logic without external dependencies like serde_json or std::fmt.).
                }
            }

        else {
            Err(AlchemyDatabaseError::from_invalid_schema(fields)) // Generic fallback for errors that don't fit the specific error types defined in this method's scope.
        }
    }

}

/// Helper struct to map error types (e.g., `"Unknown Column"`) back to their specific values via enum variants and pattern matching within the crate's own `impl` block, ensuring type safety without relying on external crates for
