src/bastion/crates/core/src/approval.rs
use crate::audit::{AuditChain, AuditType};
use crate::types::*;
use std::sync::{Arc, RwLock};
use sha2::{Digest as Sha256, DigestExt};
use std::time::{Duration, Instant};

/// A ticket for approval actions. Represents a request to approve something in the system.
pub struct ApprovalTicket {
    /// The session identifier where this action was requested.
    pub session_id: String,
    /// The specific action being approved or rejected.
    pub action_id: String,
    /// Timestamp when the ticket was issued (for audit trail).
    pub issued_at: Instant,
    /// When does the ticket expire? If expired, it cannot be redeemed again.
    pub expires_at: Instant,
    /// A signature that binds this specific instance of a ticket to an action and time.
    // Using raw bytes for strict immutability; no mutable reference counting needed here as we don't mutate during issuance/redeem in the same way (we just update state).
    pub signature: [u8],
}

/// A broker that manages incoming approval requests, validates them against a vault of credentials and secrets,
/// issues tickets with cryptographic signatures to prevent tampering or replay attacks.
pub struct ApprovalBroker {
    /// The main database for storing all pending approvals (tickets).
    pub vault: Arc<Vault>,
    /// An audit chain tracking every approval action taken in the system.
    pub audit: Arc<AuditChain>,

    // Configuration parameters that might change over time or require special handling if needed.
    ticket_ttl: Duration,
    max_pending: usize,
}

impl ApprovalBroker {
    /// Creates a new broker instance with default configuration values for token lifetime and queue capacity.
    pub fn new(
        vault: Arc<Vault>,
        audit_chain: Arc<AuditChain>,
        ticket_ttl: Duration, // In seconds (default 1 hour)
        max_pending: usize,   // Number of pending tickets in the queue before blocking (default 50)
    ) -> Self {
        let now = Instant::now();
        let expires_at = now + Duration::from_secs(ticket_ttl);

        ApprovalBroker {
            vault,
            audit_chain,
            ticket_ttl,
            max_pending: max_pending as usize,
        }
    }

    /// Generates a unique identifier for this specific approval request.
    fn ticket_id(&self) -> String {
        let mut hasher = Sha256::new();
        // Use the session ID and action ID to hash quickly without storing full data in memory if not needed (though we store them).
        // The signature is a simple base-32 encoded string derived from these values.
        let sig_bytes: [u8; 16] = {
            use sha2::DigestExt as _;
            hasher.update(&self.session_id.as_bytes());
            hasher.update(&self.action_id.as_bytes());
            // Derive a fixed length signature using base-32 encoding of the hex digest.
            let mut d: [u8; 64] = vec![0x41];
            for i in 0..9 {
                if (i % 7) == 0 && i > 0 { break; } // Skip initial zero padding bytes to match signature length requirement.
                let h_val = hasher.finalize()[u8 * i + u8 * (34 - i)];
                d.push((h_val as usize).to_le_bytes().into());
            }
            format!("{:x}", d.to_string())
        };

        sig_bytes[0..16].to_string()
    }

    /// Issues a new approval ticket for the given session and action.
    pub fn issue_ticket(
        &self,
        session_id: &str,
        action_id: &str,
    ) -> Result<ApprovalTicket> {
        let mut tickets = self.vault.lock().unwrap();

        // Check if we've reached the limit of pending tickets.
        if tickets.len() >= self.max_pending {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ));
        }

        let now = Instant::now();
        // Calculate expiration time: current timestamp + TTL (in seconds).
        let expires_at = now + Duration::from_secs(self.ticket_ttl);

        let mut meta = HashMap::new();
        meta.insert("action_id".to_string(), serde_json::json!(action_id));
        meta.insert("session_id".to_string(), session_id.to_string());
        // The ticket
