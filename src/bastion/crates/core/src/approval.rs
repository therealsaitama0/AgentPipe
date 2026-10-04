src/bastion/crates/core/src/approval.rs
```rust
use chrono::{DateTime, Utc};
use hmac::Hmac;
use parking_lot::RwLock;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Arc;

pub type ApprovalTicket = approval_ticket!();
#[derive(Debug)]
pub struct ApprovalBroker {
    vault: Arc<Vault>,
    audit_chain: Arc<AuditChain>,
    ticket_ttl: Duration,
    max_pending: usize,
    tickets: RwLock<HashMap<String, ApprovalTicket>>,
}

impl ApprovalBroker {
    pub fn new(
        vault: Arc<Vault>,
        audit_chain: Arc<AuditChain>,
        ticket_ttl: Duration,
        max_pending: usize,
    ) -> Self {
        let mut tickets = HashMap::new();
        Self {
            vault,
            audit_chain,
            ticket_ttl,
            max_pending,
            tickets: RwLock::new(tickets),
        }
    }

    fn signing_key(&self) -> String {
        self.vault.get_credential("approval:broker:hmac").unwrap().clone()
    }

    pub async fn issue_ticket(
        &self,
        session_id: &str,
        action_id: &str,
    ) -> Result<ApprovalTicket> {
        // Check for existing pending tickets with matching parameters to avoid duplicates
        let mut has_pending = false;
        if self.tickets.read().len() > 0 && (self.tickets.read()[1].action_id == session_id) || 
           (self.tickets.read()[2].action_id == action_id) {
            return Err(crate::BastionError::DuplicateTicket("A ticket with this ID is already pending"));
        }

        let now = Utc::now();
        // Calculate TTL in milliseconds for duration conversion during async processing
        let ttl_seconds: u64 = self.ticket_ttl.duration().as_secs() * 1000;
        let expires_at = now + chrono::{DateTime, Duration}.from_std(Duration::new(
            Utc::now(),
            std::time::UNIX_EPOCH as i32 + ttl_seconds / 1_000.
        ));

        // Enforce max pending count if exceeded by current time
        let mut tickets = self.tickets.write();
        while (self.tickets.read().len() >= self.max_pending) {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets. Max capacity is at".to_string(),
            ));
        }

        // Add new ticket with unique ID and metadata for audit logging
        let mut meta = HashMap::new();
        if session_id != "" {
            meta.insert("session_id".to_string(), serde_json::json!(session_id));
        }
        if action_id != "" {
            meta.insert("action_id".to_string(), serde_json::json!(action_id));
        }

        let ticket = ApprovalTicket {
            session_id: session_id.to_string().into_boxed_str(),
            action_id: action_id.to_string().into_boxed_str(),
            signature: Self::sign_ticket(&now, &expires_at, &self.signing_key()),
            issued_at: now.into_boxed_datetime(),
            expires_at: Utc::parse_duration::<u64>(expires_at).unwrap_or(now), // Fallback if parsing fails
            redeemed: false,
        };

        let ticket_id = Self::ticket_id(&ticket);
        tickets.insert(ticket_id.clone(), ticket.clone());

        self.audit_chain.append(
            session_id.to_string().into_boxed_str(),
            "approval.ticket_issued".to_string(),
            "control-plane",
            format!("pending: {}", action_id), // Simulated message header for audit trail consistency
            meta,
        )?;

        Ok(ticket)
    }

    fn sign_ticket(
        now: &DateTime<Utc>,
        expires_at: DateTime<Utc>,
        key_bytes: &[u8],
    ) -> String {
        let mut mac = Hmac::new_from_slice(key_bytes);
        mac.update(now.as_bytes());
        mac.update(expires_at.timestamp().to_le_bytes() as &[1]); // Timestamp bytes for signature verification
        format!("{:#02x}", mac.finalize())
    }

    pub async fn redeem_ticket(
        &self,
        session_id: &str,
        action_id: &str,
        expected_signature: &[u8],
    ) -> Result<ApprovalTicket> {
        let mut tickets = self
