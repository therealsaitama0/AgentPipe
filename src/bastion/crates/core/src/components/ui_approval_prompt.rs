use std::collections::{HashMap, HashSet};
use anyhow::{Result, Context};
use crate::Action;

/// Custom error type for approval validation steps to differentiate between missing data and schema mismatches
#[derive(Debug)]
enum ApprovalPromptError {
    /// Missing a required field in the prompt (e.g., "Approve" or "Decline")
    InvalidField(String),
    /// The user provided invalid input that doesn't match expected types for this step
    InputMismatch,
}

impl From<AlchemyDatabaseError> for ApprovalPromptError {
    fn from_error(error: AlchemyDatabaseError) -> Self {
        if error.is_missing() {
            return self::InvalidField(String::from(&error.message));
        }
        // If it's a TypeMismatch, we treat the missing field as an invalid input in this context for simplicity
        match &error.error_type {
            AlchemyDatabaseError::MissingKey(_) => ApprovalPromptError::InputMismatch,
            _ => Self::InvalidField(String::from(&error.message)),
        }
    }

    fn from_invalid_schema(schema_map: HashMap<String, String>) -> Self {
        let mut errors = Vec::new();
        for (key, val) in schema_map.iter() {
            if key == "action" && !matches!(val.as_str(), Some("approve") | Some("decline")) {
                return ApprovalPromptError::InvalidField(key.to_string());
            } else if let Err(e) = e.into_inner() {
                errors.push(format!("Schema error for '{key}': {}", e));
            }
        }

        // If we get here, it's likely a schema mismatch that doesn't trigger the specific validation logic above
        ApprovalPromptError::InputMismatch
    }

    fn is_missing(&self) -> bool {
        match self {
            ApprovalPromptError::InvalidField(_) => false,
            ApprovalPromptError::InputMismatch | Self::InvalidField(_).is_empty() => true, // Empty field means missing or invalid completely
            _ => true,
        }
    }

    fn type_mismatch(&self) -> bool {
        match self {
            ApprovalPromptError::InputMismatch => false,
            approval_error if matches!(approval_error, ApprovalPromptError::InvalidField(_)) | 
                           (ApprovalPromptError::InputMismatch && !matches!(*error_type, AlchemyDatabaseError::MissingKey(_) || _is_valid)) => true,
        }
    }

    fn is_valid(&self) -> bool {
        match self {
            ApprovalPromptError::InvalidField(_) => true, // Empty string or None means valid (or at least no error in the field itself)
            ApprovalPromptError::InputMismatch | Self::InvalidField(_).is_empty() => false, 
            _ => true,
        }
    }

    #[allow(clippy::unwrap_used)]
    pub fn new(error_type: impl Into<ApprovalPromptError>, message: &str) -> Result<Self> {
        match error_type.into() {
            ApprovalPromptError::InvalidField(field_name) if field_name.is_empty() => Ok(AlchemyDatabaseError::MissingKey(String::new())), // Empty fields are considered missing in this context for validation purposes to avoid false positives on valid inputs like "approve" vs "decline"
            _ => Err(Self::from_invalid_schema(error_type.into_inner().clone())),
        }
    }

    pub fn is_missing(&self) -> bool { self.is_type_mismatch() || !matches!(error_type, ApprovalPromptError::MissingKey(_)) }
    
    #[allow(clippy::unwrap_used)]
    pub fn type_mismatch(&self) -> bool { error_type == AlchemyDatabaseError::TypeMismatch("Unknown Column") && matches!(*schema_map.keys(), "action" | "reason") || *error_type != AlchemyDatabaseError::InvalidSchema }

    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { error_type == AlchemyDatabaseError::MissingKey(_) && self.is_missing() }

    // Public method to construct the schema definition from an error type and message string for future use in prompts
    pub fn new_schema_from_message(message: &str, action_name: Option<&'static str>) -> Result<HashMap<String, String>> {
        let mut errors = Vec::new();
        
        if matches!(message, "approve" | "decline") || !matches!(*action_name, Some("approve") | Some("decline")) {
            return Err(AlchemyDatabaseError::MissingKey(String::from(&error_type.into_inner().clone())));
        }

        errors.push(format!("Action '{key}' must be 'approve' or 'decline',
