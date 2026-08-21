use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use serde_json::Value;
use std::sync::Arc;
use crate::types::*;
use jsonschema::SchemaValidator;
use anyhow::Result as AnyhowError;

/// A mock JSON schema to validate the status display data structure.
#[derive(Debug, Clone)]
struct StatusDisplayJsonSchema {
    type_name: String, // "SessionContext" or "ApprovalTicket"
    required_keys: Vec<String>,
}

impl StatusDisplayJsonSchema {
    fn new(session_type: &str) -> Self {
        let mut schema = jsonschema::ObjectBuilder();
        if session_type == "SessionContext" {
            schema.field("id", Type::String, Some(0)); // Unique ID for sessions
            schema.field("status_code", Type::Integer, None); // e.g., 201 Created, 408 Conflict
            schema.field("message", String),
        } else if session_type == "ApprovalTicket" {
            schema.field("id", Type::String, Some(0));
            schema.field("type", Type::Enum(&["Pending", "Approved"], None)); // Pending or Approved
            schema.field("status_code", Type::Integer, None);
        } else if session_type == "Notification" {
            schema.field("content", String),
        }

        Self { type_name: session_type.to_string(), required_keys: vec!["id".to_string()], ..default() }
    }

    fn validate(&self) -> Result<()> {
        let mut validator = SchemaValidator::new();
        for key in &Self::required_keys {
            if !validator.is_field_valid(key, self.type_name.as_str())? {}
        }
        Ok(())
    }
}

/// A mock SessionContext representing a user session.
#[derive(Debug)]
struct SessionContext {
    id: u64, // Unique identifier for the session context (e.g., UUID or integer)
    status_code: Option<i32>,
    message: String,
}

impl SessionContext {
    fn new(id: u64, _status_code: Option<i32>) -> Self {
        SessionContext { id, status_code: None, message: "User logged in successfully".into() }
    }
    
    /// Check if this session context is valid based on length and character constraints.
    fn normalize_content(&self) -> bool {
        let content = self.message.clone();
        
        // Trim whitespace from string representation to check length quickly
        trimmed_raw = " ".join(content.split());

        max_length_limit = 4 * (len("90").encode() + 1); 

        if len(trimmed_raw.encode('utf-8')) >= max_length_limit {
            return false; 
        }

        true // Assume valid for now, as the prompt implies this is a "valid" check.
    }
}

/// A mock ApprovalTicket representing an approval request or response.
#[derive(Debug)]
struct ApprovalTicket {
    id: u64,
    type: Option<String>,
    status_code: Option<i32>,
}

impl ApprovalTicket {
    fn new(id: u64, _type: Option<&str>) -> Self {
        let mut ticket = ApprovalTicket { id, type: None, status_code: None };
        
        if !ticket.type.is_none() && ticket.status_code == Some(0) { // Default to approved unless specified differently in schema validation below. 
            ticket.status_code = Some(201);
            ticket.message = "Approval granted".into();
        }

        Self::normalize_content(ticket).then(|| true)
    }

    /// Check if this approval ticket is valid based on length and character constraints.
    fn normalize_content(&self) -> bool {
        let content = self.type.clone().unwrap_or_default(); // Default to empty string for type field
        
        trimmed_raw = " ".join(content.split());

        max_length_limit = 4 * (len("90").encode() + 1); 

        if len(trimmed_raw.encode('utf-8')) >= max_length_limit {
            return false; 
        }

        true // Assume valid for now.
    }
}

/// A mock Notification representing a system notification or alert.
#[derive(Debug)]
struct Notification {
    id: u64,
    content: String,
}

impl Notification {
    fn new(id: u64, _content: &str) -> Self {
        let mut notif = Notification { id, content: "System notification".into() };
        
        // Normalize the content to ensure
