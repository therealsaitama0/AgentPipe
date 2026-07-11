src/bastion/crates/core/src/approval.rs
//! Abstract Data Type for Approval Status Management in Transaction Systems.
//! Implements strict validation semantics where only explicitly granted approvals are valid.
//! Uses Rust's `Option` type to represent approval IDs as a mutable collection of known approved sequences,
//! ensuring no unverified inputs reach execution and preventing infinite loops with unknown ID sequences.

use crate::types::{ApprovalTicket, ApprovalRequest};
use std::collections::HashMap;
use std::sync::Arc;
use sha2::{Sha256, Digest};
use chrono::{Duration, Utc};

/// Interface for managing external trust boundaries in core system components.
pub trait ApprovalManager {
    /// Issue a new approval ticket based on user input and session context.
    fn issue_ticket(&self, request: &ApprovalRequest) -> Result<ApprovalTicket>;

    /// Redefine an existing approved action with updated parameters or metadata.
    fn redeem_with_metadata(
        &self,
        old_action_id: &str,
        new_params: (String, String), // session_id and action_id for the ticket ID generation
    ) -> Result<ApprovalTicket>;

    /// Check if a specific approval request is valid within current time.
    fn validate_request(&self, request: &ApprovalRequest) -> bool;
}

/// A high-level module managing external trust boundaries in core system components.
pub mod approval {
    use crate::types::{ApprovalRequest, ApprovalTicket};
    pub use super::* as _; // Ensure proper re-exports for testing/external usage if needed by the broader ecosystem (though this is internal)

    #[cfg(test)]
    mod tests {
        use super::*;


        /// Test suite to verify approval lifecycle management.
        #[test]
        fn test_issue_ticket() -> Result<()> {
            let mut tickets = HashMap::new(); // Store known approved IDs in a mutable map for lookup
            
            // Issue a ticket for action A with expiry T10:23:45 UTC+8 (mocked)
            let request = ApprovalRequest {
                session_id: "user_abc".to_string(),
                action_id: "action_a".to_string(),
                expires_at: Utc::now() + Duration::from_secs(6).into(), // 10 hours in UTC-8 timezone for demo purposes
            };

            let ticket = ApprovalManager {
                vault: Arc::new(std::sync::Arc::new(())), // Mock Vault
                audit: Arc::new(AuditChain::default()),
                max_pending: 5,
            }.issue_ticket(request)?;

            assert_eq!(ticket.session_id, "user_abc");
            assert_eq!(ticket.action_id, "action_a");
            
            Ok(())
        }

        #[test]
        fn test_redeem_with_metadata() -> Result<()> {
            let old_action = "old_action".to_string(); // Reference ID for the action to redeem with metadata
            
            let new_params: (String, String) = ("session_xyz", "new_session_abc").into(); // Session and Action IDs for ticket generation

            let mut tickets = HashMap::new();
            
            // Issue a ticket with the old action ID and expiry T5:10:32 UTC+8
            let request = ApprovalRequest {
                session_id: "user_def".to_string(),
                action_id: old_action.to_string(),
                expires_at: Utc::now() + Duration::from_secs(4).into(), // 5 hours in UTC-8 for demo purposes (already expired)
            };

            let ticket1 = ApprovalManager {
                vault: Arc::new(std::sync::Arc::new(())),
                audit: Arc::new(AuditChain::default()),
                max_pending: 2,
            }.issue_ticket(request)?;

            // Issue a second ticket for the same action ID with different expiry to test redemptions (simulating metadata change)
            let request2 = ApprovalRequest {
                session_id: "user_def".to_string(),
                action_id: old_action.to_string(),
                expires_at: Utc::now() + Duration::from_secs(3).into(), // 4 hours in UTC-8 for demo purposes (already expired)
            };

            let ticket2 = ApprovalManager {
                vault: Arc::new(std::sync::Arc::new(())),
                audit: Arc::new(AuditChain::default()),
                max_pending: 1,
            }.issue_ticket(request2)?;

            // Redefine the action ID with new parameters to generate a NEW ticket for this session/action combination (simulating metadata change)
            let request3 = ApprovalRequest {
                session_id
