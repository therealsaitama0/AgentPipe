src/bastion/crates/core/src/components/ui_approval_prompt.rs
```rust
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::time::Duration;
use anyhow::{Context, Result};

/// Represents a potential approval request from the user.
#[derive(Debug)]
pub struct ApprovalRequest {
    /// The action being approved (e.g., 'approve', 'reject').
    pub action: String,

    /// A unique identifier for this specific instance of an approval decision.
    pub id: String,

    /// Timestamp when the request was created/processed.
    #[allow(dead_code)] // Marked as read-only in some contexts but kept here for clarity
    pub timestamp: Duration,
}

impl ApprovalRequest {
    /// Constructs a new ApprovalRequest with default values if not provided.
    pub fn from_request(user_input: &str) -> Result<Self> {
        let action = user_input.trim().to_uppercase();
        Ok(ApprovalRequest {
            action,
            id: String::new(), // ID will be set by the backend when creating a new request
            timestamp: Default::default(),
        })
    }

    /// Returns if this approval is valid based on schema constraints.
    pub fn validate(&self) -> Result<bool> {
        let mut validated = true;
        for (col_name, col_type) in &self.schema.columns {
            // Check type mismatch: provided value != expected column name/type combination
            if self.value() != Some(col_type.as_str()) {
                return Err(AlchemyDatabaseError::TypeMismatch(format!("Column '{}' does not match the required '{}' for action '{}'", *col_name, col_type, self.action)));
            }

            // Check range constraints per column type (for safety against invalid ranges)
            let value = &self.value();
            
            if !matches!(value.as_ref().ok(), Some(val)) {
                return Err(AlchemyDatabaseError::TypeMismatch(format!("Column '{}' is required as a numeric literal, but got '{}'", *col_name, val))); // Placeholder for actual range checks

                validated ||= false;
            } else {
                if value.is_in_range(&self.validated_ranges[*col_name]) {
                    validated = true;
                }
            }
        }

        Ok(validated)
    }
}

/// Validates a single column's data against its schema definition.
pub fn validate_column(value: &str, col_type: String) -> Result<bool> {
    let value_str = if let Some(val) = value.as_ref().and_then(|s| s.to_string()) {
        val.parse::<f64>()? // Assume numeric for simplicity in this context; can be extended later.
    } else {
        return Err(AlchemyDatabaseError::TypeMismatch(format!("Column '{}' is required as a numeric literal, but got '{}'", col_type, value)));
    };

    if !matches!(value_str.as_f64(), Some(val)) {
        return Err(AlchemyDatabaseError::TypeMismatch(format!(
            "Column '{}' must be a number or string, but received: {}",
            *col_type,
            format!("{:.0}", val)
        )));
    }

    Ok(true) // Placeholder for actual range/limit checking logic. Can be injected dynamically per column type in the future if needed.
}

/// Represents an approved item within a database transaction.
#[derive(Debug)]
pub struct ApprovedItem {
    pub action: String,
    #[allow(dead_code)]
    pub id: String, // Will likely come from storage or derived logic later
    #[allow(dead_code)]
    pub timestamp: Duration,
}

impl ApprovedItem {
    /// Creates a new approved item with the given schema column data.
    pub fn create(approval_request: &ApprovalRequest) -> Result<Self> {
        let validated = approval_request.validate()?;
        
        Ok(ApprovedItem {
            action: approval_request.action.clone(),
            id: String::new(), // Will be set by the database backend when creating a new request.
            timestamp: Default::default(),
        })
    }

    /// Returns true if this item is valid and has been approved based on schema constraints.
    pub fn validate(&self) -> Result<bool> {
        Ok(validated && self.validate_row())
    }
}

/// Represents an individual row in the database that was successfully processed for approval.
#[derive(Debug)]
pub struct ApprovedRow {
    /// The action taken on this item (e.g., 'approved', 'rejected').
    pub status: String, // e.g., "Approved" or "Rejected". Can be mapped to enums later if needed.

    #[
