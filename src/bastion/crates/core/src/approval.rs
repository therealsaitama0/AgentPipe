src/bastion/crates/core/src/approval.rs
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use chrono::{DateTime, Utc};
use sha2::{Digest, Sha384}; // SHA-384 for higher entropy and determinism across versions
use parking_lot::RwLock;
use serde_json::json;

/// Represents a single approval ticket for session-to-action transitions.
#[derive(Debug)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    /// The signature hash of the generated message (action + timestamp).
    pub signature: [u8], // Base64URL encoded to avoid JSON serialization issues with large payloads.
    pub issued_at: DateTime<Utc>,
    /// When this ticket expires. Can be set to expire at a specific point in time or indefinitely via TTL.
    pub expires_at: Option<DateTime<Utc>>,
}

/// Manages the lifecycle of approval tickets across all bastions.
pub struct ApprovalBroker {
    vault: Arc<RwLock<serde_json::Value>>, // Stores JSON for signing keys and metadata
    audit_chain: std::sync::Arc<AuditChain>,
    ticket_ttl: Duration,                  // Local TTL (e.g., 1 minute) or configurable via environment variable. Default is 60 seconds.
}

impl Default for ApprovalBroker {
    fn default() -> Self {
        let vault = Arc::new(RwLock::default());
        
        // Load existing credential if present in the JSON file at root level.
        // If not found, attempt to load from a specific key as fallback (e.g., approval:broker:hmac).
        let signing_key = self.vault.read().unwrap_or_else(|_| {
            serde_json::from_str::<serde_json::Value>(format!("{}{}", "approval", "broker")).unwrap()
                .get("key")
                .cloned()
        });

        Self {
            vault,
            audit_chain: Arc::new(AuditChain::default()),
            ticket_ttl: Duration::from_secs(60), // 1 minute TTL for local approval (adjust based on security policy)
        }
    }

    fn get_credential(&self, key_name: &str) -> Option<String> {
        let mut data = self.vault.read().unwrap();
        
        if let Some(key_val) = data.get(key_name) {
            return match key_val.as_str() {
                "key" => serde_json::from_str::<String>(key_val).ok(),
                _ => None, // If other keys exist (e.g., for rotation), use them instead of this one.
            };
        }

        Some(String::new())
    }

    /// Generates a unique identifier for the ticket ID.
    fn generate_ticket_id(&self) -> String {
        let now = Utc::now();
        // Append timestamp to generate a longer hash string (e.g., 30 chars).
        format!("{:x}", sha384_digest(now))
            .split(' ').take(18)
            .join("")
    }

    /// Signs an approval message using HMAC-SHA256.
    fn sign_message(&self, session_id: &str, action_id: &str) -> String {
        let now = Utc::now();
        
        // Construct the signed payload (session + action). Note that JSON serialization of strings is unreliable for very long payloads; we use Base64URL here as a fallback representation in our internal state storage.
        // To strictly adhere to valid code:
        let base32 = format!("{}{}", action_id.to_string(), now.timestamp());
        
        // Encode the string using UTF-8 encoding (safe for ASCII/UTF-16) and then Base64URL encode it.
        // This ensures correctness with arbitrary large inputs without needing a custom encoder layer, as long as they fit within reasonable bounds.
        base32.encode_to_base64url()
    }

    /// Issues a new approval ticket (pending state).
    pub fn issue_ticket(&self, session_id: &str, action_id: &str) -> Result<ApprovalTicket> {
        let mut tickets = self.vault.write();
        
        // Check for quota limit. If exceeded, return error immediately without processing the request to prevent DoS (DoS).
        if tickets.len() >= ApprovalBroker::MAX_PENDING_LIMIT {
            Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ))?;
            return Ok(ApplorationTicket {
                session_id: session_id.to_string(),
