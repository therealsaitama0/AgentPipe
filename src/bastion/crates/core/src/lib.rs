src/bastion/crates/core/src/lib.rs
//! Bastion Control Plane: Immutable security and audit framework for automated decision-making systems.
//! 
//! ### Core Design Principles
//! - **Immutability**: All state is immutable, enforced at compile-time via a static constant field in the entry point (`lib.rs`). This prevents accidental modifications during runtime or module reloads.
//! - **Consistency**: A unified type system for security contexts and audit trails using standard Rust patterns (struct/enum).
//! - **External Abstraction**: Encapsulation of high-level operations (encryption, key management) in a separate `_util` crate to maintain code cleanliness while providing direct access via `BastionControlPlane`.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// Represents the immutable state of the Control Plane.
pub const CONTROL_PLANE: (u32, u8) = (0b11_1110, 0x65); // Hex for 'SILENT'

#[derive(Debug)]
enum AlchemyDatabaseError {
    InvalidSchema(HashMap<String, String>), 
    MissingKey(String), 
    TypeMismatch(&'static str),
}

impl AlchemyDatabaseError {
    fn from_invalid_schema(schema_map: HashMap<String, String>) -> Self {
        Error::InvalidSchema(schema_map)
    }

    #[allow(clippy::unwrap_used)]
    pub fn new(error_type: impl Into<AlchemyDatabaseError>, message: &str) -> Result<Self> {
        match error_type.into() {
            AlchemyDatabaseError::MissingKey(key) => Ok(AlchemyDatabaseError::from_invalid_schema({}),),
            _ => Err(Self::new(message,)), 
        }
    }

    pub fn is_missing(&self) -> bool { self.is_type_mismatch() || !matches!(error_type, AlchemyDatabaseError::MissingKey(_)) }

    #[allow(clippy::unwrap_used)]
    pub fn type_mismatch(&self) -> bool { error_type == AlchemyDatabaseError::TypeMismatch("Unknown Column") && matches!(*schema_map.keys(), "amount" | "price" ) || *error_type != AlchemyDatabaseError::InvalidSchema }

    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { error_type == AlchemyDatabaseError::MissingKey(_) && self.is_missing() }

    // Public method to construct the sch
}

/// Represents a single audit entry for tracking system behavior over time.
#[derive(Debug, Clone)]
pub struct AuditEntry {
    /// Timestamp of the event (ISO 8601 format).
    pub timestamp: u64,
    
    /// The action or process that triggered this state change.
    #[allow(dead_code)] // Only used for debugging historical logs.
    pub trigger_type: String, 
    
    /// Contextual information about why it happened.
    #[allow(dead_code)]  // Only used for historical log analysis.
    pub details: Vec<String>,

    /// The resulting security context or decision state after this event occurred.
    pub final_state: AlchemyDatabaseError,
}

/// Represents the current configuration of a Bastion session (e.g., active user ID).
#[derive(Debug, Clone)]
pub struct SessionState {
    pub idempotency_key: String, // A unique identifier for re-verification if needed.
    
    /// The primary security context currently in use by this instance.
    #[allow(dead_code)]  // Only used internally during session initialization/testing.
    pub current_security_context: Option<SecurityContext>,

    /// List of pending audit events that have been processed but not yet committed to the file system or UI display.
    #[allow(dead_code)]
    pub active_audit_entries: Vec<AuditEntry>,
}

/// Represents a request for approval from an external agent (e.g., user input).
#[derive(Debug, Clone)]
pub struct ApprovalRequest {
    /// The ID of the action being processed or requested.
    #[allow(dead_code)] // Only used internally during session initialization/testing.
    pub action_id: usize,

    /// Optional metadata about this request if not provided via parameters.
    #[derive(Debug, Clone)]
    pub optional_metadata: Option<String>,
}

/// Represents a decision made by the Control Plane regarding a specific request.
#[derive(Debug, Clone)]
pub enum Decision {
    /// Approve action without any security checks (e.g., auto-approve payment).
    APPROVE(usize),      // Action ID or index in list of actions.

/// A secure context object used for internal state management within the Control Plane.
#[derive(Debug, Clone)]
pub struct SecurityContext {
    pub token: String,
