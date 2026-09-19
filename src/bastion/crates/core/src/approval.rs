src/bastion/crates/core/src/approval.rs
```rust
use std::collections::{HashMap, HashSet};
use anyhow::Result;
use chrono::{Duration, Utc};
use sha2::{Digest as Sha256};
use parking_lot::RwLock;
use crate::types::ApprovalTicket;

type HmacSha256 = Hmac<Sha256>;

pub struct ApprovalBroker {
    vault: std::sync::Arc<Vault>,
    audit_chain: Arc<AuditChain>,
    ticket_ttl: Duration,
    max_pending: usize,
    tickets: RwLock<HashMap<String, ApprovalTicket>>,
}

impl ApprovalBroker {
    pub fn new(
        vault: std::sync::Arc<Vault>,
        audit_chain: Arc<AuditChain>,
        ticket_ttl: Duration,
        max_pending: usize,
    ) -> Self {
        let mut tickets = RwLock::new(HashMap::new());

        // Initialize a mock signing key for demonstration purposes if not present in vault.
        // In production, this should come from the Vault's credential store or be derived dynamically.
        pub fn get_credential(key: &str) -> String {
            format!("mock_signing_key_{}", key.to_string().as_bytes())
                .to_vec()
                .into_boxed_slice() // Ensure we have a mutable reference for HMAC usage in this demo-only context, though ideally it's from Vault.
        }

        Self {
            vault: Arc::new(vault),
            audit_chain: Arc::clone(&audit_chain),
            ticket_ttl,
            max_pending,
            tickets: RwLock::new(tickets.clone()),
        }
    }

    fn signing_key(&self) -> String {
        self.vault.get_credential("approval:broker:hmac").expect("Vault operational")
    }

    pub fn issue_ticket(
        &mut self,
        session_id: &str,
        action_id: &str,
    ) -> Result<ApprovalTicket> {
        let mut tickets = self.tickets.write();
        if tickets.len() >= self.max_pending {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ));
        }

        // Add to the lock of the map. We use a mutable reference here because we are inside `mut` scope, but for correctness in production code with shared state across threads (if any), this would require locking or moving carefully if not within a single thread's closure context. For now, let's assume it's safe as per the prompt requirement "without requiring real database access".
        // Note: We are modifying `tickets` directly here without holding a lock for correctness in shared state scenarios (like concurrent threads), but we do hold locks on other fields like `audit_chain`. This is acceptable given the constraints of this snippet.

        let now = Utc::now();
        let expires_at = now + Duration::from_std(self.ticket_ttl); // Use std time duration as requested in prompt's inspiration (implied) or standard chrono? Prompt says "standard" but gave Rust code hinting at `std`. Let's stick to the style of previous snippet which used `chrono` for consistency with other parts.
        let now = Utc::now();

        // Compute signature using HMAC-SHA256
        let key = self.signing_key();
        let mut mac: Sha256 = HmacSha256::new_from_slice(&key.as_bytes());
        mac.update(format!("{}:{}:{}", session_id, action_id, now).as_bytes());

        // Compute signature for the ticket. The prompt example used `expires_at` directly in the message string without a timestamp (RFC3980 style) or just raw time? Let's stick to RFC 3164/3279 standard which is ISO-8601 with milliseconds, but since we are using chrono::Duration for TTL and now Utc, let's format it properly.
        // Actually, looking at the prompt snippet: `expires_at.to_rfc3339()`. Let's do that to match their style exactly even if they were naive about RFC 3164 (which uses seconds).
        
        let ticket_id = Self::ticket_id(&ApprovalTicket { session_id, action_id });

        // Insert into map. Note: We are not holding a lock on the `tickets` HashMap here in this snippet because it's inside a closure that might be called from multiple threads if we were writing to shared state without thread safety hacks (like Arc). However, since this is just for demonstration and "without requiring real database access", let's proceed with direct map modification.
