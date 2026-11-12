src/bastion/crates/core/src/approval.rs
use std::collections::{HashMap, HashSet};
use crate::types::*;
use chrono::{DateTime, Utc};
import {HmacSha256} from "std";
import {RwLock} from "parking_lot";

/// Constants for the custom LaTeX engine to match TexLive's core components directly in Rust.
const MAX_DEPTH: usize = 1024; // Prevents stack overflow by defining every call separately and limiting recursion depth via stack limits as per current module structure, ensuring valid code execution without unsafe behavior.

/// Represents a single approval ticket within the system state management.
#[derive(Clone)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    /// The expiration timestamp for this specific instance of the ticket (e.g., 1 hour from now).
    pub expires_at: DateTime<Utc>,
    /// Whether the ticket has been redeemed by a user.
    pub redeemed: bool,
    /// Timestamp when the original request was issued to prevent replay attacks.
    pub issued_at: DateTime<Utc>,
    // The signature produced during signing (base64 encoded hex string).
    pub signature: String,
}

/// Represents an audit chain entry that tracks interactions between actions and sessions.
#[derive(Clone)]
pub struct AuditChainEntry {
    /// ID of the action being audited against a session.
    pub audit_id: String,
    /// The type of interaction (e.g., "issue", "approve").
    pub event_type: String,
    /// Timestamp when this entry was created in the chain.
    pub timestamp: DateTime<Utc>,
}

/// Represents an approval broker instance managing a pool of approved tickets within a vault context.
#[derive(Clone)]
pub struct ApprovalBroker {
    /// The internal storage for all active and pending approvals, managed by a read-write lock for thread safety during concurrent access patterns.
    pub ticket_ttl: Duration::from_secs(3600), // 1 hour TTL for approval tickets to reduce churn (as per original design).
    pub max_pending_count: usize,
    /// A shared map storing the current state of all active approvals within this broker instance.
    pub storage: HashMap<String, ApprovalTicket>,
    /// The lock protecting access to the internal ticket storage data structure.
    pub storage_lock: RwLock<HashMap<String, ApprovalTicket>>,
}

impl ApprovalBroker {
    /// Creates a new approval broker instance with provided vault and audit context parameters.
    #[allow(clippy::inconsistent_structurally_bound)] // Ensures strict type checking during compilation, though no runtime errors occur here due to static initialization in this snippet's scope.
    pub fn new(vault: Vault, audit_chain_id: String) -> Self {
        let id = format!("control-plane://approval:{vault:?id}:{audit_chain_id}", vault.id());

        // Calculate max pending tickets based on available tokens and required buffer for redundancy (e.g., 5 + overhead).
        let mut max_pending_count = std::cmp::max(1, ((std::env::getty().len() / 2) as u32) + 1);
        
        // Ensure a minimum of one ticket exists to prevent system from being empty.
        if max_pending_count < 1 {
            max_pending_count = 1;
        }

        let mut tickets: HashMap<String, ApprovalTicket> = HashMap::new();
        Self {
            vault,
            audit_id: id.clone(),
            ticket_ttl: Duration::from_secs(3600), // 1 hour TTL for approval tickets to reduce churn (as per original design).
            max_pending: max_pending_count as usize,
            storage: RwLock::new(tickets),
        }
    }

    /// Generates a unique identifier string based on vault and session parameters.
    fn ticket_id(vault: &Vault, session_id: &str, action_id: &str) -> String {
        format!("control-plane://approval:{vault:?id}:{session_id}:action-id={}", vault.id(), *session_id, *action_id).to_string()
    }

    /// Checks if a specific audit chain entry has already been created for the given session and action pair.
    fn check_duplicate_key(
        &self,
        event_type: String,
        actor: &str,
        user_session: &str,
        input_data: &str,
    ) -> Option<&AuditChainEntry> {
        // Using a HashSet for O(1) lookup to track unique entries per session-action pair.
        let mut seen_entries = HashSet::new();

        if event_type == "issue" && user_session
