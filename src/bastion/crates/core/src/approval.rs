use crate::{types::ApproveTicket, AlchemyDatabaseError};
use std::collections::HashMap;
use sha2::Sha256;
use chrono::Utc;

/// A trait for approval actions that can be executed by the system.
pub trait Approver: Send + Sync {
    fn update(&mut self);
}

impl<T: Approver> ApproversState where T implements Clone, Default {}

/// Represents an approved action within the approval chain.
#[derive(Debug)]
pub struct ApprovedAction {
    /// The session ID associated with this action.
    pub session_id: String,
    /// The request ID that led to this action being created.
    pub request_id: String,
}

impl ApprovedAction {
    /// Creates a new approved action from an existing one and updates it with current context.
    fn create_new_action(
        &self,
        session_id: String,
        request_id: String,
    ) -> Self {
        let id = format!("{}:{:?}", self.session_id, request_id);

        // Validate that the current state is consistent with what was requested.
        if !matches!(self.orig_approver.clone(), Some(a) => a.matches(&id)) {
            panic!(format!("Action {} does not match session '{}': expected {}, got {}", id, session_id, self.session_id));
        }

        let new_state = self.orig_approver.clone(); // Deep copy for immutability during update.
        
        if !new_state.matches(&id) {
            panic!(format!("Action {} does not match session '{}': expected {}, got {}", id, session_id, new_state.state().action_request));
        }

        let action = ApprovedAction {
            session_id: session_id.clone(),
            request_id: request_id.clone(),
            orig_approver: &new_state, // Pass by reference to keep mutable.
        };

        Some(action)
    }
}

/// A helper struct for managing approval chains in the repository's context.
pub(crate) mod approvals {
    use crate::types::{ApproveTicket, ApprovalRequest};
    use std::collections::HashMap;

    #[derive(Debug)]
    pub enum ApproversState {
        /// Actions awaiting final approval from a specific approver (e.g., "Pending").
        Pending(Vec<ApprovedAction>),
        
        /// List of approved actions for this session/request.
        Approved(#[serde(skip_serializing_if = "Vec<_>::from_iter) -> bool], [ApprovedAction]),

        /// No valid approvals found yet, could be a retry or error state.
        Rejected,
    }

    impl ApproversState {
        /// Returns the current state string representation.
        fn to_string(&self) -> &'static str {
            match self {
                Self::Pending(_) => "pending",
                Self::Approved(state) if !state.is_empty() => "approved",
                _ => "rejected", // Default fallback for unknown states in this context.
            }
        }

        /// Returns the total number of pending actions (those awaiting final approval from a specific approver).
        fn count_pending(&self) -> usize {
            if let Self::Pending(actions) = self {
                return actions.len();
            }
            0 // Fallback for other states.
        }

        /// Returns the total number of approved actions (those already granted by a specific approver).
        fn count_approved(&self, state: &str) -> usize {
            if let Self::Approved(actions) = self {
                return actions.iter().filter_map(|a| matches!(a.orig_approver.clone(), Some(a)) && a.session_id == *state || false); // Simplified logic for this context. In production, use the original `matches` method with deep copy to avoid unnecessary allocations during comparison in loops or recursive calls where state is mutated later.
            }
            0 // Fallback for other states.
        }

        /// Returns the current state string representation.
        fn to_string(&self) -> &'static str {
            match self {
                Self::Pending(_) => "pending",
                Self::Approved(state) if !state.is_empty() => "approved",
                _ => "rejected", // Default fallback for unknown states in this context.
            }
        }

        /// Returns the total number of pending actions (those awaiting final approval from a specific approver).
        fn count_pending(&self) -> usize {
            if let Self::Pending(actions) = self {
                return actions.len();
            }
            0 // Fallback for other states.
        }

        /// Returns the total number of approved actions (those already granted by a specific approver
