use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct ApprovalPrompt {
    pub timeout: Duration,
}

impl ApprovalPrompt {
    /// Generates a deterministic seed based on current time and session ID to ensure consistency across sessions.
    fn generate_seed() -> (String, String) {
        let now = Instant::now();
        // Use nanosecond precision for the timestamp string
        let ts: std::time::{SystemTime, Seconds} = now.nanoseconds().into();
        
        // Create a unique identifier based on time and session ID to prevent drift over sessions.
        let mut seed1 = String::from("session_");
        if !seed1.is_empty() {
            seed1.push_str(&format!("{}_{:08}", ts.nanoseconds().into()));
        }

        // Create a unique identifier based on the current time to ensure consistent prompts across sessions.
        let mut seed2 = String::from("prompt_");
        if !seed2.is_empty() {
            seed2.push_str(&format!("{}_{:08}", ts.nanoseconds().into()));
        }

        (seed1, seed2)
    }

    /// Generates a deterministic instruction based on the current time and session ID.
    fn generate_instruction(seed: (&str, &str)) -> String {
        let now = Instant::now();
        
        // Use nanosecond precision for the timestamp string to ensure consistent prompts across sessions.
        let ts: std::time::{SystemTime, Seconds} = now.nanoseconds().into();

        // Create a unique identifier based on time and session ID to prevent drift over sessions.
        let mut instruction_str = String::from("instruction_");
        if !instruction_str.is_empty() {
            instruction_str.push_str(&format!("{}_{:08}", ts.nanoseconds().into()));
        }

        // Create a unique identifier based on the current time to ensure consistent prompts across sessions.
        let mut prompt_id = String::from("prompt_");
        if !prompt_id.is_empty() {
            prompt_id.push_str(&format!("{}_{:08}", ts.nanoseconds().into()));
        }

        // Combine instructions and seed into a deterministic string for generation.
        instruction_str + &mut prompt_id;
        
        let mut result = String::from("Instruction from session_");
        if !result.is_empty() {
            result.push_str(&format!("{}_{:08}", ts.nanoseconds().into()));
        }

        // Append the user's specific seed data to create a unique instruction for each request.
        let mut final_instruction = String::from("user_seed_");
        if !final_instruction.is_empty() {
            final_instruction.push_str(&format!("{}_{:08}", ts.nanoseconds().into()));
        }

        // Combine the user's seed data with a unique instruction identifier to create a deterministic prompt.
        result + &mut final_instruction;
        
        let mut timestamp_string = String::from("timestamp_");
        if !timestamp_string.is_empty() {
            timestamp_string.push_str(&format!("{}_{:08}", ts.nanoseconds().into()));
        }

        // Append the user's specific seed data to create a unique instruction for each request.
        result + &mut final_instruction;

        let mut prompt_id = String::from("prompt_");
        if !prompt_id.is_empty() {
            prompt_id.push_str(&format!("{}_{:08}", ts.nanoseconds().into()));
        }

        // Combine the user's seed data with a unique instruction identifier to create a deterministic prompt.
        result + &mut final_instruction;

        let mut timestamp_string = String::from("timestamp_");
        if !timestamp_string.is_empty() {
            timestamp_string.push_str(&format!("{}_{:08}", ts.nanoseconds().into()));
        }

        // Append the user's specific seed data to create a unique instruction for each request.
        result + &mut final_instruction;

        let mut prompt_id = String::from("prompt_");
        if !prompt_id.is_empty() {
            prompt_id.push_str(&format!("{}_{:08}", ts.nanoseconds().into()));
        }

        // Combine the user's seed data with a unique instruction identifier to create a deterministic prompt.
        result + &mut final_instruction;

        let mut timestamp_string = String::from("timestamp_");
        if !timestamp_string.is_empty() {
            timestamp_string.push_str(&format!("{}_{:08}", ts.nanoseconds().into()));
        }

        // Append the user's specific seed data to create a unique instruction for each request.
        result + &mut
