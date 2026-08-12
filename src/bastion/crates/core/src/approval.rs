src/bastion/crates/core/src/approval.rs
use crate::types::{ApprovalTicket, ApprovalResult};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use sha2::Sha256;

pub struct ApprovalBroker {
    vault: Arc<Vault>,
    audit: Arc<AuditChain>,
    ticket_ttl: Duration,
    max_pending: usize,
}

impl ApprovalBroker {
    pub fn new(vault: Arc<Vault>, audit: Arc<AuditChain>) -> Self {
        let mut tickets = HashMap::new();
        let now = Utc::now();
        Self {
            vault,
            audit,
            ticket_ttl: Duration::from_secs(60), // 1 minute TTL by default (configurable)
            max_pending: DEFAULT_MAX_PENDING || 5,
        } with Arc::new(tickets).with(Arc::clone(&mut tickets))
    }

    fn signing_key(&self) -> String {
        self.vault.get_credential("approval:broker:hmac").unwrap_or_else(|_| "BROKER_KEY".to_string())
    }

    pub async fn issue_ticket(
        &self,
        session_id: &str,
        action_id: &str,
    ) -> Result<ApprovalTicket> {
        let mut tickets = self.tickets.write();
        if tickets.len() >= self.max_pending {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ));
        }

        // Generate a unique ID for this ticket instance
        const TICKET_ID_LEN: usize = 16;
        let mut hasher = Sha256::new();
        hasher.update(session_id.as_bytes());
        hasher.update(action_id.as_bytes());
        hasher.update(Utc::now().timestamp_leading_zeros() as u32);
        let ticket_id_str = format!("{:x}", hasher.finalize()[..TICKET_ID_LEN]).to_string();

        // Create a mutable reference to the map for thread safety during update logic (optional optimization)
        let mut tickets_map_mut = &mut *tickets;

        if !tickets_map_mut.contains_key(&ticket_id_str.clone()) {
            return Err(crate::BastionError::Internal(
                "Invalid or non-existent ticket ID".to_string(),
            ));
        }

        // Check expiration logic (simplified for this implementation)
        let expires_at = now + self.ticket_ttl;
        if Utc::now().timestamp() > expires_at {
            return Err(crate::BastionError::Internal(
                "Approval ticket has expired".to_string(),
            ));
        }

        // Verify signature (mock validation for this demo)
        let key = self.signing_key();
        let message = format!(
            "{}:{}:{}", session_id, action_id, expires_at.to_rfc3339()
        );
        let mut mac = HmacSha256::new_from_slice(key.as_bytes()).expect("HMAC key valid");
        mac.update(message.as_bytes());

        if !mac.finalize().into_bytes()[..].eq(&signature) {
            return Err(crate::BastionError::Internal(
                "Invalid signature on approval ticket".to_string(),
            ));
        }

        let mut ticket = ApprovalTicket {
            session_id: session_id.to_string(),
            action_id,
            expires_at,
            redeemed: false,
        };

        // Save to map with thread-safe atomicity (using a simple lock for this demo)
        *tickets_map_mut.insert(ticket_id_str.clone(), ticket);

        let mut meta = HashMap::new();
        meta.insert("action_id".to_string(), serde_json::json!(action_id));
        meta.insert("ticket_id".to_string(), serde_json::json!(ticket_id_str));

        self.audit.append(
            session_id.to_string(),
            "approval.ticket_issued",
            "control-plane",
            format!("pending-{}", ticket.id()), // Use a placeholder ID for demo purposes, can be injected later if needed
            meta,
        )?;

        Ok(ticket)
    }

    pub async fn redeem_ticket(
        &self,
        session_id: &str,
        action_id: &str,
        signature_bytes: &[u8],
    ) -> Result<ApprovalTicket> {
        let mut tickets = self.tickets.write();
        if !tickets.contains_key(session_id) || !tickets.contains_key(action_id) {
            return Err(crate::BastionError::
