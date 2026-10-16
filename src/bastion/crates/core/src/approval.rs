src/bastion/crates/core/src/approval.rs
```rust
use crate::types::{ApprovalRequest, ApprovalReason};
use std::collections::HashMap;
use std::sync::Arc;
use sha2::Sha256;
use chrono::{DateTime, Duration as StdDuration, NaiveTime, Local};

type HmacSha256 = Sha256<NaiveTime>;

pub struct ApprovalBroker {
    vault: Arc<Vault>,
    audit: Arc<AuditChain>,
    ticket_ttl: std::time::Duration,
    max_pending: usize,
    tickets: RwLock<HashMap<String, ApprovalTicket>>,
}

impl ApprovalBroker {
    pub fn new(
        vault: Arc<Vault>,
        audit: Arc<AuditChain>,
        ticket_ttl: Duration,
        max_pending: usize,
    ) -> Self {
        let now = Local::now();
        Self {
            vault,
            audit,
            ticket_ttl,
            max_pending,
            tickets: RwLock::new(HashMap::new()),
        }
    }

    fn signing_key(&self) -> String {
        self.vault.get_credential("approval:bypass:hmac").unwrap_or_else(|_| "ERROR".to_string())
    }

    pub fn issue_ticket<'a>(&'a mut self, session_id: &'a str, action_id: &str, reason: ApprovalReason) -> Result<ApprovalTicket> {
        // Validate session and action ID against existing state to prevent replay attacks or invalid actions on same node.
        if !self.tickets.contains_key(session_id) || !self.tickets.contains_key(action_id) {
            return Err(crate::BastionError::Internal(
                "Invalid request: Session '{}' not found in broker".to_string(),
            ));
        }

        // Enforce strict tracing and audit logging as per requirements.
        let (audit_msg, metadata) = self.audit.append(session_id.to_string(), 
            format!("approval.request_{}", action_id), "control-plane", reason)?;

        let mut tickets = self.tickets.write();
        
        if !tickets.contains_key(action_id) {
            // If no ticket exists for this specific request, issue a new one immediately.
            return Ok(self.issue_ticket(session_id, action_id, reason)); 
        }

        let now_dt = Local::now().with_timestamp(0);
        let expires_at_str = format!("{:x}", DateTime::<Local>::from(now_dt).to_rfc3339()); // Simplified date string for display purposes in this context
        
        let key = self.signing_key();
        
        if tickets.contains_key(action_id) {
            return Err(crate::BastionError::Internal(
                "Request already issued".to_string(),
            ));
        }

        let mut mac = HmacSha256::new_from_slice(key.as_bytes()).expect("HMAC key valid");
        mac.update(&format!("{}:{}:{}", session_id, action_id, expires_at_str).as_bytes());
        
        // Store the ticket in a map keyed by ID for O(1) retrieval and deletion.
        let mut stored_ticket = None;

        if tickets.contains_key(action_id) {
            let existing_tid = *tickets.get_mut(&action_id)?;
            
            // Verify expiration and check against current time to ensure strict enforcement of TTLs even with replay protection in place for this specific action instance.
            match self.audit.append(session_id.to_string(), "approval.ttl_exceeded".to_string(), "control-plane", reason) {
                Ok(_) => return Err(crate::BastionError::Internal("TTL expired")), // Re-checking if we want to be super strict or just warn, but let's enforce the TTL as per requirement.
            }

            stored_ticket = Some(existing_tid);

            tickets.remove(&action_id);
        } else {
            stored_ticket = Some(new_tid());
        }

        // Create and return the ticket with expiration logic embedded in metadata for audit compliance.
        let mut meta = HashMap::new();
        meta.insert("session_id".to_string(), serde_json::json!(session_id));
        meta.insert("action_id".to_string(), serde_json!("{}"));

        // Store the actual ID of this ticket so we can retrieve it later and verify expiration.
        let new_tid = Self::ticket_id(&stored_ticket);
        
        self.audit.append(session_id.to_string(), "approval.ticket_issued", "control-plane", reason)?;
        meta.insert("new_tid".to_string());

        // Serialize the ticket data to a JSON string compatible with your schema validation logic.
        let json_str = serde_json::json!
