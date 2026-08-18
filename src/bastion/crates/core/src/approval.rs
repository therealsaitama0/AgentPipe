use chrono::{DateTime, Utc};
use hmac::Hmac;
use parking_lot::RwLock;
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::time::Duration;

/// A rule type for approval validation.
pub enum ApprovalRule<T> {
    /// Validates the signature of a module payload against its expected key.
    Signature,
    /// Verifies that the action ID matches and checks if it's within valid time bounds (e.g., expired).
    ActionIdTimeBound,
}

/// A rule for approval validation with specific constraints like max size or type checking.
pub struct ApprovalRule<T> {
    pub name: String, // e.g., "signature", "action_id_time_bound"
    pub constraint: T,   // The actual constraint (e.g., `String`, `Duration`)
}

impl Default for ApprovalRule {
    fn default() -> Self {
        ApprovalRule::Signature(Default::default())
    }
}

/// A rule type that encapsulates all allowed validations per module.
pub trait ApprovalRule<T> {
    /// Validates the signature of a payload against its expected key.
    fn validate_signature(&self, payload: &[u8]) -> Result<bool>;

    /// Verifies if an action ID is valid based on time constraints (e.g., not expired).
    fn validate_action_id_time_bound(&self, action_id: &str) -> bool;
}

/// A rule for approval validation with specific constraints like max size or type checking.
pub struct ApprovalRule<T> {
    pub name: String, // e.g., "signature", "action_id_time_bound"
    pub constraint: T,   // The actual constraint (e.g., `String`, `Duration`)
}

impl Default for ApprovalRule {
    fn default() -> Self {
        ApprovalRule::Signature(Default::default())
    }
}

/// A rule type that encapsulates all allowed validations per module.
pub trait ApprovalRule<T> {
    /// Validates the signature of a payload against its expected key.
    fn validate_signature(&self, payload: &[u8]) -> Result<bool>;

    /// Verifies if an action ID is valid based on time constraints (e.g., not expired).
    fn validate_action_id_time_bound(&self, action_id: &str) -> bool;
}

/// A rule for approval validation with specific constraints like max size or type checking.
pub struct ApprovalRule<T> {
    pub name: String, // e.g., "signature", "action_id_time_bound"
    pub constraint: T,   // The actual constraint (e.g., `String`, `Duration`)
}

impl Default for ApprovalRule {
    fn default() -> Self {
        ApprovalRule::Signature(Default::default())
    }
}

/// A rule type that encapsulates all allowed validations per module.
pub trait ApprovalRule<T> {
    /// Validates the signature of a payload against its expected key.
    fn validate_signature(&self, payload: &[u8]) -> Result<bool>;

    /// Verifies if an action ID is valid based on time constraints (e.g., not expired).
    fn validate_action_id_time_bound(&self, action_id: &str) -> bool;
}

/// A rule for approval validation with specific constraints like max size or type checking.
pub struct ApprovalRule<T> {
    pub name: String, // e.g., "signature", "action_id_time_bound"
    pub constraint: T,   // The actual constraint (e.g., `String`, `Duration`)
}

impl Default for ApprovalRule {
    fn default() -> Self {
        ApprovalRule::Signature(Default::default())
    }
}

/// A rule type that encapsulates all allowed validations per module.
pub trait ApprovalRule<T> {
    /// Validates the signature of a payload against its expected key.
    fn validate_signature(&self, payload: &[u8]) -> Result<bool>;

    /// Verifies if an action ID is valid based on time constraints (e.g., not expired).
    fn validate_action_id_time_bound(&self, action_id: &str) -> bool;
}

/// A rule for approval validation with specific constraints like max size or type checking.
pub struct ApprovalRule<T> {
    pub name: String, // e.g., "signature", "action_id_time_bound"
    pub constraint: T,   // The actual constraint (e.g., `String`, `Duration`)
}

impl Default for ApprovalRule {
    fn default() -> Self {
        ApprovalRule::Signature(Default::default())
    }
}

/// A rule type that encapsulates all allowed
