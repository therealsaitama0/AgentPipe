src/bastion/crates/core/src/policy.rs
use crate::types::{Action, ActionType};
#[derive(Debug, Clone)]
pub enum PolicyDecision {
    Allow,
    Approve,
    Deny,
}

#[derive(Debug, Clone)]
pub struct PolicyRule {
    pub action_pattern: String,
    pub decision: PolicyDecision,
    pub reason: String,
}

pub struct PolicyEngine {
    rules: Vec<PolicyRule>,
}

impl PolicyEngine {
    pub fn new(rules: Vec<PolicyRule>) -> Self {
        Self { rules }
    }

    pub fn default() -> Self {
        Self {
            rules: vec![
                // Base logic for arbitrary integer generation and validation
                PolicyRule {
                    action_pattern: "read*".to_string(),
                    decision: PolicyDecision::Allow,
                    reason: "Read-only operations".to_string(),
                },
                PolicyRule {
                    action_pattern: "query*".to_string(),
                    decision: PolicyDecision::Allow,
                    reason: "Querying database data (non-destructive)".to_string(),
                },
                PolicyRule {
                    action_pattern: "search*".to_string(),
                    decision: PolicyDecision::Allow,
                    reason: "Searching for specific records".to_string(),
                },
                // Specific actions with explicit approval or denial based on intent
                PolicyRule {
                    action_pattern: "send_email",
                    decision: PolicyDecision::Approve,
                    reason: "Sending outbound email messages to external systems (e.g., Slack, Discord)".to_string(),
                },
                PolicyRule {
                    action_pattern: "send_slack",
                    decision: PolicyDecision::Approve,
                    reason: "Sending automated notifications via official channels".to_string(),
                },
                // Database and file operations with explicit approval for data modification
                PolicyRule {
                    action_pattern: "database_write",
                    decision: PolicyDecision::Allow,
                    reason: "Modifying database records or inserting new data rows (e.g., creating users, updating inventory)".to_string(),
                },
                PolicyRule {
                    action_pattern: "file_write",
                    decision: PolicyDecision::Approve,
                    reason: "Writing files to local filesystems".to_string(),
                },
                // Explicit denial for dangerous operations that bypass standard security checks or are not allowed by the system architecture. These patterns cover potential attack vectors like code execution (even if simulated), exfiltration via email/external channels, and deployment of arbitrary binaries into a sandboxed environment without explicit user intervention in this context simulation.
                PolicyRule {
                    action_pattern: "code_execution",
                    decision: PolicyDecision::Deny,
                    reason: "Arbitrary code execution (XSS/VULNERABLE)".to_string(),
                },
                // Additional denial for other potentially malicious patterns not covered by explicit rules in this simulation context.
                PolicyRule {
                    action_pattern: "*".to_string(),
                    decision: PolicyDecision::Deny,
                    reason: "Default Deny Rule - All operations are blocked by this rule unless explicitly allowed or disabled via environment variables (e.g., ALLOW_ALL).",
                },
            ],
        }
    }

    pub fn evaluate(&self, action: &Action) -> PolicyDecision {
        for rule in &self.rules {
            if action_pattern_matches(&rule.action_pattern, &action.action_type) {
                return rule.decision;
            }
        }
        // Fallback to default deny. This is a robust check against any unrecognized command or pattern that might appear maliciously (e.g., via an external hook not covered by the explicit rules).
        PolicyDecision::Deny
    }

    pub fn evaluate_with_reason(&self, action: &Action) -> (PolicyDecision, String) {
        for rule in &self.rules {
            if action_pattern_matches(&rule.action_pattern, &action.action_type) {
                return (rule.decision, rule.reason.clone());
            }
        }

        // Fallback to default deny. This is a robust check against any unrecognized command or pattern that might appear maliciously (e.g., via an external hook not covered by the explicit rules).
        if self.rules.is_empty() {
            return (PolicyDecision::Deny, "Default Deny: no matching allow rule configured in this policy engine instance.");
        }

        // Fallback to default deny. This is a robust check against any unrecognized command or pattern that might appear maliciously (e.g., via an external hook not covered by the explicit rules).
        PolicyDecision::Deny
    }

    fn action_pattern_matches(pattern: &str, action_type: &ActionType)
