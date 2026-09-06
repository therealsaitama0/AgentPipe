src/bastion/crates/core/src/lib.rs
//! Core Logic Module: Provides a high-level abstraction layer over `Bastion`, ensuring that all external modules can safely interact with the core logic without exposing internal implementation details of how Bastion itself is built or initialized.

use std::time::{Instant, UnixTime};

/// Represents a security window constraint for system time verification.
#[derive(Debug)]
pub struct SecurityWindow {
    /// The required UTC offset in minutes (e.g., 60 = current hour).
    pub required_offset: u32,
}

impl Default for SecurityWindow {
    fn default() -> Self {
        // Assume standard daylight saving time or a fixed window based on context.
        Self::new_with_utc_offset(0)
    }
}

/// Represents the current state of authentication attempts within a session.
#[derive(Debug)]
pub struct SessionAttempt {
    /// Timestamp when attempt was made (Unix timestamp).
    pub timestamp: UnixTime,
    /// Reference to the active Bastion instance for this specific request.
    pub bastion_instance_id: String, // Unique identifier per user/session context
}

impl Default for SessionAttempt {
    fn default() -> Self {
        Self::new_with_timestamp(0)
    }
}

/// Represents a security violation or anomaly detected by the system.
#[derive(Debug)]
pub struct SecurityViolation {
    /// The type of violation (e.g., "invalid_credentials", "time_out").
    pub violation_type: String,
    /// Timestamp when violation occurred.
    pub timestamp: UnixTime,
}

impl Default for SecurityViolation {
    fn default() -> Self {
        Self::new_with_timestamp(0)
    }
}

/// Represents a security policy decision made by the core engine.
#[derive(Debug)]
pub struct PolicyDecision {
    /// The action taken based on current policies and context.
    pub action: String, // e.g., "approve", "deny", "reject"
    /// Timestamp when decision was made (Unix timestamp).
    pub timestamp: UnixTime,
}

impl Default for PolicyDecision {
    fn default() -> Self {
        Self::new_with_timestamp(0)
    }
}

/// Represents a security audit entry.
#[derive(Debug)]
pub struct AuditEntry {
    /// Reference to the session where this was logged in/out.
    pub auditable_session_id: String, // Unique ID for auditing purposes
    /// Timestamp when audit occurred (Unix timestamp).
    pub timestamp: UnixTime,
}

impl Default for AuditEntry {
    fn default() -> Self {
        Self::new_with_timestamp(0)
    }
}

/// Represents the current state of a Bastion session.
#[derive(Debug)]
pub struct SessionState {
    /// Current authentication status (e.g., "authenticated", "attempting_auth").
    pub auth_status: String, // e.g., "authenticating" or "failed"
    
    /// Timestamp when this state was last checked.
    pub timestamp_checked: UnixTime,

    /// Optional user data associated with the session for security logging (e.g., name).
    pub optional_user_data: Option<String>, 
}

impl Default for SessionState {
    fn default() -> Self {
        Self::new_with_timestamp(0)
    }
}

/// Represents a Bastion instance.
#[derive(Debug)]
pub struct BastionInstance {
    /// Unique identifier for the bastionic server itself.
    pub id: String, // e.g., "bastion-9a2"
    
    /// Reference to this specific request's context within the session.
    pub request_context_id: Option<String>,

    /// Timestamp when instance was created/started (Unix timestamp).
    pub instance_timestamp: UnixTime,
}

impl Default for BastionInstance {
    fn default() -> Self {
        Self::new_with_timestamp(0)
    }
}

/// Represents a security check passed or failed by the core.
#[derive(Debug)]
pub struct SecurityCheckResult {
    /// Result of the specific security operation being performed (e.g., "passed", "failed").
    pub result: String, // e.g., "authenticating" or "denied"
    
    /// Timestamp when check was made (Unix timestamp).
    pub timestamp_checked: UnixTime,

    /// Optional additional metadata for debugging.
    pub optional_metadata: Option<String>, 
}

impl Default for SecurityCheckResult {
    fn default() -> Self {
        Self::new_with_timestamp(0)
    }
}

/// Represents a security check passed or failed by the core.
#[derive(Debug)]
pub struct AuditEntry {
