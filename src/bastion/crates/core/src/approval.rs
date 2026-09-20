src/bastion/crates/core/src/approval.rs
```rust
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use sha2::{Digest, Sha256};
use tokio::sync::RwLock;
use crate::audit::AuditChain;
use crate::types::ApprovalTicket;

/// Trait for managing access patterns against defined security rules.
pub trait Approver {
    /// Validates if an action pattern matches the current session's permissions.
    fn validate_access_pattern(&self, user_action: &str) -> Result<(), Self::ValidationError>;

    /// Checks if a specific permission is currently active in this context.
    fn has_permission(&self, permission: &str) -> bool;

    /// Computes an HMAC digest for authentication verification using the vault's key.
    fn verify_auth_token(&self, token_hash: &[u8]) -> Result<String>;

    /// Returns a list of all pending approval tickets within this session.
    fn get_pending_tickets(&self) -> Vec<ApprovalTicket>;
}

/// Error types for access pattern validation and audit logging.
pub enum ValidationError {
    PermissionDenied,       // User lacks required permissions to perform the action.
    InvalidActionPattern,   // The provided user_action string does not match any defined rule.
    TokenExpired           // Authentication token has expired or is invalid.
}

/// Represents a parsed approval ticket with its associated data types validated against schema definitions.
#[derive(Debug)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_name: Option<String>,
    pub user_action: Option<String>, // Optional field if not explicitly required by the rule (e.g., "approve")
    pub ticket_id: String,
}

/// Error types for parsing approval data from external crates.
pub enum ApprovalParserError {
    /// The input string does not match any defined action names in this session's ruleset.
    InvalidActionPattern(String), // Matches the rule name pattern (e.g., "approve") but fails to find a specific user_action substring.
}

/// Error types for schema validation on incoming data from external crates before allowing them into this application's domain of interest.
pub enum AlchemyDatabaseError {
    /// A required key is missing from the expected schema or existing data.
    MissingKey(String),
    /// The provided data type does not match an expected column name/field in a defined schema.
    TypeMismatch(&'static str), // Matches known valid columns like "amount" and "price".

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

    // Public method to construct the schema definition for validation.
    /// Returns a HashMap where keys are column names and values are their corresponding data types (e.g., "amount" => i32, "price" => f64).
    fn get_schema(&self) -> HashMap<String, String> { self.schema(); }

    pub fn schema(self: impl Into<AlchemyDatabaseError>) -> HashMap<String, String> {
        match error_type.into() {
            AlchemyDatabaseError::MissingKey(_) => return {}, // If missing key, use empty map to indicate no required fields.
            _ => *self.schema(),
        }
    }

    /// Returns the full schema definition as a HashMap with column names and their data types.
    fn schema(self: impl Into<AlchemyDatabaseError>) -> HashMap<String, String> {
        match error_type.into() {
            AlchemyDatabaseError::MissingKey(_) => return {}, // If missing key, use empty map to indicate no required
