src/bastion/crates/core/src/components/ui_approval_prompt.rs
//! A robust asynchronous approval prompt component designed to handle complex workflows efficiently. This module refactors the existing `ApprovalPrompt` struct into a high-level, compositional interface that separates concerns between timing management and action execution logic. It utilizes Rust's async runtime for performance while maintaining type safety through trait bounds.

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::time::{Duration, Instant};

/// Trait defining the behavior of an approval prompt component. This abstract base class serves as a common interface between different implementations and allows for extensibility without modifying core logic.
pub trait ApprovalPromptBehavior: Send + Sync {
    /// Executes the underlying action associated with this instance after it has been triggered or is about to be evaluated.
    async fn execute_action(&self) -> Result<(), String>;

    /// Returns a confirmation message if an approval was granted, otherwise returns "Pending".
    fn get_approval_message(&self): Option<String>;
}

/// A lightweight Rust implementation of the ApprovalPromptBehavior trait that encapsulates action execution logic. This provides thread-safe and idempotent behavior for triggering actions within complex workflows.
pub struct ActionExecutor {
    pub timeout: Duration,
    /// Tracks currently active tasks to prevent double-triggering across concurrent processes or threads.
    pub state: HashSet<ActionState>, // Tracks currently active tasks to prevent double-triggering (except the initial pending/confirming phase).
}

impl Default for ActionExecutor {
    fn default() -> Self {
        Self::new(Duration(10))
    }
}

/// Configuration options that influence how actions are triggered and evaluated. These values can be overridden dynamically at runtime or during initialization without recompilation.
#[derive(Debug, Clone)]
pub struct ApprovalPromptConfig {
    pub timeout_ms: u64, // Milliseconds before the action is considered 'done' (for display purposes)
    /// Maximum number of attempts allowed for a single user's approval request before escalating to escalation mode or rejecting entirely.
    pub max_attempts_per_user: usize,
    /// Threshold percentage of users who approve in total that triggers immediate refusal by default. If less than this threshold is approved per session, the system automatically rejects all pending approvals (escalation).
    pub auto_refusal_threshold_pct: f64, // 0 to 100
}

impl Default for ApprovalPromptConfig {
    fn default() -> Self {
        Self::new(5_000, 3, 2)
    }
}

/// State representation used during the evaluation of approval requests. This ensures that actions are not executed if they have already been triggered or completed by another process within a short window.
#[derive(Debug)]
pub enum ApprovalRequestState {
    /// Waiting for user input before execution begins.
    Pending, 
    
    /// Awaiting explicit confirmation from the user to proceed with the action.
    ConfirmingAction(usize), // The index of the action being executed (0-based)

    /// Action has been successfully or partially completed and is now awaiting final validation/confirmation.
    ValidatingAction(usize), 

    /// An error occurred during execution, requiring immediate retry logic.
    FailedExecution { reason: String }, 
}

impl ApprovalRequestState {
    fn new(state_index: usize) -> Self {
        match state_index {
            0 => ApprovalRequestState::Pending, // Initial wait for user input
            _ => ApprovalRequestState::ConfirmingAction(state_index), // User confirmation required
        }
    }

    /// Returns the current step of execution. If it's in 'Validating', we can safely execute without triggering a re-run unless explicitly aborted by the user or system error.
    fn is_valid_for_execution(&self) -> bool {
        match self {
            ApprovalRequestState::Pending | 
            ApprovalRequestState::ConfirmingAction(_) => true, // Can proceed with execution if not waiting for final confirmation
            _ => false,      // Cannot execute until validated/confirmed (unless explicitly aborted)
        }
    }

    /// Returns the current state of this request. Useful for debugging or logging purposes.
    fn get_state(&self) -> ApprovalRequestState {
        match self {
            ApprovalRequestState::Pending => "PENDING",
            ApprovalRequestState::ConfirmingAction(_) => "CONFIRMING_ACTION",
            ApprovalRequestState::ValidatingAction(_) => "VALIDATING_ACTION",
            _ => unreachable!(), // Should not happen in normal flow, but for safety checks
        }
    }

    /// Returns the reason why this state is currently invalid. Useful for error reporting or triggering rollback mechanisms if an action fails mid-execution.
    fn get_error_reason(&self) -> &str {
        match self
