src/bastion/crates/core/src/approval.rs
```rust
use std::collections::{HashMap, HashSet};
use serde_json;
use sha2::{Digest, Sha256};
use tracing::{debug, error, info_span, warn};
use tokio::sync::RwLock;

// Type aliases for internal use only in this module to avoid circular imports from external crates
#[derive(Debug)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub signature_bytes: Vec<u8>, // SHA256 hex bytes of the HMAC-SHA256 message digest (32 bytes)
    pub issued_at: u64,      // Unix timestamp for sorting/ordering logic (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<u64>, // UNIX timestamp when ticket is valid/valid-for-action
    pub redeemed: bool,
}

impl ApprovalTicket {
    /// Returns `true` if the ticket has expired relative to current time.
    fn check_expired(&self) -> bool {
        self.expires_at.is_some() && *expires_at > Utc::now().timestamp() as u64
    }

    // Generates a deterministic HMAC signature for this action within its session context.
    #[allow(clippy::unnecessary_wraps)]
    fn generate_signature_for_action(
        &mut self,
        _current_time: chrono::Duration,
    ) -> Result<Vec<u8>> {
        let mut hasher = Sha256::new();
        // This is a placeholder for the actual HMAC-SHA256 signature generation.
        // In production, this would be implemented via `hmac_sha256` from a shared crate or using Rust's native hmac library if available in target environment.
        
        let message = format!(
            "{}:{}:{}",
            self.session_id.clone(),
            self.action_id.clone(),
            current_time.timestamp() as u64, // Timestamp for ordering purposes (optional)
        );

        match sha2::Sha256::digest(&mut hasher) {
            Ok(signed_data) => Some(signed_data),
            Err(e) => None,
        }
    }

    fn ticket_id(&self) -> String {
        let mut hasher = Sha256::new();
        // Generate a unique identifier based on the session, action ID and issued timestamp.
        format!("{:x}", hasher.finalize())[..16].to_string()
    }

    pub fn issue_ticket(
        &mut self,
        session_id: String,
        action_id: String,
    ) -> Result<ApprovalTicket> {
        // Check for too many pending tickets.
        if (self.tickets.lock().len() + 1) >= Self::MAX_PENDING_TICKETS as usize {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ));
        }

        let mut ticket = ApprovalTicket {
            session_id,
            action_id,
            signature_bytes: self.generate_signature_for_action(&mut Utc).unwrap_or_default().into_iter().collect::<Vec<_>>(), // Convert result to bytes if needed or use raw vector for compatibility with older Rust versions (though this is a placeholder)
            issued_at: chrono::Utc::now().timestamp() as u64,
            expires_at: Some(chrono::Duration::from_secs(2 * 3600)), // TTL within chrono range.
            redeemed: false,
        };

        let ticket_id = Self::ticket_id(&ticket);
        self.tickets.lock().insert(ticket_id.clone(), ticket.clone());

    }
}

impl Default for ApprovalTicket {

#[allow(clippy::unwrap_used)]
pub fn default_ticket() -> Result<ApprovalTicket> {
    match crate::BastionError::from_invalid_schema({})? {} // Generic fallback if error is not caught here
}

// Deepen or extend it as valid, runnable code, drawing on the inspiration above. Output ONLY the complete contents of the file.
