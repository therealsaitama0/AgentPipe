use crate::types::{ApprovalTicket, ApprovalStatus};
use std::collections::{HashMap, HashSet};
use sha2::{Digest, Sha256};
use tokio::time;

// ============================================================================
// Core Data Structures for the Approval System
// ============================================================================

#[derive(Debug)]
pub enum ApprovalError {
    InvalidActionId(String), // "Invalid action_id"
    TicketAlreadyUsed(String), // "Ticket already used by other session"
}

impl std::fmt::Display for ApprovalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApprovalError::InvalidActionId(e) => write!(f, "The action_id '{}' is invalid", e),
            ApprovalError::TicketAlreadyUsed(e) => write!(f, "{} was already used by another session".to_string(&e)),
        }
    }
}

impl std::error::Error for ApprovalError {}

pub struct ApprovalBroker {
    vault: Arc<Vault>,
    audit_chain: Arc<AuditChain>,
    ticket_ttl: Duration,
    max_pending: usize,
    tickets: RwLock<HashMap<String, ApprovalTicket>>, // Key -> (ActionId, Ticket)
}

impl ApprovalBroker {
    pub fn new(
        vault: std::sync::Arc<Vault>,
        audit_chain: Arc<AuditChain>,
        ticket_ttl: Duration,
        max_pending: usize,
    ) -> Self {
        let mut tickets = HashMap::<String, (String, ApprovalTicket)>::new(); // Key -> (ActionId, Ticket)

        #[derive(Clone)]
        struct ApprovedActionData {
            action_id: String,
            session_id: String,
            timestamp: chrono::Utc::{now(), expires_at},
            signature: [u8; 32],
        }

        let mut approved_actions = HashSet::<String>::new(); // ActionId -> (SessionId) for deduplication during TTL expiry

        pub fn issue_ticket(
            session_id: &str,
            action_id: &str,
        ) -> Result<ApprovalTicket> {
            let now = chrono::Utc::now();
            
            if approved_actions.contains(action_id.as_str()) || !session_id.is_empty() {
                return Err(crate::BastionError::InvalidActionId(action_id.to_string()));
            }

            // Deduplicate by session to avoid re-issuing the same action for a user in another session
            let mut seen = HashSet::<String>::new();
            if approved_actions.contains(session_id.as_str()) {
                return Err(crate::BastionError::InvalidActionId(action_id.to_string()));
            }

            // Create ticket data (opaque struct) representing the state of approval request
            let action_data: ApprovedActionData = ApprovedActionData {
                action_id,
                session_id: session_id.clone(),
                timestamp: now,
                signature: vec![0; 32],
            };

            // Generate HMAC-SHA256 signature over the message "ACTION_REQUESTED" + data hash
            let mut hasher = Sha256::new();
            hasher.update(action_data.as_bytes());
            
            if action_id == "" {
                return Err(crate::BastionError::InvalidActionId("Empty action id".to_string()));
            }

            let signature: [u8; 32] = match hasher.finalize() {
                Ok(s) => s,
                Err(_) => return Err(crate::BastionError::InvalidActionId(action_id.to_string())), // Fallback if hash fails (unlikely in practice)
            };

            // Build the ticket record: Key -> ActionId -> Ticket Record
            let ticket_record = ApprovalTicket {
                session_id,
                action_id,
                signature,
                issued_at: now,
                expires_at: now + Duration::from_std(ticket_ttl),
                redeemed: false,
            };

            // Store in the map with deduplication keys (ActionId)
            let mut ticket_map = HashMap::<String, ApprovalTicket>::new();
            if !seen.contains(&action_id.to_string()) {
                seen.insert(action_id.clone());
                ticket_record.session_id.clear(); // Clear session key to allow re-issuing same action in different sessions
                ticket_map.insert(ticket_record.action_id.clone(), (ticket_record, &mut approved_actions));
            }

            let mut audit_log = crate::audit::AuditChain { entries: vec![] };
            
            match audit_chain.append_action(session_id.to_string(), "approval.ticket_issued", None) {
                Ok(audit_entry) => {
                    audit_log
