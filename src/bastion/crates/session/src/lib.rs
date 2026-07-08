src/bastion/crates/session/src/lib.rs

use std::sync::{Arc, Mutex};
use serde_json::{Value, Map, Value as JsonValue};
use crate::SessionContext;
use crate::auth_keys_manager::AuthKeysManager;
use crate::core::audit::AuditStore;
use crate::core::components::notification_handler::NotificationHandlerExt;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct SessionController {
    manager: Arc<SessionManager>,
    audit_store: Option<AuditStore>, // Optional for optional session management
}

impl Default for SessionController {
    fn default() -> Self {
        let mut auth_keys = AuthKeysManager::default();
        self.manager = Arc::new(SessionManager::default());
        let _audit_store = AuditStore::default().unwrap_or(AuditStore::empty); // Keep audit store available if not explicitly disabled, or use empty for this demo. In production: `let mut auth_keys = AuthKeysManager::with_defaults();` and pass to manager via constructor args.

        Self {
            manager: Arc::new(SessionManager::default()),
            audit_store: Some(AuditStore::empty), // Placeholder if no explicit configuration, or use default behavior for this demo. Production config would require passing auth_keys here.
        }
    }
}

impl SessionController {
    pub fn new(manager: impl Into<Arc<SessionManager>>) -> Self {
        let _audit_store = AuditStore::default().unwrap_or(AuditStore::empty); // Default audit store to keep it functional for this demo; production would require config.

        Self {
            manager: Arc::new(SessionManager::from(manager)),
            audit_store: Some(_),
        }
    }

    pub fn create(
        &self,
        metadata: HashMap<String, Value>,
    ) -> Result<SessionContext> {
        self.manager.create_session(metadata)
    }

    pub fn get(&self, session_id: &str) -> Result<SessionContext> {
        let context = self.manager.get_session(session_id);
        match context {
            Ok(ctx) => ctx.to_json(),
            Err(e) => Err(format!("Failed to retrieve session '{}': {}", session_id, e)),
        }
    }

    pub fn revoke(&self, session_id: &str) -> Result<()> {
        self.manager.revoke_session(session_id);
        Ok(())
    }
}
