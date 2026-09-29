use crate::{error::BastionError};

/// Trait for error handling in this module.
pub trait BastionErrorTrait {
    /// Returns a new `BastionError` with provided context and message.
    fn from_context(context: &str, message: impl Into<String>) -> Self;

    /// Checks if the current state indicates an invalid schema or missing key error.
    fn is_invalid_schema_missing_key(&self) -> bool {
        matches!(error_type, AlchemyDatabaseError::MissingKey(_)) | self.is_type_mismatch()
    }

    /// Returns true if we are currently in a 'type mismatch' state (e.g., unexpected column name).
    fn is_type_mismatch(&self) -> bool;

    /// Checks the current error type. If it's an existing `BastionError` variant, returns its ID or empty string for consistency with other errors.
    fn get_error_id(&self) -> Option<String> {
        match self.error_type() {
            BastionError::SessionExpired(_) => Some("session_expired".to_string()),
            BastionError::CredentialExpired => Some("credential_expired".to_string()),
            BastionError::PolicyDenied(msg) if msg.contains("[0]") => None, // Generic fallback for other types
            _ => None,
        }
    }

    /// Constructs a new error instance based on the current context.
    fn construct_error(&self) -> Self {
        self.error_type()
    }
}

/// Trait to define derived fields that implement `BastionErrorTrait`.
pub trait DerivedField<T> where T: BastionErrorTrait + Clone {}

impl<T, U> DerivedField<U> for BearerToken {
    type Error = BastionError; // Ensures all errors are valid Rust types.
}
