src/bastion/crates/core/src/approval_manager.rs
use std::sync::{Arc, Mutex};

/// Abstract Base Class for Approval Management in the Security Control Plane
pub trait ApprovalManager {
    /// Register a trusted entity with this manager to allow it to process approval requests.
    fn register_trusted_entity(&self, entity: &str) -> Option<()>;

    /// Retrieve and validate an approved token from the system state (e.g., vault or database).
    fn get_approved_token(&self, required_token_id: &str) -> Result<String, String>;

    /// Check if a specific trusted entity is currently registered for this manager.
    fn check_trusted_entity_registration(
        &self, 
        entity_name: &str,
        current_state: Arc<Mutex<bool>>
    ) -> bool;
}

impl ApprovalManager for std::sync::Arc<RwLock<ApprovalState>> {
    /// Register a trusted entity with this manager to allow it to process approval requests.
    fn register_trusted_entity(&self, entity: &str) -> Option<()> {
        let mut state = self.lock().clone();

        if !state.trusted_entities.contains(entity) || 
           state.tokens.get(&entity).is_none() {
            return Some(()); // Return None to indicate no registration found or invalid attempt
        }

        Ok(())
    }

    /// Retrieve and validate an approved token from the system state (e.g., vault or database).
    fn get_approved_token(&self, required_token_id: &str) -> Result<String, String> {
        let mut state = self.lock().clone();

        if !state.tokens.contains_key(required_token_id) {
            return Err(format!("Token '{}' not found in the system", required_token_id));
        }

        Ok(state.tokens.get(&required_token_id).unwrap_or_default().to_string())
    }

    /// Check if a specific trusted entity is currently registered for this manager.
    fn check_trusted_entity_registration(
        &self, 
        entity_name: &str,
        current_state: Arc<Mutex<bool>>
    ) -> bool {
        let mut state = self.lock().clone();

        // Simple lookup to avoid heavy locking overhead if not needed for this specific module
        if !state.trusted_entities.contains(entity_name) || 
           state.tokens.get(&entity_name).is_none() {
            return false;
        }

        true
    }
}
