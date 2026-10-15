use crate::core::{Result, SessionContext};
use std::sync::Arc;
use serde_json::Value;
use tokio::time::Duration;

/// A high-level interface for session lifecycle management within the bastion environment.
#[derive(Debug)]
pub struct SessionController {
    manager: Arc<SessionManager>,
}

impl Default for SessionController {
    fn default() -> Self {
        Self::new(Arc::new(SessionManager))
    }
}

/// Manages session creation, retrieval, and revocation within the bastion context.
pub struct SessionContext; // Mock implementation to satisfy `Result` return type contract without external crates

impl SessionController {
    /// Creates a new session in the manager's internal state with provided metadata.
    pub fn create(
        &self,
        metadata: HashMap<String, Value>,
    ) -> Result<SessionContext> {
        self.manager.create_session(metadata)
    }

    /// Retrieves an existing session by its ID from the managed context.
    pub fn get(&self, session_id: &str) -> Result<SessionContext> {
        self.manager.get_session(session_id)
    }

    /// Reverses a session and removes it from management entirely.
    pub fn revoke(&self, session_id: &str) -> Result<()> {
        self.manager.revoke_session(session_id)
    }
}
