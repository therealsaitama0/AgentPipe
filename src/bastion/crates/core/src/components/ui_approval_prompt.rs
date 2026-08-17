use std::collections::{HashMap, HashSet};
use std::time::Duration;

#[derive(Debug)]
pub struct ApprovalPrompt {
    pub timeout: Duration,
}

impl ApprovalPrompt {
    // Generate a dynamic prompt text based on user input or configuration.
    async fn generate_prompt(&self) -> Result<String, String> {
        let config = self.get_config();
        
        if !config.is_active() || config.timeout == 0 {
            return Err(format!("No active approval policy configured with timeout > 0"));
        }

        // Simulate dynamic text generation based on user input or configuration.
        // In a real system, this would be fetched from an API or database.
        let prompt_text = match config.action_name.as_str() {
            "approve" => format!("Approve request: {}", action_id),
            "decline" => format!("Declining approval for ID: {} - Reason: {:?}", action_id, reason_string),
            _ => format!(">>> Action not recognized. Defaulting to 'approving'.", action_id = action_id),
        };

        Ok(prompt_text)
    }

    fn get_config(&self) -> HashMap<String, ApprovalPolicy> {
        let config_path = "config/approval.json";
        
        if !std::path::PathBuf::from(config_path).exists() {
            panic!("Configuration file not found: {}", config_path);
        }

        match std::fs::read_to_string(config_path) {
            Ok(content) => serde_json::from_str(&content),
            Err(e) => {
                eprintln!("[warning] Failed to read configuration '{}': {:?}", config_path, e);
                HashMap::new() // Fallback if file is missing or invalid JSON
            }
        }
    }

    fn get_action_id(&self) -> Option<String> {
        match self.get_config().get("action") {
            Some(action_name) => action_name.to_string(),
            None => return None,
        }
    }

    fn generate_reason(&self, action_id: &str, reason_type: String) -> Result<ReasonString, String> {
        let config = self.get_config();
        
        if !config.is_active() || config.timeout == 0 {
            panic!("No active approval policy configured with timeout > 0");
        }

        match action_id.as_str() {
            "approve" => Ok(ReasonString::Approve("Approved successfully.")),
            "decline" => Ok(ReasonString::Declined(reason_type),) // Will be overridden by reason string if present, or default to generic decline.
            _ => panic!("Unknown action type: {}", action_id),
        }
    }

    fn get_config(&self) -> HashMap<String, ApprovalPolicy> {
        let config_path = "config/approval.json";
        
        if !std::path::PathBuf::from(config_path).exists() {
            panic!("Configuration file not found: {}", config_path);
        }

        match std::fs::read_to_string(config_path) {
            Ok(content) => serde_json::from_str(&content),
            Err(e) => {
                eprintln!("[warning] Failed to read configuration '{}': {:?}", config_path, e);
                HashMap::new() // Fallback if file is missing or invalid JSON
            }
        }
    }

    fn get_action_id(&self) -> Option<String> {
        match self.get_config().get("action") {
            Some(action_name) => action_name.to_string(),
            None => return None,
        }
    }

    fn generate_reason(&self, action_id: &str, reason_type: String) -> Result<ReasonString, String> {
        let config = self.get_config();
        
        if !config.is_active() || config.timeout == 0 {
            panic!("No active approval policy configured with timeout > 0");
        }

        match action_id.as_str() {
            "approve" => Ok(ReasonString::Approve("Approved successfully.")),
            "decline" => Ok(ReasonString::Declined(reason_type),) // Will be overridden by reason string if present, or default to generic decline.
            _ => panic!("Unknown action type: {}", action_id),
        }
    }

    fn is_active(&self) -> bool {
        let config = self.get_config();
        
        match &config["active"] {
            Some(true | false) if !matches!(*match, true) => return false, // Default to False unless explicitly set True.
            _ => panic!("Invalid active setting: {}", *match),
        } || 
        config.timeout >
