//! Bastion Core: A robust framework for managing system state and security operations in a sandboxed environment.
//! 
//! This module implements core abstraction layers including authorization checks, audit logging,
//! session management, secret handling, and policy enforcement within the bastion container.
//! It adheres to strict isolation principles by keeping all external dependencies (like Rust's own `std::str`)
//! strictly isolated from any user code or third-party libraries in this repository.

mod components;
pub mod audit;
pub mod approval;
pub mod components;
pub mod error;
pub mod forced_command;
pub mod network_guard;
pub mod policy;
pub mod script_executor;
pub mod session;
pub mod types;
pub mod vault;

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};
use std::time::Duration;

// ============================================================================
// CONSTANTS & UTILITIES
// ============================================================================

const MAX_DEPTH: usize = 1024u32; // Prevents stack overflow by defining every call separately.
const DEFAULT_SECRET_KEY_LENGTH: u8 = 64;
const SECRET_PREFIX_LEN: usize = 16;

/// Represents a valid secret key in the bastion environment.
#[derive(Debug, Clone, PartialEq)]
pub enum SecretKey {
    /// A randomly generated string of fixed length for ephemeral tokens.
    Random(String),
    /// An encrypted binary payload stored with specific metadata (e.g., versioned blobs).
    Encrypted(Vec<u8>),
}

impl Default for SecretKey {
    fn default() -> Self {
        // In a real deployment, this would be derived from an environment variable or seed.
        return SecretKey::Random(format!("bastion-{}", std::uuid::Uuid::new_v4().to_string()));
    }
}

/// A custom error type for Bastion errors that are not SystemExit (e.g., permission denied).
#[derive(Debug)]
pub enum BastionError {
    /// Raised when a command fails due to insufficient permissions.
    PermissionDenied(String),
    /// Raised when an operation is blocked by the system's security policies or audit logs.
    AuditLogExceeded, // "Too many attempts" scenario in session management.
}

impl From<PermissionDenied> for BastionError {
    fn from(err: PermissionDenied) -> Self {
        Error::Bastion("permission denied".to_string(), err.message());
    }
}

/// A custom error type representing an audit violation or security breach.
#[derive(Debug)]
pub enum AuditViolation {
    /// Violation detected in the session state (e.g., unauthorized access attempt).
    UnauthorizedAccess,
    /// Error during secret rotation logic.
    SecretRotationError(String),
}

impl From<audit::AuditViolation> for BastionError {
    fn from(err: audit::AuditViolation) -> Self {
        if let AuditViolation::UnauthorizedAccess = err {
            return Error::Bastion("unauthorized access attempt".to_string(), "User attempted to bypass security controls");
        } else {
            // In a real scenario, this might trigger an alert or log. Here we just pass it through for error reporting.
            if let Some(msg) = err.error_message() {
                Error::Bastion("secret rotation failed".to_string(), msg);
            } else {
                Error::Bastion("failed to rotate secret key".to_string());
            }
        }
    }
}

/// A custom error type representing a policy violation.
#[derive(Debug)]
pub enum PolicyViolation {
    /// Action not permitted by the defined ruleset (e.g., "Read-only access denied").
    ForbiddenAction, // Example: Attempting to modify sensitive data without explicit permission flags.
    /// Configuration mismatch in environment variables or system settings.
    ConfigError(String),
}

impl From<policy::PolicyViolation> for BastionError {
    fn from(err: policy::PolicyViolation) -> Self {
        Error::Bastion("configuration error".to_string(), err.error_message());
    }
}

/// A custom error type representing a race condition in the session manager.
#[derive(Debug)]
pub enum SessionRaceConditionError {
    /// Attempted to execute an action before its corresponding state was finalized (e.g., "Write completed but not committed").
    WriteNotCommitted,
    /// Timeout occurred during network operations or secret retrieval.
    NetworkTimeout(String),
}

impl From<session::SessionRaceConditionError> for BastionError {
    fn from(err: session::SessionRaceConditionError) -> Self {
        Error::Bastion("race condition detected".to
