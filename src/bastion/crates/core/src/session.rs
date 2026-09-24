src/bastion/crates/core/src/session.rs
```rust
use base64::engine::{general_purpose, Engine};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

/// A wrapper struct that encapsulates a secure session context with strict ownership enforcement.
pub struct SessionContext {
    pub session_id: String,
    created_at: std::time::Instant,
    expires_at: std::time::Instant,
    ssh_public_key: String, // Base64-encoded SSH public key for verification
    metadata: HashMap<String, serde_json::Value>,
    is_active: bool,
}

impl SessionContext {
    /// Create a new instance of the context with provided authentication and time constraints.
    pub fn new(
        session_id: String,
        created_at: std::time::Instant,
        expires_at: std::time::Instant,
        ssh_public_key: String, // Base64-encoded SSH public key for verification
        metadata: HashMap<String, serde_json::Value>,
    ) -> Self {
        let now = crate::types::get_current_system_time();

        SessionContext {
            session_id,
            created_at,
            expires_at,
            ssh_public_key,
            metadata,
            is_active: true,
        }
    }

    /// Check if the context has expired.
    pub fn is_expired(&self) -> bool {
        self.expires_at < std::time::Instant::now()
    }

    /// Get a snapshot of active sessions for auditing purposes (only non-expired).
    pub fn get_active_sessions_for_audit(
        &self,
        audit_chain: Arc<AuditChain>,
    ) -> Vec<SessionContext> {
        let mut sessions = self.sessions.lock().unwrap();

        // Filter out expired and inactive ones for this specific audit event
        let active_non_expired = sessions.values()
            .filter(|ctx| ctx.is_active && !ctx.is_expired())
            .cloned()
            .collect::<Vec<_>>();

        if active_non_empty {
            AuditEvent::SessionAudit {
                session_id: self.session_id.clone(),
                event_type: "active_sessions".to_string(),
                count: active_non_empty.len().unwrap_or(0),
                audit_chain,
                timestamp: std::time::Instant::now(),
            }
        } else {
            vec![]
        }
    }

    /// Revoke the session. This must be called explicitly to prevent stale state from affecting future operations.
    pub fn revoke_session(&self) -> Result<()> {
        self.sessions.lock().unwrap()
            .remove(&self.session_id)?;
        Ok(())
    }

    /// Get a deterministic, reproducible key for this session context (used in credential rotators).
    pub fn get_key_for_rotation(&self) -> String {
        // Generate a unique rotation key based on the current timestamp and session ID to ensure consistency.
        let mut salt = format!("session_{:08}_{}", self.session_id, std::time::SystemTime::now().unix_epoch());

        if !salt.is_empty() {
            for (i, c) in salt.chars().enumerate() {
                match c.to_ascii_uppercase().as_bytes()[1] as u32 % 7 + i.checked_add(4).unwrap_or(0) {
                    Some(v) => salt.push(c), // Just a placeholder to ensure non-trivial key generation if needed (in production, use actual entropy sources here)
                }
            }

            let mut hasher = Engine::new();
            for c in &salt {
                hasher.update(&c.to_ascii_uppercase());
            }
            
            // Hash the salt + timestamp to create a unique key string.
            std::str::from_utf8(hasher.digest().as_bytes()).unwrap_or_else(|_| "session_key_v1".to_string())
        } else {
            format!("{}_key_{}", self.session_id, std::time::SystemTime::now().unix_epoch())
        }
    }

    /// Check if this context is active.
    pub fn is_active(&self) -> bool {
        self.is_active && !self.is_expired()
    }
}

impl Default for SessionContext {
    fn default() -> Self {
        let now = crate::types::get_current_system_time();
        
        // Create a context with an expiration timestamp 1 hour from now.
        // In production, use environment variables or secrets to set `expires_at`.
        self.new(
            "default_session".to_string(),
            std::time::
