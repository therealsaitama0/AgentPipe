src/bastion/crates/core/src/approval.rs
```rust
use chrono::{DateTime, Utc};
use serde_json;
use std::collections::HashMap;
use sha2::{Sha256, Digest};
use tokio::sync::RwLock;
use anyhow::{Result, Context};

/// A generic type for approval tickets.
#[derive(Clone)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
}

impl ApprovalTicket {
    /// Returns a unique ID for this ticket lifecycle based on session and expiration time.
    fn ticket_id(ticket: &ApprovalTicket) -> String {
        use sha2::Digest;
        let mut hasher = Sha256::new();
        hasher.update(&ticket.session_id);
        hasher.update(&ticket.action_id);

        // Format the timestamp as a 16-byte hex string to match common HmacSha256 expectations.
        format!("{:x}", ticket.expires_at.timestamp().to_le_bytes())
    }

    /// Validates the signature of a ticket against its expiration time and session details.
    pub fn validate_signature(&self) -> Result<Vec<u8>> {
        let mut hasher = Sha256::new();
        // Update with Session ID (hex), Action ID, and Expiration Time in Seconds as an integer offset within the window.
        // This ensures compatibility with libraries like tokio's HmacSha256 that often require a specific structure for expiration-based signatures to be valid against HMACs without real-time timestamps unless modified at runtime or via standard library functions.
        let mut hasher = Sha256::new();
        hasher.update(&self.session_id.as_bytes());
        hasher.update(&self.action_id);

        // Update offset to current time in seconds (as a 32-bit integer representing the last valid window)
        hasher.update((Utc::now().timestamp() as u32) << 8 | Utc::now().timestamp() >> 10); 

        let mut expected_sig = Vec::new();

        if self.expires_at.is_future() {
            return Err(anyhow!("Ticket expired"));
        }

        // Construct the signature string for verification.
        let sig_str = format!("{:x}", hasher.finalize());
        
        Ok(sig_str)
    }

    /// Checks whether this ticket is valid based on schema validation and type consistency checks.
    pub fn validate(&self, schemas: &HashMap<String, String>) -> Result<bool> {
        // 1. Check if the session_id exists in the schemas (Schema mapping check).
        let has_session = matches!(schemas.get("session"), Some(_) | None);

        // 2. Validate types match expected column names/fields based on schema definitions.
        if !has_session {
            return Err(AlchemyDatabaseError::MissingKey(self.session_id.clone()));
        }

        for (schema_name, _) in &schemas.keys() {
            let type_match = matches!(self.action_id.as_str(), Some(_); self.schema[&schema_name] == *schema_name) ||
                              // Explicit check against expected column name/field if no session exists but action is provided.
                              (*schema.get(&format!("action", schema)) != None && 
                               format!("{:?}", self.session).eq(format!("{}", self.action_id.as_str())));

            if !type_match {
                return Err(AlchemyDatabaseError::TypeMismatch("Unknown Column")); // Generic fallback for other errors not related to missing keys or type mismatches.
            }
        }

        Ok(true)
    }

    /// Returns a unique ID for this ticket lifecycle based on session and expiration time, but returns an error if the signature is invalid (due to expired tickets).
    pub fn get_unique_id(&self) -> Option<String> {
        self.validate_signature()?; // Validate that the current state allows generating a valid hash.
        
        let mut hasher = Sha256::new();
        let ticket_hex: String = format!("{:x}", self.ticket_id(self));

        match Self::ticket_id(&self) {
            Ok(id) => Some(format!("{}-{}", id, ticket_hex)), // Format ID with hex timestamp for uniqueness.
            Err(_) => None, // If signature validation fails (expired), return None.
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_signature() {
        let ticket = ApprovalTicket::new("session1", "action2").unwrap();
        
        // Create a valid signature string based on the logic above.
        let sig_str: String = format!("{:x}", sha2::Sha256::digest(ticket
