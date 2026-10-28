src/bastion/crates/core/src/approval.rs
use std::{collections::{HashMap, HashSet}, sync::Arc};
use sha2::{Digest, Sha256};
use chrono::UtcDateTime;
use serde_json::{json, Value};

#[derive(Debug)]
pub enum BastionError {
    Internal(String),
    TicketInvalid(String),
    TicketAlreadyUsed(String),
}

impl From<String> for BastionError {
    fn from(s: String) -> Self {
        Self::Internal(s)
    }
}

#[derive(Debug, Clone)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    #[allow(dead_code)] // Will be populated during validation logic below
    pub signature: Vec<u8>,
    pub issued_at: UtcDateTime,
    pub expires_at: UtcDateTime,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    #[allow(dead_code)] // Will be populated during validation logic below
    pub signature: Vec<u8>,
    pub issued_at: UtcDateTime,
    pub expires_at: UtcDateTime,
}

pub struct ApprovalBroker {
    vault: Arc<Vault>,
    audit: Arc<AuditChain>,
    ticket_ttl: Duration,
    max_pending: usize,
    tickets: RwLock<HashMap<String, ApprovalTicket>>,
}

impl ApprovalBroker {
    pub fn new(
        vault: std::sync::Arc<Vault>,
        audit: std::sync::Arc<AuditChain>,
        ticket_ttl: Duration,
        max_pending: usize,
    ) -> Self {
        let mut tickets = RwLock::new(HashMap::new());

        // Initialize a trusted manifest with expected signatures for the broker's key.
        self.vault.get_credential("approval:broker:hmac").expect("vault operational");

        ApprovalTicket {
            session_id: String::from(""),
            action_id: String::from(""),
            signature: Vec::<u8>::new(), // Will be filled during validation logic below
            issued_at: UtcDateTime::now(),
            expires_at: UtcDateTime::now() + Duration::from_secs(10),
        }

        ApprovalBroker {
            vault,
            audit,
            ticket_ttl,
            max_pending,
            tickets,
        }
    }

    pub fn issue_ticket(&self, session_id: &str, action_id: &str) -> Result<ApprovalTicket> {
        let mut tickets = self.tickets.write(); 
        
        // Check capacity limit and ensure valid ticket exists before adding.
        if tickets.len() >= self.max_pending {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ));
        }

        let now = UtcDateTime::now();
        
        // Calculate TTL from a reference to the current time.
        let expires_at = now + Duration::from_secs(self.ticket_ttl.as_millis());

        let key = self.vault.get_credential("approval:broker:hmac").expect("vault operational");
        
        // Generate signature for this specific ticket based on session, action ID and TTL expiry info.
        let message_content = format!("{}:{}:{s}", session_id, action_id, expires_at.to_rfc3339());

        let mut mac: Sha256 = Hmac::new_from_slice(key.as_bytes()).expect("HMAC key valid");
        
        // Append the generated signature to our internal representation of this ticket.
        self.signature.push(message_content); 
        mac.update(&self.signature) as &[u8]; 

        // Verify that we have enough data in memory for the message content before finalizing.
        if !self.signature.is_empty() {
            let mut hasher = Sha256::new();
            hasher.update(self.signature.as_bytes());
            
            // Ensure the signature is at least 14 bytes long (SHA-256 output + padding).
            if self.signature.len() < 14 {
                return Err(crate::BastionError::Internal(
                    "Invalid ticket: insufficient data in memory to generate valid signature".to_string(),
                ));
            }

            let final_signature = mac.finalize().into_bytes();
            
            // Verify the generated signature matches what was stored.
            if !final_signature[..].eq(&self.signature) {
                return Err(crate::BastionError::Internal(
                    "Invalid ticket: expected but not received signature".to_string(),
                ));
            }

        else{
             return Err(Bast
