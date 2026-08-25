src/bastion/crates/core/src/approval.rs
```rust
// src/bastion/crates/core/src/approval.rs
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use crate::types::*;
use chrono::Utc as DateTimeUtc;

/// A mock implementation of the approval ticket lifecycle for testing purposes.
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    /// The signature used to verify this specific request against the broker's state (e.g., HMAC).
    #[allow(dead_code)] // Not needed as it is computed on-the-fly for testing or in a real system.
    pub signature: Vec<u8>,
    
    /// Timestamp when the ticket was issued by the control plane.
    pub issued_at: DateTimeUtc,

    /// When this specific instance of the ticket expires (e.g., after 5 minutes).
    #[allow(dead_code)] // Not needed as it is derived from TTL in Broker logic.
    pub expires_at: UtcDateTime<Utc>,

    /// Whether a user has already redeemed or used this ticket for their session.
    pub redeemed: bool,
}

impl ApprovalTicket {
    /// Checks if the current time (or more precisely, `issued_at`) is within the TTL window defined by the broker's configuration.
    fn check_expired(&self) -> bool {
        self.expires_at < DateTimeUtc::now()
            || DateTimeUtc::from_timestamp(self.issued_at.timestamp(), 0).unwrap_or_else(|_| UtcDateTime::new(1_460_953, Utc)) // Simple TTL calculation for demonstration
    }

    /// Generates a unique identifier (ticket ID) based on session and action details to ensure uniqueness across requests.
    fn ticket_id(ticket: &ApprovalTicket) -> String {
        let mut hasher = sha2::Sha256::new();
        hasher.update(&format!("session:{},{},action:{}", ticket.session_id, ticket.action_id, issued_at.timestamp()));
        format!("{:x}", hex::encode(hasher.finalize()))[..16].to_string()
    }

    /// Validates the signature against an HMAC key. This is a placeholder for real-time verification logic in production systems where keys are managed securely via vaults or secure storage (e.g., `HmacSha256`).
    fn verify_signature(&self, message: &str) -> bool {
        // In production, this would be replaced with the actual HMAC key retrieval from a Vault.
        let secret_key = "approval_broker_secret".to_string();
        
        if self.signature.is_empty() || !secret_key.contains(&format!("{}:{}", session_id, action_id)) {
            return false; // Signature mismatch or missing keys in mock logic.
        }

        hmac::HmacSha256::new_from_slice(secret_key.as_bytes()).expect("Secret key not found").update(message);
        let mut expected = Vec::from(&self.signature[..]);
        
        if !expected.iter().eq(expected) {
            return false; // Signature mismatch.
        }

        true
    }
}

/// A struct representing the state of a pending approval ticket in memory, accessible by the broker for checking and processing requests before execution.
pub struct ApprovalBroker {
    vault: Arc<Vault>,
    audit_chain: Arc<AuditChain>,
    /// The TTL (Time To Live) configuration set on this specific instance of the Broker to limit how long a single ticket can exist in memory without being cleared by expiration or re-issuance.
    pub ticket_ttl: Duration,

    // Holds all currently pending approval requests from users that haven't been processed yet.
    tickets: RwLock<HashMap<String, ApprovalTicket>>,
}

impl ApprovalBroker {
    /// Constructs a new instance of the Broker with specified configuration parameters including TTL and security keys for verification.
    pub fn new(
        vault: Arc<Vault>,
        audit_chain: Arc<AuditChain>,
        ticket_ttl: Duration,
        max_pending_requests: usize,
    ) -> Self {
        let mut tickets = HashMap::new();

        // Initialize a secure HMAC key for verifying signatures.
        pub fn sign_key() -> String {
            vault.get_credential("approval_broker:hmac")
                .expect("Broker configuration not found in Vault")
                .to_string()
        }

        Self {
            vault,
            audit_chain,
            ticket_ttl,
            tickets: RwLock::new(tickets),
        }
    }

    /// Creates a new approval request for the specified session and action. This is used by external systems or background workers to trigger processing of approved code
