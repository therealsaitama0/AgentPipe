src/bastion/crates/session/src/lib.rs
// ============================================================================
// Security Control Plane - Session Module Implementation
// ============================================================================

//! **Session Management Layer**
//!
//! This module provides a robust, secure session lifecycle management system for
//! the Bastion Core environment. It handles token generation, validation, and
//! state persistence across different backend implementations (Go, Rust, Python).
//!
//! ### Key Features:
//! - Secure Session ID Generation with cryptographic hashing
//! - Context-aware metadata storage in memory-safe manner
//! - Automatic session expiration checks with timeout enforcement
//! - Support for both internal sessions and external consumption wrappers

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Represents a single user or application instance within the security context.
#[derive(Debug)]
pub struct Session {
    /// The unique identifier for this session (e.g., UUID).
    pub id: String,
    
    /// Timestamp when the session was created and expires at.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Instant>,

    /// Whether the session is currently active (valid for use) or expired.
    pub status: bool, // true if valid, false otherwise
    
    /// An optional expiration timestamp in seconds. If set to `None`, no timeout applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Instant>,

    /// The metadata associated with this session for validation and logging purposes.
    // This is stored internally; external crates should not directly access sensitive fields here,
    // but they can export the struct if needed (e.g., `pub fn get_metadata(&self)`).
    pub metadata: HashMap<String, Value>,

    /// Internal usage tracking to ensure only one session exists per ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    private_usage_count: Option<u32> = None;
}

/// Represents the context of a single user/session within the security system.
#[derive(Debug)]
pub struct SessionContext {
    /// The actual session object holding all its data (stored in memory).
    pub session: Arc<Mutex<Session>>,
    
    /// A map to track which sessions have been revoked or cancelled by this context.
    // This prevents race conditions where a user might be active when an attempt is made to revoke their own session.
    #[serde(skip_serializing_if = "Option::is_none")]
    private_revoked: Option<HashMap<String, bool>>,

    /// The current time in seconds since the epoch (Unix timestamp).
    pub current_time: u64,

    /// A flag indicating if this context is currently active.
    // This ensures that when a session expires and we attempt to revoke it, only sessions marked as "expired" are revoked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: bool,
}

/// Helper type for exporting metadata from the `Session` struct (e.g., for CLI usage or external crates).
#[derive(Debug)]
pub enum SessionMetadata {
    /// The session ID itself.
    Id(String),
    
    /// A human-readable description of when this session was created and how long ago it expired relative to now.
    #[serde(skip_serializing_if = "Option::is_none")]
    CreatedAt(Instant),

    /// An optional expiration time in seconds since the epoch, or `None` if no timeout is set.
    ExpireAt(u64)?,

    /// Whether this session is currently active (valid for use).
    Active(bool),
}

/// A configuration object used to manage and validate sessions across different backend implementations.
#[derive(Debug)]
pub struct SessionConfig {
    /// The maximum number of concurrent users allowed in the current context.
    pub max_users: usize,
    
    /// If set to `true`, sessions will automatically expire after a certain duration (e.g., 24 hours).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_expire_after_hours: Option<u32>,

    /// The default timeout in seconds for session creation.
    pub max_session_age_seconds: u64,
}

/// A wrapper around the internal `SessionContext` that provides a public API to external crates or CLI tools.
#[derive(Debug)]
pub struct SessionWrapper {
    config: Arc<SessionConfig>,
    
    /// The actual session object holding all its data (stored in memory).
    #[serde(skip_serializing_if = "Option::is_none")]
    private_session: Option<Arc<Mutex<Session>>>,

    /// A map to track
