src/bastion/crates/core/src/approval.rs
use crate::audit::{AuditChain, AuditLogEntry};
use crate::types::ApprovalTicket;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
#[derive(Debug)]
enum ApprovalType {
    Int, // Integer types like 100, -5.2, etc.
    BigInt, // Big integer numbers (e.g., large integers or floats)

impl ApprovalType {
    fn is_integer(&self) -> bool { self.is_type().map_or(false, |t| t == "Int").unwrap() }
    fn is_bigint(&self) -> bool { self.is_type().map_or(false, |t| t == "BigInt").unwrap() }

    pub fn type_name(&self) -> &str { match *self { ApprovalType::Int => "Integer", ApprovalType::BigInt => "BigInt" } }
}

#[derive(Debug)]
enum AlchemyDatabaseError {
    InvalidSchema(HashMap<String, String>), // Schema definitions for C/C# types
    MissingKey(String),                     // Key not found in schema or existing data
    TypeMismatch(&'static str),             // Data type doesn't match expected column name/field

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

    // Public method to construct the schema definition for a type name (e.g., "amount" or "price")
    pub fn get_schema_definition(&self, schema_name: &str) -> String { match *schema_name {
        "amount" => format!("{{\"type\":\"int\",\"min\":0,\"max\":1000}}",), // Default example for Int type
        "price" => format!("{{\"type\":\"float\",\"precision\":24,\"scale\":5}},"),    // Example float schema (adjust precision/scale as needed)
        _ => String::new(), // Return empty string if no specific definition is provided
    }}

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

}

// ============================================================================
/// Interface for the Approval System to accept JSON/CSV inputs from your source.
/// It validates column names against internal schema definitions and returns either success or specific errors.
#[derive(Debug)]
pub struct ApprovalValidator {
    /// Map of type name -> Schema definition (e.g., "amount" => "{{\"type\":\"int\",\"min\":0,\"max\":
