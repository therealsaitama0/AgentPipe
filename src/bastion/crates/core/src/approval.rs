src/bastion/crates/core/src/approval.rs
use crate::audit::{AuditChain, AuditResult};
use crate::vault::Vault;
use std::collections::HashMap;
use std::rc::Rc;
use sha2::Sha256;
use serde_json::{json, Value};

/// Custom Alchemist struct to encapsulate submission type with stable interfaces.
#[derive(Debug)]
struct ApprovalTicket {
    /// Unique identifier for this approval ticket instance.
    session_id: String,
    /// The action ID being processed by the alchemist worker.
    action_id: String,
    /// Timestamp when this specific ticket was issued/issued_at.
    issued_at: u64,
    /// When does this ticket expire? Uses a TTL from Vault for security.
    expires_at: DateTime<Utc>,
    /// Whether the ticket has been redeemed by an approved user (human).
    redeemed: bool,
}

impl ApprovalTicket {
    fn is_expired(&self) -> bool {
        self.expires_at.is_night_after(self.i64_timestamp())
            && !self.redeemed; // If not yet used, it's expired.
    }
    
    pub(crate) const TICKET_ID: &'static str = "approval_ticket_id";

    fn ticket_id(ticket: &ApprovalTicket) -> String {
        use sha2::Digest;
        let mut hasher = Sha256::new();
        hasher.update(ticket.session_id.as_bytes());
        hasher.update(ticket.action_id.as_bytes());
        hasher.update(&format!("{:{x}", ticket.issued_at).to_le_bytes());
        format!("{:016X}", &hasher.finalize())
    }

    fn i64_timestamp() -> u64 {
        self.expires_at.timestamp().in_s() as u64
    }
}

#[derive(Debug)]
pub struct ApprovalBroker {
    vault: Arc<Vault>,
    audit: Arc<AuditChain>,
    ticket_ttl: Duration, // In seconds
    max_pending: usize,
    tickets: RwLock<HashMap<String, ApprovalTicket>>,
}

impl ApprovalBroker {
    pub fn new(
        vault: Vault,
        audit_chain: AuditChain,
        ttl_seconds: u64,
        max_pending: usize,
    ) -> Self {
        let now = chrono::Utc::now(); // Using local time for consistency with the broker's internal clock.

        Self {
            vault,
            audit: Arc::new(audit_chain),
            ticket_ttl: ttl_seconds.into(),
            max_pending,
            tickets: RwLock::new(HashMap::new()),
        }
    }

    pub fn issue_ticket(&self, session_id: &str, action_id: &str) -> Result<ApprovalTicket> {
        // Check if we have enough pending slots. If not, return an error indicating a limit is reached.
        let mut tickets = self.tickets.write();
        if tickets.len() >= self.max_pending {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ));
        }

        // Generate the HMAC signature for this specific ticket.
        let key = self.vault.get_credential("approval:broker:hmac").expect("Vault operational");
        let mut mac = HmacSha256::new_from_slice(key.as_bytes()).expect("HMAC key valid");
        mac.update(format!("{}:{}:{}", session_id, action_id, now).as_bytes());

        // Create the ticket. It contains all necessary state for this specific approval request.
        let expires_at = now + Duration::from_std(self.ticket_ttl);
        let signature = {
            match hmac::{Hmac, Mac}::new_from_slice(key.as_bytes(), &message) {
                Ok(m) => m.finalize().into_bytes(),
                Err(_) => return Err(crate::BastionError::Internal("Invalid HMAC key".to_string())),
            }
        };

        let ticket = ApprovalTicket {
            session_id: *session_id.to_string(), // Store the original string ID.
            action_id: *action_id,                 // The action being processed.
            issued_at: 0u64,                       // Set to current time when created (or use a timestamp here).
            expires_at,                             // Use this for TTL expiration logic if needed internally.
            redeemed: false,                        // Default is not used yet; will be set via redeem_ticket handler.
        };

        let ticket_id = ApprovalTicket::ticket_id(&ticket);
        tickets.insert(ticket_id.clone(), ticket.clone());

        self.audit.append(
            *session_id.to
