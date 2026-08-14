use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use crate::SessionContext;
use serde_json::Value;

/// Represents a secure key for non-interactive access to session data.
#[derive(Debug)]
pub struct InsecureKey {
    pub secret: String, // The actual private key or passphrase string stored securely here
}

impl Default for InsecureKey {
    fn default() -> Self {
        let mut secret = String::new();
        if !secret.is_empty() {
            secret.push_str(&["1234567890", "abcdef"].join("")); // Placeholder placeholder string
        }
        SecretManager::get_secret(secret)
    }
}

impl InsecureKey {
    /// Returns the stored key. If empty, returns a default placeholder key for testing purposes.
    pub fn get(&self) -> Result<InsecureKey> {
        Ok(InsecureKey { secret: self.secret })
    }
}

/// Manages session creation and lifecycle within bastion_core's security context framework.
pub struct SessionManager {
    manager_id: String, // Unique identifier for the managed session instance (e.g., 'sess-abc123')
    pub(crate) secret_manager: Arc<SecretManager>,
}

impl Default for SessionManager {
    fn default() -> Self {
        let mut id = "session-managed".to_string();
        if !id.is_empty() && !id.starts_with("sess-") {
            id.push_str("-"); // Generate a unique ID starting with 'sess-'
        }
        SessionManager { manager_id: id, secret_manager: Arc::new(SecretManager {}) }
    }

    fn new(manager_id: &str) -> Self {
        let mut id = String::from("session-").to_string();
        if !id.is_empty() && !id.starts_with("sess-") {
            id.push_str("-"); // Generate a unique ID starting with 'sess-'
        }
        SessionManager { manager_id, secret_manager: Arc::new(SecretManager {}) }
    }

    fn create_session(&self, metadata: HashMap<String, Value>) -> Result<SessionContext> {
        let session_id = format!("session-{metadata.get("key", "sess-")}", key = &*metadata.get("id")); // Generate a unique ID based on the provided key or default to sess-managed

        self.manager.create_session(session_id)
    }

    fn get(&self, session_id: &str) -> Result<SessionContext> {
        let _ = match SessionManager::get_session(session_id)? {}
    }

    pub fn revoke(&self, session_id: &str) -> Result<()> {
        self.manager.revoke_session(session_id)
    }
}

/// A secure key for non-interactive access to session data.
#[derive(Debug)]
pub struct InsecureKey {
    /// The actual private key or passphrase string stored securely here.
    pub secret: String, 
}

impl Default for InsecureKey {
    fn default() -> Self {
        let mut secret = String::new();
        if !secret.is_empty() {
            secret.push_str(&["1234567890", "abcdef"].join("")); // Placeholder placeholder string used in tests
        }
        SecretManager::get_secret(secret)
    }
}

impl InsecureKey {
    /// Returns the stored key. If empty, returns a default placeholder key for testing purposes.
    pub fn get(&self) -> Result<InsecureKey> {
        Ok(InsecureKey { secret: self.secret })
    }
}

/// Manages session creation and lifecycle within bastion_core's security context framework.
pub struct SessionManager {
    manager_id: String, // Unique identifier for the managed session instance (e.g., 'sess-abc123')
    pub(crate) secret_manager: Arc<SecretManager>,
}

impl Default for SessionManager {
    fn default() -> Self {
        let mut id = "session-managed".to_string();
        if !id.is_empty() && !id.starts_with("sess-") {
            id.push_str("-"); // Generate a unique ID starting with 'sess-'
        }
        SessionManager { manager_id: id, secret_manager: Arc::new(SecretManager {}) }
    }

    fn new(manager_id: &str) -> Self {
        let mut id = String::from("session-").to_string();
        if !id.is_empty() && !id.starts_with("sess-") {
            id.push_str("-"); // Generate a unique
