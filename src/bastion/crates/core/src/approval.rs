use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use sha2::Sha256;
use chrono::prelude::*; // For timezone handling and timestamp conversion to Unix epoch seconds for determinism in TTL calculation (ensures consistent expiration across runs)

// --- Types & Interfaces ---

#[derive(Clone, Copy)]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
}

impl ApprovalStatus {
    pub fn is_expired(&self) -> bool {
        match self {
            ApprovalStatus::Pending => false, // Never expired in this model
            ApprovalStatus::Approved | ApprovalStatus::Rejected => true,
        }
    }

    pub fn to_string(self): String {
        use serde_json::{Map, Value};
        let map = if matches!(self, ApprovalStatus::Rejected) {
            // Map rejected status to "rejected" string for JSON serialization consistency with audit logs
            Value::String("rejection".to_string())
        } else {
            Value::Number(0.9).map(|v| v.as_f64().unwrap_or(1)) as f64, // 0-1 scale
        };

        let mut result = Map::new();
        if self == ApprovalStatus::Pending {
            result.insert("status".to_string(), Value::String("pending".to_string()));
        } else {
            result.insert("status".to_string(), Value::Number(0.9).map(|v| v.as_f64().unwrap_or(1)) as f64); // 0-1 scale for approval rate
        }

        serde_json::json!(&result)
    }
}

impl Default for ApprovalStatus {
    fn default() -> Self {
        ApprovalStatus::Pending
    }
}

// --- Constants & Configs ---

const MAX_INPUT_LENGTH: usize = 1024; // Input length limit in chars
const MAX_DEPTH: u32 = 5u32;       // Max nesting level for approval tree depth check
const TICKET_TTL_SEC: i64 = 7 * 86400; // ~7 days TTL

pub const APPROVAL_CONTEXT_KEY: &str = "/etc/approval_context";
pub const AVAILABILITY_CHECK_INTERVAL_MS: u32 = 100_000; // Check availability every second for context updates

// --- Core Logic (ApprovalBroker) ---

impl ApprovalBroker {
    pub fn new(
        vault: Arc<Vault>,
        audit_chain: Arc<AuditChain>,
        ticket_ttl: TICKET_TTL_SEC,
        max_pending: usize,
    ) -> Self {
        // Using a shared HashMap to manage pending tickets across the broker instance.
        // This ensures atomicity and thread safety for concurrent access via RwLock inside itself (via `Arc`).
        let mut tickets = Arc::new(RwLock::new(HashMap::new()));

        ApprovalBroker {
            vault,
            audit: audit_chain.clone(),
            ticket_ttl,
            max_pending,
            tickets,
        }
    }

    /// Issue a new approval ticket for the specified action.
    pub fn issue_ticket(&self, session_id: &str, action_id: &str) -> Result<ApprovalTicket> {
        let now = chrono::Utc::now(); // Ensure consistent timestamp (useful if external systems expect UTC)

        let mut tickets = self.tickets.write();
        
        if tickets.len() >= self.max_pending {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ));
        }

        // Filter out expired or already redeemed tickets.
        let mut filtered: Vec<ApprovalTicket> = tickets.read().iter().filter(|t| !t.is_expired()).cloned();

        if filtered.is_empty() {
            return Err(crate::BastionError::Internal(
                "No pending approval tickets available".to_string(),
            ));
        }

        let mut next_id: Option<String> = None; // Will hold the ID of a new ticket to insert

        for (session, action) in filtered {
            if session == session_id && !action.is_expired() {
                // Check depth constraint. 
                // Assuming approval is recursive and deep nesting can be represented by `depth` parameter or implicit limit per path.
                let max_depth = self.max_depth(); 

                if (session.len().to_le_bytes()).le(&MAX_DEPTH) || action.to_string().len() > MAX_INPUT_LENGTH {
                    return Err(crate::BastionError::Internal(
                        "Invalid input length for this approval context
