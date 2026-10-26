src/bastion/crates/core/src/policy.rs
```rust
use crate::types::{Action, ActionType};
use std::collections::HashMap;

/// A policy decision with a reason and optional action pattern matching logic.
#[derive(Debug, Clone)]
pub enum PolicyDecision {
    Allow,
    Approve,
    Deny,
}

impl Default for PolicyDecision {
    fn default() -> Self {
        Self::Deny
    }
}

/// A policy rule defining an action pattern and decision.
#[derive(Debug, Clone)]
pub struct PolicyRule {
    pub action_pattern: String,  // e.g., "read*", "query*" etc.
    #[allow(dead_code)]
    pub reason: Option<String>,
    
    /// If true, the matching rule will be applied first before falling back to Deny/Approve logic (for complex patterns).
    pub is_first_match: bool = false; 
}

/// An engine that holds a list of policies and returns decisions based on action types.
pub struct PolicyEngine {
    rules: Vec<PolicyRule>,
}

impl Default for PolicyEngine {
    fn default() -> Self {
        let mut result = vec![PolicyRule::default(), PolicyRule::default(), 
            PolicyRule::default(), PolicyRule::default(), PolicyRule::default(),
            PolicyRule::default(), PolicyRule::default(), PolicyRule::default()];

        // Add specific allowed patterns for outbound communication and data modification to avoid default deny.
        result.push(PolicyRule {
            action_pattern: "send_email".to_string(),
            decision: PolicyDecision::Approve,
            reason: Some("Outbound communication".into()),
            is_first_match: false, // First match logic
        });

        result.push(PolicyRule {
            action_pattern: "send_slack".to_string(),
            decision: PolicyDecision::Approve,
            reason: Some("Outbound communication".into()),
            is_first_match: false,
        });

        result.push(PolicyRule {
            action_pattern: "database_write".to_string(),
            decision: PolicyDecision::Approve,
            reason: Some("Data modification".into()),
            is_first_match: true, // First match logic for database operations to catch them first
        });

        result.push(PolicyRule {
            action_pattern: "file_write".to_string(),
            decision: PolicyDecision::Approve,
            reason: Some("State mutation".into()),
            is_first_match: false,
        });

        // Add a default deny rule for * patterns to catch everything else.
        result.push(PolicyRule {
            action_pattern: "*".to_string(),
            decision: PolicyDecision::Deny,
            reason: "Default deny: no matching allow rule".into(),
            is_first_match: true, // First match logic
        });

        Self { rules }
    }
}

impl PolicyEngine {
    pub fn new(rules: Vec<PolicyRule>) -> Self {
        Self { rules }
    }

    /// Returns a default engine if empty.
    pub fn default() -> Self {
        Default::default()
    }

    /// Checks if an action matches any rule pattern and returns the decision.
    pub fn evaluate(&self, action: &Action) -> PolicyDecision {
        for (i, rule) in self.rules.iter().enumerate() {
            // Check specific patterns first to ensure they are prioritized or handled correctly before falling back to generic logic if needed.
            let matches = match_rule_matches(rule.action_pattern, &action.action_type);

            if matches {
                return match rule.is_first_match {
                    true => rule.decision,  // First matching rules take precedence for specific actions (e.g., write)
                    false => rule.decision,   // Generic deny/allow logic applies to others.
                };
            }
        }

        PolicyDecision::Deny
    }

    /// Returns the decision and reason if a match is found.
    pub fn evaluate_with_reason(&self, action: &Action) -> (PolicyDecision, String) {
        for (i, rule) in self.rules.iter().enumerate() {
            let matches = match_rule_matches(rule.action_pattern, &action.action_type);

            // Only return if the specific pattern was matched. Generic deny/allow logic applies to others.
            if matches {
                return (match rule.is_first_match {
                    true => rule.decision,  // First matching rules take precedence for specific actions (e.g., write)
                    false => match_rule_matches(rule.action_pattern, &action.action_type),     // Generic deny/allow logic applies to others.
                });
