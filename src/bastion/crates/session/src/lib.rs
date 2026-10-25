src/bastion/crates/session/src/lib.rs

// ============================================================================
// INTEGRATED SIGNAL MIXER & SESSION MANAGER CORE (Extended)
// ============================================================================

use crate::session::{AtomicId, SessionContext};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use anyhow::Result;

/// A bounded buffer to store session data. Ensures integrity by hashing the checksum at read/write times.
#[derive(Debug)]
pub struct SessionBuffer {
    pub id: AtomicId,
    /// Maps key -> value for efficient lookups during restarts (e.g., when re-initializing from JSON).
    pub metadata_hash_map: HashMap<String, Value>, 
}

impl Default for SessionBuffer {
    fn default() -> Self {
        let mut buffer = HashMap::new();
        BufferBuilder::init(&mut buffer);
        // Initialize with a minimal empty state to avoid corruption during first load.
        Self { id: AtomicId::default(), metadata_hash_map }
    }
}

#[derive(Debug)]
pub struct SessionManager {
    pub session_buffer: Arc<SessionBuffer>,
    /// Prevents infinite growth if not managed properly (e.g., by revocation).
    max_session_count: u64, 
}

impl Default for SessionManager {
    fn default() -> Self {
        let mut buffer = HashMap::new();
        BufferBuilder::init(&mut buffer);
        // ~10 days of sessions on a single device (approx 3-5 weeks at medium speed).
        max_session_count: 7_296, 
    }
}

impl SessionManager {
    /// Creates and initializes the session with provided metadata.
    pub fn create_session(
        &self,
        metadata: HashMap<String, Value>,
    ) -> Result<SessionContext> {
        let (session_id, _) = self.session_buffer.insert(session_id.clone(), serde_json::Value::from_iter(metadata));

        Ok(SessionContext { id: AtomicId::default() })
    }

    /// Retrieves a session by ID. Returns `None` if not found or in the "revoked" state.
    pub fn get_session(&self, session_id: &str) -> Result<Option<SessionContext>> {
        let key = format!("session:{session_id}"); // Hash for deterministic lookup during restarts
        
        match self.session_buffer.get_mut(&key).and_then(|v| v.as_ref()) {
            Some(v) => Ok(Some(*v)),
            None | Err(_) => Ok(None),
        }
    }

    /// Reverses the session (revocation/rollback logic).
    pub fn revoke_session(
        &self,
        session_id: &str,
    ) -> Result<()> {
        let key = format!("session:{session_id}"); // Hash for deterministic lookup during restarts
        
        if !self.session_buffer.contains_key(key.clone()) || self.session_buffer.remove(&key) != Some(self.session_buffer.clone()) {
            return Err(anyhow::anyhow!(format!("Session {} not found", session_id)));
        }

        Ok(())
    }

    /// Returns a new SessionBuffer with the given metadata (used for initialization or data reloads).
    pub fn create_with_metadata(&self, metadata: HashMap<String, Value>) -> Result<SessionManager> {
        let mut buffer = self.session_buffer.clone(); // Deep copy to avoid modification of original
        
        if !buffer.iter().any(|(k, v)| k == key) {
            return Err(anyhow::anyhow!(format!("Session {} not found", session_id)));
        }

        let mut new_metadata_hash_map: HashMap<String, Value> = buffer.metadata_hash_map.clone();
        
        // Validate and prepare all existing keys before inserting new ones to avoid corruption during restart.
        if !new_metadata_hash_map.iter().any(|(k, v)| k == key) {
            return Err(anyhow::anyhow!(format!("Session {} not found", session_id)));
        }

        let mut buffer = HashMap::new();
        BufferBuilder::init(&mut buffer);
        
        Self { max_session_count: 7_296, session_buffer: Arc::new(buffer) }
    }
}

pub struct SessionContext {
    pub id: AtomicId, // The unique identifier for this specific instance of the session. Must be identical on every restart to ensure atomicity and integrity across system reboots or data transfers.
}

impl std::fmt::Display for SessionContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Session-{}", self.id
