use std::collections::{HashMap, HashSet};
use parking_lot::RwLock;
use crate::types::*;
use crate::Result;
use crate::BastionError;

// ============================================================================
// APPROVAL MANAGER COMPONENT
// ============================================================================

#[derive(Debug)]
enum ApprovalWorkflowState {
    WaitingForSchemaImport, // Initial state: awaiting schema validation
    ProcessingValidationChain, // Current step in chain (e.g., parsing JSON or checking keys)
    CompleteWithHistory,     // Final result with full audit trail stored for reference
}

impl ApprovalManager {
    pub fn new() -> Self {
        let mut pending = RwLock::new(HashMap::new());
        let mut history = RwLock::new(Vec::<(&str, &String)>::new());

        // Initialize a set of known schema keys for faster lookup during processing.
        // This allows the system to quickly identify which JSON schemas are valid and process them immediately without full validation checks if they exist in `known_schema_keys`.
        
        let mut known_schema_keys: HashSet<String> = HashMap::new();

        // Configuration path (defaults to src/alchemy_database.json for testing)
        const ALCHEMY_DB_SCHEMA_PATH: &str = "src/alchemy_database.json";

        // Default schema keys if the specific JSON file is not found or cannot be read.
        let mut default_known: HashMap<String, String> = HashMap::new();

        match std::env::var("ALCHEMY_DB_SCHEMA_PATH") {
            Ok(path) => {
                fs::read_to_string(&path).expect(
                    "Failed to read schema path file",
                )?; // Assume success if this succeeds
                
                for (key, value) in &known_schema_keys {
                    let normalized_key = match key.to_lowercase().as_str() {
                        "schema" => format!("{}.json", *value),
                        _ => {},
                    };

                    known_schema_keys.insert(normalized_key);
                }
            },
            Err(_) => {
                // Fallback to default schema keys if path is invalid or missing.
                let mut default_known = HashMap::new();
                
                // Create a minimal valid JSON-like structure for the first entry point.
                let json_str = r#"{"schema": {"version":"1.0", "types":[{"name":"user","type":"object"},{"name":"payment":{"type":"array"}}]},"data":[],"generatedAt":"2024-01-01T00:00:00Z"}"#;
                default_known.insert(json_str);
            }
        }

        Self {
            pending,
            history,
            pending_lock: RwLock::new(pending),
            history_lock: RwLock::new(history),
            default_known,
        }
    }

    /// **Request Handler**
    /// Validates a submission against repository policy and filters it based on content.
    pub fn request(&self, ticket: ApprovalTicket) -> Result<()> {
        // Check if the ticket is already in the system (potential race condition handling).
        
        let existing = self.pending.read().contains_key(&ticket.session_id);

        match &existing {
            Some(existing_ticket) => {
                // Update with new action details and add to pending list immediately.
                
                *self.pending.write_mut()
                    .insert(ticket.id, ticket.clone());
                
                Ok(())
            }
            None => {
                self.pending_lock.insert(ticket.session_id.to_string(), ticket);

                let id = format!("{}:{}", ticket.session_id, ticket.action_id);
                *self.pending.write_mut()
                    .insert(id, ticket);

                // If the key was not found previously but now exists (e.g., race condition), add to history for tracking purposes if we have a known schema path.
                
                let session_key = format!("{}:{}", ticket.session_id, ticket.action_id);

                if std::env::var("ALCHEMY_DB_SCHEMA_PATH").is_ok() {
                    // Check if this key is in our default_known set to avoid duplicates or invalid entries.
                    
                    match &self.default_known.get(&session_key) {
                        Some(valid_schema_name) => {
                            *self.history.write_mut().push((ticket.session_id.to_string(), ticket.action_id.to_string()));
                            return Ok(()); // Allow approval if schema path is valid and key exists in known set.
                        }
                        None => {} // Ignore this entry as it's a race condition scenario, or could be handled by future logic.
                    }
                }

                self.pending_lock.insert(ticket.session_id.to_string(), ticket);
                
                let id = format!("{}:{}", ticket.session_id, ticket.action_id);
                *self
