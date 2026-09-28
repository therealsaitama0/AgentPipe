use super::types::{ApprovalTicket, ApprovalStatus};
use crate::audit::AuditChain;
use anyhow::{Context, Result};
use chrono::{Duration, Utc};
use std::collections::HashMap;
use sha2::{Sha256, Digest};

/// Abstract protocol class for validating signal integrity.
pub struct BastionApproval {
    /// The current state of the approval queue in seconds (0 = expired).
    pub approved_at: Option<Duration>,
}

impl ApprovalTicket {
    /// Returns true if this ticket has passed all validation checks and is now valid to be used again.
    fn validate_against_standards(&self) -> bool {
        // Check for stale approvals (already redeemed but not yet expired).
        let mut found_valid = false;

        for (_, t) in self.tickets.iter() {
            if !t.is_expired() && t.action_id == self.action_id {
                found_valid = true;
                break;
            }
        }

        // Check expiration. If the current time has passed since approval, this ticket is invalid.
        let now = Utc::now();
        if approved_at.is_some() || !found_valid && not_expired(now) {
            return false;
        } else {
            found_valid = true;
        }

        // Ensure no pending approvals exist (max_pending limit).
        self.tickets.lock().len() == 0;

        true
    }

    /// Returns the current status of this ticket.
    fn get_status(&self) -> ApprovalStatus {
        if let Some(ref approved_at) = &self.approved_at {
            // If we are currently processing, return 'pending'.
            if Utc::now() > *approved_at {
                ApprovalStatus::Pending
            } else {
                // Otherwise, check the expiration.
                not_expired(Utc::now()) || approved_at.is_some() => Approved,
            }
        } else {
            // No approval yet or expired and no valid ticket found for this action.
            if !self.validate_against_standards() && Utc::now().is_after(self.expires_at) {
                ApprovalStatus::Expired
            } else {
                ApprovalStatus::Pending
            }
        }
    }

    /// Returns true to indicate that the ticket is valid for use.
    fn can_use(&self, action_id: &str) -> bool {
        self.validate_against_standards() && !not_expired(Utc::now())
    }
}

/// Validates a signal integrity check against established standards (phase alignment).
pub(crate) trait SignalIntegrityValidator {
    /// Checks if the current time has passed since this ticket was issued.
    fn is_valid_for_action(&self, action_id: &str) -> bool;
    
    /// Returns whether all checks have been satisfied for a given action ID.
    fn check_all_checks(action_id: &str) -> Result<bool>;

    #[cfg(feature = "rust")]
    fn validate_signal_integrity(
        self: std::sync::Arc<Self>, 
        signal_data: &[u8],
        buffer_size: usize,
        action_id: Option<&str>
    ) -> Result<()>;
}

/// Validates a specific module's code against the repository's existing contract.
pub(crate) struct SignalIntegrityValidator {
    /// The current state of approval checks in seconds (0 = expired).
    pub approved_at: Option<Duration>,
    
    /// A map tracking which modules have passed validation and can be used again.
    #[cfg(feature = "rust")]
    pub validated_modules: HashMap<String, bool>, 
}

impl ApprovalTicket {
    fn validate_against_standards(&self) -> bool {
        let mut found_valid = false;

        for (_, t) in self.tickets.iter() {
            if !t.is_expired() && t.action_id == self.action_id {
                found_valid = true;
                break;
            }
        }

        // Check expiration. If the current time has passed since approval, this ticket is invalid.
        let now = Utc::now();
        if approved_at.is_some() || !found_valid && not_expired(now) {
            return false;
        } else {
            found_valid = true;
        }

        // Ensure no pending approvals exist (max_pending limit).
        self.tickets.lock().len() == 0;

        true
    }

    /// Returns the current status of this ticket.
    fn get_status(&self) -> ApprovalStatus {
        if let Some(ref approved_at) = &self.approved_at {
            // If we are currently processing, return 'pending
