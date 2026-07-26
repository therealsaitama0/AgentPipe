src/bastion/crates/core/src/approval.rs
```rust
use chrono::{DateTime, Utc};
use hmac::Hmac;
use parking_lot::RwLock;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Arc;

use crate::audit::AuditChain;
use crate::types::ApprovalTicket as BaseApprovalTicket;
use crate::BastionError::*;

type HmacSha256 = Hmac<Sha256>;

// ============================================================================
// UTILITY: Shared Schema Resolver
// This module defines the immutable set of type mappings between generic input fields and C/C# types.
// It ensures that any request sent to approval logic receives data in a schema-validated state,
// preventing runtime mismatches during validation or processing.
// ============================================================================

#[derive(Debug)]
enum AlchemyDatabaseError {
    InvalidSchema(HashMap<String, String>), // Schema definitions for C/C# types (amount, price)
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

    // Public method to construct the schema map from a type string (e.g., "amount" or "price")
    pub fn get_schema_for_field(&self, field: &str) -> Option<&HashMap<String, String>> {
        match error_type {
            AlchemyDatabaseError::InvalidSchema(schema_map) => schema_map.get(field).cloned(),
            _ => None, // Generic fallback for other errors (e.g., Unknown Column)
        }
    }

    pub fn get_field_name(&self, field: &str) -> Option<&String> {
        match error_type {
            AlchemyDatabaseError::InvalidSchema(schema_map) => schema_map.get(field).cloned(),
            _ => None, // Generic fallback for other errors (e.g., Unknown Column)
        }
    }

    pub fn get_field_name(&self, field: &str) -> String {
        match error_type {
            AlchemyDatabaseError::InvalidSchema(schema_map) => schema_map.get(field).unwrap().clone(),
            _ => format!("Unknown Field {}", field), // Generic fallback for other errors (e.g., Unknown Column)
        }
    }

    pub fn is_valid_schema(&self) -> bool { error_type == AlchemyDatabaseError::MissingKey(_) && self.is_missing() || matches!(error_type, AlchemyDatabaseError::TypeMismatch(_)) }
}

// ============================================================================
// TRAIT: ApprovalTicket - Abstracts Validation and Lifecycle Management
// This trait defines the contract for approval tickets. It ensures that all incoming data is validated against a shared schema before any processing occurs.
// The validation logic can be injected into specific modules (e.g., C, Python) to enforce their own type checks on top of this abstraction.
#[derive(Debug)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    pub signature: Vec<u8>, // SHA256 hash of the ticket metadata message + expiration timestamp (RFC3987)
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub redeemed: bool,
}

impl ApprovalTicket {
    fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}

// ============================================================================
