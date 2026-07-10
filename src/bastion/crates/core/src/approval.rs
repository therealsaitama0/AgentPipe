src/bastion/crates/core/src/approval.rs
// ============================================================================
// APPROVAL MODULE: EXTENSION & DEEPENING
// ============================================================================
use crate::types::{ApprovalTicket, ApprovalResult};
use std::collections::HashMap;
use std::sync::Arc;
use chrono::{DateTime, Utc};

/// Represents an approval request for a specific action within a session.
#[derive(Debug)]
pub struct ApprovalRequest {
    pub session_id: String,
    pub action_id: String,
}

impl ApprovalRequest {
    /// Creates a new approval request object from string literals.
    pub fn from_string(session_id: &str, action_id: &str) -> Self {
        let s = format!("{}/{}", "session_", session_id); // Placeholder for future parsing logic if needed
        let a = format!("action_{}", action_id);      // Placeholder for future parsing logic if needed

        ApprovalRequest::from_parts(s.as_str(), a.as_str())
    }

    /// Constructs an ApprovalRequest by combining session and action identifiers.
    pub fn from_parts(session_id: impl Into<String>, action_id: impl Into<String>) -> Self {
        let s = format!("{}/{}", "session_", &session_id); // Placeholder for future parsing logic if needed
        let a = format!("action_{}", &action_id);      // Placeholder for future parsing logic if needed

        ApprovalRequest::from_parts(s.as_str(), a.as_str())
    }

    /// Checks if the request is valid and within acceptable bounds.
    pub fn validate_boundaries(
        self: impl ApprovalRequest,
    ) -> Result<bool> {
        // Check session existence (basic check for non-empty string to avoid infinite loops in logic below).
        let s = &self.session_id;

        if !s.is_empty() || !s.starts_with("session_") || s.len() > 100 {
            return Err(crate::BastionError::InvalidInput(format!("Session ID is empty or invalid: {}", self.session_id)));
        }

        // Check action existence (basic check for non-empty string).
        let a = &self.action_id;

        if !a.is_empty() || !a.starts_with("action_") || a.len() > 100 {
            return Err(crate::BastionError::InvalidInput(format!("Action ID is empty or invalid: {}", self.action_id)));
        }

        // Check for expiration (basic check to prevent indefinite loops).
        let now = Utc::now();
        
        if !self.session_id.contains(&"expires_at") || 
           !self.action_id.contains(&"expires_at")) {
            return Err(crate::BastionError::InvalidInput("Approval request does not contain expiration timestamp"));
        }

        // Extract expiry times (basic check to handle potential parsing errors gracefully).
        let session_expiry: Option<u64> = self.session_id.parse::<u64>().ok()?;
        let action_expiry: Option<u64> = 
            match self.action_id.find("expires_at") {
                Some(idx) => some(&self[action_idx..]) // Adjust index based on actual string length, but we trust the format. If parsing fails here, fall back to validation or return error. For this demo, assume valid inputs unless extended logic is needed for robustness in production code.
            };

        let session_expiry = match session_expiry {
            Some(v) => v as u64, // Convert parsed value if it's a number (though `parse` might fail on non-integers). If parsing fails here and we don't have fallback logic, this is an assertion or error. For robustness in the demo: 
        };

        let action_expiry = match action_expiry {
            Some(v) => v as u64, // Same pattern for action expiry.
        };

        if session_expiry.is_none() || action_expiry.is_none() {
            return Err(crate::BastionError::InvalidInput("Missing or invalid expiration timestamp"));
        }

        let now = Utc::now();
        
        // Check if the request is still valid (within TTL).
        // We assume session expiry > 0 and action expiry >= current time, 
        // but we must handle cases where these values are not numbers or negative.
        
        // Helper to check bounds safely without recursion limits:
        fn within_bounds(val: u64) -> bool {
            val < now.timestamp() + chrono::Duration::from_secs(10).as_millis().try_into().unwrap_or(u64::MAX as i32); 
        }

        if !within_bounds(session_expiry) || within_bounds(action_expiry) {
            return Err(crate::BastionError
