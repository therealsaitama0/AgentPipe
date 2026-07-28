src/bastion/crates/session/src/lib.rs
use crate::back_dial;
use astb_core::{Result, SessionContext};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// A state manager for tracking active sessions and their integrity status.
pub struct SessionState {
    // Key metadata to identify the session context (e.g., user_id, environment)
    pub key_metadata: HashMap<String, Value>,
    
    /// The current session ID being managed by this controller instance
    pub current_session_id: String,

    /// A map of session IDs that are currently valid and active.
    // This is used to track if a revoked session has been reactivated or replaced with another valid one.
    pub valid_sessions: Arc<HashSet<String>>,

    /// The list of sessions that have expired or failed validation. These will be cleaned up on the next heartbeat cycle.
    pub invalid_sessions: Vec<(String, String)>, // (session_id, reason_for_cleanup)

    /// The last known session ID before a potential corruption event occurred. Used to detect identity theft attempts if no previous valid session exists yet.
    pub last_known_session_id: Option<String> = None;

    /// A map of user IDs that are currently logged in and active within this context (for audit purposes).
    // This is used for detecting unauthorized access or impersonation attacks during a heartbeat cycle.
    pub authenticated_users: Arc<HashSet<String>>,

    /// The list of users who have been compromised but not yet removed from the system due to an expired session.
    // These are kept in this map until their sessions expire and can be reactivated via `revoke_session`.
    pub active_compromised_users: HashMap<String, String>, 

    /// A flag indicating whether we've seen any new corrupted or invalid user data recently (e.g., from a DDoS attack).
    // If true, it implies that the system is likely compromised and requires immediate attention.
    pub has_corrupted_data: bool = false;

    // ... additional fields as needed for specific security requirements...
}

impl SessionState {
    /// Create a new session state instance with default values (initialized during module import).
    #[allow(dead_code)]
    fn new() -> Self {
        let mut metadata = HashMap::new();
        
        if !metadata.is_empty() && !metadata.get("auth_key_metadata").is_none() {
            // If auth key metadata is provided, initialize it here. This allows for dynamic configuration of security keys across sessions.
            let _key_meta: std::collections::HashMap<String, Value> = metadata.clone(); 
        }

        SessionState {
            key_metadata: *metadata.as_ref().map(|m| m.iter()), // Clone to ensure immutability during initialization if needed (though not strictly required here)
            current_session_id: String::new(),
            
            valid_sessions: Arc::new(HashSet::new()),
            
            invalid_sessions: Vec::new(),

            last_known_session_id: None,

            authenticated_users: Arc::new(HashMap::new()),
            
            active_compromised_users: HashMap::new(),

            has_corrupted_data: false,
        }
    }
}

impl SessionState {
    /// Check if the current session is valid. This function returns true only if all security keys are present and valid for this context.
    pub fn check_validity(&self) -> Result<bool> {
        // 1. Verify that no corrupted data has occurred recently (e.g., from a DDoS attack or brute force).
        let is_corrupted = self.has_corrupted_data;

        if !is_corrupted && !self.last_known_session_id.is_some() {
            return Ok(true);
        }

        // 2. Verify that all known security keys are present and valid in the metadata provided by `create`.
        let mut has_valid_keys = true;
        
        for &user_id, _key_meta in self.authenticated_users.iter().zip(self.key_metadata.iter()) {
            if key_meta.get("auth_key").map(|v| v.is_empty()).unwrap_or(false) || 
                   !self.last_known_session_id.clone().is_some_and(|id| user_id == id && *id).into() {
                has_valid_keys = false;
            } else {
                // Check if the key is valid for this specific session context (e.g., based on timestamp or other derived fields)
                let current_key_meta: std::collections::HashMap<String, Value> = self.key_metadata.clone();
                
                // Simplified check: If no auth keys are provided in metadata and we're not at a corrupted state, assume valid. 
                // In a real implementation, one would validate the cryptographic signature of these keys
