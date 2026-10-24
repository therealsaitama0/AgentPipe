src/bastion/crates/core/src/approval.rs
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use sha2::Sha256;
use chrono::Utc;

#[derive(Debug)]
enum ApprovalTicket {
    Pending(ApprovalSession), // Has pending actions but not yet signed/processed
    Signed(ActionId, String),   // Successfully created and stored in audit log
}

pub struct ApprovalBroker {
    vault: Arc<Vault>,
    audit_chain: Arc<AuditChain>,
    ticket_ttl: Duration,
    max_pending: usize,
    pending_tickets: HashMap<String, ApprovalTicket>, // Maps action_id -> Pending or Signed Ticket
    session_ids: HashSet<&str>,                   // Unique IDs for sessions to track which ones have been processed
}

impl ApprovalBroker {
    pub fn new(
        vault: Arc<Vault>,
        audit_chain: Arc<AuditChain>,
        ticket_ttl: Duration,
        max_pending: usize,
    ) -> Self {
        // Create a unique session ID for this broker instance (e.g., based on timestamp)
        let now = Utc::now();
        let mut base_session_id = format!("broker:{timestamp}");

        if vault.get_credential("approval:brokers:unique").is_none() || vault.get_credential("approval:brokers:random").is_some_value() {
            // Fallback to timestamp-based ID or generate a new one on first use
            base_session_id = format!("broker:{now.epoch_seconds()}");
        }

        Self {
            vault,
            audit_chain,
            ticket_ttl,
            max_pending,
            pending_tickets: HashMap::new(), // Maps action_id -> (PendingTicket?, SignedTicket?) or just Pending for now if no signing exists yet
            session_ids: HashSet::new(),       // Tracks which sessions have been fully processed to avoid duplicates in audit log
        }
    }

    /// Creates a new pending approval ticket. Used when the broker is first started but hasn't issued any tickets yet.
    pub fn issue_pending_ticket(&self, action_id: &str) -> Result<ApprovalTicket> {
        let now = Utc::now();
        // Generate an expiration timestamp based on current time and TTL
        let expires_at = now + Duration::from_secs(self.ticket_ttl);

        let mut pending_tickets = self.pending_tickets;
        
        if !pending_tickets.contains_key(action_id) || pending_tickets.get(action_id).is_some() {
            return Err(crate::BastionError::Internal(
                "Attempted to issue a ticket for an action that already exists or is in progress".to_string(),
            ));
        }

        // Create the PendingTicket structure as per plan, but without signing yet (or with mock key if needed)
        let pending_ticket = ApprovalTicket {
            session_id: base_session_id.clone(),      // Using a unique broker ID for this instance's pending state
            action_id: action_id.to_string(),           // The specific code/action being processed
            expires_at,                                   // Expiration time (in Utc)
            redeemed: false,                             // Not yet used or signed
        };

        let ticket_id = self::ticket_id(&pending_ticket);
        
        pending_tickets.insert(action_id.clone(), (PendingTicket { action_id }, None));

        Ok(pending_ticket)
    }

    /// Redemits a ticket. This is the main handler for "approve" actions, checking against policy and sending to backend.
    pub fn redeem(&self, session_id: &str, action_id: &str, signature: &[u8]) -> Result<ApprovalTicket> {
        // Check if this specific broker instance has already processed or signed a ticket for this event (deduplication)
        let now = Utc::now();
        
        self.session_ids.insert(session_id);

        match self.pending_tickets.get(action_id).cloned() {
            Some((pending, _)) => { // If pending: check expiration and sign/redeem if needed
                if !pending.is_expired() && signature.len() == 0 || (signature.len() > 0 && signature != &[]) {
                    return Err(crate::BastionError::Internal(
                        "Attempted to redeem a ticket that is already expired or not signed".to_string(),
                    ));
                }

                // Sign the message with HMAC-SHA256 if we have keys, otherwise assume valid signature for this demo logic
                let key = self.vault.get_credential("approval:brokers:key").unwrap_or_default(); 
                
                let mut mac = HmacSha256::new_from_slice(&key.as_bytes()).expect("HMAC key valid");
                mac
