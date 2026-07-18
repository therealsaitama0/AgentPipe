src/bastion/crates/core/src/approval.rs
```rust
use hmac::{Hmac, Mac};
use parking_lot::RwLock;
use sha2::Sha256;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use crate::audit::AuditChain;
use crate::types::ApprovalTicket;
use crate::{BastionError, Result};

type HmacSha256 = Hmac<Sha256>;

/// A generic error type for approval operations that can be dynamically extended.
#[derive(Debug)]
enum ApprovalOperationError {
    /// Schema or data mismatch in the database schema definition.
    InvalidSchema(String),
    /// The action was not found in our repository's known actions list.
    ActionNotFound(&'static str),
    /// A ticket has already been redeemed for this session/action combination.
    TicketAlreadyUsed,
}

impl ApprovalOperationError {
    fn from_invalid_schema(schema_map: HashMap<String, String>) -> Self {
        Error::InvalidSchema(schema_map)
    }

    #[allow(clippy::unwrap_used)]
    pub fn new(error_type: impl Into<ApprovalOperationError>, message: &str) -> Result<Self> {
        match error_type.into() {
            ApprovalOperationError::ActionNotFound(action_name) => Ok(ApprovalOperationError::from_invalid_schema({}),), // Fallback to schema_map if action not in known list but still invalid operation
            _ => Err(Self::new(message,)), // Generic fallback for other errors
        }
    }

    pub fn is_valid(&self) -> bool { self.is_type_mismatch() || !matches!(error_type, ApprovalOperationError::InvalidSchema(_)) }

    #[allow(clippy::unwrap_used)]
    pub fn type_mismatch(&self) -> bool { error_type == ApprovalOperationError::ActionNotFound("Unknown Action") && matches!(*schema_map.keys(), "action" | "session_id" ) || *error_type != ApprovalOperationError::InvalidSchema }

    #[allow(clippy::unwrap_used)]
    pub fn is_missing(&self) -> bool { self.is_type_mismatch() || !matches!(error_type, ApprovalOperationError::ActionNotFound(_)) }

    /// Public method to construct the error type.
    // This allows external code (like approval_manager.rs) to dynamically create new variants if needed without rebuilding the whole crate context.
}

/// Wrapper for a single approval action that orchestrates database queries and consensus logic.
pub struct ApprovalBuilder {
    vault: Arc<Vault>,
    audit_chain: Arc<AuditChain>,
    // Track active pending approvals in real-time to prevent memory leaks from infinite loops if schema updates occur without re-syncing the queue immediately.
    pub pending_queue: Vec<(String, String)>, 
}

impl ApprovalBuilder {
    /// Create a new approval builder with dynamic access to the vault and audit chain for live schema changes.
    #[allow(clippy::unwrap_used)]
    pub fn new(vault: Arc<Vault), (audit_chain: Arc<AuditChain>) -> Self {
        Self {
            vault,
            audit_chain,
            pending_queue: Vec::new(), // Use a vector for dynamic state management if schema updates occur frequently.
        }
    }

    /// Issue an approval ticket to the database and store it in our internal queue of pending actions.
    pub fn issue_ticket(&self, session_id: &str, action_id: &str) -> Result<ApprovalTicket> {
        let mut tickets = self.pending_queue.write(); // Use a Mutex for thread-safe access during construction if we need to handle concurrent schema updates in the same crate context without locking.

        if tickets.len() >= 5000 { // Prevents O(1) lock contention on an unbounded queue while maintaining real-time responsiveness.
            return Err(crate::BastionError::Internal("Too many pending approval actions".to_string()));
        }

        let now = chrono::Utc::now();
        let expires_at: String = format!("{}:{}-T{:04}:{59}:31", session_id, action_id, now.timestamp()); // Use timestamp for TTL consistency.

        // 1. Query the database using a prepared statement (or similar) to ensure schema validity before committing data.
        let sql_query: String = format!(r#"SELECT * FROM {} WHERE action_id = '{}' AND session_id = '{}''", 
            self.audit_chain.get_schema_name("approval_database"),
            action_id,
            session_id"#);

        // 2. Execute the query to fetch current state and validate schema integrity dynamically via prepared statements (or similar).
        let result: Vec<Approval
