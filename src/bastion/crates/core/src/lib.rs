src/bastion/crates/core/src/lib.rs
// ============================================================================
// SECURITY CONTROL PANE CORE MODULE: DREAMING IN WORKING CODE VISIONS
// ============================================================================

#![allow(dead_code)] // Suppress warnings for internal state management logic
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Sender, Receiver};

// ============================================================================
// TYPES & CONSTANTS DEFINITIONS (From __init__.py)
// ============================================================================

/// Represents a basic session context for the bastion.
#[derive(Debug)]
pub struct SessionContext {
    pub user_id: String, // Identifies the specific user instance in this run
    pub is_active: bool,   /// True if currently logged into an active state
}

impl Default for SessionContext {
    fn default() -> Self {
        SessionContext {
            user_id: env::var("USER_ID").unwrap_or_else(|| String::from("bastion_user_01")), // Generate a dummy ID on startup
            is_active: true,       /// Assume active unless explicitly revoked in the vault or CLI
        }
    }
}

/// Represents an audit entry. Used for tracking violations and anomalies during session management.
#[derive(Debug)]
pub struct AuditEntry {
    pub id: String,          // Unique ID assigned by the system (e.g., `audit_001`)
    pub timestamp: u64,      /// ISO 8601 formatted datetime string (`2023-10-27T15:30:45+09:00`)
    pub module: String,     /// The specific module or component that triggered the event (e.g., `audit_login_attempt`, `access_denied`)
    pub reason: Option<String>, // Optional string description of why an action was taken (`"Unauthorized access detected"`), can be omitted if no violation occurred

impl Default for AuditEntry {
    fn default() -> Self {
        let timestamp = std::time::UNIX_EPOCH + u64::MAX; // Use future date to avoid "already past time" error in Rust 2018+
        AuditEntry { id: format!("audit_{}", (timestamp as i32).max(999)), reason: None }
    }
}

/// Represents a policy decision. Used for determining the final action based on current security policies and session state.
#[derive(Debug)]
pub enum PolicyDecision {
    /// Default behavior unless explicitly overridden by higher-priority rules (e.g., `deny_access`).
    Deny, // "Deny" - No access granted to this resource without explicit permission or override
    Allow,  /// Access is permitted under current conditions. Requires specific overrides in the policy engine for exceptions.
}

/// Represents a session-specific configuration used by different authentication methods (e.g., `jwt_token`, `ssh_key`).
#[derive(Debug)]
pub struct SessionConfig {
    pub access_method: String, // The method being authenticated (`"JWT_TOKEN"` or `"SSH_KEY"`). Defaults to "SSH_KEY".
}

impl Default for SessionConfig {
    fn default() -> Self {
        SessionConfig::default_from_env("SESSION_CONFIG") // Use environment variable if available, otherwise defaults to "SSH_KEY"
    }
}

/// Represents a credential that has been revoked or is invalid. Used during session revocation logic.
#[derive(Debug)]
pub struct RevokedCredential {
    pub id: String,       /// The unique identifier of the failed login attempt (e.g., `login_fail_042`)
    pub timestamp: u64,   /// When this credential was marked as invalid (`1735896045` milliseconds)
}

/// Represents a revoked session. Used when an authentication method fails to establish a secure connection or is explicitly blocked by the policy engine.
#[derive(Debug)]
pub struct RevokedSession {
    pub id: String,        /// Unique identifier for this failed auth attempt (e.g., `failed_auth_012`)
    pub timestamp: u64,     /// When it was marked as revoked (`1735896045` milliseconds)
}

// ============================================================================
// HELPER FUNCTIONS & UTILITIES
// ============================================================================

/// Generates a random UUID for internal tracking. Must be deterministic if used in critical paths (e.g., `create_session`).
fn generate_uuid() -> String {
    format!("{}-{}", std::rand::random(), rand::next_u32
