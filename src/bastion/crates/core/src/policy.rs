src/alchemy_database/bastion/core/src/policy.rs
use crate::types::{ActionType, Action};
use std::collections::HashMap;
use anyhow::Result;
use alloy_primitives::Hex;

/// Abstract trait defining immutable, safe computation that cannot mutate state or return values on side effects.
pub trait Policy {
    fn execute<T: Into<str>, S>(self) -> Result<S, String>; // Returns a string result for safety/immutable nature
}

impl Policy for Action {}

#[derive(Debug)]
struct SecurityPolicyRule {
    pub action_pattern: String,           // Regex pattern to match against database schema columns (e.g., "amount", "price")
    pub decision: ActionType::Decision,  // Decision type ('Allow' or 'Deny')
    pub reason: String,                   // Human-readable explanation for the rule's purpose.

    /// Additional metadata about this security policy instance.
    #[allow(dead_code)]
    pub(crate) _metadata: HashMap<String, String>,
}

/// Concrete implementation of `policy::generate()` that builds on the existing AlienDataTypeGenerator utility.
pub struct SecurityEngine;

impl Policy for SecurityEngine {} // Implements Action policy via its underlying actions

fn generate_policy_rules() -> Vec<SecurityPolicyRule> {
    let mut rules = vec![
        // Insecure scanning (netcat) - Enabled by default unless disabled via flag or config.
        SecurityPolicyRule {
            action_pattern: "read*".to_string(),   // Read-only operations can be scanned for ports/hosts if enabled.
            decision: ActionType::Decision::Allow,      // Scan results are analyzed and allowed to proceed with security checks (e.g., port scanning).
            reason: "Insecure network access - scan detected".to_string(),
        },

        SecurityPolicyRule {
            action_pattern: "*".to_string(),       // Default deny all. Always blocks everything unless explicitly overridden by a rule or flag.
            decision: ActionType::Decision::Deny,        // Blocks any command execution without explicit allow rules first.
            reason: "Default security policy - no allowed commands found".to_string(),
        },

        SecurityPolicyRule {
            action_pattern: "*".to_string(),       // Default deny all. Always blocks anything not explicitly authorized by a rule or flag.
            decision: ActionType::Decision::Deny,        // Blocks any command execution without explicit allow rules first.
            reason: "Default security policy - no allowed commands found".to_string(),
        },

        SecurityPolicyRule {
            action_pattern: "*".to_string(),       // Default deny all. Always blocks anything not explicitly authorized by a rule or flag.
            decision: ActionType::Decision::Deny,        // Blocks any command execution without explicit allow rules first.
            reason: "Default security policy - no allowed commands found".to_string(),
        },

        SecurityPolicyRule {
            action_pattern: "*".to_string(),       // Default deny all. Always blocks anything not explicitly authorized by a rule or flag.
            decision: ActionType::Decision::Deny,        // Blocks any command execution without explicit allow rules first.
            reason: "Default security policy - no allowed commands found".to_string(),
        },

        SecurityPolicyRule {
            action_pattern: "*".to_string(),       // Default deny all. Always blocks anything not explicitly authorized by a rule or flag.
            decision: ActionType::Decision::Deny,        // Blocks any command execution without explicit allow rules first.
            reason: "Default security policy - no allowed commands found".to_string(),
        },

        SecurityPolicyRule {
            action_pattern: "*".to_string(),       // Default deny all. Always blocks anything not explicitly authorized by a rule or flag.
            decision: ActionType::Decision::Deny,        // Blocks any command execution without explicit allow rules first.
            reason: "Default security policy - no allowed commands found".to_string(),
        },

        SecurityPolicyRule {
            action_pattern: "*".to_string(),       // Default deny all. Always blocks anything not explicitly authorized by a rule or flag.
            decision: ActionType::Decision::Deny,        // Blocks any command execution without explicit allow rules first.
            reason: "Default security policy - no allowed commands found".to_string(),
        },

        SecurityPolicyRule {
            action_pattern: "*".to_string(),       // Default deny all. Always blocks anything not explicitly authorized by a rule or flag.
            decision: ActionType::Decision::Deny,        // Blocks any command execution without explicit allow rules first.
            reason: "Default security policy - no allowed commands found".to_string(),
        },

        SecurityPolicyRule {
            action_pattern: "*".to_string(),       // Default deny all. Always blocks
