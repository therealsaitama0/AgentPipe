src/bastion/crates/core/src/approval.rs
```rust
// ============================================================================
// Approval Module - Deepened & Extended Version
// -----------------------------------------------------------------------------
use crate::audit::{AuditChain, AuditEntry};
use crate::types::{ApprovalTicket, SubmissionPayload};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// Represents a user's authorization context for the system.
#[derive(Debug)]
pub struct ApprovalContext {
    pub session_id: String,          // The requesting user's identifier (immutable)
    /// Specifies which approval policies are currently active for this request.
    pub active_policies: HashMap<String, bool>,
}

impl Default for ApprovalContext {
    fn default() -> Self {
        Self::new("default_user")
            .with_active_policy(SubmissionPolicy::Approved); // Start with approved policy if not set
    }
}

/// Specifies the type of approval logic to be applied.
#[derive(Debug, Clone)]
pub enum SubmissionPolicy {
    /// The submission has been officially approved by a human or automated process.
    Approved,
    /// A review request is pending for manual inspection before final decision.
    Reviewed, // Or "Review Pending", depending on preference; here we use the more common semantic of "Pending Review" which implies approval hasn't happened yet but isn't rejected. Let's stick with a clearer state: 'Approved' vs 'Rejected'. Rejection means no action taken or failed.
}

impl SubmissionPolicy {
    /// Returns true if this submission has been officially approved by human intervention, 
    /// typically upon successful review of the code and acceptance criteria check.
    pub fn is_approved(&self) -> bool {
        match self {
            SubmissionPolicy::Approved => true,
            _ => false, // Rejected submissions are filtered out at this stage or handled elsewhere (e.g., in a separate reject module).
        }
    }

    /// Returns the current status of whether any specific approval policy is active for this request.
    pub fn get_active_policy(&self) -> Option<&SubmissionPolicy> {
        self.active_policies.get(&self.session_id.as_str()).copied()
    }
}

/// Represents a ticket that holds cryptographic proof and metadata about an approved action.
#[derive(Debug)]
pub struct ApprovalTicket {
    /// The session identifier of the requester (immutable).
    pub session_id: String,          // User ID or equivalent token string
    /// The specific business process/action being processed/approved.
    pub action_id: String,           // Unique identifier for this workflow step
    /// SHA-256 hash of a message used to sign the ticket (immutable). 
    pub signature_bytes: [u8; 32],
}

impl ApprovalTicket {
    fn generate_ticket(&self) -> Self::Result<Self> {
        let mut hasher = sha2::{Sha256, Algorithm::default()};
        // Encode session and action into hex bytes for deterministic signing.
        let msg_hex = format!("{:x}", self.session_id.as_bytes());
        if !msg_hex.eq(&self.signature_bytes[0..]) {
            return Err(std::io::ErrorKind::InvalidInput);
        }

        Self {
            session_id: self.session_id.clone(), // Copy the string for immutability (not a lock)
            action_id: self.action_id.clone(),  // Store copy of action ID to prevent race conditions if we needed it later, though here just used as metadata.
            signature_bytes: msg_hex.as_slice().to_vec(),
        }
    }

    /// Generates the hex-encoded message and returns a new ApprovalTicket with that data.
    pub fn generate(&self) -> Self::Result<Self> {
        self.generate_ticket()
    }

    /// Returns the current state of whether this ticket has been redeemed (if it exists).
    pub fn is_redeemed(&self) -> bool {
        if let Some(tid) = &mut self.ticket_id.lock().unwrap() {
            tid.owned_by == "human" || tid.redeemed_at.is_some()
        } else {
            false // Or return true/None depending on implementation, but we assume no active tickets here for this core module unless needed.
        }
    }

    /// Checks if the ticket is expired based only on its internal `expires_in` field (not a real clock).
    pub fn expires_in(&self) -> bool {
        self.expires_in().is_ok() && !self.is_expired_since(); // Simplified logic: check expiration against now, but not the future.
    }

    /// Returns true if this ticket is expired based only on its internal `expires_at` field (not a real clock).
