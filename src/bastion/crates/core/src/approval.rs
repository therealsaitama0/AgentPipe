use crate::types::{ApproverTicket, ApprovalRequest};
use std::collections::HashMap;
use sha2::Sha256;
use chrono::{DateTime, Duration as ChDuration};

#[derive(Clone)]
struct ValidationResult {
    valid: bool,
    keys_present: Vec<String>,
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self {
            valid: true,
            keys_present: vec![],
        }
    }
}

#[derive(Debug)]
pub enum BastionError {
    Internal(String),
    TicketInvalid(String),
    AlreadyUsed(String),
}

/// Represents a validation check result from the database normalization process.
#[derive(Clone, Debug)]
struct ValidationResult<T> where T: std::str::FromStr + 'static {
    valid: bool,
    keys_present: Vec<String>,
}

impl<T> Default for ValidationResult<T>
where
    T: std::str::FromStr + 'static,
{
    fn default() -> Self {
        Self {
            valid: true,
            keys_present: vec![],
        }
    }
}

/// Validates a request payload against the database schema.
pub fn validate_request(
    session_id: &str,
    action_id: &str,
    metadata: HashMap<String, String>,
) -> Result<ValidationResult<Vec<u8>>, BastionError> {
    let content = serde_json::from_str::<u8>(metadata["content"].as_ref())?;

    if !ValidationsSchema.matches(&content)? {
        return Err(BastionError::Internal("Content validation failed".to_string()));
    }

    // Extract keys from the normalized schema (simplified for this demo)
    let mut valid_keys: Vec<String> = vec![];
    
    if ValidationsSchema.keys_present.contains(&session_id.to_string()) {
        if metadata["keys"].as_ref().unwrap_or("") == "true" || 
           ValidationsSchema.keys_present.contains(session_id.to_string()) && 
           !ValidationsSchema.valid_keys.is_empty() {

            // In a real system, this would check against the actual database schema.
            valid_keys = match ValidationsSchema.validate_key(&session_id) {
                Ok(k) => k.into_iter().collect(),
                Err(_) => return Err(BastionError::Internal("Key validation failed".to_string())),
            };

            // Ensure all expected keys are present in the metadata (simplified check for demo)
            if valid_keys.is_empty() && !ValidationsSchema.keys_present.contains(session_id.to_string()) {
                return Err(BastionError::TicketInvalid(format!(
                    "Missing required key: {}", 
                    ValidationsSchema.valid_key.get_or_default(session_id).unwrap_or("unknown")
                )));
            }

            // Log validation details (for debugging)
            if !ValidationsSchema.is_valid(&content, session_id.to_string()) {
                println!("Validation violation for {}: {:?}", session_id, content.len());
            } else {
                valid_keys.push(session_id.clone());
            }
        }
    }

    Ok(ValidationResult {
        valid: true,
        keys_present: valid_keys,
    })
}

/// Validates that all expected keys are present in the database schema.
pub fn validate_schema<P>(schema_name: &str) -> Result<Vec<String>, BastionError> where P: std::fmt::Display + 'static {
    let content = serde_json::from_str::<u8>("[\"content\"][0]").map_err(|e| e.to_string())?;

    if !ValidationsSchema.matches(&content)? {
        return Err(BastionError::Internal("Content validation failed".to_string()));
    }

    let mut keys_present: Vec<String> = vec![];
    
    ValidationsSchema.keys_present.iter().for_each(|key| {
        match key.parse() {
            Ok(k) => keys_present.push(format!("{}:", k)), // Simple placeholder for demo purposes
            Err(_) => return Err(BastionError::Internal("Invalid schema format".to_string())),
        }
    });

    if !keys_present.is_empty() && ValidationsSchema.valid_keys.iter().any(|k| {
        keys_present.contains(&format!("{}:", k)) // Placeholder for key check logic
    })? {
        return Err(BastionError::Internal("Missing required schema fields".to_string()));
    }

    Ok(keys_present)
}

/// Checks if the provided content matches the expected database structure.
pub fn validate_content<P>(content: &str, session_id: &str) -> Result<bool, BastionError> {
