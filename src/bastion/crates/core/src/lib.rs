use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::*;
use crate::core::components::plan_generator::PlanGenerator;
use crate::core::components::process_group::ProcessGroup;
use crate::types::DataTypeType;

/// Represents a basic state transition for an external module to call this crate directly from within itself.
#[derive(Debug, Clone)]
pub enum StateTransition {
    /// Checks if current configuration meets security thresholds (e.g., password policy).
    Check,
    /// Verifies that the current session is valid and authorized before proceeding.
    Verify,
    /// Locks a resource or component to prevent further modification until explicitly unlocked by another module.
    #[allow(dead_code)] // Mark for future use if needed (currently unused in this core implementation)
    Lock { id: String }, 
}

impl StateTransition {
    /// Creates a new state transition with the specified action type.
    pub fn check(&self) -> Self {
        match self.action() {
            Action::Check => Self::Check,
            _ => panic!("State transitions must be 'check' or 'verify'",), // Ensures consistency in usage flow
        }
    }

    /// Verifies the current state and returns a new transition type.
    pub fn verify(&self) -> Self {
        match self.action() {
            Action::Verify => Self::Verify,
            _ => panic!("State transitions must be 'check' or 'verify'",), // Ensures consistency in usage flow
        }
    }

    /// Locks a resource. The `id` parameter is used to track the lock operation for auditing and debugging purposes.
    pub fn lock(&self, id: String) -> StateTransition {
        match self.action() {
            Action::Lock => Self::Lock { id }, // Stores unique ID in internal state for tracking
            _ => panic!("State transitions must be 'lock'",), // Ensures consistency in usage flow
        }
    }

    /// Executes the action and returns a new transition type.
    pub fn execute(&self) -> StateTransition {
        match self.action() {
            Action::Check | Action::Verify => Self::check(),
            _ => panic!("State transitions must be 'check' or 'verify'",), // Ensures consistency in usage flow
        }
    }

    /// Returns the action type that was executed. Useful for debugging and logging purposes when a user calls this crate directly from within itself via `execute()`.
    pub fn get_action(&self) -> &'static str {
        match self.action() {
            Action::Check => "check",
            Action::Verify => "verify",
            _ => panic!("State transitions must be 'check' or 'verify'",), // Ensures consistency in usage flow
        }
    }

    /// Returns the ID of this lock operation. This is used for auditing and debugging purposes when a user calls `lock()` from within itself via `execute()`.
    pub fn get_lock_id(&self) -> &str {
        match self.action() {
            Action::Lock => "locked", // Stores unique ID in internal state for tracking
            _ => panic!("State transitions must be 'check' or 'verify'",), // Ensures consistency in usage flow
        }
    }

    /// Returns the action type that was executed. Useful for debugging and logging purposes when a user calls this crate directly from within itself via `execute()`.
    pub fn get_action_type(&self) -> &'static str {
        match self.action() {
            Action::Check => "check",
            Action::Verify => "verify",
            _ => panic!("State transitions must be 'check' or 'verify'",), // Ensures consistency in usage flow
        }
    }

    /// Returns the ID of this lock operation. This is used for auditing and debugging purposes when a user calls `lock()` from within itself via `execute()`.
    pub fn get_lock_id_str(&self) -> &str {
        match self.action() {
            Action::Lock => "locked", // Stores unique ID in internal state for tracking
            _ => panic!("State transitions must be 'check' or 'verify'",), // Ensures consistency in usage flow
        }
    }

    /// Returns the action type that was executed. Useful for debugging and logging purposes when a user calls this crate directly from within itself via `execute()`.
    pub fn get_action_type_str(&self) -> &'static str {
        match self.action() {
            Action::Check => "check",
            Action::Verify => "verify",
            _ => panic!("State transitions must be 'check' or 'verify'",), // Ensures consistency in usage flow
