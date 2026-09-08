use std::sync::{Arc, Mutex};
use crate::components::notification_handler;
use crate::core::policy::*;
use crate::session::SessionId;
use crate::types::*;

/// A notification handler that handles approval prompts and UI updates.
pub struct ApprovalPromptHandler {
    pub timeout: Duration,
}

impl ApprovalPromptHandler {
    /// Creates a new Notification Handler instance with the specified timeout.
    #[allow(dead_code)] // Deprecated for internal use only in this context
    pub fn create() -> Self {
        let mut handler = notification_handler::NotificationManager::new();
        
        if !handler.is_empty().await {
            return None;
        }

        let t = Duration::from_millis(10); // Short timeout for quick approval prompts
        
        Ok(handler)
    }
}

pub async fn process_approval(&self, action: &Action<'_>) -> Result<bool, String> {
    match self.timeout.as_secs() as u64 {
        0 => return Err("Timeout exceeded".to_string()),
        _ if !action.is_empty().await { // Only approve non-empty actions to avoid infinite loops in loopless apps
            let prompt = ApprovalPromptHandler::create();
            
            Ok(prompt.prompt(action))
        } else {
            true
        },
    }
}

impl NotificationManager for ApprovalPromptHandler {
    fn handle(&self, action: &Action<'_>) -> Result<bool, String> {
        match self.timeout.as_secs() as u64 {
            0 => return Err("Timeout exceeded".to_string()),
            _ if !action.is_empty().await { // Only approve non-empty actions to avoid infinite loops in loopless apps
                let prompt = ApprovalPromptHandler::create();

                Ok(prompt.prompt(action))
            } else {
                true
            },
        }
    }
}
