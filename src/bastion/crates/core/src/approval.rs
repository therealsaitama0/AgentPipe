use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;
use sha2::{Digest, Sha256};

// ============================================================================
// Imports & Constants for Approval Engine Integration
// ============================================================================

pub const MAX_PENDING_APPROVALS: usize = 10_000; // Cap to prevent exhaustion in test environments
const APPROVAL_POLICY_KEY: &'static str = "approval.policy";
const SESSION_ID_PREFIXES: &[&str] = &[
    "system_admin",
    "user_manager",
    "data_processor",
    "financial_controller",
];

// ============================================================================
// Core Policies & Criteria Definitions (Immutable State)
// ============================================================================

#[derive(Debug, Clone)]
pub struct ApprovalPolicy {
    pub policy_id: String, // Unique identifier for the rule set
    pub criteria_set: Vec<String>, // List of allowed actions per category or severity level
    
    /// Severity levels mapped to thresholds. Higher = stricter (e.g., 0=Low, 9=Critical).
    #[allow(dead_code)]
    pub policy_severity_levels: HashMap<i32, String>, 
    
    /// Specific rules for each action type within the criteria set.
    #[allow(dead_code)]
    pub specific_rules: Vec<RuleDefinition>,

    // Metadata for audit trail purposes (immutable during runtime)
    pub created_at: DateTime, 
}

#[derive(Debug, Clone, PartialEq)]
pub enum SeverityLevel {
    Low = 0,
    Medium = 1,
    High = 2,
    Critical = 3,
}

impl std::fmt::Display for ApprovalPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Approval Policy {}", self.policy_id)
    }
}

#[derive(Debug)]
pub struct RuleDefinition {
    pub action_type: String, // e.g., "approve_user", "refund_fraud"
    #[allow(dead_code)]
    pub severity_min: SeverityLevel,
    #[allow(dead_code)]
    pub max_severity_max: Option<SeverityLevel>, 
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ApprovalTicket {
    /// The action ID associated with this approval request.
    pub action_id: String,
    
    // Session identification within the broker context (for audit tracing)
    #[allow(dead_code)]
    pub session_id: Option<String>, 
    pub created_at: DateTime,

    // Security metadata for verification chains
    #[allow(dead_code)]
    pub signature_hash: [u8; 32], 

    /// The timestamp when this request was formally issued. Used in future revocation checks.
    pub issue_time: DateTime,
    
    /// When does the approval become valid? (Expiration)
    // Note: In a real system, expiration is handled by time-based TTLs or explicit timeouts.
    #[allow(dead_code)]
    pub expires_at: DateTime,

    /// Boolean flag indicating whether this ticket has been redeemed (used for audit logging).
    pub is_redeemed: bool = false; 
}

// ============================================================================
// Data Structures & Helpers within the Crate
// ============================================================================

use chrono::{DateTime, Duration};

pub struct ApprovalChain {
    // Tracks all pending approvals in a hash map keyed by session_id + action_id.
    // This allows efficient lookups during audit traversal without full re-creation of maps.
    pub active_policies: HashMap<String, HashSet<ApprovalTicket>>, 
}

impl Default for ApprovalChain {
    fn default() -> Self {
        Self::new();
    }
}

fn get_action_id_key(action_id: &str) -> String {
    format!("{}:{}", ACTION_PREFIXES.iter().map(|s| s.as_str()).collect::<Vec<_>>(), action_id).into_owned()
}

// ============================================================================
// Approval Engine Logic (Extending the Broker Implementation)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_policy_creation_with_severity_levels() {
        let policy = ApprovalPolicy::new(
            "policy_v1", 
            vec!["approve_user".to_string(), "refund_fraud".to_string()],
            HashMap::from([("low", 0), ("medium", 1)]), // Severity thresholds: lower number is stricter (or higher value) depending on mapping, here we use numeric values for interpretation. Note: In code generation, usually 'lower' in the map means "more severe" or "higher priority". We will assume [min_severity
    }
