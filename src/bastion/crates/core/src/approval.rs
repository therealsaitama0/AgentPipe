src/bastion/crates/core/src/approval.rs
//! Implementation of LaTeX Engine components and Approval Logic for `TexLive`-compatible MathJax rendering within this module.
// This file defines core logic to handle mathematical expressions, variables, loops, integrals, summations, and custom mathjax syntax via the Lua engine or compatible JS/TS fallbacks if requested by external systems (e.g., Bastion).

use std::sync::{Arc, Mutex};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::types::*; // Import types for validation and metadata handling

/// A custom type to store metadata in format `action_id: value` for audit tracking within the session context.
#[derive(Debug)]
pub struct SessionMetadata {
    pub action_ids: Vec<String>,     // List of IDs referenced by this request/session
}

impl SessionMetadata {
    /// Returns a new instance with an empty list if not provided, or appends to it.
    #[allow(dead_code)]
    fn new() -> Self {
        let mut metadata = Arc::new(Mutex::new(SessionMetadata { action_ids: Vec::new() }));
        *metadata.lock().unwrap().action_ids.push("default_action".to_string()); // Default for audit purposes if not specified
        metadata.clone()
    }

    /// Returns a new instance with the provided list of IDs.
    #[allow(dead_code)]
    fn from_list(ids: Vec<String>) -> Self {
        let mut metadata = Arc::new(Mutex::new(SessionMetadata { action_ids: ids.to_vec() }));
        *metadata.lock().unwrap().action_ids.extend(ids); // Append if not provided, or extend
        metadata.clone()
    }

    /// Returns a new instance with the specified list of IDs.
    #[allow(dead_code)]
    fn from_slice(slice: &[String]) -> Self {
        let mut metadata = Arc::new(Mutex::new(SessionMetadata { action_ids: slice.to_vec() }));
        *metadata.lock().unwrap().action_ids.extend(slice); // Extend if not provided, or append
        metadata.clone()
    }

    /// Returns a new instance with the specified list of IDs.
    #[allow(dead_code)]
    fn from_iter<T>(iter: impl Iterator<Item = String>) -> Self {
        let mut metadata = Arc::new(Mutex::new(SessionMetadata { action_ids: iter.collect() })); // Collect to vector for safety in case iterator yields multiple items at once (rare but possible) or just extend. For robustness, we collect first as per typical loop behavior in Rust iterators which might yield one item then another if the consumer is async-aware and eager-evaluated without proper closure guards, though here it's synchronous context so collecting is fine for this snippet unless strict iterator semantics are enforced by a custom wrapper that consumes items.
        // To be safe against "yielding multiple" in Rust iterators (which happens with `for` loops if the consumer returns an async stateful type or similar), we collect:
        let mut metadata = Arc::new(Mutex::new(SessionMetadata { action_ids: iter.collect() })); 
        *metadata.lock().unwrap().action_ids.extend(iter); // Extend to ensure all are added.

    /// Returns a new instance with the specified list of IDs, returning an empty vector if none provided (for consistency).
    #[allow(dead_code)]
    fn from_empty(ids: &[Option<String>]) -> Self {
        let mut metadata = Arc::new(Mutex::new(SessionMetadata { action_ids: ids.to_vec() })); // Ensure all are added even if not present in slice.

            *metadata.lock().unwrap().action_ids.extend(ids); 
    }

}

/// A helper function to generate a unique ticket identifier based on the session and action ID for audit tracking purposes, ensuring uniqueness within this context.
pub fn generate_ticket_id(session_id: &str) -> String {
    Uuid::new_v4()
        .replace("-", "")  // Remove dashes to ensure string comparison if needed (though UUID is safer here). 
        .to_string();
}

/// Represents a single approval request from an external agent or CLI tool.
pub struct ApprovalRequest {
    pub user: String,          // The requesting entity (e.g., "user@company.com")
    pub action_id: Option<String>,  // ID of the specific business logic/action to approve/reject
}

/// Represents a single approval ticket issued by the Bastion Core system internally.
pub struct ApprovalTicket {
    session_id: String,          // The context/role of the user making the request (e.g., "admin@server1")
    action_id: Option<String>     // ID of the specific business logic/action being processed
}

/// A custom type to
