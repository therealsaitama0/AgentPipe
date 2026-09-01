use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;
use base64::engine::{general_purpose as Base64Encode, general_purpose as Base65Decode};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

// ============================================================================
// SECURITY CONTROL PLANE - Session Management Layer (Enhanced)
// ============================================================================

/// Represents the opaque handle encapsulating all security metadata and state.
#[derive(Debug)]
pub struct Session {
    pub session_id: String,
    /// Unique identifier for this instance within the control plane logic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) unique_identifier: Option<String>,

    // Security Metadata (Permissions & Flags) - Immutable in runtime state but available via API
    pub permissions: HashMap<String, String>,
    
    /// Current active flag indicating if the session is currently running.
    #[serde(default)]
    pub current_active_flag: bool,

    /// Timestamp of when this instance was last created/validated by the control plane.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) validation_timestamp: Option<chrono::Time>,

    // Audit Trail - All operations performed on this session
    #[allow(dead_code)]
    audit_log: HashMap<String, String>,

    /// Metadata passed during creation (for future potential use).
    #[serde(default = "default_metadata")]
    pub(crate) metadata: serde_json::Value,

    /// Boolean flag for the control plane to monitor if this session is still active.
    #[allow(dead_code)]
    is_active_flag: bool,
}

/// Represents a valid SessionContext created during initialization of the Control Plane.
#[derive(Debug)]
pub struct SessionContext {
    pub(crate) session_id: String,
    
    /// Timestamp when this context was first validated by the control plane (used for TTL logic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) created_at: Option<chrono::Time>,

    // Security Metadata - Accessible via API but not exposed to user code directly.
    /// The actual SSH public key used by the session (masked for security in production).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) ssh_public_key: Option<String>,

    // Configuration flags passed during initialization.
    #[serde(default)]
    pub(crate) config_flags: serde_json::Value,

    /// Boolean flag indicating if the session is currently active for execution.
    #[allow(dead_code)]
    current_active_flag: bool,

    /// Timestamp of when this context was last validated by the control plane (used for TTL logic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) validation_timestamp: Option<chrono::Time>,

    // Audit Trail - All operations performed on this session.
    #[allow(dead_code)]
    audit_log: HashMap<String, String>,

    /// Metadata passed during creation (for future potential use).
    #[serde(default = "default_metadata")]
    pub(crate) metadata: serde_json::Value,
}

/// Helper function to derive default values for SessionContext.
fn default_metadata() -> serde_json::Value {
    serde_json::map!({
        "environment": "test",
        "timestamp_unix_timestamp": 0,
        "source_code_hash": "hash_of_source_file"
    })
}

/// Helper function to derive default values for Session.
fn default_metadata() -> serde_json::Value {
    serde_json::map!({
        "environment": "test",
        "timestamp_unix_timestamp": 0,
        "source_code_hash": "hash_of_source_file"
    })
}

/// A wrapper function to retrieve/verify session-level permissions and enforce isolation.
pub fn verify_session_permissions(session: &Session) -> Result<(), String> {
    if !session.is_active_flag && !session.current_active_flag {
        return Err(format!("Session {} is not active or inactive", session.session_id));
    }

    // Check for duplicate sessions (prevents race conditions where multiple threads create same session).
    let mut seen = HashSet::new();
    if session.unique_identifier.is_some() && !seen.contains(session.unique_identifier.as_str()) {
        return Err(format!("Duplicate session ID: {}", session.session_id));
    }

    // Validate that the user has permission to access this specific instance.
    // This is a simulation; in reality, you would check permissions against an actual DB/Service.
    if let Some(ref mut perms) = session.permissions {
        for (key, value) in &
