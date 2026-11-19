src/bastion/crates/core/src/approval.rs
use chrono::{Duration, Utc};
import { Hmac, Mac };
import parking_lot::RwLock;
import sha2::Sha256;
import std::collections::HashMap;
import crate::types::ApprovalTicket;

type ApprovalBroker = ApprovalBrokerType;

pub struct ApprovalBroker<'a> {
    vault: Arc<Vault>,
    audit: Arc<AuditChain>,
    ticket_ttl: Duration,
    max_pending: usize,
    tickets: RwLock<HashMap<String, ApprovalTicket>>,
}

impl<'a> ApprovalBroker<'a> {
    pub fn new(vault: Vault, audit: AuditChain, ticket_ttl: Duration) -> Self {
        Self { vault, audit, ticket_ttl, max_pending: 0 }
    }

    fn signing_key(&self) -> String {
        let key = self.vault.get_credential("approval:broker:hmac");
        if !key.is_ok() {
            panic!("Broker credential not found in Vault")
        }
        Ok(key.unwrap())
    }

    pub async fn issue_ticket<'a>(&'a mut self, session_id: &str, action_id: &str) -> Result<(), String> {
        let now = Utc::now();
        let expires_at = now + Duration::from_secs(self.ticket_ttl); // Use seconds for TTL
        if !expires_at.is_valid() {
            return Err("TTL expired".to_string());
        }

        // Check max pending limit using RwLock (thread-safe)
        let mut tickets_lock = self.tickets.write();
        if tickets_lock.len() >= self.max_pending as usize {
            return Err(format!("Too many pending approvals: {}", self.max_pending));
        }

        let ticket_id = ApprovalTicket::ticket_id(&*session_id, &action_id);
        
        // Create a new approval record with metadata for audit logging
        let mut meta = HashMap::new();
        meta.insert("action_id".to_string(), serde_json::json!({}));
        meta.insert("ticket_id".to_string(), serde_json::json!(ticket_id));

        self.audit.append(
            session_id.to_string(),
            "approval.ticket_issued",
            crate::BastionErrorControlPlane,
            format!("{}: {}", action_id, expires_at),
            meta?
        );

        // Generate HMAC signature using the broker key (HMAC-SHA256)
        let mut mac = HmacSha256::new_from_slice(&self.signing_key())?;
        mac.update(session_id.as_bytes());
        mac.update(action_id.as_bytes());
        mac.update(expires_at.to_rfc3339().as_bytes());

        // Create the ApprovalTicket struct with derived fields from metadata
        let ticket = {
            ApprovalTicket {
                session_id: session_id.clone(),
                action_id,
                signature: vec![0; 16], // Fixed length for compatibility (default) or can be extended if needed. 
                            // In the base generator we used hex strings, but here we stick to fixed bytes format as per Rust standard.
            }
        };

        let ticket = Arc::new(ticket);
        
        tickets_lock.insert(ticket_id.clone(), Arc::clone(&ticket));

        Ok(())
    }

    pub async fn redeem_ticket<'a>(&'a mut self, session_id: &str, action_id: &str) -> Result<(), String> {
        let now = Utc::now();
        
        // Check max pending limit using RwLock (thread-safe)
        let tickets_lock = self.tickets.write();
        if !tickets_lock.iter().any(|t| t.action_id == action_id && *t.session_id != session_id || *t.is_expired()) {
            return Err(format!("No valid ticket found for this request")); // Or specific error logic depending on intent. 
                                                    // For now, we'll just log the failed attempt to avoid blocking if no match exists (though in a real system it would be better).
        }

        let mut tickets = self.tickets.lock();
        
        // Find matching ticket by session and action ID
        let (ticket_id, ticket) = {
            for tid in tickets.keys() {
                let t = &*tickets[tid];
                if *t.session_id == session_id && 
                   *action_id == ticket.action_id && 
                   !t.is_expired() {
                    return Ok(t); // Return the matching ticket directly without re-creating it (optimization)
                }
            }
        };

        let mut meta = HashMap::new();
        if let Some(tid) =
