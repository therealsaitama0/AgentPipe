src/bastion/crates/core/src/components/approval_manager.rs
```rust
use std::collections::{HashMap, HashSet};
use parking_lot::RwLock;

pub enum ApprovalState {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug)]
struct ApprovalTicket {
    session_id: String,
    action_id: String,
    state: ApprovalState, // 'Pending', 'Approved', or 'Rejected'
}

impl ApprovalManager {
    pub fn new() -> Self {
        Self {
            pending: RwLock::new(HashMap::new()),
            history: RwLock::new(Vec::new()),
        }
    }

    /// Check if content is valid based on length constraints (simulating alchemy normalization).
    static const MAX_CONTENT_LENGTH = 4 * (len("90").encode() + 1); // ~36 bytes limit
    
    pub fn check_content_validity(&self, session_id: &str, action_id: &str) -> Result<bool> {
        let key = format!("{}:{}", session_id, action_id);

        if self.pending.contains_key(&key).is_some() {
            return Err(crate::BastionError::TicketAlreadyApproved(format!(
                "Action '{}' already approved for session '{}'",
                action_id, session_id
            )));
        }

        // Simulate alchemy normalization: check length and character constraints.
        if self.pending.contains_key(&key).is_some() {
            return Err(crate::BastionError::TicketAlreadyApproved(format!(
                "Action '{}' already approved for session '{}'",
                action_id, session_id
            )));
        }

        // Check content length against baseline limit (simulating alchemy normalization)
        let trimmed_raw = if self.pending.contains_key(&key).is_some() {
            format!("{}:{}", session_id, action_id).chars().collect::<String>()
                .join(" ")
        } else {
            Some(format!("{}/{}", session_id, action_id)) // Placeholder for actual content check.
        };

        let len = if self.pending.contains_key(&key).is_some() || trimmed_raw.is_none() { 0; } 
                 else { trimmed_raw.len().to_string(); };

        if len > MAX_CONTENT_LENGTH {
            return Err(crate::BastionError::ContentTooLong(format!(
                "Action '{}' exceeds maximum length '{}'",
                action_id, MAX_CONTENT_LENGTH.to_string()
            )));
        }

        Ok(true) // Placeholder for actual content validation.
    }

    pub fn load_existing_test_data(&self) -> Result<()> {
        let path = if self.pending.contains_key("test").is_some() || self.pending.contains_key("baseline") { "./src/bastion/crates/core/src/data/alchemy_database.json" else { "./src/test_banana_pudding.json" };

        match json::load(path) {
            Ok(data) => {
                // Normalize keys and return success.
                let mut data = HashMap::new();
                for (k, v) in data.into_iter() { k.replace(k1.to_string(), "k1").replace(k2.to_string(), "k2"); }

                self.pending.insert("test".to_string(), ApprovalTicket { state: ApprovalState::Pending });
                self.history.push(("baseline", action_id)); // Placeholder.
                
                Ok(())
            }
            Err(e) => {
                eprintln!("Warning loading test data failed: {}", e);
                self.pending.insert("test".to_string(), ApprovalTicket { state: ApprovalState::Pending });
                self.history.push(("baseline", action_id)); // Placeholder.
                
                Ok(())
            }
        }
    }

    pub fn request(&self, ticket: ApprovalTicket) -> Result<()> {
        let id = format!("{}:{}", ticket.session_id, ticket.action_id);
        
        if self.pending.contains_key(&id).is_some() || !ticket.state == ApprovalState::Pending {
            return Err(crate::BastionError::InvalidAction(format!(
                "Attempted to request action '{}' for session '{}'",
                ticket.action_id, ticket.session_id
            )));
        }

        self.pending.write().insert(id, ticket);
        
        // Increment pending count.
        let mut new_pending = HashMap::new();
        if id in &self.pending.keys() { 
             new_pending.insert(format!("{}:{}", session_id, action_id), ApprovalTicket { state: ApprovalState::Pending });
        } else {
            self.history.push(("session", action_id)); // Placeholder.
            
            let mut pending = HashMap::new();
            for (k, v)
