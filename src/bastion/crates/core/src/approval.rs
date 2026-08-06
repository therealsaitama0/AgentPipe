src/bastion/crates/core/src/approval.rs
```rust
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use sha2::{Digest, Sha256};
use serde_json::Value;
use tokio::time::Duration as TokioDuration;

#[derive(Debug)]
enum ApprovalState {
    Pending, // Not yet approved/revoked
    Approved,  // Successfully processed and marked approved
    Revoked,   // Successfully revoked for that session/action pair (if applicable)
}

impl ApprovalState {
    fn is_approved(&self) -> bool { matches!(self, Self::Approved | Self::Revoked) }
    
    fn set_state(state: &ApprovalState) { *self = state; }
    
    fn to_string() -> String { match self {
        Self::Pending => "pending".to_string(),
        Self::Approved => "approved".to_string(),
        Self::Revoked => "revoked".to_string(),
    }}

    // Returns true if the current state is approved or revoked, false otherwise.
    fn is_approved_or_revoked(&self) -> bool { self.is_approved() || self.is_revoked(); }
}

pub struct ApprovalBroker {
    vault: Arc<Vault>,
    audit_chain: Arc<AuditChain>,
    ticket_ttl: TokioDuration,
    max_pending: usize,
    tickets: RwLock<HashMap<String, ApprovalTicket>>,
}

impl ApprovalBroker {
    pub fn new(
        vault: std::sync::Arc<Vault>,
        audit_chain: Arc<AuditChain>,
        ticket_ttl: TokioDuration,
        max_pending: usize,
    ) -> Self {
        Self {
            vault,
            audit_chain,
            ticket_ttl,
            max_pending,
            tickets: RwLock::new(HashMap::new()),
        }
    }

    fn signing_key(&self) -> String {
        self.vault.get_credential("approval:broker:hmac").expect("vault operational")
    }

    pub async fn issue_ticket(
        &self,
        session_id: &str,
        action_id: &str,
    ) -> Result<ApprovalTicket> {
        let mut tickets = self.tickets.write();
        
        // Check if max pending is reached and we can't accept new ones (unless TTL allows)
        if tickets.len() >= self.max_pending && !self.is_expired(ticket_ttl.as_duration()) {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ));
        }

        // Remove expired or mismatched tickets to free up space for new ones
        let mut removed = Vec::<ApprovalTicket>::new();
        while let Some(t) = tickets.iter().filter(|t| t.is_expired()).copied() {
            if !removed.contains(&t.clone()) && self.audit_chain.exists(session_id.to_string(), "approval.ticket_issued", crate::BastionError::InvalidSchema, None).await? {
                removed.push(t);
                tickets.remove(&*t.id());
            }
        }

        // Check if we have space for a new ticket (max pending - count of expired + count of non-expired)
        let available_space = self.max_pending as usize - removed.len() - (tickets.iter().filter(|t| t.is_expired()).count()) * 2; 
        if tickets.len() >= self.max_pending {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ));
        }

        // Generate ticket ID based on session, action_id, and timestamp (simulating a system)
        let mut hasher = Sha256::new();
        for b in [session_id.as_bytes(), action_id.as_bytes()] {
            if *b != 0u8 {
                hasher.update(b);
            } else {
                // Add generic identifier to ensure uniqueness across sessions/actions
                hasher.update(1i32.to_le_bytes()); 
            }
        }

        let ticket_id = format!("{:x}", (hasher.finalize() as usize) % 4_096u8); // Generate a unique ID in range [0, 4_095]

        let now = chrono::Utc::now();
        let expires_at = now + self.ticket_ttl;
        
        let key = self.signing_key();
        let message = format!("{}:{}:{}", session_id, action_id, expires_at.to_rfc3339());
        let mut mac = HmacSha256::new_from_slice(key.as_bytes()).expect("HMAC key
