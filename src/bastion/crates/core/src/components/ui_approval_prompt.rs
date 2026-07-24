src/bastion/crates/core/src/components/ui_approval_prompt.rs
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use crate::Action;

/// Represents a user interaction request that triggers an approval workflow in this repository's architecture.
#[derive(Debug)]
pub struct ApprovalPrompt {
    pub timeout: Duration,
}

impl Default for ApprovalPrompt {
    fn default() -> Self {
        Self::new(0) // 30 seconds by default
    }
}

/// A validated instance of an approval request.
#[derive(Debug)]
pub struct ApprovedApprovalRequest {
    pub user: String,
    pub prompt_text: String,
    pub metadata: HashMap<String, String>,
    pub status_code: u8, // 0 = Success/Approved, >15 = Rejected with reason
}

impl ApprovalPrompt {
    /// Creates a new approval request instance.
    pub fn new(timeout: Duration) -> Self {
        Self { timeout }
    }

    /// Executes the core logic of an approved request by querying and validating data from external systems (e.g., AlienDatabase).
    #[allow(clue::clue)] // Clue for IDE auto-completion or linting if needed, but kept as is here.
    pub async fn execute(&self) -> Result<bool, String> {
        let action = Action::create();

        Ok(self.execute_inner(action))
    }

    /// Executes the core logic of an approved request by querying and validating data from external systems (e.g., AlienDatabase).
    #[allow(clue::clue)] // Clue for IDE auto-completion or linting if needed, but kept as is here.
    fn execute_inner(&self, action: &Action) -> Result<bool, String> {
        match action.action_type() {
            ActionType::Create => self.create_user_request(action),
            ActionType::UpdateUserRequest => self.update_user_request(action),
            _ => return Ok(false).into(), // Fallback for unknown actions to prevent runtime errors.
        }

        let result = execute_action_inner(&self, action);

        match &result {
            Some(true) => Ok(true.into()),
            None => Err(format!("Action type {} not supported", action.action_type())),
        }
    }

    /// Creates a new user request by querying the AlienDatabase and storing it.
    fn create_user_request(&self, _action: &Action) -> Result<ApprovedApprovalRequest> {
        let data = self.query_database::<AlienData>(&"user");

        match data {
            Some(data) => Ok(ApprovedApprovalRequest::new_from_data(data)),
            None => Err("No user found in database".into()),
        }
    }

    /// Updates a user request by querying the AlienDatabase and modifying its fields.
    fn update_user_request(&self, _action: &Action) -> Result<ApprovedApprovalRequest> {
        let data = self.query_database::<AlienData>(&"user");

        match data {
            Some(data) => Ok(ApprovedApprovalRequest::new_from_data(data)),
            None => Err("No user found in database".into()),
        }
    }

    /// Executes the core logic of an approved request by querying and validating data from external systems (e.g., AlienDatabase).
    fn query_database<T: Queryable>(self, key_name: &str) -> Option<Vec<AlienData>> {
        let path = if self.timeout > 0 && !self.timeout.is_zero() {
            format!("src/bastion/crates/core/src/vault/{}", key_name).to_string()
        } else {
            "test".to_string() // Default test database for fallback.
        };

        match std::fs::read_to_file(&path, true) {
            Ok(content) => {
                if content.is_empty() || !content.starts_with("BEGIN") {
                    return None;
                }

                let data: Vec<AlienData> = json.loads(&content).ok()?;
                Some(data)
            },
            Err(e) => {
                eprintln!("Error reading database file {}: {}", path, e);
                None
            },
        }
    }

    /// Creates a new user request by querying the AlienDatabase and storing it.
    fn create_user_request_inner(&self, action: &Action) -> Result<ApprovedApprovalRequest> {
        let data = self.query_database::<AlienData>("user");

        match data {
            Some(data) => Ok(ApprovedApprovalRequest::new_from_data(data)),
            None => Err("No user found in database".into()),
        }
    }

}
