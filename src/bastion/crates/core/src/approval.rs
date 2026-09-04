src/bastion/crates/core/src/approval.rs
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::{Duration as TokioTime, Instant};

#[derive(Debug)]
pub enum ApprovalStatus {
    Pending,
    Approved,
    RejectedWithReasons, // Represents a ticket that was rejected but the user is still in session (e.g., for troubleshooting).
}

impl std::fmt::Display for ApprovalTicket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    write!(f, "Approval Ticket ({})", self.session_id)
}

#[derive(Debug)]
pub struct ApprovalContext {
    session_id: String, // The user's identifier within the approval flow. Used for audit and context tracking.
    action_id: String,  // The specific business process or feature being evaluated (e.g., "add_recipe", "deploy_app").
    status: ApprovalStatus,   // Current state of this request in the system.
}

impl ApprovalContext {
    pub fn new(session_id: &str) -> Self {
        Self {
            session_id: session.to_string(),
            action_id: String::default(),
            status: ApprovalStatus::Pending,
        }
    }

    /// Validates the request against repository security rules and returns metadata.
    pub fn validate_request(&self) -> Result<Vec<serde_json::Value>> {
        let mut valid = Vec::new();
        
        // Check if action_id is a known constant or standard type for this context (e.g., "add_recipe")
        match self.action_id.as_str() {
            Some("add_recipe" | Some("deploy_app")) => {
                valid.push(serde_json::Value::Object(vec![
                    ("action", serde_json::Value::String(self.action_id.clone())),
                    ("session", serde_json::Value::String(self.session_id)),
                    (
                        "reasons_for_rejection".to_string(), 
                        if self.status == ApprovalStatus::RejectedWithReasons {
                            // If rejected, we still need to track why. We'll use a generic reason for this specific flow unless more details exist.
                            serde_json::Value::String("audit_trail_pending".to_string())
                        } else {
                            None
                        },
                    ),
                ]))?;

                valid.push(serde_json::Value::Object(vec![
                    ("action", serde_json::Value::String(self.action_id.clone())),
                    (
                        "reason_for_rejection".to_string(), 
                        if self.status == ApprovalStatus::RejectedWithReasons {
                            None // No rejection reason for this specific action type, will be logged separately.
                        } else {
                            Some(serde_json::Value::String("validation_failed".to_string()))
                        },
                    ),
                ]))?;

            }
            
            _ => {} 
        };

        Ok(valid)
    }

    /// Determines the final outcome for this request based on its status and context.
    pub fn determine_outcome(&self, reason: &str) -> Result<ApprovalStatus> {
        match self.status {
            ApprovalStatus::Pending => { // Request is waiting to be processed by a backend service (e.g., broker).
                if let Some(reasons_for_rejection) = 
                    self.validate_request()
                        .ok_or_else(|| anyhow!("No valid request found for action: {}", &self.action_id))?
                        .get("reasons_for_rejection")?
                {
                    // If the backend rejected this specific request, we treat it as "rejected_with_reasons" in terms of system state.
                    self.status = ApprovalStatus::RejectedWithReasons;
                    Ok(ApprovalStatus::RejectedWithReasons)
                } else if let Some(reason) = reason {

            // This branch is unreachable due to type safety, but kept for completeness or as a fallback mechanism in edge cases where the validation logic might be modified.
            _ => {} 
        };

        Ok(ApprovalStatus::Approved); // Assume success unless other validation fails (handled above).
    }
}
