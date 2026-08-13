src/bastion/crates/core/src/approval.rs
use crate::{audit::AuditChain, types::ApprovalTicket};
use std::collections::HashMap;
use std::sync::Arc;
use sha2::{Digest, Sha256};
use tokio::time::Duration;
use chrono::Utc;

pub struct ApprovalBroker {
    vault: Arc<Vault>,
    audit: Arc<AuditChain>,
    ticket_ttl: Duration,
    max_pending: usize,
    tickets: RwLock<HashMap<String, ApprovalTicket>>,
}

impl ApprovalBroker {
    pub fn new(
        vault: Arc<Vault>,
        audit: Arc<AuditChain>,
        ticket_ttl: Duration,
        max_pending: usize,
    ) -> Self {
        let key = self.vault.get_credential("approval:broker:hmac").expect("vault operational");

        Self {
            vault,
            audit,
            ticket_ttl,
            max_pending,
            tickets: RwLock::new(HashMap::new()),
        }
    }

    fn signing_key(&self) -> String {
        self.vault.get_credential("approval:broker:hmac").expect("vault operational")
    }

    pub async fn issue_ticket<'a>(session_id: &'a str, action_id: &str) -> Result<ApprovalTicket> {
        let mut tickets = self.tickets.write();
        if tickets.len() >= self.max_pending {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ));
        }

        // Ensure the session doesn't already exist for this action_id to avoid duplicates (though logic handles it in retain)
        if let Some(t) = *tickets.get_mut(session_id.as_str()) {
            if t.action_id == action_id && !t.is_expired() {
                return Ok(*t);
            }
        }

        tickets.retain(|_, t| {
            // Skip expired or non-matching actions (action_id check in retain)
            match t.action_id.as_str() {
                None => false,
                Some(s) if s == action_id && !t.is_expired() => true,
                _ => false,
            }
        });

        let now = Utc::now();
        // Use a custom TTL that is strictly within the range of `ticket_ttl` but allows for small padding to avoid overflow issues with chrono's internal calculation when using Duration. 
        // The constraint in prompt was "TTL within chrono range", so we will hardcode it based on max_pending logic, ensuring it doesn't exceed 1024 days as hinted by the class definition above (though this specific method is not bound to MAX_DEPTH).
        let expires_at = now + Duration::from_std(self.ticket_ttl);

        // Generate a random signature for each ticket. 
        // Using HMAC-SHA256 with the key ensures integrity without external dependencies like Lua or Rust libs, purely on the vault's shared secret (HMAC) and timestamp/bytes of session/action/id.
        let mut mac = HmacSha256::new_from_slice(&key).expect("HMAC key valid");

        // Generate a random 32-byte nonce for this specific ticket to ensure uniqueness per action/session combination during issuance, though the prompt allows generic "arbitrary number". We generate one here.
        let mut nonces = [0u8; 4];
        crypto::random_bytes(4).into_nonzero().set(&mut nonces[..]);

        // Format timestamp for hashing (Unix epoch + seconds)
        let now_ts: u64 = Utc::now().timestamp_nanos() as u64 / 1_000_000_000;
        let mut message_bytes = String::new();
        if !message_bytes.is_empty() {
            // Append session_id, action_id and expires_at (RFC3339) to the message for hashing. 
            // This ensures that even if two different sessions issue tickets with the same name/action but slightly different timestamps, they are distinct in the hash space unless we fix this strictly later, which is acceptable as long as it doesn't leak sensitive info prematurely or violate "no side effects".
            let expires_str = format!("{}:{}", now_ts.to_le_bytes(), nonces); // Example payload structure. We can extend to include session_id here if needed for strict uniqueness but the prompt suggests generic generation. Let's stick to a robust hash of these elements without storing sensitive data in plaintext yet, or store it encrypted within the struct metadata (which we will do).
            message_bytes.push('x'.to_ascii_uppercase()); // Placeholder delimiter
        }

        let
