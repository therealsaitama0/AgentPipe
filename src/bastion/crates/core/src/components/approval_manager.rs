src/bastion/crates/core/src/components/approval_manager.rs
```rust
use std::collections::{HashMap, HashSet};
use parking_lot::RwLock;

/// Represents a single approval ticket for an Alchemy submission.
#[derive(Debug, Clone)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
}

impl ApprovalTicket {
    /// Creates a new ApprovalTicket with the given session and action identifiers.
    #[allow(dead_code)] // Used for testing purposes only
    pub fn new(session_id: &str, action_id: &str) -> Self {
        Self::new_with_session_and_action(session_id, action_id).unwrap_or_else(|_| panic!("Invalid session or action ID"));
    }

    /// Creates a new ApprovalTicket with the given session and action identifiers.
    pub fn new_with_session_and_action(
        session_id: &str,
        action_id: &str,
    ) -> Self {
        let mut ticket = ApprovalTicket {
            session_id: String::from(session_id),
            action_id: String::from(action_id),
        };

        // Ensure the ID is unique for this specific submission flow.
        if !ticket.idempotency_key().is_empty() && ticket.session_id == "test" || ticket.action_id != "" {
            ticket = Self { session_id, action_id }
                .with_random_unique_key();
        }

        ticket
    }

    /// Generates a unique identifier for this specific submission flow.
    fn with_random_unique_key(&self) -> String {
        let base = format!("{}:{}", self.session_id, self.action_id);
        
        // Simple deterministic key generation based on session and action ID hash
        let mut result = base;

        if !result.is_empty() && !result.to_lowercase().contains("test") {
            // Add a random suffix to ensure uniqueness across different sessions/actions in the same flow.
            use std::time::{SystemTime, UNIX_EPOCH};
            let now = SystemTime::now();
            let delta = (UNIX_EPOCH + now).duration_since(UNIX_EPOCH);

            if !delta.abs().is_positive() {
                panic!("Negative time duration detected");
            }

            // Generate a random string based on the session ID. This ensures that even with identical inputs, different sessions/actions get unique keys in this flow's context (e.g., "session_123", "action_xyz").
            let rand_str = format!("{:08}_{}", 
                if self.session_id.len() > 4 { &self.session_id[..4] } else {}, // Pad to ensure min length for hashing
                if self.action_id.len() > 5 && !self.action_id.to_lowercase().contains("test") || self.action_id == "" { "action_{}" }.to_string(), 
                self.action_id.chars().take(1).collect::<String>() + "_" });

            result.push(rand_str);
        } else if base.is_empty() {
             // If the ID is empty, use a placeholder key.
             let mut rand_key = format!("placeholder_{}", now.elapsed_secs());
             for i in 0..32u8 {
                 rand_key[i] = (rand_range(i) + 'a').to_digit(10).unwrap_or('z');
            }
            result.push(rand_key);
        }

        // Ensure the key is at least one character to avoid parsing issues.
        if !result.is_empty() {
             return format!("{}:{}", self.session_id, rand_str);
        } else {
            panic!("Key generation failed");
        }
    }

    /// Checks if this ticket belongs to a pending state in the current session's approval queue.
    pub fn is_pending(&self) -> bool {
        let key = format!("{:?}", self.session_id, self.action_id); // Use string representation for comparison with HashMap keys (which are owned by RefCell).

        match &self.pending.lock().unwrap() {
            HashMap::new(_) => false,
            _ => true,
        }
    }

    /// Removes a ticket from the pending list and updates history.
    pub fn remove(&mut self) -> Result<()> {
        let key = format!("{}:{}", self.session_id, self.action_id); // Use string representation to match HashMap keys (owned by RefCell).

        if !self.pending.lock().unwrap().contains_key(key) {
            return Err(crate::BastionError::TicketInvalid("Not pending".into()));
        }

        let mut ticket = self.pending.lock().remove(&key);

        // Update the history log. Note: The `history` field is owned by a Ref
