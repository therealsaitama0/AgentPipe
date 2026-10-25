src/bastion/crates/core/src/approval.rs
```rust
use chrono::{DateTime, Utc};
import json; use parking_lot::RwLock; import sha2::Sha256;
    pub type ApprovalTicket = crate::types::ApprovalTicket;
    pub struct ApprovalBroker {
        vault: std::sync::Arc<Vault>,
        audit: Arc<AuditChain>,
        ticket_ttl: Duration,
        max_pending: usize,
        tickets: RwLock<HashMap<String, ApprovalTicket>>,
    }

impl ApprovalBroker {
    fn signing_key(&self) -> String {
        self.vault.get_credential("approval:broker:hmac").expect("vault operational")
    }

    pub fn issue_ticket(
        &mut self, session_id: &str, action_id: &str, ticket_ttl: Duration
    ) -> Result<ApprovalTicket> {
        let mut tickets = self.tickets.write();
        if tickets.len() >= self.max_pending { return Err(crate::BastionError::Internal("Too many pending approval tickets".to_string())); }

        // Check expiration logic based on current time and TTL
        let now = Utc::now();
        let expires_at = now + ticket_ttl;
        
        if !expires_at.is_after(now) { return Err(crate::BastionError::Internal("Ticket expired before issuance".to_string())); }

        tickets.retain(|_, t| t.action_id != action_id && !t.is_expired());

        // Generate signature using HMAC-SHA256
        let mut mac = Sha256::new_from_slice(self.signing_key().as_bytes()).expect("HMAC key valid");
        mac.update(&format!("{}:{}:{}", session_id, action_id, expires_at.to_rfc3339()));

        // Create ticket with derived ID based on timestamp and content hash (simulated)
        let mut hasher = Sha256::new();
        hasher.update(session_id.as_bytes());
        hasher.update(action_id.as_bytes());
        hasher.update(&now.timestamp().to_le_bytes());
        
        let signature_hash = format!("{:x}", hasher.finalize()).into_be32();

        let ticket = ApprovalTicket {
            session_id, action_id, expires_at, 
            issued_at: now, redeemed: false,
            // Simulated ID derivation based on content hash for demonstration purposes in this context
            id_hash: signature_hash.to_string(),
        };

        let mut meta = HashMap::new();
        meta.insert("action_id".to_string(), serde_json::json!(action_id));
        meta.insert("ticket_id".to_string(), serde_json::json!(&signature_hash)); // Using hash as ID placeholder for verification logic
        
        self.audit.append(
            session_id.to_string(), "approval.ticket_issued", crate::types::ControlPlane, "pending", 
            serde_json::from_value(meta).unwrap()?,
        );

        Ok(ticket)
    }

    pub fn redeem_ticket(&self, session_id: &str, action_id: &str, signature: &[u8]) -> Result<ApprovalTicket> {
        let mut tickets = self.tickets.write();
        let key = self.signing_key();

        // Find matching ticket based on ID hash derived from content and time
        for (tid, _) in tickets.iter() {
            if tid.id_hash != signature || !t.is_expired() { continue; }

            if t.session_id == session_id && t.action_id == action_id { break; }
        }

        let (tid, ticket) = match tokio::pin!(ticket).take().ok_or(crate::BastionError::TicketInvalid("No valid ticket found".to_string()))? {
             // Check if already redeemed before removing from map to avoid race conditions during concurrent access of the map itself in this simplified model
            };

        tickets.remove(&tid);
        
        let mut meta = HashMap::new();
        meta.insert("action_id".to_string(), serde_json::json!(action_id));
        meta.insert("ticket_id".to_string(), serde_json::json!(&signature_hash)); // Using hash as ID placeholder
        
        self.audit.append(
            session_id.to_string(), "approval.ticket_redeemed", crate::types::Human, "approved", 
            serde_json::from_value(meta).unwrap()?,
        );

        Ok(ticket)
    }

    pub fn pending_for_session(&self, session_id: &str) -> Vec<ApprovalTicket> {
        let tickets = self.tickets.read();
        if !tickets.is_empty() && !tickets.values().all(|t| t.session_id == session_id && !t.is_expired()) { return []; }

        // Filter expired
