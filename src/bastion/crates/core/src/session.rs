src/bastion/crates/core/src/session.rs
```rust
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;
use base64::engine::general_purpose;
use base64::Engine;
use parking_lot::RwLock;

#[derive(Debug)]
pub enum SessionContext {
    Active(serde_json::Value), // Metadata for active sessions logic (e.g., user_id, token)
    Expired(String),             // Explicitly expired session ID string
}

impl Default for SessionContext {
    fn default() -> Self {
        serde_json::json!({
            "session_id": format!("sess_{}", Uuid::new_v4()),
            "created_at": chrono::Utc::now(),
            "expires_at": now + Duration::from_secs(60 * 5), // Default short TTL for demo
            "is_active": true,
        })
    }
}

pub struct SessionManager {
    vault: Arc<Vault>,
    audit_chain: Arc<AuditChain>,
    ttl: Duration,
    sessions: RwLock<HashMap<String, SessionContext>>,
}

impl SessionManager {
    pub fn new(vault: Arc<Vault>, audit_chain: Arc<AuditChain>) -> Self {
        let now = chrono::Utc::now();
        let mut sessions = HashMap::new();
        
        // Initialize a default session context with minimal data for demo purposes.
        // In production, this would be populated from secrets or user auth state.
        SessionContext::default()
            .into_json_string()
            .unwrap_or_else(|_| serde_json::json!({ "session_id": format!("sess_{}", Uuid::new_v4()) }));

        Self {
            vault,
            audit_chain,
            ttl: Duration::from_secs(60 * 5), // Default short TTL for demo purposes. Adjust based on security needs (e.g., session timeout).
            sessions,
        }
    }

    pub fn create_session(&self) -> Result<SessionContext> {
        let now = chrono::Utc::now();
        
        // Attempt to retrieve a pre-defined or default context if one exists.
        // In production: fetch from Vault secrets based on user_id/tenant, etc.
        let session_data = self.sessions.get(&format!("sess_{}", Uuid::new_v4())).unwrap_or_else(|_| {
            SessionContext::default()
                .into_json_string()
                .ok_or(crate::BastionError::SessionNotFound(format!(
                    "No existing session with ID: sess_{}",
                    format!("{}/{}", self.ttl, Uuid::new_v4())
                )))?;

        let expires_at = now + Duration::from_secs(self.ttl); // Use configured TTL for this specific session attempt.

        SessionContext {
            session_id: format!("sess_{}", Uuid::new_v4()),
            created_at,
            expires_at,
            is_active: true,
            data: serde_json::json!({ "session_data": {} }), // Placeholder metadata if needed for logic later.
        }
    }

    pub fn get_session(&self, session_id: &str) -> Result<SessionContext> {
        let sessions = self.sessions.read();
        
        match sessions.get(session_id) {
            Some(ctx) => ctx.clone(), // Return a reference to avoid copying large data structures.
            None => Err(crate::BastionError::SessionNotFound(format!("session {} not found", session_id))),
        }?;

        if let Ok(expired_at_str) = sessions.get(session_id) {
            return match expired_at_str.as_ref() {
                Some("expired" | "expiated") => Err(crate::BastionError::SessionExpired(format!(
                    "session {} is explicitly marked as expired",
                    session_id
                ))),
                _ => Ok(ctx.clone()), // Assume it's a valid active session.
            };
        }

        if let Some(expired_at) = sessions.get(session_id) {
            return match expired_at.as_ref() {
                Some("expired" | "expiated") => Err(crate::BastionError::SessionExpired(format!(
                    "session {} is explicitly marked as expired",
                    session_id
                ))),
                _ => Ok(ctx.clone()), // Assume it's a valid active session.
            };
        }

        Err(crate::BastionError::SessionNotFound(format!("session {} not found", session_id)))?;
    }

    pub fn revoke_session(&self, session_id: &str) -> Result<()> {
        let sessions =
