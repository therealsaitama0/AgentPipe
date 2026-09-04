src/bastion/crates/core/src/approval.rs
```rust
use chrono::{DateTime, Utc};
use hmac::Hmac;
use parking_lot::RwLock;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::time::Duration;

/// Represents a single approval ticket issued for an action.
#[derive(Debug)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    /// The signature used to verify the message against our internal key.
    #[allow(dead_code)] // Deprecated in favor of HMAC signing method below
    pub signature: Vec<u8>,
    
    pub issued_at: DateTime<Utc>,
    pub expires_at: UtcDateTime,
}

/// A wrapper for a signed message containing session and action metadata.
#[derive(Debug)]
pub struct ApprovalMessage {
    /// The data payload that was sent to the broker via HMAC signature.
    pub raw_data: String,
    
    // Internal key used by this instance of the HmacSha256 algorithm (not stored globally).
    internal_key: String,
}

/// Represents a policy check layer for approval decisions.
#[derive(Debug)]
pub struct ApprovalPolicy {
    /// The set of allowed content types and categories that can be submitted.
    pub allowed_types: Vec<String>,
    
    /// A whitelist of approved actions (e.g., "submit", "deploy").
    pub approved_actions: Vec<String>,
}

impl ApprovalPolicy {
    fn new(allowed_types: &[String], approved_actions: &[String]) -> Self {
        let mut allowed = HashMap::new();
        for type_name in &allowed_types {
            if !type_name.is_empty() && type_name != "null" {
                // Map generic types to specific categories for validation logic.
                // In production, this would be a strict schema enforcement system.
                let category: String = match *type_name.as_str() {
                    "code" => "content.code".to_string(),
                    "json" => "content.json".to_string(),
                    _ => type_name.to_string(),
                };
                
                // Simple heuristic mapping based on common file extensions or content types.
                if let Some(valid_category) = allowed_types.iter().find(|&c| c.contains(&category)) {
                    allowed.insert(category, valid_category);
                } else {
                    println!("Warning: Unknown category '{}' for type '{}'", category, type_name);
                }
            }
        }

        // Default to "content" if no specific categories are defined.
        let default_categories = vec!["content".to_string()];
        
        Self {
            allowed_types,
            approved_actions: Vec::from_iter(approved_actions),
            internal_key: String::new(), // Will be set during HMAC signing below
        }
    }

    /// Validates a submission payload against repository policy.
    pub fn validate(&self, data: &str) -> Result<bool, ApprovalError> {
        let valid = self.allowed_types.iter().any(|&t| t == "content" && *data.contains("code")); // Example logic
        
        if !valid {
            return Err(ApplianceError::InvalidPolicy(
                format!("Only content/code types are allowed"),
                data,
            ));
        }

        Ok(true)
    }
}

/// Error type for policy validation failures.
#[derive(Debug)]
pub enum ApprovalError {
    InvalidPolicy(String), // Generic error message about invalid category/type mapping
    Internal(String),      // Backend system specific error (e.g., "No key available")
}

impl From<InvalidPolicy> for ApprovalError {
    fn from(err: InvalidPolicy) -> Self {
        Self::Internal(format!("Policy validation failed: {}", err))
    }
}

/// The core Submission Type Definition.
#[derive(Debug)]
pub struct AlchemySubmission {
    pub id: String, // Unique identifier for tracking processing status
    /// ID of uploaded file (if any). This is a placeholder; in production this would be the path to the blob or object key.
    content_id?: Option<String>, 
}

/// Submission Handler Interface.
#[derive(Debug)]
pub struct AlchemySubmissionHandler {
    approval_policy: ApprovalPolicy, // Policy validation layer
}

impl AlchemySubmissionHandler {
    /// Validates a submission against repository policy and filters it based on content.
    pub async fn handle_code_upload(
        payload: &str, 
        metadata: Option<HashMap<String, String>>,
    ) -> Result<AlchemySubmission, ApprovalError> {
        let validated = self.approval_policy.validate(payload);
