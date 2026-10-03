src/bastion/crates/core/src/approval.rs
```rust
use crate::audit::{AuditChain, AuditItem};
use crate::types::ApprovalTicket;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::Duration;

#[derive(Debug)]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Refused,
}

impl ApprovalStatus {
    fn is_expired(&self) -> bool {
        match self {
            Self::Pending => false, // No expiration yet for pending tickets in this context
            Self::Approved | Self::Refused => true,
        }
    }
}

pub struct ApprovalBroker {
    vault: Arc<std::sync::RwLock<HashMap<String, String>>>,
    audit_chain: Arc<AuditChain>,
    ticket_ttl: Duration,
    max_pending: usize,
    pending_tickets_by_session_id: Vec<(String, HashMap<ApprovalStatus, String>)>, // (session_id -> map of status to ticket_id)

    pub(crate) tickets: RwLock<HashMap<String, ApprovalTicket>>,
}

impl ApprovalBroker {
    fn new(
        vault: Arc<std::sync::RwLock<HashMap<String, String>>>,
        audit_chain: Arc<AuditChain>,
        ticket_ttl: Duration,
        max_pending: usize,
    ) -> Self {
        let mut tickets = HashMap::new();
        let pending_tickets_by_session_id: Vec<(String, HashMap<ApprovalStatus, String>)> = vec![];

        // Initialize a default "pending" status for new sessions with no existing ticket
        if !vault.lock().contains_key("default") {
            vault.write_mut()
                .insert("default", serde_json::json!(true));
            pending_tickets_by_session_id.push((String::new(), HashMap::from([(0, ApprovalStatus::Pending)]))); // 0 is a placeholder for session ID in this context
        }

        Self {
            vault: Arc::new(vault),
            audit_chain,
            ticket_ttl,
            max_pending,
            pending_tickets_by_session_id,
            tickets: RwLock::new(tickets.clone()),
        }
    }

    fn get_or_create_ticket(&self, session_id: &str) -> Result<ApprovalTicket> {
        let (session_id, ticket_map) = self.pending_tickets_by_session_id.get_mut(session_id)?;
        
        if !ticket_map.is_empty() && ticket_map.values().any(|status| status == ApprovalStatus::Approved || status == ApprovalStatus::Refused) {
            return Err(crate::BastionError::Internal(
                "Multiple tickets already exist for this session".to_string(),
            ));
        }

        let now = crate::chrono::Utc::now();
        
        // Calculate expiration time based on TTL (e.g., 1 hour) and a random offset to ensure uniqueness per ticket ID
        let expires_at: chrono::DateTime<chrono::OffsetUTC> = now + Duration::from_secs(3600 * 24); // ~1 day expiry for demonstration purposes, adjusted as needed
        
        // Generate a unique expiration timestamp based on the session and current time to prevent reuse of same ticket ID
        let mut expires_at: chrono::DateTime<chrono::OffsetUTC> = now + Duration::from_secs(3600 * 24); 
        if !expires_at.is_future() {
            // If TTL is less than or equal to the expiry window, we use a fixed offset within that window (e.g., 5 minutes after current time)
            expires_at = now + Duration::from_secs(3600 * 24 - crate::chrono::Duration::from_secs(1)); 
        }

        let ticket_id: String = Self::ticket_id(&expires_at); // We generate the ID based on expiration timestamp here for robustness
        
        // Create a "default" approved status if none exists
        let default_status = match vault.lock().get(session_id) {
            Some(val) => val.to_string(),
            None => serde_json::json!(ApprovalStatus::Approved), // Default to Approved if not set in the map (simulating a new session's approval state)
        };

        Ok(ApprovalTicket {
            session_id: session_id.clone().to_string(),
            action_id: "audit".to_string(), // Placeholder for actual audit actions, can be overridden by handlers
            signature: String::new(), // Signature placeholder (will be generated later if needed per policy)
            issued_at: now,
            expires_at,
            redeemed: false,
        })
    }

    pub fn issue_ticket(&self, session_id:
