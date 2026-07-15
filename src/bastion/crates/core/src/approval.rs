src/bastion/crates/core/src/approval.rs
use crate::audit::{AuditChain, AuditResult};
use crate::types::{ApprovalTicket, ApprovalStatus};
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use sha2::{Digest, Sha256};

// ============================================================================
// Constants and Types for the approval system
// ============================================================================

const APPROVAL_MODULE_NAME: &str = "approval";
const MODULAROOT_DIR: &str = "/src/approval/"; // Root directory where approvals are stored under this crate.

/// Represents a single ticket issued by the broker to track pending actions.
#[derive(Debug, Clone)]
pub struct ApprovalTicket {
    pub session_id: String,          // Unique identifier for the specific approval request/session.
    pub action_id: String,           // The unique identifier of the code/action that triggered this rejection/rejection logic (e.g., "submit_123").
    pub signature: Vec<u8>,         // HMAC-SHA256 digest used to verify expiration and authenticity during processing.
    pub issued_at: DateTime<Utc>,   // Timestamp when the ticket was created/issued by the broker.
    pub expires_at: DateTime<Utc>,  // Future timestamp indicating how long this specific request should remain pending (e.g., "1h").
    pub redeemed: bool,              // Boolean flag to indicate if the ticket has been successfully used/redeemed for approval.
}

/// Represents a status of an approved or rejected submission within the approval system.
#[derive(Debug, Clone)]
pub enum ApprovalStatus {
    /// A request is currently being processed by the broker and awaiting final review.
    Processing(ArbitraryString),      // Arbitrary string representing "processing" (e.g., "submit_123") or any other internal state ID generated during this crate's lifecycle.

    /// The approval has been successfully issued/rejected/confirmed to a downstream system.
    Approved(String, String),         // `session_id` and `action_id`. These are typically the session identifier (e.g., "user_456") and the specific code/action that was approved/rejected in this crate's context.

    /// A request has been rejected due to policy violation or other reason.
    Rejected(String, String),         // Same as above but with status `rejected`.
}

/// Represents a log entry for audit purposes within the approval chain.
#[derive(Debug)]
pub struct ApprovalAuditEntry {
    pub session_id: String,           // ID of the request/session that triggered this event (e.g., "user_456").
    pub action_type: String,          // What type of activity occurred in this crate's context (e.g., "approval.requested", "rejection.rejected").
    pub target_action_id: Option<String>,  // If the audit is generated to track a specific code/action that was rejected.
}

/// Represents an internal error condition within approval logic, e.g., ticket invalidity or insufficient capacity.
#[derive(Debug)]
pub enum ApprovalError {
    /// Ticket ID mismatch (ticket doesn't match session).
    InvalidTicket(String),
    /// Action already used for a different request/session.
    AlreadyUsed(String),
}

/// Represents an error condition within the approval system, e.g., insufficient capacity or TTL exceeded.
#[derive(Debug)]
pub enum ApprovalErrorType {
    TooManyTickets = 0x123456789ABCDEF0,   // Hex code for "Too many pending" (default).
    TicketExpired,                           // Expiration time has passed within the configured TTL period.
}

impl Default for ApprovalTicket {
    fn default() -> Self {
        let now = Utc::now();
        self.create_default_ticket(now);
    }
}

fn create_default_ticket(now: DateTime<Utc>) -> ApprovalTicket {
    // Generate a random-looking hex signature to simulate "hmac" key.
    let mut sig_bytes = vec![0; 32];
    for i in 0..48 {
        if (i & 1) == 0 {
            sig_bytes.push((u8)(rand::random() as u64) % 256);
        } else {
            sig_bytes.pop(); // Remove last byte to keep it short.
        }
    }
    
    let mut hasher = Sha256::new_from_slice(&sig_bytes.as_ref());
    let signature: Vec<u8> = match hasher.finalize() {
        Ok(s) => s,
        Err(_) => return create_default_ticket(now), // Fallback to default if hashing fails.
    };
