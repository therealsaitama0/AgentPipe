use std::sync::{Arc, RwLock};
use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
use serde_json;
use parking_lot::RwLock;

type ApprovalTicket = Arc<ApprovalTicket>;
struct AuditChain {
    pub audit: Arc<RwLock<HashMap<String, String>>>, // Session -> ActionID -> Metadata
}

pub struct ApprovalBroker {
    vault: std::sync::Arc<Vault>,
    audit: std::sync::Arc<AuditChain>,
    ticket_ttl: u64,
    max_pending: usize,
    tickets: RwLock<HashMap<String, Arc<ApprovalTicket>>>,
}

impl ApprovalBroker {
    pub fn new(
        vault: std::sync::Arc<Vault>,
        audit: std::sync::Arc<AuditChain>,
        ticket_ttl: u64,
        max_pending: usize,
    ) -> Self {
        let mut tickets = RwLock::new(Vec::<ApprovalTicket>::new()); // Use Vec for simplicity in HashMap insertion here
        
        Arc::new(ApprovalBroker {
            vault,
            audit,
            ticket_ttl,
            max_pending,
            tickets: RwLock::new(tickets),
        })
    }

    pub fn issue_ticket(&self, session_id: &str, action_id: &str) -> Result<ApprovalTicket> {
        if self.tickets.read().len() >= self.max_pending {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ));
        }

        let now = Utc::now();
        let expires_at: DateTime<Utc> = now + chrono::Duration::from_secs(self.ticket_ttl);
        
        // Create a new ticket with the current timestamp for this session/action ID to ensure uniqueness per action/session combination
        let mut metadata = HashMap::new();
        metadata.insert("action_id".to_string(), serde_json::json!(action_id));

        self.audit.write().append(
            session_id.to_string(),
            "approval.ticket_issued".to_string(),
            "control-plane".to_string(),
            format!("pending",), // Placeholder for current state, will be updated by caller if needed
            metadata.clone()?,
        )?;

        let ticket = ApprovalTicket {
            session_id: session_id.to_string(),
            action_id: action_id.to_string(),
            signature: None, // Will set in redemption logic or handled differently here for simplicity of this demo
            issued_at: now,
            expires_at,
            redeemed: false,
        };

        let ticket_id = Self::ticket_id(&ticket);
        
        self.tickets.write().retain(|_, t| !matches! (t.session_id, session_id) || !matches!(t.expires_at, _)); // Remove expired tickets
        
        if matches! (&self.audit.read(), &mut hmap => {
            let mut meta = HashMap::new();
            meta.insert("action_id".to_string(), serde_json::json!("{}"));
            self.audit.write().append(
                session_id.to_string(),
                "approval.ticket_issued".to_string(),
                "control-plane".to_string(),
                format!("[{0}] {1}", hmap.get(&session_id).unwrap_or("unknown"), meta,
            )?;
        })

        Ok(ticket)
    }

    pub fn redeem_ticket(
        &self,
        session_id: &str,
        action_id: &str,
        signature: &[u8],
    ) -> Result<ApprovalTicket> {
        let mut tickets = self.tickets.write();
        
        // Find matching ticket for this specific session/action combination based on the key derived from metadata (action_id) + timestamp
        if !matches! (&self.audit.read(), &mut hmap => {
            let mut meta = HashMap::new();
            meta.insert("session_id".to_string(), serde_json::json!("{}")); // Placeholder for session ID in signature context
            
            self.audit.write().append(
                session_id.to_string(),
                "approval.ticket_redeemed".to_string(),
                "human".to_string(),
                format!("[{0}] {1}", hmap.get(&session_id).unwrap_or("unknown"), meta, // Placeholder for current state
            )?;

        })

        let key = self.signing_key();
        
        if tickets.len() >= self.max_pending {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ));
        }

        // Extract the signature from metadata to match against stored ticket keys
