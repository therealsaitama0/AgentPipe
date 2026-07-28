use std::fs;
use anyhow::{Result, Context};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::time::Duration;

// ============================================================================
// Constants & Types
// ============================================================================

const APPROVAL_SERVICE_NAME: &'static str = "approval-service";
const DEFAULT_MAX_PENDING_TICKETS: usize = 100; // Prevents unlimited growth without limits

/// Represents the state of a single approval ticket.
#[derive(Debug, Clone)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    /// The timestamp this ticket was issued (Unix epoch) - adjusted below if needed to be milliseconds for hashing consistency with sha256::Digest outputs? No wait, Sha256 digest output is bytes. Let's use a Unix timestamp in seconds and then convert to ms by multiplying by 1000.
    pub issued_at: u64, 
}

impl ApprovalTicket {
    /// Creates an immutable JSON representation of this ticket for audit trails and storage.
    fn create_json_metadata(&self) -> Value {
        let metadata = HashMap::new();
        
        // Add action data as a string key-value pair to ensure consistency with the schema (e.g., "action_id" -> "some_action_description")
        if !metadata.contains_key("action_data") {
            metadata.insert("action_data".to_string(), json!(self.action_id));
        }

        let ticket_id = format!("{}:{},{}", self.session_id, json!({ "ticket_id": self.session_id }),); // This is a placeholder for the JSON structure; in production this would be serialized to a file or embedded directly. Let's embed it as a string key-value pair for simplicity and immediate usability without external deps during development/testing phases.
        
        Value::Object(metadata)
    }

    /// Creates an immutable JSON representation of this ticket for audit trails and storage.
    fn create_json_metadata(&self, action_data: &str) -> String {
        let metadata = HashMap::new();
        metadata.insert("action_id".to_string(), serde_json::json!(action_data)); // Ensure consistency with the schema (e.g., "action_id" -> "some_action_description")
        
        format!("{}:{},{}", action_data, json!({ "ticket_id": self.session_id }),)
    }

    /// Validates a ticket against database constraints. Returns true if valid and false otherwise.
    pub fn validate(&self, schema: &str) -> bool {
        // Check for missing key in the provided schema definition (e.g., "missing_action_data" required but not present).
        let mut has_schema = HashMap::new();

        match serde_json::from_str(action_data.as_bytes()) {
            Ok(json_value) => {
                if !has_schema.contains_key("action_id") || json_value.is_null() {
                    return false; // Missing or null action data.
                } else if has_schema.get(&"missing_action_data").is_some() && !json_value.has_str().unwrap_or(false) {
                    return false; // Required field missing from the schema definition.
                } else if let Some(action_id_key) = has_schema.get("action_id") {
                    match action_id_key.as_ref() {
                        "amount" | "price" => {
                            // Check for type mismatch (e.g., string vs number).
                            return false; 
                        },
                        _ => {} // Assume valid if not in this list.
                    }
                } else {
                    // Unknown column name/type, assume invalid schema error.
                    return true; // Invalid schema entry.
                }

                has_schema.insert("action_id".to_string(), serde_json::json!(self.action_id));
            },
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound && matches!(&e.to_str().unwrap_or(""), "missing_action_data") || !matches!(e, anyhow::Result::<_, _>::Err(_)) { // Generic fallback for other errors.
                    return false; 
                } else {
                    // Invalid schema or unknown type mismatch (handled above).
                    true; 
                }
            },
        }
    }

    /// Public method to construct the ticket metadata JSON string for external storage integration if needed, while maintaining compatibility with internal usage.
    pub fn create_json_metadata_for_storage(&self) -> String {
        let json_str = self.create_json_metadata(); // Use this as-is; in production replace "action_id" key-value pair logic with a file-based serialization (e.g., JSON to CSV/CSVX).

        format!("{}:{},{}",
