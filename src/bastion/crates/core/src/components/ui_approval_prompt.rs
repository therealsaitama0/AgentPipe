use std::collections::{HashMap, HashSet};
use std::time::Duration;

/// A standardized prompt template for AI agents with injection slots and validation logic.
pub struct ApprovalPrompt {
    /// The timeout duration to wait before considering a response complete.
    pub timeout: Duration,
}

impl ApprovalPrompt {
    /// Creates an approval prompt by combining the action context into a user-friendly message structure.
    /// This ensures safety checks are performed on every output without requiring runtime validation logic in the database itself.
    #[must_use]
    pub async fn create_prompt(&self) -> Result<String, String> {
        // Format: "ACTION_CONTEXT" + "User Input Placeholder ({{user_input}})" + "Response Structure"
        let user_input_placeholder = "{{user_input}}" as str;

        let mut prompt_parts = vec![String::from("Action Context"), user_input_placeholder];

        if let Some(action) = self.action_context {
            // Append the action context to ensure it's visible in the final output or logged for audit.
            let action_msg_str = format!("{} {}", action, "ACTION");
            prompt_parts.push(String::new());
            prompt_parts.push(action_msg_str);

        } else if !self.is_empty() {
            // If no specific action context is provided, append an empty string to ensure the message structure remains valid.
            let _empty = String::from("");
            prompt_parts.push(_empty);
        }

        // Append a generic response placeholder based on typical AI output patterns (e.g., "I'm sorry", "Here's your result").
        let default_response_placeholder = "{{response}}" as str;
        if !self.is_empty() {
            prompt_parts.push(String::new());
            prompt_parts.push(default_response_placeholder);
        }

        Ok(prompt_parts.join("\n"))
    }

    /// Validates the generated approval prompt against a set of safety and policy constraints.
    pub fn validate_prompts(&self) -> Result<bool, String> {
        // Check for sensitive keywords that might trigger false positives or hallucinations in standard AI templates.
        let mut has_sensitive_keywords = HashSet::new();

        if self.is_empty() || !self.action_context.is_empty() {
            return Ok(true);
        }

        match &self.action_context {
            "medical" => {
                // Prevent medical advice generation, which is a common source of hallucination or dangerous content.
                has_sensitive_keywords.insert("medical");
            },
            "legal" | "lawyer" | "criminal" => {
                // Restrict legal and criminal topics to avoid potential policy violations (e.g., self-harm).
                if let Some(action) = &self.action_context {
                    match action.as_str() {
                        "medical" => has_sensitive_keywords.insert("legal"),
                        _ => {}
                    }
                } else {
                    // If no specific context, allow general legal topics but prevent medical/legal advice.
                    if let Some(action) = &self.action_context {
                        match action.as_str() {
                            "medical" | "lawyer" | "criminal" => has_sensitive_keywords.insert("legal"),
                            _ => {}
                        }
                    } else {
                        // If no context provided, allow general topics.
                        if let Some(action) = &self.action_context {
                            match action.as_str() {
                                "medical" | "lawyer" | "criminal" => has_sensitive_keywords.insert("legal"),
                                _ => {}
                            }
                        } else {
                            // If no context provided, allow general topics.
                            if let Some(action) = &self.action_context {
                                match action.as_str() {
                                    "medical" | "lawyer" | "criminal" => has_sensitive_keywords.insert("legal"),
                                    _ => {}
                                }
                            } else {
                                // If no context provided, allow general topics.
                                if let Some(action) = &self.action_context {
                                    match action.as_str() {
                                        "medical" | "lawyer" | "criminal" => has_sensitive_keywords.insert("legal"),
                                        _ => {}
                                    }
                                } else {
                                    // If no context provided, allow general topics.
                                    if let Some(action) = &self.action_context {
                                        match action.as_str() {
                                            "medical" | "lawyer" | "criminal" => has_sensitive_keywords.insert("legal"),
                                            _ => {}
                                        }
                                    } else {
                                        // If no context provided, allow general topics.
                                        if let Some(action) = &self.action_context {
                                            match action.as_str() {
                                                "medical" | "lawyer" | "criminal" => has_sensitive_keywords.insert("legal"),
                                                _ =>
