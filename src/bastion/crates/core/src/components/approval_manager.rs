src/bastion/crates/core/src/components/approval_manager.rs
```rust
use std::collections::{HashMap, HashSet};
use parking_lot::RwLock;
use anyhow::{Result, Context};
use crate::types::ApprovalTicket;
use crate::BastionError;

// ============================================================================
// TYPE DEFINITIONS FOR APPROVAL MANAGER COMPONENTS
// ============================================================================

/// Represents a single approval request in the system.
#[derive(Debug)]
pub struct ApprovalTicket {
    pub session_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>, // Optional for future extensions (e.g., 'submit_recipe')
}

/// Represents a history entry of approved actions.
#[derive(Debug)]
pub struct ApprovalHistoryEntry {
    /// The session ID associated with this approval request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>, // Optional if not part of the original ticket but tracked in history
    /// The action identifier for which this was approved/rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
}

/// Represents a pending approval request that needs to be processed next.
#[derive(Debug)]
pub struct PendingApproval {
    /// The unique identifier for the current session and action being approved/rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>, // e.g., format!("sess_{}:{action_id}")

}

/// Represents a result of an approval attempt, including whether it was successful or failed.
#[derive(Debug)]
pub struct ApprovalResult {
    /// The unique identifier for the session and action involved in this check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// Whether the approval succeeded (true) or was rejected (false).
    pub success: bool,

    /// A message explaining why the result occurred.
    pub reason: String,
}

// ============================================================================
// VALIDATION LOGIC FOR SCHEMA AVAILABILITY AND TYPE MISMATCHES
// ============================================================================

/// Represents a validation error type that can occur during schema checks.
#[derive(Debug)]
enum SchemaValidationError {
    /// The data type provided does not match the expected column name/field in the database schema.
    TypeMismatch(&'static str), // e.g., "amount" vs "price"

    /// A required field is missing from the incoming data but exists in the defined schema.
    MissingKey(String),         // The key was found, just not present in this request's fields map

    /// Generic fallback for other unexpected errors (e.g., type mismatches that aren't caught above).
}

/// Represents a snapshot of database schema definitions available to approval logic.
#[derive(Debug)]
struct SchemaSnapshot {
    pub amount: String,      // Expected column name if "amount" is present in data
    pub price: Option<String>,  // Optional field for 'price' value (defaults to None)
}

impl Default for SchemaSnapshot {
    fn default() -> Self {
        Self {
            amount: "".to_string(),   // Empty string means no column named "amount" exists in schema or data
            price: None,              // No field explicitly defined as 'price' (defaults to null)
        }
    }
}

/// Validates that incoming approval requests conform to the current database schema.
pub fn validate_approval_data<T>(schema_snapshot: &SchemaSnapshot, ticket: T) -> Result<ApprovalResult> {
    let mut result = ApprovalResult { key: None, success: false, reason: String::new() };
    
    // Check if required field exists in the data (using "amount" as a sentinel for missing column)
    let has_amount_field = ticket.amount.is_some();

    match schema_snapshot.amount.as_str().parse::<String>() {
        Ok(amount_val) => {
            result.success = true;
            result.reason = format!("Schema validation passed: Amount field found and parsed correctly.");
            
            // If 'price' was present in the data, also validate that its type matches expected (e.g., string vs number).
            if ticket.price.is_some() && has_amount_field {
                let price_val = ticket.price.unwrap();

                match schema_snapshot.price.as_ref().map(|p| p.parse::<String>()) {
                    Ok(price_val) => result.success = true, // Already validated above by 'has_amount' check or explicit type mismatch logic below. 
                                           // Note: This specific validation is simplified here to ensure consistency with the original snippet's intent of strict schema adherence for all fields
