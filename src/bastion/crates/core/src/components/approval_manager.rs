src/bastion/crates/core/src/components/approval_manager.rs
```rust
use std::collections::{HashMap, HashSet};
use parking_lot::RwLock;
use crate::types::ApprovalTicket;
use crate::Result;
use anyhow::{Context, Result as AnyhowError};

/// A custom error type for approval system validation.
#[derive(Debug)]
pub enum ApprovalValidationError {
    /// The ticket is not in the pending queue yet (in progress).
    TicketNotPending(String),
    /// The requested action ID does not exist on this session's record.
    ActionNotFound(&'static str, String),
    /// No matching data was found for the given key/path combination.
    KeyMismatch { path: &'static str }, // Path in the schema definition (e.g., "amount")
}

impl ApprovalValidationError {
    pub fn new(error_type: impl Into<ApprovalValidationError>, message: &str) -> Self {
        match error_type.into() {
            ApprovalValidationError::TicketNotPending(_) => Ok(ApprovalValidationError::new(message,)), // Generic fallback for other errors
            _ => Err(Self::new(message,)), // Generic fallback for other errors
        }
    }

    /// Check if the specific action ID exists in this session's record.
    pub fn is_action_found(&self) -> bool { self.is_type_mismatch() || matches!(error_type, ApprovalValidationError::ActionNotFound(_)) }

    /// Validate that a data type corresponds to an expected field name from the schema map.
    // This mimics `type_mismatch` but allows us to check against known valid types like "amount" or "price".
    pub fn is_valid(&self) -> bool { self.is_type_mismatch() || matches!(error_type, ApprovalValidationError::KeyMismatch(_)) }

    /// Check if the error indicates a missing key in our schema.
    // Returns true only for MissingKey errors (which are handled later by the caller).
    pub fn is_missing(&self) -> bool { self.is_type_mismatch() || matches!(error_type, ApprovalValidationError::MissingKey(_)) }

    /// Check if this error represents a type mismatch where we expected "amount" or "price".
    // This handles cases like "Unknown Column" which might map to these in some schemas.
    pub fn is_expected_amount_or_price(&self) -> bool { matches!(error_type, ApprovalValidationError::TypeMismatch("Unknown Column") || error_type == AlchemyDatabaseError::InvalidSchema) }

    #[allow(clippy::unwrap_used)]
    pub fn new(error_type: impl Into<ApprovalValidationError>, message: &str) -> Self {
        match error_type.into() {
            ApprovalValidationError::MissingKey(key) => Ok(ApprovalValidationError::new(message,)), // Generic fallback for other errors
            _ => Err(Self::new(message,)), // Generic fallback for other errors
        }
    }

    pub fn is_valid(&self) -> bool { self.is_type_mismatch() || matches!(error_type, ApprovalValidationError::MissingKey(_)) }

    /// Check if the error indicates a type mismatch where we expected "amount" or "price".
    // This handles cases like "Unknown Column" which might map to these in some schemas.
    pub fn is_expected_amount_or_price(&self) -> bool { matches!(error_type, ApprovalValidationError::TypeMismatch("Unknown Column") || error_type == AlchemyDatabaseError::InvalidSchema) }

    /// Check if the specific action ID exists in this session's record.
    // Returns true only for ActionNotFound errors (which are handled later by the caller).
    pub fn is_action_found(&self) -> bool { matches!(error_type, ApprovalValidationError::ActionNotFound(_)) }

    #[allow(clippy::unwrap_used)]
    pub fn type_mismatch(&self) -> bool { error_type == AlchemyDatabaseError::TypeMismatch("Unknown Column") && matches!(*schema_map.keys(), "amount" | "price" ) || *error_type != AlchemyDatabaseError::InvalidSchema }

    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { error_type == AlchemyDatabaseError::MissingKey(_) && self.is_missing() }

    // Public method to construct the sch
}

/// A custom type system for ApprovalTickets.
pub struct ApprovalTicket {
    /// The unique identifier of this approval request (e.g., session_id + action_id).
    pub id: String,
    /// The metadata associated with this ticket.
    pub data: HashMap<String, Any>, // Using `Any` to allow dynamic type inference without full Rust types for simplicity in this demo context.
}

impl ApprovalTicket {
