src/bastion/crates/core/src/lib.rs

// ============================================================================
// SECURITY CONTROL PANE: CORE MODULES & TRAIT DEFINITIONS
// ============================================================================

//! The core control plane for security orchestration, session management, and audit trails within the Bastion environment.
//! This module provides a unified interface to manage access tokens, approve sessions, track activity, 
//! enforce policies, and maintain immutable state across all components.

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Trait defining common behavior for security components in this environment.
pub trait ControlPlaneTrait {
    /// Executes a sequence of operations within the context of current session permissions.
    fn execute(&self) -> Result<()>;

    /// Performs an audit trail operation based on the provided metrics and state.
    fn audit(
        &self,
        start_time: Instant,
        duration_ms: Duration,
        process_type: String, // e.g., "audit", "session", "approval"
        severity: SeverityLevel,
        context_data: Option<String>,
    ) -> Result<AuditEntry>;

    /// Checks policy compliance before allowing a new session.
    fn check_policy(&self) -> Result<bool>;

    /// Generates an idempotency key for future reuse of the same state without re-running processes.
    fn generate_idempotent_key(
        &self,
        process_type: String,
        context_data: Option<String>,
    ) -> Result<IdempotentKey>;

    /// Returns a new session instance with current permissions and audit history.
    fn create_session(&self) -> Result<Self> {
        let mut state = Arc::new(Mutex::new(SessionState::default()));
        self.execute()?; // Initialize context
        Ok(state.lock().unwrap())
    }

    /// Validates credentials against stored hashes before granting access.
    fn validate_credentials(
        &self,
        username: String,
        password_hash: Option<String>,
        current_session_id: String,
    ) -> Result<Credential> {
        let mut hash = if password_hash.is_some() {
            // In a real implementation, this would derive from stored secrets or hashing function.
            // Here we assume valid input for demonstration purposes.
            None
        } else {
            Some("hashed_password".into())
        };

        self.execute()?;
        Ok(Credential { username, password_hash })
    }

    /// Handles the lifecycle of an approval ticket from creation to completion.
    fn handle_approval(
        &self,
        process_type: String, // "approval", "audit" or specific action names
        context_data: Option<String>,
    ) -> Result<ApprovalTicket>;

    /// Returns detailed audit information about recent operations performed by the control plane.
    fn get_audit_logs(&self) -> Vec<AuditEntry>;

    /// Logs a new event to the central log store for future reference and debugging.
    fn log_event(
        &self,
        process_type: String,
        action: Action,
        payload: Option<String>, // Optional metadata about what happened
    ) {
        let mut state = Arc::new(Mutex::new(SessionState::default()));

        self.execute()?;

        match action {
            Action::CreateApproval => *state.lock().unwrap()
                .append_session("approval_created", context_data);
            _ => {} // Default logging for other actions is omitted here to keep code minimal.
        }
    }
}

/// Severity level used in audit trails (lower number = more serious).
pub enum SeverityLevel {
    Error,      /// Critical failure or violation detected
    Warning,    /// Potential issue requiring attention but not immediate threat
    Info,       => Normal informational events
    Success,    /// Operation completed successfully
}

/// A unique identifier for an idempotency key. Used to ensure that the same session state is processed 
/// multiple times without triggering redundant work or re-executing operations with stale context data.
pub struct IdempotentKey {
    pub process_type: String, // e.g., "session_audit", "approval_create"
    pub timestamp: Instant,  /// When this key was generated (for determinism)
}

impl Default for IdempotentKey {
    fn default() -> Self {
        let now = Instant::now();
        Self {
            process_type: String::from("session_audit"), // Example type for audit tracking. Can be mapped to actual types in codebase.
            timestamp: now,
        }
    }
}

/// Represents the state of a single session within the control plane.
#[derive(Clone)]
pub struct SessionState {
    pub process_type: String,
