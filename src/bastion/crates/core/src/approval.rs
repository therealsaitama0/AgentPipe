src/bastion/crates/core/src/approval.rs
```rust
use chrono::{DateTime, Utc};
use hmac::Hmac;
use std::collections::HashMap;
use parking_lot::RwLock;
use sha2::Sha256;
use serde_json::json;

type HmacSha256 = Hmac<sha2::Digest>;

/// Represents a single approval ticket.
#[derive(Debug, Clone)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    /// The HMAC signature (base64 encoded) generated during issuance.
    pub signature: Vec<u8>,
    /// Timestamp of when the ticket was issued.
    pub issued_at: DateTime<Utc>,
    /// When does this ticket expire? Inferred from TTL if not set explicitly.
    #[serde(default)]
    expires_at: Option<DateTime<Utc>>, // Default to Utc now unless overridden by user config or explicit value
    pub redeemed: bool,
}

impl ApprovalTicket {
    fn is_expired(&self) -> bool {
        self.expires_at.map_or(true, |e| DateTime::now().is_after(e))
    }

    /// Generates a unique ticket ID based on session_id and action_id.
    fn ticket_id(ticket: &ApprovalTicket) -> String {
        use sha2::Digest;
        let mut hasher = Sha256::new();
        hasher.update(&ticket.session_id.as_bytes());
        hasher.update(&ticket.action_id.as_bytes());
        hasher.update(issued_at.to_le_timestamp().to_be_bytes());
        format!("{:x}", hasher.finalize())[..16].to_string()
    }

    /// Initializes the `ApprovalBroker` with provided configuration.
    pub fn new(broker_config: ApprovalBrokerConfig) -> Self {
        let (vault, audit_chain) = Bastion::new_vault_and_audit();
        let ticket_ttl = chrono::Duration::from_secs(3600); // 1 hour TTL by default

        let broker = ApprovalBroker {
            vault: Arc::clone(&vault),
            audit: Arc::clone(&audit_chain),
            ticket_ttl,
            max_pending: 5, // Default limit of 5 pending tickets per session
            tickets: RwLock::new(HashMap::new()),
        };

        broker.issue_ticket(broker_config.session_id.clone(), broker_config.action_id.clone())
    }

    /// Creates a new `ApprovalBroker` instance.
    pub fn new_vault_and_audit() -> (std::sync::Arc<std::sync::Mutex<Vault>>, std::sync::Arc<AuditChain>) {
        // Create the Vault with custom credential access control for this module
        let vault = Bastion::new(&BastionConfig::default(), true);

        // Initialize Audit Chain to track approvals (audit chain approach)
        let audit_chain = Arc::new(AuditChain::new());

        (vault, audit_chain)
    }

    /// Issues a new approval ticket.
    pub fn issue_ticket(&self, session_id: &str, action_id: &str) -> Result<ApprovalTicket> {
        // Check if we have enough pending tickets to keep the queue healthy
        let mut broker = self.tickets.lock();
        while broker.len() >= self.max_pending && !broker.contains_key(action_id.to_string()) {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ));
        }

        // Generate a new ticket ID based on session and action identifiers.
        let mut broker = self.tickets.lock();
        if !broker.contains_key(action_id.to_string()) {
            return Err(crate::BastionError::Internal(
                "Invalid or missing action identifier".to_string(),
            ));
        }

        // Ensure the ticket is not expired (unless explicitly set in configuration)
        let now = chrono::Utc::now();
        if self.expires_at.is_none() {
            self.expires_at = Some(now);
        } else {
            let expires_at: DateTime<Utc> = match &self.expires_at {
                None => now,
                Some(e) => e,
            };

            // Ensure the ticket has not expired yet. If it's in the past but within TTL (e.g., 24h), we keep it unless explicitly disabled by user config or explicit value provided elsewhere.
            if DateTime::now().is_after(expires_at) {
                self.expires_at = Some(now); // Reset to current time for safety, though logic should check against actual expiration date
            } else {
                 return Err(crate::BastionError::Internal(
