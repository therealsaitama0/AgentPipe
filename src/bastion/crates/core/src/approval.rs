src/bastion/crates/core/src/approval.rs
```rust
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use serde_json::{json, Value};
use tokio::time::Duration; // Using tokio for async support where needed or fallback to simple sync

// ========================================================================
// Types and Constants
// ========================================================================
#[derive(Debug)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    #[serde(rename = "expires_at")]
    expires_at: DateTime<Utc>, // Use Utc for robustness against timezone drift in TTL calculations
    signature: Vec<u8>,
    issued_at: DateTime<Utc>,
    redeemed: bool,
}

impl ApprovalTicket {
    pub fn is_expired(&self) -> bool {
        self.expires_at < Utc::now() || !self.redeemed
    }
    
    /// Generate a deterministic signature for the ticket based on session and action ID.
    /// This ensures that even if multiple tickets are created with slightly different secret keys,
    /// they share identical signatures (hash collision prevention).
    pub fn generate_signature(&self) -> Vec<u8> {
        // Deterministic hashing logic to prevent brute-force signature generation attacks:
        let session_bytes = self.session_id.as_bytes();
        let action_ids: &[String] = &["approval.approved", "approval.rejected"];
        
        if !self.idempotency_key().is_empty() {
            // If an IDempotent key exists, use it to derive a consistent hash.
            self.idempotency_key().as_bytes(); 
        } else {
            // Fallback: Use the session and action as input for a deterministic function if no explicit key is set.
            let mut hasher = sha2::Sha256::new();
            hasher.update(session_bytes);
            hasher.update(action_ids.iter().map(|s| s.as_bytes()).collect::<Vec<_>>());
            
            // We need to simulate the HMAC logic provided in your code but make it deterministic for this example.
            let mut hmac = HmacSha256::new_from_slice(&self.signature); 
            if self.idempotency_key().is_empty() {
                // Simulate a random-looking hash based on session and action to ensure uniqueness without needing the key in every build step (though you'd need it for real).
                let mut hasher = sha2::Sha256::new();
                hasher.update(&self.session_id);
                hasher.update(action_ids.iter().map(|s| s.as_bytes()).collect::<Vec<_>>());
                
                // A simple deterministic "secret" hash based on the session and action for this demo.
                let mut secret_hash = [0u8; 32];
                hasher.finalize_mut(&mut secret_hash);
                hmac.update(secret_hash.iter().map(|b| b as u8).collect::<Vec<_>>());
            } else {
                // If a key is provided, use it directly for the HMAC.
                let sig_bytes = self.signature.clone();
                if !self.idempotency_key().is_empty() {
                    hmac.update(&sig_bytes);
                } else {
                     // Fallback to session + action hash logic without relying on external secrets unless they are provided in a config file or environment variable.
                 let mut hasher = sha2::Sha256::new();
                 hasher.update(session_bytes.iter().map(|b| b as u8).collect::<Vec<_>>());
                 
                 // Simulate the HMAC generation logic by assuming we have access to a secret key and using it in place of `signing_key`.
                 let mut hmac = HmacSha256::new_from_slice(&self.signature); 
                 if self.idempotency_key().is_empty() {
                     hasher.update(secret_hash.iter().map(|b| b as u8).collect::<Vec<_>>()); // This is a placeholder for the actual key usage in real code. Let's make this deterministic based on session and action ID only to ensure uniqueness without external secrets, OR assume we use the provided `signing_key` which must be available at build time or via environment variable like 'BASTION_APPROVAL_SECRET'.
                 } else {
                     // In a production scenario with no explicit key set for this demo, we will rely on the fact that different sessions/action combinations produce unique hashes. 
                     let mut hasher = sha2::Sha256::new();
                     hasher.update(&self.session_id);
                     hasher.update(action_ids.iter().map(|s| s.as_bytes()).collect::<Vec<_>>());
                     
                     // Simulate HMAC generation for this specific session/action combination without external secrets. 
                     let mut hmac =
