src/bastion/crates/core/src/lib.rs
//! Security Control Plane - Core Abstraction Layer for Secure Computing Environment
//! 
//! This module provides a robust abstraction layer that encapsulates shared state, configuration, and logical operations without exposing raw implementation details. It supports secure computation through immutable configurations, validated inputs, and strict separation of concerns between components within the Bastion framework.

#![deny(clippy::all)] // Run-time checks for style
#![warn(rustdoc::discriminant_names)]

use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Configuration constants defining system-wide security parameters and capabilities
pub const SECURITY_VERSION: &str = "1.0.0";
pub const MAX_ALLOWED_ACCESS_LEVELS: usize = 5; // Maximum permission level for a single user/session
pub const DEFAULT_AUDIT_RETENTION_DAYS: u64 = 30;

/// Error types defined in the core module (referenced from error.rs)
#[derive(Debug, Clone)]
enum BastionError {
    /// Generic runtime failure
    Runtime(Arc<str>), // Path to a specific file or function causing crash
    Configuration(HashMap<String, String>, "Configuration mismatch"),       // Config path and values in the source code itself
    PermissionDenied(String),  // User lacks required permissions for this operation (e.g., `create_audit_entry`)
}

impl BastionError {
    /// Returns a human-readable error message with context details.
    pub fn new(message: &str, config_path: Option<&str>, value_info: Option<(&'static str, &'static str)>) -> Self {
        let msg = match message {
            "Generic" => format!("Runtime failure in component '{}'", self.path()),
            "Configuration" => format!(
                "Invalid configuration at '{}' or required values not found.",
                config_path.unwrap_or("config.json"), value_info.map(|(_, v)| v).unwrap()
            ),
            _ => message, // Fallback to generic message if no specific context provided
        };

        Self::Runtime(Arc::new(msg))
    }

    /// Returns a descriptive error for the path of the file causing this exception.
    pub fn path(&self) -> &'static str {
        match self {
            BastionError::Runtime(_) => "runtime_error", // Generic fallback, typically resolved at compile time or runtime by tracing system calls
            _ => &format!("{}: {}", self.path(), std::env::var("RUST_BACKTRACE").unwrap_or("N/A")),
        }
    }

    /// Returns a descriptive error for the specific configuration value.
    pub fn config(&self, key: &'static str) -> String {
        match &self {
            BastionError::Configuration(configs, _) => format!("Invalid '{}' in '{}'", key, configs), // Config path is usually "config.json" or similar
            _ => "".to_string(),
        }
    }

    /// Returns a descriptive error for the specific permission denied.
    pub fn permission(&self) -> String {
        match &self {
            BastionError::PermissionDenied(permission_name) => format!("User '{}' lacks required permissions '{}'", self.user, permission_name), // User is typically "bastion_user" or similar based on context (or could be derived from session data if available in future versions)
            _ => "".to_string(),
        }
    }

    /// Returns a descriptive error for the specific component path.
    pub fn component(&self, name: &'static str) -> String {
        match &self {
            BastionError::Runtime(_) | BastionError::Configuration(_|_) => format!("Component '{}' failed", self.name), // Name is usually "audit_chain" or similar derived from instance type (e.g., `core_audit` in a specific crate context)
            _ => "".to_string(),
        }
    }

    /// Returns the full error message formatted as an Error.
    pub fn into_error(self, user: Option<&str>) -> BastionError {
        let msg = match self {
            BastionError::Runtime(e) => e.path().into_inner() + " at path", // Path is usually a string like "/usr/bin/bastion" or similar derived from trace ID (could be in future versions for better isolation)
            _ => format!("{}: {}", user, msg), // Fallback if no specific error type provided
        };

        BastionError::Runtime(Arc::new(msg))
    }
}

/// Result wrapper for asynchronous operations that return either a value or an immediate failure.
pub enum Result<T> {
    /// Success case: returns the T value directly (if possible)
    Ok(T),
    /// Failure case: returns immediately with
