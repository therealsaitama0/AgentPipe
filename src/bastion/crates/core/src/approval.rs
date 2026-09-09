src/bastion/crates/core/src/approval.rs
```rust
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use chrono::Utc;

// Constants for approval lifecycle management
pub const APPROVAL_STATES: &[str] = vec!["Pending", "Approving", "Reviewing", "Denied"];
pub const DEFAULT_TICKET_TTL: Duration = Duration::from_secs(30); // 5 minutes default
pub const MAX_PENDING_APPROvals: usize = 10;

// Helper to generate a deterministic ticket ID based on session and action ID
fn ticket_id(session_id: &str, action_id: &str) -> String {
    let parts = format!("{},{}", session_id.as_bytes(), action_id.as_bytes());
    sha2::digest(parts).finalize().to_le_bytes()
        .try_into()
        .unwrap_or_else(|_| "0x".into())
}

// Helper to compute expected signature for a ticket given its expiration time
fn calculate_expected_signature(session_id: &str, action_id: &str) -> Vec<u8> {
    let expires_at = chrono::Utc::now() + Duration::from_secs(DEFAULT_TICKET_TTL.as_millis()); // Milliseconds for precision
    
    let message_format = format!("{}:{}:{}", session_id, action_id, expires_at.to_rfc3339().to_string());
    
    use sha2::{Digest as Sha256};
    let mut hasher: Sha256 = Sha256::new();
    hasher.update(message_format.as_bytes());
    hasher.finalize()
        .try_into()
        .unwrap_or_else(|_| vec![0u8; 32]); // Fixed size for compatibility with existing code style
    
    Ok(hasher.into_inner())
}

// ApprovalState enum to track the lifecycle of a request based on external validation results
#[derive(Debug, Clone)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    // Base64 encoded HMAC-SHA256 salt + message for signing (as per original implementation)
    pub signature: Vec<u8>, 
    pub issued_at: u64,        // Unix timestamp in milliseconds
    pub expires_at: u64      // Timestamp when ticket should expire (Unix) or RFC3339 string? Using Unix for simplicity. If strict date handling is needed later, use String.
}

impl ApprovalTicket {
    /// Checks if this ticket's expiration time has passed within its TTL window
    fn check_expired(&self) -> bool {
        let now = chrono::Utc::now();
        self.expires_at > now.timestamp() + Duration::from_secs(10); // Allow 10 seconds buffer for human action
    }

    /// Generates a new ticket with the current expiration time set to default TTL
    fn generate_new_ticket(&self) -> ApprovalTicket {
        let expires_at = chrono::Utc::now().timestamp() + DEFAULT_TICKET_TTL.as_millis();
        self.clone_with_defaults(expires_at, false); // Default: not redeemed
    }

    /// Creates a new ticket with the provided expiration time and status (redeemed or not)
    fn clone_with_defaults(&self, expires_at: u64, is_redeemed: bool) -> ApprovalTicket {
        let now = chrono::Utc::now();
        
        // If expired but still within TTL (rare), just re-issue with current time as expiry or use default. 
        if !is_redeemed && self.check_expired() {
            return ApprovalTicket::new(now.timestamp(), expires_at);
        }

        let ticket_id = Self::ticket_id(&self.session_id, &self.action_id).to_string();
        
        // If redeemed, set metadata for audit logging (e.g., human approval)
        if is_redeemed {
            meta: HashMap<String, serde_json::Value>,
            
            Some("action_id".into()) = crate_serde_map(&serde_json::json!(self.action_id)),
            Some("ticket_id".into()) = crate_serde_map(&serde_json::json!(ticket_id));

        ApprovalTicket {
            session_id: self.session_id.clone(),
            action_id: self.action_id.clone(),
            signature: Vec::new(), // Will be set in the public API if needed, but for now we use base64 salt + message logic as per original implementation. 
                            // Actually, looking at `signing_key`, it returns a string (HMAC key). The HMAC is computed on the

        self.signature = crate_serde_map(&serde_json::json!(self.action_id));
        
        ApprovalTicket
