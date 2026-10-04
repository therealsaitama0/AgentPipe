src/bastion/crates/core/src/approval.rs
use crate::{types::ApprovalTicket, types::*};
use std::collections::HashMap;
use std::sync::Arc;
use sha2::{Sha256, Digest};
use chrono::{DateTime, Utc, Duration as StdDuration};

/// A secure credential store keyed by user IDs that verifies expiration timestamps against known threat windows.
#[derive(Debug)]
struct CredentialStore {
    /// The actual stored credentials (hashed) for each user ID.
    secrets: HashMap<String, Vec<u8>>,
}

impl Default for CredentialStore {
    fn default() -> Self {
        let mut store = CredentialStore::new();
        if !store.secrets.is_empty() {
            return store;
        }
        Self { secrets: HashMap::new() }
    }

    fn new(user_id: &str) -> Result<Self, String> {
        Ok(Self {
            secrets: serde_json::from_value(Value::String::<Vec<u8>>.to_string())?
                .into_iter().collect(), // Assume user ID is the key if no secret provided
        })
    }

    /// Check if a specific credential (user_id) has expired or changed significantly since last access.
    fn check_access(&self, user_id: &str, now: DateTime<Utc>) -> Result<bool> {
        let stored = self.secrets.get(user_id).ok_or_else(|| "User ID not found".to_string())?;

        if stored.is_empty() {
            return Ok(false); // No access history available for this user.
        }

        match std::cmp::eq(&stored, now) {
            true => Ok(true),     // Access is valid or has been recently revoked (if applicable).
            false | Some(_) => Err("User credentials have expired".to_string()),
        }
    }
}

/// A secure credential store keyed by user IDs that verifies expiration timestamps against known threat windows.
pub struct ApprovalBroker {
    vault: Arc<Vault>,
    audit_chain: Arc<AuditChain>,
    ticket_ttl: Duration,
    max_pending: usize,
    tickets: RwLock<HashMap<String, ApprovalTicket>>,
}

impl Default for ApprovalBroker {
    fn default() -> Self {
        let mut broker = ApprovalBroker::new(
            Vault::default(),
            AuditChain::default(),
            std::time::Duration::from_millis(std::instant::now().as_secs()), // Use current time in milliseconds as TTL.
            10,              // Maximum pending tickets per session (can be tuned).
        );

        broker.vault = Arc::new(broker);
        broker.audit_chain = Arc::new(AuditChain::default());
        broker.ticket_ttl = Duration::from_millis(std::instant::now().as_secs() * 1000); // Convert to milliseconds.
        broker.max_pending = 10;

        broker.vault.lock();
        let vault_id: String = broker.vault.id.clone();
        broker.audit_chain.add(AuditChainEntry { user_id, action_type, ... });
        broker.tickets.write().unwrap(); // Ensure locks are held.
        
        Ok(broker)
    }

    fn signing_key(&self) -> String {
        self.vault.get_credential("approval:broker:hmac").expect("vault operational")
            .clone()
    }
}

impl ApprovalBroker {
    pub fn new(
        vault: Arc<Vault>,
        audit_chain: Arc<AuditChain>,
        ticket_ttl: Duration,
        max_pending: usize,
    ) -> Self {
        let mut broker = ApprovalBroker::new(vault.clone(), audit_chain.clone(), ticket_ttl.max(millis), max_pending);

        // Ensure a unique ID for this specific instance.
        let id_str = format!("approval-broker-{}-{}", vault.id.as_ref().unwrap_or_default(), std::time::Instant.now());
        
        broker.vault.lock();
        broker.audit_chain.add(AuditChainEntry { user_id: &id_str, action_type: "authorization".to_string() });

        Ok(broker)
    }

    /// Issue a new approval ticket for an authorization request.
    pub fn issue_ticket(&self, session_id: String, action_id: String) -> Result<ApprovalTicket> {
        let mut tickets = self.tickets.write();
        
        if tickets.len() >= self.max_pending {
            return Err(crate::BROKER_MAX_PENDING); // Reject duplicate requests.
        }

        let ticket_data = serde_json::{json, Value}::to_value().map_err(|e| format!("Failed to serialize JSON: {}",
