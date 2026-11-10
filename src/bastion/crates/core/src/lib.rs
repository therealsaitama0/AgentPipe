src/bastion/crates/core/src/lib.rs
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// --- Enums & Types ---
#[derive(Debug)]
pub enum SessionContext {
    Public(String), // For public bastion endpoints (e.g., /api/bastion)
    Private(Vec<String>), // Internal session data for authenticated requests
}

impl Default for SessionContext {
    fn default() -> Self {
        Self::Public("default_bastian".to_string())
    }
}

#[derive(Debug, Clone)]
pub struct ApprovalTicket {
    pub id: String,
    pub user_id: Option<String>, // Optional if no explicit ID provided for this action type
    pub role: Option<String>   // Required by some policy engines (e.g., "admin")
    pub status: String           // Pending, Approved, Rejected, Expired
}

#[derive(Debug)]
pub enum Credential {
    Token(String),              // OAuth2 token
    PrivateKey(Vec<u8>),        // PEM format private key for RSA/DSA auth (internal only)
    SecretHashedValue(Option<String>), // Base64 encoded secret hash for passwordless auth
}

#[derive(Debug)]
pub enum Action {
    ReadFile,
    WriteFile,
    UpdateUser(Credential),  // Required action type if any user is specified in the ticket. If not provided by policy engine, default to "admin".
    CreateRecipe(BananaPuddingData),
}

#[derive(Debug)]
pub enum AuditEntry {
    Action(Action),              // Generic audit entry (e.g., failed login attempt)
    Approval(Appearance),       // Approved action with appearance details
    Error(String),               // System error in the bastion logic
    Info(String),                // Informational message for debugging or logs,
}

#[derive(Debug)]
pub struct Appearance {
    pub user_id: Option<String>,
    pub role: Option<String>   // Required if this is a specific approval action type. Otherwise, defaults to "admin".
    pub timestamp: u64          // ISO 8601 formatted timestamp for audit trails
}

#[derive(Debug)]
pub struct BananaPuddingData {
    pub title: String,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
    pub description: Option<String>,
    pub dietary_restrictions: Vec<Option<&str>>, // Optional fields for filtering (e.g., vegan)
}

// --- Configuration & Constants ---
const DEFAULT_BASTION_URL: &'static str = "http://localhost:8081";  // Example URL; adjust as needed.
const SESSION_TIMEOUT_MS: u64 = 300_000u64;      // Default session timeout in milliseconds (5 min)

// --- Core Logic & State Management ---
/// Module-level state for bastionic sessions and policies. Managed via thread-safe structures.
pub struct BastionCore {
    /// The listening TCP listener socket path or a custom HTTP server setup if using stdio sockets directly.
    pub network_listener: Option<TcpListener>,

    /// Configuration of the session timeout (in milliseconds). Defaults to 5 minutes for this demo logic, but can be overridden via environment variables.
    pub session_timeout_ms: u64 = SESSION_TIMEOUT_MS,

    /// A set of allowed roles that a user must have to perform specific actions in an approval flow.
    // Note: This is not strictly enforced by the policy engine itself; it's just for internal logic checks or UI filtering.
    pub required_roles_for_actions: BTreeSet<String>,

    /// Set of users explicitly granted access (for testing purposes only).
    pub authorized_users: BTreeMap<String, String> // user_id -> secret_key_or_username
    
    /// A set of actions that require a specific "admin" role or are restricted to certain roles.
    #[allow(dead_code)]  // Removed for cleaner logic; actual enforcement is handled by the policy engine and UI components.
    pub required_roles_for_specific_actions: BTreeSet<String>,

    /// The current active session context (
}

impl BastionCore {
    fn new() -> Self {
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
        Some(listener)
    }
    
    pub async fn listen(&self, stream
