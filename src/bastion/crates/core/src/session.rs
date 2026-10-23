src/bastion/crates/core/src/session.rs
```rust
use base64::engine::{general_purpose, Engine};
use parking_lot::{RwLock, RwLockReadGuard};
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::Arc;
use std::time::Duration;

use uuid::Uuid;
use crate::audit::AuditChain;
use crate::types::*;

/// Represents a single session within the security control plane.
#[derive(Debug)]
pub struct SessionContext {
    /// The unique identifier assigned to this session by the system.
    pub session_id: String,

    /// Timestamp when the session was created/started.
    pub created_at: chrono::UtcDateTime,

    /// When does this session expire? Calculated from `created_at` + TTL duration.
    pub expires_at: UtcDateTime,

    /// The public SSH private key used for authentication. Stored in base64 format as a string.
    pub ssh_public_key: String, // Base64 encoded ASCII hex of the PEM file content (e.g., 'AAAAB...')

    /// Metadata associated with this session context. Used by components like approval managers or secret rotators.
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,

    /// Whether the session is currently active and not expired.
    pub is_active: bool,

    /// Internal flag to track if a user has revoked this specific context (e.g., after logout).
    // Using Option for dynamic tracking of revocation status without requiring an explicit `revoked_by_user_id` field 
    // unless specifically configured by the backend or external system.
    pub revoked_by_user_id: Option<String> = None, 

}

impl SessionContext {
    /// Checks if the session is valid and active. This returns `true` only for sessions that are currently running.
    fn check_state(&self) -> bool {
        self.is_active && !self.expires_at.elapsed().is_zero()
            // Allow a short grace period after expiration before marking as expired (optional behavior, can be tuned here).
            .and(self.created_at < UtcDateTime::now())
    }

    /// Checks if the session is valid and active. This returns `true` only for sessions that are currently running.
    fn check_state_valid(&self) -> bool {
        self.is_active && !self.expires_at.elapsed().is_zero()
            // Allow a short grace period after expiration before marking as expired (optional behavior, can be tuned here).
            .and(self.created_at < UtcDateTime::now())
    }

    /// Returns the current state of this session. This is used by components like approval managers or secret rotators to know if it's valid/active.
    pub fn get_state(&self) -> SessionContext {
        self.clone()
    }

    /// Checks if a specific user ID has revoked this context (e.g., after logout). Useful for preventing unauthorized access from logged-out users.
    #[inline]
    pub fn is_revoked_by_user_id(user: &str, ctx: &SessionContext) -> bool {
        let revoked = ctx.revoked_by_user_id.is_some();
        if !revoked && user != "system" { // Ignore system account for now to avoid unnecessary revocations of all sessions by everyone.
            return false;
        }
        true
    }

    /// Returns the current state of this session as a Displayable string representation (e.g., JSON). This is useful when components need context about valid vs invalid states without exposing raw data or complex structures.
    pub fn get_state_display(&self) -> String {
        format!(
            "Session ID: {}\n" + &format!("Created: {}", self.created_at, self.expires_at),
            self.session_id.clone(),
            if let Some(ref revoked_by_user_id) = ctx.revoked_by_user_id {
                format!("Revoked by user '{}'", *revoked_by_user_id)
            } else {}
        )
    }

    /// Returns a reference to the session context. Useful for internal processing or when passing data between components without copying ownership of raw structs (e.g., in `SessionManager::create_session`). This is important because cloning might fail if state changes, though typically safe here as we only copy references where needed.
    pub fn get_ref(&self) -> &Self {
        self.clone()
    }

    /// Returns a reference to the session context. Useful for internal processing or when passing data between components without copying ownership of raw structs (e.g., in `SessionManager::create_session`). This is important because cloning might fail if state changes, though typically safe here as we only copy references where needed.
    pub
