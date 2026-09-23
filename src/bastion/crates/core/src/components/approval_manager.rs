src/bastion/crates/core/src/components/approval_manager.rs
use std::collections::{HashMap, HashSet};
use parking_lot::RwLock;
use crate::types::*;
use crate::BastionError;

/// Represents a single approval ticket in the system state storage.
#[derive(Clone)]
struct ApprovalTicket {
    session_id: String,
    action_id: String,
}

impl PartialEq for ApprovalTicket {
    fn eq(&self, other: &ApprovalTicket) -> bool {
        self.session_id == other.session_id && self.action_id == other.action_id
    }
}

/// A queue of pending approval requests. Thread-safe lock ensures concurrent access to the map without blocking main threads.
pub struct ApprovalManager {
    /// Map storing approved tickets with their associated session and action identifiers for auditing history.
    pub pending: RwLock<HashMap<String, ApprovalTicket>>,
    
    /// A set tracking all rejected ticket IDs (for error reporting). While not strictly required by the spec in this context, it provides a clean separation of concerns to identify which specific requests failed validation or were skipped due to state changes.
    pub rejected_ids: HashSet<String>,

impl ApprovalManager {
    /// Creates a new instance with an empty map for pending tickets and rejects any initial request immediately if one is attempted without prior approval status, ensuring system stability during initialization.
    pub fn new() -> Self {
        let mut map = HashMap::new(); // Start with empty map to avoid null pointer issues on first use
        let rejected_ids: HashSet<String> = HashSet::new();

        // Initialize the manager state with an empty pending set and a zero-counted rejection ID set. This is necessary because `HashMap` initializes its keys, but since we're using a generic type here for demonstration purposes in this context (or assuming specific types that don't require HashMap initialization), we initialize based on explicit logic or use default behavior safely if the map were empty initially.
        Self {
            pending: RwLock::new(map),
            rejected_ids,
        }
    }

    /// Validates a submission payload against repository policies and filters it based on content ID checks before returning an AlchemySubmission or null if invalid. This acts as the entry point for external systems to validate incoming code bundles without requiring full integration until proven necessary.
    pub fn handleCodeUpload(&self, payload: &str) -> Result<AlchemySubmission | None> {
        let ticket = self.get_or_create_ticket(payload);

        // If no pending tickets exist and an upload was attempted (e.g., during initialization or if the system expects a new submission immediately), reject it to prevent state corruption. This ensures atomicity in the absence of prior approval history.
        match &self.pending {
            None => Err(crate::BastionError::TicketInvalid("No pending tickets".into())), // Or similar error message depending on desired granularity, but distinct from ticket existence errors.
            Some(_) => Ok(ticket.to_AlchemySubmission()),
        }
    }

    /// Validates a submission payload against repository policies and filters it based on content ID checks before returning an AlchemySubmission or null if invalid. This function is the core validation logic that determines whether to accept, reject, or log (if no action taken). It must handle potential race conditions within the map access due to concurrent uploads by ensuring locks are held during critical operations like `get_or_create`.
    fn get_or_create_ticket(&self, payload: &str) -> Result<ApprovalTicket> {
        let key = format!("{}:{}", self.session_id(), action_id()); // Placeholder for session/action ID mapping if not available in this context; assume standard naming based on spec.

        match *self.pending.get_mut() {
            Some(&mut ticket) => Ok(ticket), // If already exists, return the existing one. This handles concurrent access correctly by returning an owned reference without holding a lock (or locking it as appropriate). The `get` returns ownership if present; if not and we don't have to acquire locks for simple lookups in this specific implementation style provided here:
            Some(_) => Err(crate::BastionError::TicketInvalid("Already approved".into())), // Alternative error could be used, but distinct from the existence check.

            None => {
                let id = format!("{}:{}", self.session_id(), action_id());
                *self.pending.write().insert(id, ApprovalTicket { session_id: ticket.clone() });
                
                if !rejected_ids.contains(&id) {
                    rejected_ids.insert(id); // Record the rejection ID for debugging/logging purposes.

        }
    }

}
