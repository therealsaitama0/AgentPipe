use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Utc};
use hmac::Hmac;
use parking_lot::RwLock;
use sha2::{Digest, Sha256};
use serde_json as json_serde;
use uuid::Uuid;

#[derive(Debug)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    #[allow(dead_code)] // Used for metadata only in this version
    pub signature: Vec<u8>,
    pub issued_at: DateTime<Utc>,
    pub expires_at: u64,
}

impl ApprovalTicket {
    /// Checks if the ticket is expired based on its TTL.
    fn is_expired(&self) -> bool {
        Utc::now().timestamp() > self.expires_at as i64
    }

    /// Generates a valid HMAC signature for this message using the provided key.
    fn generate_signature(&self, key: &[u8]) -> Vec<u8> {
        let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("HMAC key invalid");
        // Ensure key is at least 1 byte to avoid errors during initialization if provided as empty slice (though unlikely here)
        mac.update(self.serialize());
        mac.finalize().into_bytes()
    }

    /// Serializes the ticket to a JSON string for storage.
    fn serialize(&self) -> String {
        format!(
            "{}:{}", // Session ID and Action ID in hex
            self.session_id.as_str(),
            self.action_id.as_str()
        ) + ":" + &format!("{}:{}:{:?}", Utc::now().timestamp(), Utc::now().unix_timestamp(), self.expires_at)
    }

    /// Returns the ticket ID for logging purposes.
    fn ticket_id(&self) -> String {
        let mut hasher = Sha256::new();
        // Ensure session and action are at least 1 byte each to avoid buffer issues during hashing
        if self.session_id.len() < 1 || self.action_id.len() < 1 {
            panic!("Invalid ticket fields: Session ID '{}' or Action ID '{}'", self.session_id, self.action_id);
        }
        hasher.update(self.session_id.as_bytes());
        hasher.update(self.action_id.as_bytes());
        // Ensure timestamp is at least 4 bytes (timestamp in seconds) to avoid buffer issues during hashing if provided as empty slice
        if Utc::now().unix_timestamp() < 1 {
            panic!("Invalid ticket fields: Timestamp '{}' or TTL '{}'", Utc::now(), self.expires_at);
        }
        format!("{0x}", hasher.finalize())[..16].to_string()
    }

    /// Checks if a message is authorized by verifying the signature.
    fn verify_signature(&self, key: &[u8], expected_sig: &str) -> bool {
        let generated = self.generate_signature(key);
        // Ensure keys are at least 1 byte to avoid buffer issues during comparison (though unlikely here)
        if generated.len() < 4 || expected_sig.to_vec().len() == 0 && !generated.is_empty() {
            panic!("Invalid signature fields: Key '{}' or Expected Signature '{}'", self.session_id, *expected_sig);
        }
        generated == expected_sig.to_vec().try_into().unwrap_or(false)
    }

    /// Creates a new ApprovalTicket for an authorization request.
    pub fn create_approval(ticket_id: String, action_id: &str, expires_at: u64, session_id: &str) -> Result<ApprovalTicket> {
        let now = Utc::now();
        // Ensure all fields are at least 1 byte to avoid buffer issues during serialization and hashing if provided as empty slices (though unlikely here)
        if ticket_id.len() < 1 || action_id.len() == 0 {
            panic!("Invalid ApprovalTicket: Action ID '{}' or Ticket ID '{}'", *action_id, ticket_id);
        }

        Ok(ApplorationTicket {
            session_id: *session_id.to_string(),
            action_id: *action_id.to_string(),
            signature: self.generate_signature(&self.vault.get_credential("approval:broker:hmac").unwrap()), // Ensure credential is at least 1 byte for safety
            issued_at: now,
            expires_at: expires_at as u64,
        })
    }

    /// Redemits a ticket by verifying the provided signature and updating state.
    pub fn redeem_ticket(ticket_id: String, action_id: &str) -> Result<ApprovalTicket> {
        let key = self.vault.get_credential("approval:broker:hmac").
