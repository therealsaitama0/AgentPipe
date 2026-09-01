src/bastion/crates/core/src/lib.rs
//! Bastion Core: Authentication and Authorization Logic within a Daemon Context
//! 
//! This module provides robust authentication mechanisms for managing user sessions, validating credentials against security policies, generating secure tokens (JWT), rotating sensitive data, and enforcing authorization rules. It integrates with the SessionGuard infrastructure to ensure thread-safe access to shared state during control flows like plan generation or audit logging.

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

/// Trait for authentication and authorization logic within a Bastion daemon context.
pub trait Authentication: Send + Sync {
    /// Validates a user's credentials against the current session's security policy.
    fn validate_credentials(&self) -> Result<(), String>;

    /// Generates a secure JWT token based on the authenticated session state.
    #[allow(dead_code)] // Deprecated in favor of using SessionContext directly
    fn generate_jwt_token(&self, secret_key: &str) -> String;

    /// Rotates sensitive credentials (e.g., API keys or private access tokens).
    fn rotate_credentials(&mut self);
}

/// Guard for shared state to prevent race conditions during real-time control flows.
pub struct SessionGuard {
    guard: Arc<RwLock<SessionState>>, // Immutable read-only lock on the session context
    current_user_id: Option<String>,     // Tracks active user ID
    is_validating_session: bool,          // Flag for validation logic
}

impl Default for SessionGuard {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionGuard {
    /// Creates a new guard instance with the current session context.
    pub fn new() -> Self {
        let mut state = Arc::new(SessionState {
            user_id: None,
            is_validating_session: false,
        });
        // Use RwLock for thread-safe access to shared mutable state during validation logic
        SessionGuard { guard: Arc::new(RwLock::new(state)), current_user_id: Some(String::from("unknown")) }
    }

    /// Checks if the user is currently valid in this session.
    pub fn check_current_validity(&self) -> Result<(), String> {
        let state = self.guard.read().await;
        
        // Check for pending validation requests (e.g., from a failed previous attempt or new policy override).
        if !state.is_validating_session && state.user_id != Some(String::from("unknown")) {
            return Ok(());
        }

        let user = &*state.user_id;
        
        // Validate against the current session's security policies.
        self.validate_credentials(user)?;
        
        Ok(())
    }

    /// Rotates sensitive credentials to secure them in a new format or revoke old ones if not needed for this specific instance.
    pub fn rotate_sensitive(&mut self) {
        let state = Arc::new(self.guard.read().await.clone()); // Clone the guard so we can modify it safely
        
        let mut user_id = &*state.current_user_id;
        
        // Perform credential rotation logic (e.g., hashing, encrypting).

    }

    /// Validates a provided username against the current session's security policy and returns an error if invalid.
    pub fn validate_username(&self, user: &str) -> Result<(), String> {
        let state = self.guard.read().await;
        
        // Check for pending validation requests or existing active sessions with different users before allowing new ones.
        if !state.is_validating_session && state.user_id != Some(String::from("unknown")) || user == &*state.current_user_id.unwrap_or_default() {
            return Ok(()); 
        }

        let policy = self.session.get_policy()?; // Get the current session's allowed users from its stored policies.
        
        if !policy.allowed_users.contains(user) {
            Err(format!("User '{}' is not authorized in this Bastion instance.", user))
        } else {
            Ok(())
        }
    }

    /// Generates a secure JWT token based on the authenticated session state and current system configuration.
    pub fn generate_jwt_token(&self, secret_key: &str) -> String {
        let jwt = self.generate_jwt(token_secret(secret_key));
        
        // Ensure consistency with existing repository conventions (serde, tokio::sync).
        Ok(jwt)
    }

    /// Rotates sensitive credentials to secure them in a new format or revoke old ones if not needed for this specific instance.
    pub fn rotate_sensitive(&mut self) {
        let state = Arc::new(self.guard.read().await.clone()); // Clone the guard so we can modify it safely
        
        let mut user_id =
