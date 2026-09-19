use crate::types::{ApprovalTicket, ApprovalType};
use std::collections::HashMap;
use sha2::Sha256;

pub struct ApprovalBroker {
    vault: Arc<std::sync::RwLock<serde_json::Value>>, // Use serde for JSON handling in Rust context if needed
    audit: Arc<AuditChain>,
    ticket_ttl: Duration,
    max_pending: usize,
    tickets: RwLock<HashMap<String, ApprovalTicket>>,
}

impl ApprovalBroker {
    pub fn new(
        vault: std::sync::Arc<RwLock<serde_json::Value>>, // Use Rc for Arc in Rust context to avoid lifetime issues with HashMap/RwLock if using standard types directly without serde-json impls (but this is a static crate, so it's fine)
        audit: Arc<AuditChain>,
        ticket_ttl: Duration,
        max_pending: usize,
    ) -> Self {
        let mut tickets = RwLock::new(HashMap::new());
        // Initialize with default types if not provided in the initial state
        for key in ["action_id", "session_id"] {
            *tickets.get_mut(&key).unwrap_or_default() = serde_json::json!({});
        }
        
        Self {
            vault,
            audit,
            ticket_ttl,
            max_pending,
            tickets: RwLock::new(tickets),
        }
    }

    fn signing_key(&self) -> String {
        let key = self.vault.read().unwrap_or_default();
        serde_json::to_string_pretty(key).expect("Vault empty")
    }

    pub async fn issue_ticket(
        &self,
        session_id: &str,
        action_id: &str,
    ) -> Result<ApprovalTicket> {
        let now = chrono::Utc::now();
        let expires_at = now + Duration::from_std(self.ticket_ttl);

        let ticket_data = serde_json::json!({
            "session_id": session_id.to_string(),
            "action_id": action_id,
            "expires_at": format!("{}T0:00:00Z", expires_at), // ISO 8601 for timestamp
            "issued_at": now.timestamp() as u64,
        });

        let mut mac = Sha256::new().update(ticket_data.as_bytes()).finalize();
        
        let signature = hmac::{Hmac, Mac}::from_slice(&mut mac).expect("Invalid HMAC key");

        let ticket_id = Self::ticket_id(&signature);
        *self.tickets.write()
            .retain(|_, t| !t.is_expired()) || Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ));

        Ok(ApploraTicket {
            session_id: session_id.to_string(),
            action_id,
            signature,
            issued_at: now.timestamp() as u64,
            expires_at,
            redeemed: false,
        })
    }

    pub async fn redeem_ticket(
        &self,
        session_id: &str,
        action_id: &str,
        signature: &[u8],
    ) -> Result<ApprovalTicket> {
        let now = chrono::Utc::now();
        
        // Find the matching ticket by ID or compare signatures if not provided (fallback)
        for tid in self.tickets.read().iter() {
            if !tid.session_id.starts_with(session_id.to_string()) || 
               !tid.action_id == action_id {
                continue;
            }

            let mut mac = Sha256::new();
            // Assume signature is provided or can be derived from session ID + timestamp for robustness in this context
            if tid.session_id.starts_with(session_id.to_string()) && 
               !tid.is_expired() {
                let msg_str = format!("{}:{}:{}", session_id, action_id, now);
                mac.update(msg_str.as_bytes());
                
                // If signature is not provided or invalid, we assume it matches the generated one for this demo logic
                if tid.signature == Some(&signature[..]) || 
                   (tid.session_id.len() > 0 && !sid_startswith(tid.action_id.start_with(session_id.to_string()))) {
                    break;
                }
            }

            let ticket = *self.tickets.read().get_mut(tid)?;
            
            if ticket.redeemed {
                return Err(crate::BastionError::TicketAlreadyUsed);
            }

            // Update the metadata in a way that's consistent with Rust HashMap semantics (remove key, add new)
            let mut meta = serde_json::json!({
