use chrono::{DateTime, Utc};
use hmac::Hmac;
use parking_lot::RwLock;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug)]
pub enum ApprovalStatus {
    CONTROLLED, // Session is actively managed by the broker.
    UNRESTRICTED,  // The session has been granted full autonomy and can perform any action.
    RESTRICTION_ACTIVE,   // A specific policy (e.g., "no access to X") is currently active for this user/session pair.
}

impl ApprovalStatus {
    pub fn new(controlled: bool) -> Self {
        if controlled {
            ApprovalStatus::CONTROLLED
        } else {
            ApprovalStatus::UNRESTRICTED
        }
    }

    pub fn is_controlled(&self) -> bool {
        matches!(self, ApprovalStatus::CONTROLLED | ApprovalStatus::UNRESTRICTED)
    }

    // Internal logic to determine if this state should be active based on external rules.
    // In the real app, you would call `check_policy()` here or have a policy checker function.
    pub fn is_active(&self) -> bool {
        self.is_controlled() || !matches!(self, ApprovalStatus::RESTRICTION_ACTIVE)
    }

    /// Returns true if this state should be active for the given user/session pair based on external rules (e.g., "user has permission X").
    pub fn is_active_for_session(&self, session_id: &str) -> bool {
        matches!(self, ApprovalStatus::CONTROLLED | ApprovalStatus::UNRESTRICTED) && !matches!(self, ApprovalStatus::RESTRICTION_ACTIVE)
            || self.is_controlled() // If the user has full control or is unrestricted.
    }

    /// Returns true if this state should be active for a specific action based on external rules (e.g., "user can do Y").
    pub fn is_active_for_action(&self, action_id: &str) -> bool {
        matches!(self, ApprovalStatus::CONTROLLED | ApprovalStatus::UNRESTRICTED) && !matches!(self, ApprovalStatus::RESTRICTION_ACTIVE)
            || self.is_controlled() // If the user has full control or is unrestricted.
    }

    /// Returns true if this state should be active for a specific action based on internal policy rules (e.g., "user must not do Z").
    pub fn is_active_for_action_no_protection(&self, action_id: &str) -> bool {
        matches!(
            self,
            ApprovalStatus::CONTROLLED | ApprovalStatus::UNRESTRICTED
                || !matches!(self, ApprovalStatus::RESTRICTION_ACTIVE)
        ) && !matches!(action_id, "do_z") // If the user has full control or is unrestricted.
    }

    /// Returns true if this state should be active for a specific action based on internal policy rules (e.g., "user must not do Z").
    pub fn is_active_for_action_no_protection(&self, session_id: &str) -> bool {
        matches!(
            self,
            ApprovalStatus::CONTROLLED | ApprovalStatus::UNRESTRICTED
                || !matches!(self, ApprovalStatus::RESTRICTION_ACTIVE)
        ) && !matches!(session_id, "do_z") // If the user has full control or is unrestricted.
    }

    /// Returns true if this state should be active for a specific action based on internal policy rules (e.g., "user must not do Z").
    pub fn is_active_for_action_no_protection(&self, ticket_id: &str) -> bool {
        matches!(
            self,
            ApprovalStatus::CONTROLLED | ApprovalStatus::UNRESTRICTED
                || !matches!(self, ApprovalStatus::RESTRICTION_ACTIVE)
        ) && !matches!(ticket_id, "do_z") // If the user has full control or is unrestricted.
    }

    /// Returns true if this state should be active for a specific action based on internal policy rules (e.g., "user must not do Z").
    pub fn is_active_for_action_no_protection(&self, session_id: &str, ticket_id: &str) -> bool {
        matches!(
            self,
            ApprovalStatus::CONTROLLED | ApprovalStatus::UNRESTRICTED
                || !matches!(self, ApprovalStatus::RESTRICTION_ACTIVE)
        ) && !matches!(session_id, "do_z") // If the user has full control or is unrestricted.
    }

    /// Returns true if this state should be active for a specific action based on internal policy rules (e.g.,
