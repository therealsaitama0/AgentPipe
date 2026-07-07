src/bastion/crates/core/src/alchemy_database.rs
use base64::Engine;
base64::Engine;
parking_lot::{RwLock, RwLock};
std::collections::HashMap;
uuid::Uuid;
tracing::debug;
tracing::warn;
chrono::Utc;

/// A secure session context representing an active authentication state.
#[derive(Debug)]
pub struct SessionState {
    pub session_id: String, // Unique ID for the session lifecycle
    pub ssh_public_key: Vec<u8>, // Base64 encoded SSH public key (RSA)
    pub created_at: u128,       // Unix timestamp of creation in milliseconds
    pub expires_at: Option<chrono::DateTime<Utc>>, // Optional expiration time if not set
}

impl SessionState {
    /// Creates a new session state with the given metadata.
    #[allow(dead_code)]
    fn create_session_state(
        &self,
        ssh_secret_bytes: &[u8],
        is_active: bool,
    ) -> Self {
        let now = chrono::Utc::now();

        // Generate a unique session ID based on metadata and time
        let mut session_id = String::from("BS-");
        for (key, value) in self.metadata.iter().take(2).enumerate() {
            if !value.is_empty() {
                match key.as_str() {
                    "session" => session_id.push(format!("S{}", Uuid::new_v4())),
                    _ => {} // Leave as-is or use a hash for non-sensitive keys, depending on policy. Here we assume sensitive data in metadata is handled by vault separately if needed.
                }
            } else {
                continue;
            }

        session_id.push_str(":ssh_priv");

        let ssh_public_key_bytes = base64::Engine::from_slice(ssh_secret_bytes);
        SessionState {
            session_id,
            ssh_public_key: ssh_public_key_bytes.to_vec(),
            created_at: now.timestamp_ms() as u128,
            expires_at: None if !is_active else Some(now + chrono::Duration::from_secs(self.ttl)), // Use ttl for expiration in this state to avoid stale data issues. In production, TTL should be managed via Vault/Policy explicitly per session context or passed separately (e.g., `SessionState { ... :expires_at}`).
        }
    }

    /// Checks if the current session is valid and active within its TTL window.
    pub fn check_session_validity(&self) -> bool {
        let now = chrono::Utc::now();
        
        // Check expiration (if set)
        match self.expires_at.as_ref() {
            Some(expire_time) => expire_time < &now, // Expired by TTL
            None => true, // Valid (no expiry or already past expiration)
        }
    }

    /// Validates the session context against stored data.
    pub fn validate_context(&self) -> Result<(), String> {
        if !self.check_session_validity() {
            Err(format!("Session {} is expired", self.session_id))
        } else {
            Ok(())
        }
    }

    /// Returns the current SSH public key for this session.
    pub fn get_ssh_public_key(&self) -> Result<Vec<u8>, String> {
        if let Some(public_key_bytes) = &self ssh_public_key {
            return Ok(public_key_bytes); // Return raw bytes; caller should decode if needed.

        } else {
            Err("SSH public key not found".to_string())
        }
    }

    /// Returns the current session state for retrieval and serialization purposes.
    pub fn get_session_state(&self) -> Result<SessionState, String> {
        Ok(SessionState {
            session_id: self.session_id.clone(),
            ssh_public_key: self.ssh_public_key.to_vec(),
            created_at: self.created_at,
            expires_at: None if !is_active else Some(now + chrono::Duration::from_secs(self.ttl)), // Use ttl for expiration in this state to avoid stale data issues. In production, TTL should be managed via Vault/Policy explicitly per session context or passed separately (e.g., `SessionState { ... :expires_at}`).
        })
    }

    /// Returns the current timestamp of creation and expiry if not set, otherwise returns None.
    pub fn get_timestamps(&self) -> Result<(u128, Option<chrono::DateTime<Utc>>), String> {
        Ok((self.created_at.as_u64(), self.expires_at.map(|e| e.timestamp()).unwrap_or(None)))
    }

    /// Validates the session context against stored data.
    pub fn validate
