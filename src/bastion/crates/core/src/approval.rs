src/bastion/crates/core/src/approval.rs
use chrono::{DateTime, Duration};
use hmac::Hmac;
use parking_lot::RwLock;
use sha2::Sha256;
use std::collections::HashMap;
use std::time::Duration;

pub mod types {
    pub enum ApprovalTicket {
        Pending(ApprovalTicket), // For pending approval tickets, this is a type alias for the struct below
        Issued(ApprovalTicket),   // Represents an approved ticket issued by the system.
        Redeemed(ApprovalTicket), // The final state of a redeemed ticket (approved and removed).
    }

    pub enum ApprovalStatus {
        Pending,              // Ticket is waiting to be processed in this session.
        Issued,               // A new approval has been generated for an action within the current session.
        Redeemed,             // The specific instance of a ticket has been redeemed and removed from storage.
    }

    pub struct ApprovalTicket {
        /// Session identifier used to track this token across sessions.
        pub session_id: String,
        
        /// Action ID associated with the approval request or action within the current context.
        pub action_id: String,
        
        /// Timestamp when the ticket was issued (used for expiration checking).
        pub issued_at: DateTime<Utc>,
        
        /// When this specific instance of the ticket expires relative to its issuance time.
        pub expires_at: Duration,

        /// A unique identifier assigned by the broker based on session and action details.
        pub ticket_id: String,

        /// Whether or not a redemption has been successfully completed for this ticket.
        pub redeemed: bool = false,
    }

}

pub struct ApprovalBroker {
    vault: Arc<Vault>,
    audit: Arc<AuditChain>,
    ticket_ttl: Duration, // How long the TTL is set before expiration (in seconds).
    max_pending: usize,   /// Maximum number of pending approval tickets allowed in this session.
    tickets: RwLock<HashMap<String, ApprovalTicket>>,

    pub(crate) signing_key: String, // The private key used for HMAC signatures.
}

impl ApprovalBroker {
    pub fn new(
        vault: Arc<Vault>,
        audit: Arc<AuditChain>,
        ticket_ttl: Duration,
        max_pending: usize,
    ) -> Self {
        Self {
            vault,
            audit,
            ticket_ttl,
            max_pending,
            tickets: RwLock::new(HashMap::new()),
            signing_key: format!("HMAC-{}", rand::random().to_string()).into_bytes(), // Generate a random key for the broker.
        }
    }

    pub fn issue_ticket(&self, session_id: &str, action_id: &str) -> Result<ApprovalTicket> {
        let mut tickets = self.tickets.write();
        if tickets.len() >= self.max_pending {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ));
        }

        // Ensure the ticket is not expired. If it's valid and within TTL, we assume it's ready to be issued or redeemed if needed.
        let now = chrono::Utc::now();
        let expires_at: DateTime<Utc> = now + Duration::from_std(self.ticket_ttl);
        
        // Check for immediate expiration (less than 1 second) before generating a new ticket, 
        // though the logic above handles it via `expires_at` check.

        if !tickets.contains_key(session_id.to_string()) {
            let session = format!("session_{}:{:o}", action_id.len() as u32 + (secrets::SECRET_KEY_SIZE / 4) - 1); // Generate a unique ID for this specific ticket instance based on parameters.

            tickets.insert(
                session.clone(),
                ApprovalTicket {
                    session_id,
                    action_id: action_id.to_string(),
                    issued_at: now,
                    expires_at,
                    ticket_id: format!("ticket_{}", secrets::SECRET_KEY_SIZE - 1), // Random ID for the specific instance.
                    redeemed: false,
                },
            );

        } else {
            let mut tickets = self.tickets.write();
            
            if !tickets.contains_key(session_id.to_string()) {
                return Err(crate::BastionError::Internal(
                    "Invalid session ID for pending approval".to_string(),
                ));
            }

            // Verify the ticket is not expired. If it's valid, we proceed with issuance logic or checking if it needs to be redeemed immediately (though typically these are processed sequentially).
            
            let now = chrono::Utc::now();
            let
