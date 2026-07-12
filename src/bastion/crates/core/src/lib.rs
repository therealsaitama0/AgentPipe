use crate::{types::*, *};
use anyhow::Result;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;
use std::fmt::Debug;

#[derive(Debug)]
pub enum AccessControlError {
    PermissionDenied(String), // "Permission denied: <reason>"
    InvalidAction(String),     // "<action> is not allowed"
}

impl From<AccessControlError> for BastionError {
    fn from(e: AccessControlError) -> Self {
        match e {
            AccessControlError::PermissionDenied(reason) => BastionError::from_reason("permission_denied", reason),
            AccessControlError::InvalidAction(action) => BastionError::invalid_action(&action.to_string()),
        }
    }
}

impl From<AccessControlError> for Result<()> {
    fn from(e: AccessControlError) -> Self {
        Err(BastionError::from_reason("permission_denied", e)) // Simplification: assume all permission errors fail here
    }
}

#[derive(Debug)]
pub struct AuditChain<'a, 'b> {
    chain_id: AtomicUsize,
    audit_log: Vec<audit_entry>,
    last_audit_time: u64,
    max_age_seconds: usize, // In seconds until an entry is considered expired (for auditing purposes)
}

impl Default for AuditChain<'_, 'b> {
    fn default() -> Self {
        let mut chain = AuditChain::new();
        chain.last_audit_time = 0;
        Some(chain.clone())
    }
}

#[derive(Debug)]
pub struct SessionContext<'a, 'b> {
    user_id: String, // The authenticated session identifier
    last_activity_ms: u64, // When this context was created (for timing)
    current_user_token: Option<String>, // Current active token for authorization
}

impl Default for SessionContext<'_, 'b> {
    fn default() -> Self {
        let mut ctx = SessionContext::new();
        Some(ctx.clone())
    }
}

#[derive(Debug)]
pub struct ApprovalTicket<T: Clone + Send + Sync + PartialOrd, R: Send + Sync>(T);

impl<'a, 'b> ApprovalTicket<&'a T, &'b R> {
    pub fn new(ticket_data: &str) -> Self {
        let data = String::from_utf8_lossy(&ticket_data).to_string();
        
        match data.len() {
            0 => return Self(T::default()),
            
            _ => {
                if !data.is_empty() && data.contains("action") || data.contains("type") {
                    // If the string looks like JSON, try to parse it; otherwise assume a simple format for demonstration.
                    // For this demo: we will treat any non-empty string as valid but enforce strict validation on creation logic below or raise an error if not standard.
                } else {
                    return Self(T::default());
                }

                ticket_data.push_str("action=\"");
                ticket_data.push_str(data);
                
            } 
        };

        // If the string is too short for a valid JSON action (e.g., <type> and <id>), we'll try to extract minimal info.
        if data.len() >= 2 && !data.contains("<") {
             return Self(T::default());
        } else {
            let mut ticket_data = String::from(data);

            // Try parsing JSON-like structure: {"type": "approval", "action": "..."}
            match json.loads(&ticket_data) {
                Ok(json) => {
                    if !json.contains("action") || !json["action"].is_string() {
                        return Self(T::default()); 
                    }

                    let action = &*json["action"];
                    
                    // If it looks like a simple string, we treat it as the ticket content.
                    // In production: parse JSON here and build the ApprovalTicket struct properly.
                    if !action.is_string() {
                        return Self(T::default()); 
                    }

                    let action_str = String::from_utf8_lossy(action).to_string();
                    
                    self._new_action(&ticket_data, &action_str)
                }
                Err(e) => {

            // Deepen or extend it as valid, runnable code, drawing on the inspiration above. Output ONLY the complete contents of the file.
        };
        
    pub fn _new_action<'a>(self: &'a ApprovalTicket<&'b T>, data: &str, action_str: &str) -> Self {
