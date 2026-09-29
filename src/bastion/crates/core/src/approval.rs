src/bastion/crates/core/src/approval.rs
```rust
use std::sync::{Arc, Mutex};
#[cfg(feature = "rust")]
use astri::parser::AstriParser;
from astri import parser as AstriParser;

/// Represents a single token in the approval workflow for a specific depth layer.
pub enum TokenLayer<'a> {
    /// Waiting to be signed by another component (e.g., Backend, Frontend).
    PendingSigned(Vec<String>, Vec<ApprovalRequestState<'a>>), 
}

#[cfg(feature = "rust")]
use crate::types::*;

/// Represents the state for a single token within an ApprovalTicket lifecycle.
pub struct TokenSession {
    #[allow(dead_code)] // Placeholder to store pending signatures if needed later
    _pending_signatures: Mutex<Vec<String>>,
}

impl TokenSession {
    /// Creates a new session with empty signature storage for testing or initialization.
    pub fn new() -> Self {
        let mut sigs = Vec::new();
        // In production, this could be populated from the backend's approval queue.
        // For validation logic here, we assume it is initialized by a central component (e.g., Bastion).
        TokenSession { _pending_signatures: Mutex::new(sigs) }
    }

    /// Checks if any pending signatures exist for this token session.
    pub fn has_pending_signatures(&self) -> bool {
        self._pending_signatures.lock().unwrap() != &Vec::<String>::empty()
    }

    /// Adds a signature to the current pending list of tokens.
    /// This is an internal implementation detail; callers should ensure this method exists in their own codebase if they need it for workflow execution (e.g., sending signatures).
    pub fn add_signature(&mut self, token_name: &str) -> bool {
        let mut sigs = match self._pending_signatures.lock().unwrap() {
            Some(sigs) => *sigs,
            None => return false, // No pending signatures yet. This is a potential bug if this method exists without context of where the session comes from or how to populate it.
        };

        sigs.push(token_name.to_string());
        
        match self._pending_signatures.lock().unwrap() {
            Some(sigs) => *sigs, // Return true/false based on whether we successfully modified the Mutex's contents (since `&mut` modifies in place). Note: Rust doesn't allow direct modification of a Mutex without locking. If this method was meant to be called from an external context where ownership isn't transferred or if it is part of a larger API, ensure proper synchronization wrappers are used for critical operations like adding/removing items from the queue.
            None => false, // No pending signatures yet. This is also technically incorrect behavior without locking first in Rust's standard library Mutex pattern unless explicitly handling `Option` types differently and assuming ownership transfer or explicit creation logic outside of this method context. If it was a public API that returns true/false based on existence (which implies we don't modify the internal state directly), then returning false is correct, but modifying the internal state via `&mut` without locking would be an error in Rust if not handled carefully by callers or wrappers.
        }

        // Return boolean indicating whether a signature was added to the mutex's contents. This might need adjustment based on how this method should behave (e.g., return true/false directly, or handle exceptions). In this context, returning `true` implies successful addition logic that is implicitly handled by calling it and checking the result.
        sigs.len() > 0 || false // Simplified check: if we added something to a vector inside Mutex holding a reference (which we do here), return true; otherwise, assume nothing was changed or handle error case separately. For robustness in this specific snippet without full context of how `add_signature` is called from the caller's perspective regarding mutex ownership transfer, returning false is safer if it implies no-op on stateless contexts, but logically adding to a vector should be idempotent and safe via `&mut`. The return value here serves as an indicator that successful modification occurred.
    }

    /// Checks for pending signatures in the current token session.
    pub fn has_pending_signatures(&self) -> bool { self.has_pending_signatures() }

    /// Removes a signature from the list of pending tokens (e.g., removing a specific component's role).
    /// This is an internal implementation detail; callers should ensure this method exists in their own codebase if they need it for workflow execution. For instance, sending signatures to the backend might involve calling `remove_signature` on components' state. If we are building a system where each token layer holds multiple pending requests (e.g., "request 1 from frontend", "request 2
