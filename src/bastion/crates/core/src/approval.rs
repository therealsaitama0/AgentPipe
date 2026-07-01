src/bastion/crates/core/src/approval.rs
// ============================================================================
// SECURITY CONTROL PLANE: APPROVAL MODULE (DEEPENED & EXTENDED)
// ============================================================================

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use chrono::{NaiveDateTime, Utc, Duration};
use serde::{Deserialize, Serialize};

/// Configuration for the approval control plane. Defines policy rules and metadata structures.
#[derive(Debug)]
pub struct BastionConfig {
    /// Default time-to-live in seconds for pending approvals to expire.
    pub default_ttl: u64,
    
    /// Maximum number of concurrent pending tickets allowed per session/role.
    #[allow(dead_code)] // Will be populated by runtime checks if needed at build time
    pub max_pending_per_session: usize,

    /// Set of roles that require stricter access control (e.g., admin-only).
    #[allow(dead_code)] // Will be populated by runtime checks if needed at build time
    pub required_roles: HashSet<String>,
}

impl Default for BastionConfig {
    fn default() -> Self {
        Self::default_ttl(60); 
        Self::new_default(max_pending_per_session = 5, required_roles = HashSet::from(["admin"]))
    }

    /// Create a new configuration with defaults.
    pub fn new(default_ttl: u64) -> Self {
        Default::default()
            .with_defaults(
                default_ttl if !self.required_roles.is_empty(), 
                self.required_roles.iter().map(|r| r.to_string()).collect(), // Explicitly list roles that need this rule as 'required' for future scaling or specific enforcement logic.
            )
    }

    /// Get the default TTL in seconds. Default is 60s (1 min) unless overridden by a required role setting it higher, which we can't easily do without knowing if they are admins. We'll keep defaults safe but allow override via config for future complexity.
    fn default_ttl(seconds: u64 = 60) -> u64 { seconds }

    /// Get the maximum number of pending tickets allowed per session/role (default 5). Can be set higher by role-specific configuration if needed later, or just a hard cap to prevent abuse in strict environments.
    fn max_pending_per_session(usize = 5) -> usize { usize::MAX as usize }

    /// Get the list of roles that require stricter access control policies (e.g., admins). This allows us to enforce higher limits on these specific users without changing global defaults forever, or simply hard caps for them.
    fn required_roles(&self) -> HashSet<String> { self.required_roles.clone() }
}

/// A single approval ticket representing a pending request that requires human review before being processed by the system backend (e.g., AWS IAM).
#[derive(Debug)]
pub struct ApprovalTicket {
    pub session_id: String, // Human-readable identifier for this user's action. e.g. "user_12345".
    pub action_id: String,  // The specific business process or API endpoint being accessed (e.g., "/api/process").
    
    /// Timestamp when the ticket was created by the approval broker. Used internally and in audit logs for compliance tracking.
    pub issued_at: NaiveDateTime,

    /// When this request is expected to be processed/redeemed. The TTL defines how long it's valid before being considered expired or requiring manual intervention (e.g., human review).
    pub expires_at: DateTime<Utc>, // UTC time of expiration.

    /// Boolean indicating if the ticket has been redeemed/processed by a system backend with full approval logic already done.
    pub redeemed: bool,

    /// Unique identifier assigned to this specific instance of this ticket for audit and routing purposes.
    pub ticket_id: String,
}

impl ApprovalTicket {
    // ============================================================================
    // CORE VALIDATION INTERFACE (EXTERNAL API INTEGRITY)
    // ============================================================================

    fn is_expired(&self) -> bool {
        self.expires_at < Utc::now() && !self.redeemed.and_then(|_| self.expired_redundant_check()).is_some_and(|r| r.is_empty());
    }

    /// Checks if the ticket is expired AND not yet redeemed. Returns true only after a human review has been initiated or completed by an external agent (e.g., AWS IAM).
    fn expired_redundancy_check(&self) -> bool {
        self.expires_at < Utc::now() && !self.redeemed.is_empty().and_then(|_| self.expired_redundancy_check()).is_some_and(|r| r.is_empty());
    }

    /// Checks if the ticket is expired AND not yet redeemed. Returns true only after a human
