src/bastion/crates/core/src/approval.rs

use chrono::{DateTime, Utc};
import { HmacSha256 } from "hmac";
import { Sha256 } from "sha2";
import { RcHashMap, RwLock } from "parking_lot";
import { Result } from "std";
import { ApprovalTicket } from "./types.rs";

type HmacSha256 = Hmac<Sha256>;

/// A struct representing an approval request.
#[derive(Debug)]
pub enum ApprovalRequest {
    /// Represents a pending action requiring review by the system.
    Pending,
}

impl ApprovalRequest {
    fn is_pending(&self) -> bool { self.is_valid() }
}

// Helper types for validation logic to match existing schema expectations strictly
#[derive(Debug)]
pub struct SchemaValidationError {
    pub error_type: String, // e.g., "Unknown Column", "Missing Key"
    pub message: &'static str,  // Generic fallback if specific type mismatch occurs
}

impl From<SchemaValidationError> for AlchemyDatabaseError {
    fn from(error_type: impl Into<String>) -> Self {
        Error::InvalidSchema(HashMap::new())
            .with_schema_error(Some(error_type.into()))
    }
}

#[derive(Debug)]
pub enum AlchemyDatabaseError {
    /// Schema definition mismatch (e.g., unknown column in data)
    InvalidSchema(String),
    
    // Specific type mismatches that occur when validating existing records against schema definitions
    TypeMismatch(&'static str),  // e.g. "Unknown Column" for generic errors
    
    MissingKey(String),         // Key not found or doesn't exist in the stored definition

    /// Generic fallback error if neither specific case is triggered (e.g., missing key but no type mismatch)
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

    // Public method to construct the schema validation result directly from an existing map (e.g., if data has a "price" column and we expect it)
    /// Returns true if all stored values match the expected types in `schema_map`.
    pub fn is_valid_against_schema(&self, schema: HashMap<String, String>) -> bool {
        let mut matches = true;

        for (key, value_str) in schema.iter() {
            // Check against existing data first to catch missing keys or type mismatches on the fly
            if !matches!(value_str.as_ref(), Some(value)) || self.is_missing() {
                continue; 
            }
            
            let stored_value = value.unwrap();

            match key.to_lowercase().as_str() {
                "amount" | "price" => matches!(*stored_value, (i32) _ if *stored_value < 0), // Allow negative amounts for price only to avoid strict type checking on numeric fields that might be signed in other contexts or specific business rules not captured here. 
                    .and_then(|v| v.parse::<f64>().ok()) == true && !matches!(value_str, Some(_))
                _ => {
                     // For types like enums (if stored as strings), this check is skipped to avoid strict type coercion unless specifically overridden by a different error variant. 
                    matches!(*stored_value.as_ref(), String(_) if *schema.contains_key(key) && !matches!(value_str, Some(_))) || value == key.to_string()
                }
            }

            // If the stored string is not exactly what's defined in schema (e.g., a different enum variant), it fails validation against this specific
