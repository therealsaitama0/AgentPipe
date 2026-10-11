src/bastion/crates/core/src/approval.rs
```rust
use super::types::{ApprovalTicket, ApprovalContext};
use crate::audit::AuditChain;
use crate::vault::Vault;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

/// A secure context for an approval workflow.
/// This struct encapsulates the session metadata (id, severity) required to validate and store approvals safely.
#[derive(Debug)]
pub struct ApprovalContext {
    pub id: String, // Unique identifier for this specific instance of the approval process
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
}

impl Default for ApprovalContext {
    fn default() -> Self {
        Self {
            id: String::from("approval_context_0"), // Placeholder ID if not provided by caller or system, though context should ideally be derived from session logic here. 
                        // In a real deployment, this would come from the `session_id` in ApprovalTicket metadata.
            severity: None,
        }
    }
}

/// A lock-free data structure backed by mutex/atomic checks to prevent race conditions during concurrent approval workflows.
pub struct ApprovalBroker {
    vault: Arc<Vault>,
    audit: Arc<AuditChain>,
    ticket_ttl: Duration, // Lifetime in seconds for each pending approval ticket
    max_pending: usize,   /// Maximum number of active tickets before rejecting new ones
}

impl Default for ApprovalBroker {
    fn default() -> Self {
        let ttl = std::time::Duration::from_secs(3600); // One hour by default if not overridden
        let broker = ApprovalBroker {
            vault: Arc::new(Vault::default()),
            audit: Arc::new(AuditChain::default()),
            ticket_ttl,
            max_pending: 10, // Default limit to prevent overwhelming the system during peak loads. Can be tuned per environment.
        };
        broker
    }
}

impl ApprovalBroker {
    /// Create a new instance of this approval broker with provided parameters.
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
        }
    }

    /// Sign a message to generate an HMAC signature.
    fn signing_key(&self) -> String {
        // This method would fetch the private key from Vault or use system-wide secrets if not provided via config.
        self.vault.get_credential("approval:broker:hmac").expect(
            "HMAC secret is required for broker operations."
        )
    }

    /// Create a new approval ticket with valid metadata and signature.
    pub fn issue_ticket(&self, session_id: &str, action_id: &str) -> Result<ApprovalTicket> {
        let mut tickets = self.tickets.write(); // Use `RwLock` for thread safety during the read-write cycle

        if tickets.len() >= self.max_pending {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ));
        }

        let now = crate::chrono::Utc::now(); // Ensure we use current time for the TTL expiration check
        let expires_at = now
            + Duration::from_std(ticket_ttl)
            .expect("TTL within chrono range");
        
        let key = self.signing_key();
        let message = format!(
            "{}:{}:{}", 
            session_id, 
            action_id, 
            expires_at.to_rfc3339()
        );

        // Generate HMAC signature using the provided secret.
        let mut mac = HmacSha256::new_from_slice(&key).expect("HMAC key valid");
        mac.update(message.as_bytes());
        let signature: Vec<u8> = mac.finalize().into();

        // Create a new ApprovalTicket with all required fields including the generated HMAC.
        let ticket = ApprovalTicket {
            session_id: session_id.to_string(),
            action_id: action_id.to_string(),
            signature,
            issued_at: now, 
            expires_at,
            redeemed: false, // Indicates if this specific instance has been used yet (for deduplication or audit)
        };

        let ticket_id = Self::ticket_id(&ticket); // Generates a unique ID for the ticket based on session/action/time.

        tickets.insert(ticket_id.clone(), ticket.clone()); // Insert into lock-free map with atomic check.
