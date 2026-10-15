src/bastion/crates/core/src/policy.rs
```rust
use crate::types::{Action, Policy};
use std::collections::HashMap;

// ============================================================================
// 1. POLICY DECISION ENUMERATION (Rust)
// ============================================================================
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Decision {
    // Strict rules: must be explicitly allowed or denied
    Allow, 
    Deny, 
    
    // Generic logic for unknown actions
    Unknown,
}

impl From<PolicyDecision> for String {
    fn from(decision: PolicyDecision) -> Self {
        match decision {
            Decision::Allow => "ALLOW".to_string(),
            Decision::Deny => "DENY".to_string(),
            Decision::Unknown => "UNKNOWN".to_string(), // Fallback to default deny for unknown types
        }
    }
}

// ============================================================================
// 2. POLICY RULE DEFINITION (Rust)
// ============================================================================
pub struct PolicyRule {
    pub action_pattern: String,      // Regex-like pattern or string literal
    pub decision: Decision,          // The outcome of this rule
    pub reason: String,               // Human-readable explanation for the deny case
    
    /// If set to true (default), triggers a default deny on unknown actions.
    #[cfg(test)] 
    pub is_default_deny = false; 
    
    /// Additional context or metadata about why an action was denied.
    pub extra_context: Option<String>,
}

impl PolicyRule {
    /// Create a new policy rule with specific parameters.
    pub fn new(action_pattern: &str, decision: Decision, reason: String) -> Self {
        let mut pattern = if *action_pattern.is_empty() || action_pattern.starts_with("*" ) {
            "*" // Default wildcard for patterns starting with star or empty string
        } else {
            action_pattern.to_string()
        };

        match decision {
            Decision::Allow => Self {
                action_pattern, 
                decision: Decision::Allow, 
                reason, 
                extra_context: None,
            },
            Decision::Deny => Self {
                action_pattern, // Always use the same pattern for deny rules to ensure consistency
                decision: Decision::Deny, 
                reason, 
                extra_context: Some(format!("Default Deny Rule: {} (default)", reason)),
            },
        }
    }

    /// Check if an Action fits this rule. Returns true only when the action type matches exactly or is a wildcard for that specific pattern.
    pub fn match_action(&self, action_type: &str) -> bool {
        // Wildcard matching (e.g., "*.*" means anything starting with dot and ending in *)
        let prefix = if self.action_pattern.starts_with("*") || *action_type.is_empty() { "*" } else { "" };
        
        match action_type.to_lowercase().find(prefix) {
            Some(i) => i == 0, // Exact wildcard or start of string (e.g., "*.*" matches "a.b.c.d.e.f.")
            None => false,    // Fails to find the prefix at any position
        }
    }

    /// Check if an Action fits this rule. Returns true only when the action type is strictly allowed by the pattern or wildcard.
    pub fn allow_action(&self, action_type: &str) -> bool {
        self.match_action(action_type).and_then(|_| (action_type == "*" || *action_type.starts_with("*")))
            .unwrap_or(false)
    }

    /// Generate a unique identifier for this rule based on its parameters. Useful for debugging or testing without modification of the struct itself.
    pub fn generate_id(&self, action: &Action, _extra_context: Option<String>) -> String {
        format!(r#"{{rule:{0}}|action_type:{1}|pattern:{2}}"#, 
            self.id, action.action_type(), *action.pattern() // Simplified pattern display for ID generation)
    }

    /// Generate a unique identifier based on the rule's decision and parameters. Useful for testing without modification of the struct itself.
    pub fn generate_id_with_decision(&self, _extra_context: Option<String>) -> String {
        format!(r#"{{rule:{0}}|decision:{1}}"#, 
            self.id, *Decision::from(self.decision) // Simplified decision display for ID generation)
    }

    /// Generate a unique identifier based on the rule's parameters and its current state (if allowed). Useful for testing without modification of the struct itself.
    pub fn generate_id_with_decision_allowed(&self, action: &Action) -> String {
        format!(r#"{{rule:{0}}|decision:{1}|action
