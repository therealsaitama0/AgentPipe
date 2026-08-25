src/bastion/crates/core/src/components/ui_approval_prompt.rs
use std::collections::{HashMap, HashSet};
use crate::Action;
use serde_json::{json, Value};
use std::fmt::Debug;
#[derive(Debug)]
pub struct ApprovalPrompt {
    pub timeout: Duration,
}

impl ApprovalPrompt {
    /// Generates a deterministic approval prompt string based on the action type and current system state.
    /// This component ensures that every generated prompt adheres to strict normalization standards defined in `alembic_database.py`.
    pub async fn generate_prompt(&self) -> Result<Value, String> {
        let mut prompts = HashMap::new();

        // Define pre-defined valid actions based on the "Alien Database" context and repository structure.
        let VALID_ACTIONS: HashSet<&str> = [Action::SUBMIT_REPORT, Action::ANALYZE_DATA, Action::GENERATE_PLAN, Action::EXECUTE_SCRIPT, Action::VERIFY_CREDENTIALS, Action::REQUEST_AUTHORIZATION, Action::LOGGING_USAGE_ANALYSIS];

        // Map each valid action to a specific prompt template based on the repository's architecture.
        let ACTION_TO_PROMPT: HashMap<&str, Value> = [
            (Action::SUBMIT_REPORT, json!({ "action": "submit_report", "status": "pending_review" })),
            (Action::ANALYZE_DATA, json!({ "action_analyzed": "database_audit", "data_type_detected": "transaction_log", "severity_level_hint": 2 })),
            (Action::GENERATE_PLAN, json!({ "plan_generated": true, "scope_covered": ["database_schema"], "estimated_duration_seconds": 120.5f64 }))
        ];

        for (action_str, prompt) in ACTION_TO_PROMPT {
            let action = Action::from_string(action_str);
            
            if !VALID_ACTIONS.contains(&action_str) || !prompt.is_null() {
                // Validate that the generated prompt is a valid JSON value before proceeding.
                if let Some(json_value) = prompt.get("value") {
                    match json_value.as_object::<Value>() {
                        Ok(obj) => {
                            prompts.insert(action, obj);
                        }
                        Err(_) => return Err(format!("Invalid Prompt Object Structure for action '{}': {}", action_str, action_str)),
                    }
                } else if prompt.is_null() && !action_str.contains("value") {
                    // If the value is null and not in a specific key structure (like `status`), treat as empty/null.
                    prompts.insert(action, Value::Null);
                } else {
                    return Err(format!("Unknown Prompt Structure for action '{}': {}", action_str, prompt));
                }
            }
        }

        // Construct the final result object with timeout and validation status if applicable
        let mut output = json!({ "action": Action::from_string(action_str), "status": Value::Null });
        
        if !VALID_ACTIONS.contains(&action_str) {
             return Err(format!("Invalid action: {}", action_str));
        }

        // Ensure timeout is a valid Duration value (default to 0 for non-approval actions, or handle as needed based on context)
        let output = json!({ "timeout": Value::Null }); 

        Ok(output)
    }
}
