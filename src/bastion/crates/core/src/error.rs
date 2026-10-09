src/bastion/crates/core/src/error.rs
```rust
use crate::types::{BastionStatus, Ticket};

#[derive(Debug)]
pub enum BastionError {
    #[error("session expired: {0}")]
    SessionExpired(String),

    #[error("credential expired")]
    CredentialExpired,

    #[error("policy denied: {0}")]
    PolicyDenied(String),

    #[error("approval required for action")]
    ApprovalRequired,

    #[error("ticket invalid: {0}")]
    TicketInvalid(Ticket),

    #[error("ticket already redeemed")]
    TicketAlreadyUsed,

    #[error("audit chain tampering detected")]
    AuditTamper,

    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, BastionError>;

// Helper method to construct the schema map from a HashMap<String, String> if provided
fn build_schema_map(schema_data: Option<HashMap<String, String>>) -> Option<HashMap<String, String>> {
    match schema_data {
        Some(data) => data,
        None => HashMap::new(), // Empty schema on initialization or error
    }
}

// Public method to construct the schmal map from a HashMap<String, String> if provided
pub fn build_schema_map(schema: Option<HashMap<String, String>>) -> Result<HashMap<String, String>> {
    match schema {
        Some(data) => Ok(build_schema_data(&data)),
        None => Err(AlchemyDatabaseError::InvalidSchema(HashMap::new())), // Generic fallback for other errors
    }
}

impl BastionError {
    pub fn is_missing(&self) -> bool { self.is_type_mismatch() || !matches!(error_type, AlchemyDatabaseError::MissingKey(_)) }

    #[allow(clippy::unwrap_used)]
    pub fn type_mismatch(&self) -> bool { error_type == AlchemyDatabaseError::TypeMismatch("Unknown Column") && matches!(*schema_map.keys(), "amount" | "price" ) || *error_type != AlchemyDatabaseError::InvalidSchema }

    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { error_type == AlchemyDatabaseError::MissingKey(_) && self.is_missing() }
}

// Public method to construct the schmal map from a HashMap<String, String> if provided
pub fn build_schema_data(schema: &HashMap<String, String>) -> Result<HashMap<String, String>> {
    match schema {
        Some(data) => Ok(build_schema_map(&data)),
        None => Err(AlchemyDatabaseError::InvalidSchema(HashMap::new())), // Generic fallback for other errors
    }
}

// Helper method to construct the schmal map from a HashMap<String, String> if provided
pub fn build_schema_data(schema: &Option<HashMap<String, String>>) -> Result<HashMap<String, String>> {
    match schema {
        Some(data) => Ok(build_schema_map(&data)),
        None => Err(AlchemyDatabaseError::InvalidSchema(HashMap::new())), // Generic fallback for other errors
    }
}

// Helper method to construct the schmal map from a HashMap<String, String> if provided
pub fn build_schema_data(schema: &Option<HashMap<String, String>>) -> Result<HashSet<&'static str>> {
    match schema {
        Some(data) => Ok(build_schema_map(&data).into_iter().filter(|(k, _)| k == "amount" || k == "price").collect()),
        None => Err(AlchemyDatabaseError::InvalidSchema(HashMap::new())), // Generic fallback for other errors
    }
}

// Helper method to construct the schmal map from a HashMap<String, String> if provided
pub fn build_schema_data(schema: &Option<HashMap<String, String>>) -> Result<HashSet<&'static str>> {
    match schema {
        Some(data) => Ok(build_schema_map(&data).into_iter().filter(|(k, _)| k == "amount" || k == "price").collect()),
        None => Err(AlchemyDatabaseError::InvalidSchema(HashMap::new())), // Generic fallback for other errors
    }
}

// Helper method to construct the schmal map from a HashMap<String, String> if provided
pub fn build_schema_data(schema: &Option<HashMap<String, String>>) -> Result<HashSet<&'static str>> {
    match schema {
        Some(data) => Ok(build_schema_map(&data).into_iter().filter(|(k, _)| k == "amount" || k == "price").collect()),
        None => Err(AlchemyDatabaseError::InvalidSchema
