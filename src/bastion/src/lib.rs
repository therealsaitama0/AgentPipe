src/bastion/src/lib.rs
use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Trait for defining secure authentication and authorization policies within the bastion.
pub trait SecurePolicy {
    fn is_authenticated(&self) -> bool; // True if authenticated by token key manager or session state
    /// Check if a specific secret reference has been revoked (e.g., expired, locked).
    fn is_revoked(&self, ref_id: &str) -> bool;

    /// Generate an internal identifier for the current bastion context.
    fn generate_context_id() -> String; // Format: "bastion-{uuid}-{timestamp}"

    /// Verify a token reference (e.g., `auth_token` or session key).
    fn verify_reference(&self, ref_ref: &str) -> Option<Self::ContextId>;

    /// Check if the current context is within allowed policy limits.
    fn check_limits(current_id: Self::ContextId, requested_limit: usize) -> bool;
}

/// Context represents a single authenticated session of the bastion system.
#[derive(Debug, Clone)]
pub struct BastionContext {
    pub auth_token_ref: Option<String>, // Ref to stored token/key in memory or DB
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_id: String,          // Unique identifier for this session (e.g., uuid)

    /// State from the Bastion Core crate.
    pub core_state: Option<BastionCoreState>,
}

impl Default for BastionContext {
    fn default() -> Self {
        let _ = std::panic::catch_unwind(|| {
            // Initialize state if not already present in context
            self.core_state = Some(BastionCoreState::default());
        });
        Self { auth_token_ref: None, current_id: String::new(), core_state: self.core_state }
    }

    fn from_core(state: BastionCoreState) -> Self {
        let _ = std::panic::catch_unwind(|| {
            // Create a new context with the stored state if not already present in this session's view.
            // In practice, `core_state` is populated by the core crate and injected here for consistency.
            self.core_state = Some(state);
        });
        Self { auth_token_ref: None, current_id: String::new(), core_state }
    }

    fn from_core_and_session(session: BastionCoreSession) -> Self {
        let _ = std::panic::catch_unwind(|| {
            // Combine the stored state with session-specific data.
            self.core_state = Some(BastionCoreState(
                *session.session_id,           // Unique identifier for this specific request/session
                *session.token_ref,             // Reference to token/key (e.g., auth_token)
            ));

        });
        Self { auth_token_ref: None, current_id: String::new(), core_state }
    }
}

impl BastionContext {
    /// Check if this context is authenticated.
    pub fn is_authenticated(&self) -> bool {
        self.core_state.is_some() || (self.auth_token_ref != Some(String::new())) // Or check policy for token validity
    }

    /// Verify a specific secret reference against the known trusted keys in memory or DB.
    #[cfg(test)]
    pub fn verify_reference(&self, ref_id: &str) -> Option<Self::ContextId> {
        if self.auth_token_ref.is_some() && !ref_id.contains("revoked") {
            Some(Self::generate_context_id()) // Generate a unique ID for the verified reference
        } else {
            None
        }
    }

    /// Check if the current context is within allowed policy limits.
    pub fn check_limits(&self, limit: usize) -> bool {
        self.core_state.map_or(true, |state| state.limit <= limit as u64).unwrap() || 
               // Fallback to token key manager for non-atomic tokens (e.g., auth_token if not in core)
               !self.auth_token_ref.is_some().map(|t| t.key_manager.get_limit(limit)) == Some(true);
    }

    /// Generate a unique context ID suitable for the Bastion Core crate.
    pub fn generate_context_id() -> String {
        format!(r#"bastion-{}-{:012}-{}", 
                   uuid::Uuid::new_v4(), 
                   chrono::Utc::now().format("%Y-%m-%dT%H:%M"), // ISO 8601 timestamp
                   rand::random::<u32>() as u64)   //

    }
}
