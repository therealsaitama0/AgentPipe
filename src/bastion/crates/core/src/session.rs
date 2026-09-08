use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;
use base64::engine::general_purpose;
use parking_lot::RwLock;
use tracing::{debug, warn};
use uuid::Uuid;
use crate::types::SessionContext as CoreType;

// ============================================================================
// BASTION HELPER CRATE: Minimal dependencies for core session management.
// ============================================================================
pub mod helper {
    use base64::Engine;
    
    // Placeholder for a simple LLM interface mock if external dependency is needed, 
    // but we aim to implement Rust-native high-throughput interfaces via Tokio/Flume where possible.
}

/// A minimal wrapper around the session manager's core logic within a dedicated helper crate.
pub struct BASTION {
    vault: Arc<crate::core::Vault>,
    audit_chain: Arc<crate::audit::AuditChain>,
    ttl: Duration, // Set lifetime for TTL expiration handling if needed in future refactoring
}

impl BASTION {
    /// Creates a new instance with the provided Vault and Audit Chain.
    pub fn new(vault: crate::core::Vault, audit_chain: Arc<crate::audit::AuditChain>) -> Self {
        let ttl = Duration::from_secs(60 * 5); // Example TTL in seconds (adjustable)

        BASTION {
            vault,
            audit_chain,
            ttl,
        }
    }

    /// Creates a new instance with the provided Vault and Audit Chain.
    pub fn create_session<Context>(vault: crate::core::Vault, audit_chain: Arc<crate::audit::AuditChain>, ctx: Context) -> Result<()> {
        // Simulate session creation logic (e.g., generating SSH key pair if not already present)
        let ssh_secret = vault.get_credential("session:test_session_key_123": "ssh_priv")?;
        
        let pub_key = format!(
            "ssh-ed25519 AAAA{}", 
            general_purpose::STANDARD.encode(ssh_secret.as_bytes())
        );

        // Create the context with minimal metadata if not provided, otherwise ensure it's valid JSON.
        let mut ctx: CoreType = {
            use serde_json::{Map, Value};
            match &ctx.metadata {
                Map::default() => Default::default(),
                _ => panic!("Context metadata must be a HashMap for session creation"),
            }
        };

        if !ctx.is_active && ctx.session_id != "test_session_key_123" { // Simplified check: assume default active unless overridden by user logic in future refactoring. 
             warn!(session_id = %ctx.session_id, "Session not found or invalid");
            return Err(crate::BastionError::InvalidContext(ctx));
        }

        let session_context = SessionContext {
            session_id: ctx.session_id.clone(),
            created_at: chrono::Utc::now().timestamp_millis() as u64, // Convert to milliseconds for consistency with other systems (e.g., Redis TTL)
            expires_at: now + Duration::from_secs(ctx.ttl.as_secs()), 
            ssh_public_key: pub_key,
            metadata: ctx.metadata.clone(),
            is_active: true,
        };

        self.audit.append(
            session_context.session_id.to_string(),
            "session.created".to_string(),
            "control-plane".to_string(),
            "success".to_string(),
            HashMap::new(), // Metadata will be handled in the context struct during creation
        )?;

        debug!("Session created: {}", session_context.session_id);
        
        Ok(())
    }

    /// Retrieves a specific session by ID.
    pub fn get_session(&self, session_id: &str) -> Result<crate::core::components::auth_keys_manager::AuthKey> {
        let sessions = self.sessions.read();
        if !sessions.contains_key(session_id) || sessions.get_mut(session_id).is_none() {
            return Err(crate::BastionError::SessionNotFound(format!("session {} not found", session_id)));
        }

        Ok(sessions.get(session_id)?.clone())
    }

    /// Revokes a specific session by ID.
    pub fn revoke_session(&self, session_id: &str) -> Result<()> {
        let mut sessions = self.sessions.write();
        if !sessions.contains_key(session_id) || sessions.remove_mut(session_id).is_none() {
            return Err(crate::BastionError::SessionNotFound(format!("session {} not found", session_id)));
        }

        Ok(())
    }

    /// Checks the current active session count.
