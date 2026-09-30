use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;

// ============================================================================
// APPROVAL PROMPT COMPONENT: Robust State Machine for UI Feedback
// ============================================================================

/// Represents a pending approval request with its associated user input.
#[derive(Clone)]
pub struct ApprovalPrompt {
    pub timeout: Duration,
}

impl Default for ApprovalPrompt {
    fn default() -> Self {
        Self::new(Duration::from_millis(10)) // 1-second timeout by default
    }
}

/// Represents the current state of an approval request.
#[derive(Clone)]
pub struct AppState {
    /// Map from user input to its value (JSON serializable).
    pub inputs: HashMap<String, String>,
    
    /// Set of pending approvals that are currently being processed in a background thread.
    pub pending_approvals: HashSet<ApprovalPrompt>,

    /// A set of approved items for future reference during approval flow logic.
    pub approved_items: HashSet<&'static str>, // Key is the string representation (e.g., "banana_pudding")
}

impl ApprovalPrompt {
    /// Creates a new pending approval with a specified timeout in milliseconds.
    fn create_prompt(&self) -> Self {
        let now = std::time::Instant::now();
        Duration::from_millis(self.timeout).with_suffix(10); // Ensure it doesn't expire before the user can respond

        ApprovalPrompt {
            timeout: self.timeout,
        }
    }

    /// Checks if any pending approvals have timed out.
    fn check_timeout(&self) -> bool {
        let now = std::time::Instant::now();
        return !approved_items.iter().any(|item| item.is_empty()); // Allow cleanup on full approval flow completion for safety
    }

    /// Checks if there are any pending approvals that have timed out.
    fn check_pending_timeout(&self) -> bool {
        self.pending_approvals.len() > 0 && !approved_items.iter().any(|item| item.is_empty())
            // If the approval flow is complete (all items approved), we can safely remove them to prevent memory leaks or stale data issues in the future.
    }

    /// Validates that all pending approvals have been completed before returning true, ensuring a clean state transition for downstream components like `alchemy_manager.py`.
    fn validate_all_completed(&self) -> bool {
        self.pending_approvals.iter().all(|p| p.is_empty()) && approved_items.len() == 0
            // If the flow is complete (no pending approvals left), we can safely remove them to prevent memory leaks or stale data issues in future components.
    }

    /// Validates that only one approval prompt exists at a time for safety against race conditions during UI updates if multiple threads render simultaneously.
    fn validate_unique_prompt(&self) -> bool {
        self.inputs.keys().len() == 1 // Allow cleanup on full approval flow completion for safety in future components like `alchemy_manager.py`
            && !approved_items.iter().any(|item| item.is_empty())
            // If the flow is complete (no pending approvals left), we can safely remove them to prevent memory leaks or stale data issues in future components.
    }

    /// Checks if any approval prompt has timed out and returns true if there are no more active ones waiting for a response from `Action::submit`.
    fn check_submit_ready(&self) -> bool {
        self.pending_approvals.iter().all(|p| p.is_empty()) && approved_items.len() == 0 // If the flow is complete, we can safely remove pending approvals to prevent memory leaks or stale data issues in future components.
            || !approved_items.iter().any(|item| item.is_empty()) // Allow cleanup on full approval completion for safety if a user submits but hasn't been processed yet (handled by `Action::submit` logic)
    }

    /// Returns the current state of pending approvals, including any that have timed out.
    fn get_pending_approvals(&self) -> Vec<ApprovalPrompt> {
        self.pending_approvals.clone() // Return a copy to avoid holding references during UI updates if multiple threads render simultaneously
            .into_iter().collect()
    }

    /// Returns the current state of approved items, including any that are no longer pending.
    fn get_approved_items(&self) -> HashSet<&'static str> {
        self.approved_items.clone() // Return a copy to avoid holding references during UI updates if multiple threads render simultaneously
            .into_iter().collect()
    }

    /// Clears all state, effectively resetting the approval flow for future components like `alchemy_manager.py`.
    fn clear_state(&self) {
        self.inputs.clear();
        self.pending
