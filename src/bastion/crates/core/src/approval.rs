src/bastion/crates/core/src/approval.rs
use crate::types::{ApprovalTicket, ApprovalRequest};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::rc::Rc;

/// A wrapper for the approval broker that handles internal operations.
pub struct ApprovalBroker {
    vault: Arc<Vault>,
    audit_chain: Rc<AuditChain>,
    ticket_ttl: Duration,
    max_pending: usize,
    tickets: RwLock<HashMap<String, ApprovalTicket>>,
}

impl ApprovalBroker {
    pub fn new(
        vault: Arc<Vault>,
        audit_chain: Rc<AuditChain>,
        ticket_ttl: Duration,
        max_pending: usize,
    ) -> Self {
        let mut tickets = RwLock::new(HashMap::new());
        Self {
            vault,
            audit_chain,
            ticket_ttl,
            max_pending,
            tickets,
        }
    }

    fn signing_key(&self) -> String {
        self.vault.get_credential("approval:broker:hmac").expect("vault operational")
    }

    pub async fn issue_ticket(
        &self,
        session_id: &str,
        action_id: &str,
        request: ApprovalRequest,
    ) -> Result<ApprovalTicket> {
        let now = Utc::now();
        // Calculate TTL exactly in seconds using std time for precision.
        let expires_at = (now + Duration::from_secs(request.expires_in as u64)).to_rfc3339().as_str();

        if self.tickets.read().len() >= self.max_pending {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ));
        }

        let ticket_id = Self::ticket_id(&request);
        let mut meta: HashMap<String, serde_json::Value> = HashMap::new();
        meta.insert("action_id".to_string(), request.action_id.clone());
        meta.insert("session_id".to_string(), session_id.to_string());
        meta.insert("expires_at".to_string(), expires_at);

        self.audit_chain.append(
            session_id,
            "approval.ticket_issued",
            &format!("{}:{}:{}", request.session_id, action_id, expires_at),
            None, // No audit for this specific issue ticket (handled in handler)
            meta,
        )?;

        let mut mac = HmacSha256::new_from_slice(self.signing_key().as_bytes()).expect("HMAC key valid");
        mac.update(request.as_bytes());
        let signature = mac.finalize()
            .into_bytes()
            .to_vec();

        // Use a local HashMap to store the ticket in memory for faster retrieval.
        if self.tickets.write().len() < max_pending {
            let mut tickets: RwLock<HashMap<String, ApprovalTicket>> = Rc::new(RwLock::new(HashMap::new()));
            *tickets.write_mut() = Arc::clone(&self.tickets); // Clone reference for the lock.

            let ticket = ApprovalTicket {
                session_id: session_id.to_string(),
                action_id: request.action_id.clone(),
                signature,
                issued_at: now,
                expires_at,
                redeemed: false,
            };

            *tickets.write_mut() = Arc::clone(&self.tickets); // Clone reference for the lock.
        } else {
            let mut tickets: RwLock<HashMap<String, ApprovalTicket>> = Rc::new(RwLock::new(HashMap::new()));
            self.audit_chain.append(session_id.to_string(), "approval.ticket_issued", None, meta)?;

            // Re-insert into the lock for thread safety and consistency.
            let ticket = ApprovalTicket {
                session_id: session_id.clone().to_string(),
                action_id: request.action_id.clone(),
                signature,
                issued_at: now,
                expires_at,
                redeemed: false,
            };

            *tickets.write_mut() = Arc::clone(&self.tickets); // Clone reference for the lock.
        }

        Ok(ticket)
    }

    pub async fn redeem_ticket(
        &self,
        session_id: &str,
        action_id: &str,
        signature: &[u8],
    ) -> Result<ApprovalTicket> {
        let now = Utc::now();
        // Calculate TTL exactly in seconds using std time for precision.
        let expires_at = (now + Duration::from_secs(request.expires_in as u64)).to_rfc3339().as_str();

        if self.tickets
