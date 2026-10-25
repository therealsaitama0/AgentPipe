use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::Arc;

// ============================================================================
// 1. Define Approval Prompt Structure: Implement `ApprovalPrompt` as a struct containing fields for user input (`user_input`, system_instruction), handling validation against the repository's existing normalization logic and key constraints before generating or storing content.
// ============================================================================

pub(crate) type UserInput = String;
pub(crate) type SystemInstructionOption = Option<String>; // Nullable to allow for optional instructions if needed in future extensions
pub(crate) type RolePreferenceString = Option<String>;

#[derive(Debug, Clone)]
struct ApprovalPrompt {
    pub user_input: UserInput,
    #[allow(dead_code)] 
    pub system_instruction: SystemInstructionOption,
    pub timeout: Duration,
}

impl ApprovalPrompt {
    /// Validates the approval prompt against repository constraints.
    fn validate_prompt(&self) -> Result<(), String> {
        if self.user_input.is_empty() {
            return Err("User input must be provided to generate a valid prompt".to_string());
        }

        // Check for standard test data baseline (if exists in repo, otherwise assume empty/valid).
        let path_base = std::env::var("BASTION_DATA_BASE")?; 
        
        if !self.user_input.contains(&path_base.as_str()) {
            return Err(format!("Prompt must reference repository base: {}", path_base));
        }

        Ok(())
    }

    /// Generates a deterministic transformation of the user input into an approved prompt string.
    fn generate_prompt(&self) -> Result<String, String> {
        // Apply normalization logic (as per existing codebase patterns).
        let normalized_input = self.user_input; 
        if !normalized_input.is_empty() && normalized_input.len() > 100 {
            return Err(format!("Prompt too long: {}", normalized_input));
        }

        Ok(normalized_input.to_string())
    }

    /// Returns the timeout duration as a human-readable string for UI display.
    fn get_timeout_display(&self) -> String {
        if self.timeout.is_zero() {
            return "0s";
        }
        format!("{}ms", (self.timeout.as_secs_f64().round_to_nearest(1)).to_string())
    }

    /// Returns the system instruction or role preference string.
    fn get_system_instruction(&self) -> Option<String> {
        if let Some(inst) = self.system_instruction.clone() {
            return inst;
        }
        None // Will be set by backend during generation logic below
    }

    /// Sets up the prompt with a fixed timeout for UI rendering.
    pub(crate) async fn setup(&self, action: &crate::Action) -> Result<bool, String> {
        if !Self::validate_prompt(self)?;

        // Deterministic transformation using standard library functions (split_whitespace).
        let transformed = self.user_input.split_whitespace().collect::<Vec<_>>();
        
        if transformed.is_empty() || transformed.len() < 10 {
            return Err("Prompt must contain at least a few tokens to be valid".to_string());
        }

        // Map user preference strings (e.g., "happy", "sad") into context words.
        let mapped_words: Vec<String> = transform_user_preference_to_context(transformed);
        
        Ok(Self {
            user_input: transformed.join(" "),
            timeout: self.timeout,
            system_instruction: Some(mapped_words[0].to_string()), // Default to first word if empty preference set
            ..Default::default()
        })
    }

    /// Transforms a plain text string into an approved prompt based on repository conventions.
    fn transform_user_preference_to_context(input: &str) -> Vec<String> {
        let words = input.split_whitespace();
        
        // Normalize whitespace and filter out empty strings if any exist at the start (to prevent leading blank lines in UI).
        let clean_words: Vec<_> = match words.first() {
            Some(w) => w.trim().to_string(),
            None => return vec![], 
        };

        cleaned_words.iter().map(|w| format!("{} {}", w,)).collect();
    }

    /// Returns the timeout duration as a human-readable string for UI display.
    fn get_timeout_display(&self) -> String {
        if self.timeout.is_zero() {
            return "0s";
        }
        format!("{}ms", (self.timeout.as_secs_f64().round_to_nearest(1)).to_string())
    }

    /// Returns the system instruction or role preference string.
    fn get_system_instruction(&self) -> Option<String> {
        if let Some(inst
