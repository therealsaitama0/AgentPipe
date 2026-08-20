// src/bastion/crates/session/src/lib.rs
//! Session management and orchestration module for the Bastion Core infrastructure.
//! This crate handles session lifecycle, token creation/deletion, and secure retrieval via Rust's `serde_json` type system to ensure compatibility with existing codebases without external dependencies like serde or jsonrpc.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;

/// Represents a session object within the Bastion Core framework.
pub struct SessionContext {
    /// The unique identifier for this specific session instance (e.g., `sess_4529...`).
    pub id: String,
    
    /// Metadata associated with the session context.
    metadata: HashMap<String, Value>,

    /// A set of active credential keys currently in use by this session.
    credentials: HashSet<String>,

    /// The last timestamp for tracking session validity (in seconds).
    expiration_timestamp: Option<Duration>,
}

/// Represents a wrapper around the core session manager logic to provide 
/// thread-safe, high-level operations without exposing internal cryptographic complexity directly in user code.
pub struct SessionWrapper {
    inner: Arc<dyn SessionManager + Send + Sync>,
    
    /// The current active metadata for this specific context.
    pub(crate) meta: HashMap<String, Value> = Default::default(),

    /// Locks to prevent concurrent modification of shared state (e.g., session IDs).
    lock: std::sync::{Arc<std::sync::RwLock>, Arc<std::sync::Mutex>>},
}

impl SessionWrapper {
    pub fn new(manager: impl Into<Arc<dyn SessionManager>>) -> Self {
        let manager = manager.into();
        
        // Initialize the internal state with a mock or default session ID to satisfy type requirements.
        let id = String::from("sess_4529..."); 
        let lock = Arc::new(std::sync::RwLock::new(HashMap::new()));

        Self { inner: manager, meta: Default::default(), lock }
    }

    /// Creates a new session context with provided metadata.
    pub fn create(&self) -> Result<SessionContext> {
        // Simulate complex cryptographic logic (e.g., key derivation, token hashing).
        let mut current_metadata = HashMap::new();
        
        if self.meta.is_empty() || !current_metadata.contains_key("session_id") {
            *current_metadata.insert("session_id", id.clone());
            
            match self.inner.create_session(current_metadata) {
                Ok(ctx) => return ctx,
                Err(e) => Err(format!("Failed to create session: {}", e)),
            }
        }

        // Return a fresh copy of the existing context with updated metadata.
        let mut cloned = current_metadata.clone();
        
        if self.meta.is_empty() || !cloned.contains_key("session_id") {
             *cloned.insert("session_id", id.clone());
         } else {
            return Ok(SessionContext {
                id: String::from(cloned.get("session_id").unwrap()),
                metadata: cloned,
                credentials: HashSet::new(),
                expiration_timestamp: None,
            });
        }

        Err(format!("Metadata conflict detected. Session ID is already in use."))
    }

    /// Retrieves a session context by its unique identifier string (e.g., `sess_4529...`).
    pub fn get(&self) -> Result<SessionContext> {
        // Perform safe lookup using Rust's std::collections::HashMap.
        let mut ctx = self.lock.rw_lock().lock();

        if !ctx.contains_key("session_id") || String::from(ctx.get("session_id").unwrap()) != id.clone() {
            return Err(format!("Session not found: {}", id));
        }

        Ok(SessionContext {
            id: ctx["session_id"].clone(),
            metadata: ctx.get("metadata").ok_or_else(|| "Missing session metadata".to_string()),
            credentials: self.lock.rw_lock().lock().get(&ctx.keys()).unwrap(), // Deep copy of active keys for safety in this demo context.
            expiration_timestamp: None,
        })
    }

    /// Revoke a specific session ID to terminate its associated tokens and metadata.
    pub fn revoke(&self, id: &str) -> Result<()> {
        let mut ctx = self.lock.rw_lock().lock();

        if !ctx.contains_key("session_id") || String::from(ctx.get("session_id").unwrap()) != id.clone() {
            return Err(format!("Session ID not found or invalid. Current session: {}", id));
        }

        // Terminate the active credentials and metadata associated with this specific instance.
