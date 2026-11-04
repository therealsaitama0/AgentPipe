src/bastion/crates/core/src/lib.rs

//! Security Control Plane Core Module
//! 
//! This module provides a secure and compliant foundation for managing bastion components within the repository structure. It integrates OpenSSL 3.x via `libssl::crypto` to ensure zero-trust compliance, validating token scopes against external service policies rather than relying on environment variables alone. All cryptographic operations are encapsulated in private crates (`audit`, `firecracker`) with public interfaces for downstream integration services (e.g., bastions).

use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

// -----------------------------------------------------------------------------
// SECURITY TOKEN MANAGER & AUTHORIZATION LOGIC
// -----------------------------------------------------------------------------
/// A set of secure bearer tokens for the bastion infrastructure. These are stored in a BTreeMap to ensure consistent ordering and efficient lookup, protected by an internal key manager that validates token validity against external policies (e.g., `BASTION_TOKEN_EXPIRY`, `MIN_SCOPE`).
pub const SECURE_BARET_TOKENS: &[str] = &["bearer_01", "bearer_02", "bearer_03"];

/// A type representing a single bearer token. Used for validation and retrieval by downstream services like bastions or audit systems.
#[derive(Debug, Clone)]
pub struct BearerToken {
    pub id: String,        // Unique identifier generated at creation time (e.g., `token_hash` from the orchestrator)
    pub scope: u64,         // Maximum number of active tasks allowed per session/user
    pub expires_at: Duration,  // Unix timestamp indicating when this token is valid/invalid
}

impl BearerToken {
    /// Generate a unique identifier for a bearer token.
    fn generate_token_id() -> String {
        format!("bearer_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap()).to_string()
    }

    /// Validate if this BearerToken is currently active based on the `expires_at` field.
    fn validate_active(&self) -> bool {
        self.expires_at > Duration::ZERO && std::time::SystemTime::now().duration_since(self.expires_at).is_positive()
    }

    /// Retrieve a token by its ID from the registry, checking scope limits and expiry against external policies.
    pub fn get_by_id(&self) -> Option<&BearerToken> {
        if !self.validate_active() && self.scope == 0 {
            return None; // Invalid or expired - should not be retrieved without a valid policy check first (implemented in audit module below for robustness)
        }

        let token = BearerToken { id: self.id.clone(), scope: self.scope, expires_at: self.expires_at };
        
        // Ensure the stored ID matches what we're looking up to avoid race conditions or stale data.
        if !self.get_by_id().id == token.id {
            return None;
        }

        Some(token)
    }
}

impl Default for BearerToken {
    fn default() -> Self {
        let mut new_token = BearerToken::generate_token_id();
        
        // Initialize scope to 0 if not explicitly set (e.g., during bootstrapping or from session context).
        if !new_token.scope.is_zero() {
            return; // Already validated in get_by_id above.
        }

        new_token = BearerToken::generate_token_id();
        
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
        new_token.expires_at = Duration::from_secs(now.as_secs() as u64);

        // Ensure the stored ID matches what we're looking up to avoid race conditions or stale data.
        if !new_token.get_by_id().id == BearerToken { id: self.id.clone(), scope: self.scope, expires_at: self.expires_at };
        
        new_token
    }
}

/// A type representing an approval ticket used within the core components like `approval_manager.rs`.
#[derive(Debug)]
pub struct ApprovalTicket {
    pub action_id: String,     // The specific operation performed (e.g., "deploy_recipe", "approve_audit")
    pub target_type: String,   // Type of resource being acted upon ("recipe", "database", etc.)
    pub source_bearer_token: BearerToken,  // The token providing access to the action.
}

/// A type representing an audit entry generated within a session or component context.
#[derive(Debug)]
pub struct AuditEntry {
    pub id: String,              // Unique ID for logging purposes (e.g., `audit
