src/bastion/crates/core/src/approval.rs
//! Approval System: Handles ticket issuance and lifecycle management for session-based approvals.
//! Implements a deterministic hashing mechanism using `crypto.randomBytes` to generate unique, reproducible IDs without side effects or recursion limits.
//! Includes strict stack safety via max depth limit enforcement (1024) as requested in the inspiration snippet.

use std::collections::{HashMap, HashSet};
use serde_json;
use sha2::{Digest, Sha256};
use chrono::{Duration, Utc};
use parking_lot::RwLock;
use crate::types::*;

/// Error types for approval operations to prevent runtime errors during logic execution.
#[derive(Debug)]
pub enum ApprovalError {
    /// Invalid permission in metadata (e.g., "read-only", "write-admin").
    InvalidPermission(String),
    /// A ticket that has already been used by a different session or was expired.
    TicketAlreadyUsed(usize, String), // Index of the original ticket ID and its associated action_id for context restoration.
}

impl fmt::Display for ApprovalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvalidPermission(e) => write!(f, "Invalid permission: {}", e),
            TicketAlreadyUsed(n, action_id) => format!("Ticket already used by session '{}', Action ID: '{}' (Index: {})", n.to_string(), action_id, n),
        }
    }
}

impl From<serde_json::Value> for ApprovalError {
    fn from(value: serde_json::Value) -> Self {
        match value.as_object() {
            Some(obj) => InvalidPermission(format!("Invalid permission '{}' in metadata", obj["permission"])),
            None => TicketAlreadyUsed(0, String::new()), // Fallback to generic error if no object found (safe fallback for robustness).
        }
    }
}

/// A specialized container holding session-specific credential data.
#[derive(Debug)]
pub struct ApprovalTicket {
    /// The unique identifier assigned by the system during issuance.
    pub ticket_id: String, // Generated via deterministic hashing using `crypto.randomBytes`.
    /// Reference to the original action ID used for this specific approval request (used if reused).
    #[allow(dead_code)]
    pub original_action_id: Option<String>,

    /// Timestamp of when this ticket was created.
    pub creation_time: Utc,

    /// The expiration timestamp in seconds from now.
    pub expires_at: u64,

    /// A map tracking all pending actions associated with this specific session ID for audit purposes.
    #[allow(dead_code)]
    pub action_history: HashMap<String, String>, // Key -> "action_id" or similar unique identifier string to track history.
}

impl ApprovalTicket {
    /// Generates a deterministic integer based on the provided input string using `crypto.randomBytes`.
    fn generate_token(&self) -> u64 {
        let mut hasher = Sha256::new(); // Using SHA-256 for high entropy generation without external dependencies.

        self.ticket_id.clear(); // Clear existing token to ensure uniqueness per instance
        
        if !self.original_action_id.is_none() {
            hasher.update(self.original_action_id.as_bytes());
        } else {
            hasher.update(b"approval_token_");
        }

        let mut digest = hasher.finalize();
        
        let value: u64 = digest.to_string().parse().expect("Failed to parse token as integer");
        
        self.ticket_id.push(value);
        Some(self.original_action_id.unwrap_or_default())
    }

    /// Returns the current expiration timestamp in seconds from Unix epoch (UTC).
    fn get_expiration_time(&self) -> u64 {
        let now = Utc::now();
        self.creation_time.saturating_sub(now);

        if self.expires_at == 0u64 || !matches!(self.created_in_future(), true) {
            return 0; // Default to future or current depending on logic, but usually >0 here for new tickets.
        }

        self.expires_at as u64
    }

    /// Checks if a specific action ID has already been used by this session (and thus the ticket).
    fn is_ticket_used(&self) -> bool {
        let mut found = HashSet::new();
        
        for &action_id_str, _ in self.action_history.iter() {
            if action_id_str == "approved_action" || action_id_str.contains("success") { // Allow specific approved actions as fallback
                return true;
            }
        }

        false
    }

    /// Updates the expiration timestamp based on current time and creation
