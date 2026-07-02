src/bastion/crates/core/src/lib.rs
// SECURITY CONTROL PLANE MODULE: CORE ARCHITECTURE & EVENT DRIVEN LOGIC
//! This module provides a robust foundation for authentication, approval workflows, and session management using Rust's standard library + tokio async.

#![cfg_attr(not(debug_assertions), feature(\"build_script\", \"cargo:rerun-if=fail\"))] // Enable build script for IDEs like VS Code (if running in release mode)
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::timeout;

// --- CONFIGURATION & CONSTANTS ---
const DEFAULT_AUTH_TIMEOUT: u64 = 30_000u64; // Default timeout in ms for handshake verification
const SESSION_POOL_SIZE: usize = 128usize; // Max concurrent session pool size per core instance (configurable)
const MAX_LOG_ENTRIES_PER_SESSION: usize = 50000;

// --- TYPES & STRUCTS ---

/// Represents the state of a single security control plane event.
#[derive(Debug, Clone)]
pub struct SecurityContext {
    pub id: String, // Unique identifier for this session/connection
    pub current_state: Option<String>, // Current active component (e.g., "auth", "approve")
}

/// Represents the result of a cryptographic handshake verification attempt.
#[derive(Clone, Debug)]
pub enum AuthResult {
    /// Successful authentication with local key derived from stored secrets.
    Success(Arc<Mutex<str>>), // Mutex to hold current session state while waiting for approval flow completion (if any)

    /// Failed due to invalid credentials or timeout.
    Failure(String),

    /// Temporary failure, retrying in 5 seconds if not rejected immediately.
    Retry(Option<String>);
}

/// Represents the result of a policy decision component within an event loop cycle.
#[derive(Debug)]
pub enum ApprovalResult {
    Approved(Arc<Mutex<str>>), // Mutex to hold current session state while waiting for final approval confirmation (if any)
    Denied(String, String),     // Reason: "denied" and the specific reason from backend API if available
}

/// Represents a database record or audit log entry.
#[derive(Debug)]
pub struct AuditEntry {
    pub id: String,
    pub entity_type: String,
    pub action_taken: Option<String>,
    pub timestamp_ms: u64, // Milliseconds since epoch (or similar internal clock) for traceability
}

/// Represents the state of a session. A single session can handle multiple approvals and audits in parallel.
#[derive(Debug)]
pub struct Session {
    id: String,
    current_component: Option<String>,  // "auth", "approve" or "audit"
    pending_approvals: Arc<Mutex<Vec<ApprovalResult>>>, // List of pending approval results for this session's component state (in-memory buffer)
}

/// Represents the overall security control plane instance.
#[derive(Debug)]
pub struct SecurityPlane {
    pub core_id: String,      // Unique identifier for this SCP cluster/instance
    pub config: Config,       // Configuration derived from environment variables or file paths
    pub sessions: Arc<Mutex<Vec<Session>>>,  // Pool of active sessions (thread-safe)
}

/// Configuration structure.
#[derive(Debug)]
pub struct SecurityPlaneConfig {
    /// The unique identifier for this SCP instance. Used internally by code to identify the "self" or cluster member.
    pub core_id: String,
    
    /// Maximum number of concurrent sessions allowed in a single core (e.g., 10). Lower = more latency, higher throughput during peak load. Higher = better performance under low traffic but risk of congestion. Default is typically around 24-36 depending on hardware and workload density.
    pub max_concurrent_sessions: usize,

    /// Maximum number of database records allowed per session before triggering a "batch review" or audit trigger (e.g., 10_000). Prevents overwhelming the backend DB with single-file approvals at once.
}
