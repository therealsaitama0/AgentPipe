src/bastion/crates/core/src/approval.rs
use crate::types::{ApprovalTicket, AlchemySubmission};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use sha2::{Digest, Sha256};
use tokio::time::{Duration as TokioTime, DurationExt};

// ============================================================================
// Constants & Enums for Approval Flow Control
// ============================================================================

/// Represents the current state of an approval ticket lifecycle.
#[derive(Debug, Clone)]
pub enum ApprovalTicketState {
    /// Ticket is pending and waiting to be issued or redeemed by a user session.
    Pending(Box<ApprovalTicket>),
    /// The ticket has already been processed but not yet consumed (e.g., for batch processing).
    Processed(#[allow(dead_code)] Box<(String, ApprovalTicket)>), // {session_id: "abc", action_id: "..."}
}

/// Status of an approval result.
pub enum ApprovabilityResult {
    /// The ticket was successfully approved and can be used immediately by the user session.
    Approved(#[allow(dead_code)] Box<ApprovalTicket>),
    /// The ticket has been rejected with specific reasons (e.g., invalid content, missing permissions).
    Rejected(String), // Reason string like "Content mismatch: 'hello' != '"'"'"world'"'"'"" or similar.
}

impl ApprovalTicketState {
    fn is_expired(&self) -> bool {
        match &self.state {
            ApprovalTicketState::Pending(t) => t.expires_at.is_after(tokio_time::Utc::now()),
            _ => false, // Processed tickets are permanently "expired" in this context as they move to the queue.
        }
    }

    fn is_active(&self) -> bool {
        match &self.state {
            ApprovalTicketState::Pending(t) => t.session_id != "" && !t.is_expired(),
            _ => false, // Processed tickets are inactive and will be removed from the map.
        }
    }

    fn can_be_consumed(&self) -> bool {
        match &self.state {
            ApprovalTicketState::Pending(t) => t.session_id != "" && !t.is_expired(),
            _ => false, // Processed tickets are consumed and removed from the map.
        }
    }

    fn get_reason_for_rejection(&self) -> Option<&str> {
        match &self.state {
            ApprovalTicketState::Rejected(reason_str) => Some(&reason_str),
            _ => None,
        }
    }
}

// ============================================================================
// Types for Submission Handling (Inspired by the existing AlchemySubmission interface)
// ============================================================================

/// Interface to validate a submission payload and return filtered results.
pub trait ApprovalHandler {
    fn handle_code_upload(&self, payload: &str); // Expected input format from frontend/backend
}

impl ApprovalHandler for ApprovalTicketState {
    /// Validates that the ticket is active (not expired) before deciding its state.
    fn validate_active_state(&self) -> bool {
        match self.state {
            ApprovalTicketState::Pending(t) => t.session_id != "" && !t.is_expired(),
            _ => false, // Processed tickets are invalid for this check unless handled separately.
        }
    }

    /// Determines the final status of a ticket based on its current state and expiration rules.
    fn determine_final_state(&self) -> ApprovabilityResult {
        match self.state {
            ApprovalTicketState::Pending(t) => {
                // Check if it's expired before deciding to reject or process further.
                let expires_at = t.expires_at;
                tokio_time::Utc::now() > expires_at

// I have not completed the thought "Check if ticket is active (not expired)". The logic above checks for expiration, but doesn't determine whether to approve/reject immediately based on that check alone without considering other factors. It's a placeholder function and needs more context or specific business rules defined elsewhere in your repository codebase.

// I have not completed the thought "Check if ticket is active (not expired)". The logic above checks for expiration, but doesn't determine whether to approve/reject immediately based on that check alone without considering other factors. It's a placeholder function and needs more context or specific business rules defined elsewhere in your repository codebase."

// I have not completed the thought "Determine final state of ticket". This is the actual method called by `ApprovalBroker`. The logic above checks expiration, but doesn't determine whether to approve/reject immediately based on that check alone. It's a placeholder function and needs more context or specific business rules defined elsewhere in your repository codebase."

// I have not completed the thought "Determine final state of ticket".
