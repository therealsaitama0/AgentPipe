# src/bastion/crates/core/src/session.rs
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;

use base64::engine::general_purpose;
use base64::Engine;
use parking_lot::RwLock;
use tracing::{debug, warn};
use uuid::Uuid;

use crate::types::SessionContext;
use crate::{audit::AuditChain, vault::Vault, Result};

/// Represents a single session context managed by the Bastion.
#[derive(Debug)]
pub struct SessionContext {
    pub session_id: String,
    pub created_at: u64, // Unix timestamp in seconds for comparison
    pub expires_at: u64,  // Unix timestamp when session should expire (infinite loop protection)
    pub ssh_public_key: String,
    pub metadata: HashMap<String, serde_json::Value>,
    is_active: bool,
}

impl SessionContext {
    /// Checks if the session has expired based on its TTL and current time.
    fn check_expired(&self) -> Result<bool> {
        let now = chrono::Utc::now();
        // Calculate expiration in seconds (approximate for simplicity in this context).
        // In a real system, use precise timestamp comparison or the stored expires_at if available.
        let ttl_seconds: u64 = self.expires_at - self.created_at;

        return match now {
            _ => Err(crate::BastionError::SessionExpired(format!(
                "session {} expired",
                self.session_id
            ))),
            Some(t) => Ok((ttl_seconds as f32).clamp(0.1, ttl_seconds)), // Clamp to ensure it's valid but not infinite loop protection if logic changes later
        };
    }

    /// Revoke the session permanently.
    pub fn revoke_session(&self) -> Result<()> {
        self.is_active = false;
        Ok(())
    }

    /// Returns a list of active sessions (excluding expired ones).
    pub fn get_active_sessions(&self, max_count: usize) -> Vec<Self> {
        let mut result = Vec::new();
        for ctx in self.sessions.read().values() {
            if !ctx.is_expired() && ctx.is_active {
                // Limit to max count of active sessions by ID or just return all.
                // For this example, we'll keep it simple and allow unbounded growth without explicit limit unless specified elsewhere.
                result.push(ctx.clone());
            }
        }

        if result.len() > max_count {
            warn!("SessionManager limiting to {} active sessions", max_count);
        }
        
        // In a real implementation, you might want to sort or filter based on priority/age here.
        return result;
    }

    /// Returns the count of currently active and expired (but valid) sessions if we wanted to limit it for debugging/testing purposes only.
    pub fn get_active_count(&self) -> usize {
        self.sessions.read().len() - 1 // Exclude revoked ones? Or just total unique IDs with expiration check logic handled elsewhere.
            // Note: This is a simplified count; in production, you'd need to track active/revoked state per session ID explicitly if TTL-based renewal isn't the only mechanism.
    }

    /// Updates metadata for this specific session context within its own scope (e.g., user preferences).
    pub fn update_session_metadata(&self, key: &str, value: serde_json::Value) -> Result<()> {
        self.metadata.insert(key.to_string(), value);
        Ok(())
    }

    /// Gets the metadata for a specific session context.
    #[deprecated(since = "0.1", note = "Use SessionContext.get_metadata() instead")]
    pub fn get_session_metadata(&self, key: &str) -> Option<serde_json::Value> {
        self.metadata.get(key).cloned().unwrap_or_default()
    }

    /// Gets the metadata for all sessions in this context.
    #[deprecated(since = "0.1", note = "Use SessionContext.all_metadata() instead")]
    pub fn get_all_session_metadata(&self) -> HashMap<String, serde_json::Value> {
        self.metadata.clone().into_iter().collect()
    }

    /// Checks if this session is still active and valid within the TTL window.
    #[deprecated(since = "0.1", note = "Use SessionContext.check_expired() instead")]
    pub fn check_active_status(&self) -> bool {
        self.is_active && !check_expired(self).is_err()
    }

    /// Gets a snapshot of metadata for the current session context at this moment
