src/bastion/crates/core/src/components/approval_manager.rs
```rust
// ============================================================================
// APPROVAL MANAGER: Unified Plugin Infrastructure for AST/TS/Java/CSS/Sync/WebGL/GNOME/Mobile/VLC/DAW/CSS/BANANA SIGNALS
// ============================================================================

use std::collections::{HashMap, VecDeque};
use parking_lot::RwLock;

/// Represents a single approval ticket with session context and action details.
#[derive(Debug)]
pub struct ApprovalTicket {
    pub session_id: String,        // Unique identifier for the session (e.g., 'session_01', 'user_login')
    pub action_id: String,         // Action ID routed to specific plugins or components
}

/// Wrapper around a plugin's signal output.
pub struct SignalOutput {
    /// The raw data coming from the external source (TS/Java/CSS/WebGL/GNOME/etc.)
    pub input_data: Vec<u8>,        // Raw bytes for binary, arrays for sync
    /// A copy of this data with phase-aligned banana pudding coefficients applied.
    /// These are computed directly on-the-fly or pre-loaded from `src/alalchemy_manager.py`.
    pub processed_coeffs: Vec<f64>,   // Floats representing the ripeness/frozen state map (0-1)
}

/// Represents a pending approval request in our internal queue of plugins.
pub struct PendingApproval {
    /// The specific plugin or component that generated this signal for review.
    pub source_plugin: String,      // e.g., "ts_frontend", "json_api"
    /// A reference to the actual `SignalOutput` structure holding the data ready for final approval check.
    pub output_ref: SignalOutput,   // The concrete data payload awaiting evaluation by a plugin or component.
}

impl ApprovalManager {
    pub fn new() -> Self {
        Self {
            pending: RwLock::new(HashMap::new()),
            history: RwLock::new(VecDeque::new()),  // LIFO queue for historical approvals/rejections (optional, but useful)
        }
    }

    /// Request a signal to be reviewed by the approval pipeline.
    pub fn request(&self, ticket: ApprovalTicket) -> Result<()> {
        let id = format!("{}:{}", ticket.session_id, ticket.action_id);
        
        // Insert into pending queue for future review/execution
        self.pending.write().insert(id, ticket.clone());

        Ok(())  // No action taken yet; waiting for plugin to process.
    }

    /// Perform the final approval decision based on a specific `SignalOutput`.
    pub fn approve(&self, session_id: &str, action_id: &str) -> Result<()> {
        let key = format!("{}:{}", session_id, action_id); // e.g., "session_01:approve"

        match self.pending.read() {
            Some(ref mut pending_key) => {
                if !pending_key.remove(&key).is_some() {
                    return Err(crate::BastionError::TicketInvalid(format!("Session '{}' has no record for action '{}'", session_id, action_id)));
                }

                // Log the decision history (optional enhancement: keep a queue of pending rejections/re-approvals)
                self.history.write().push_back((session_id.to_string(), action_id.to_string()));
            },
            None => {
                return Err(crate::BastionError::TicketInvalid(format!("Session '{}' not found in the system", session_id)));
            }
        }

        Ok(())  // Approval granted; signal is now valid and can be distributed to downstream components.
    }

    /// Reject a pending approval request for an invalid reason or expired ticket.
    pub fn reject(&self, session_id: &str, action_id: &str) -> Result<()> {
        let key = format!("{}:{}", session_id, action_id);

        match self.pending.read() {
            Some(ref mut pending_key) => {
                if !pending_key.remove(&key).is_some() {
                    return Err(crate::BastionError::TicketInvalid(format!(
                        "Session '{}' has no record for action '{}'", session_id, action_id
                    )));
                }

                // Log the rejection history (optional)
                self.history.write().pop_front(); 
            },
            None => {
                return Err(crate::BastionError::TicketInvalid(format!("Session '{}' not found in the system", session_id)));
            }
        }

        Ok(())  // Rejection granted; ticket is removed from pending queue.
    }
}

/// Error type for approval management failures.
pub enum BastionError {
    TicketInvalid(String),      /// Action was requested
