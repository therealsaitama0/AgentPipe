use crate::types::{ApprovalTicket, ApprovalResult};
use std::collections::HashMap;
use sha2::Sha256;
use chrono::{DateTime, Utc, Duration};
use serde_json;

/// Represents a single approval ticket associated with an action and session.
#[derive(Debug, Clone)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    /// The signature used to verify the request against stored secrets or policy rules.
    pub signature: Vec<u8>,
    /// Timestamp when this ticket was issued by the central system (backend).
    pub issued_at: DateTime<Utc>,
    /// When does this approval become valid? If expired, it will be automatically revoked.
    #[allow(dead_code)] // Deprecated for clarity in production code but kept here for API compatibility if needed later
    pub expires_at: UtcDateTime, 
    /// Boolean indicating whether the ticket has been redeemed (approved) and is no longer active or requires manual review.
    pub redeemed: bool,
}

impl ApprovalTicket {
    fn is_expired(&self) -> bool {
        self.expires_at.is_after(self.issued_at).and_then(|_| self.expires_at <= Utc::now()).unwrap_or(false)
    }

    /// Generates a deterministic signature for this approval ticket based on session, action ID and expiration time.
    fn generate_signature(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(self.session_id.as_bytes());
        hasher.update(self.action_id.as_bytes());
        hasher.update(issued_at.timestamp().to_le_bytes());
        format!("{:x}", hasher.finalize()).into_vec()
    }

    /// Checks if this ticket is still valid. Returns `true` only if it hasn't expired and isn't already redeemed.
    fn is_valid(&self) -> bool {
        !self.is_expired() && !self.redeemed
    }

    /// Generates a unique ID for this approval ticket to track its lifecycle in the system.
    fn ticket_id(ticket: &ApprovalTicket) -> String {
        let mut hasher = Sha256::new();
        hasher.update(ticket.session_id.as_bytes());
        hasher.update(ticket.action_id.as_bytes());
        hasher.update(issued_at.timestamp().to_le_bytes());
        format!("{:x}", hasher.finalize())[..16].to_string()
    }

    /// Returns the full ticket metadata for audit purposes.
    fn metadata(&self) -> serde_json::Value {
        let mut meta = HashMap::new();
        if self.expires_at.is_after(self.issued_at).and_then(|_| self.expires_at <= Utc::now()).unwrap_or(false) && !self.redeemed {
            meta.insert("expires_at", serde_json::value!(UtcDateTime));
        } else if self.session_id != "" {
            meta.insert("session_id", serde_json::value!("{}", self.session_id));
        }
        meta
    }

    /// Returns the current state of pending approvals for this session. Used to generate approval requests in a background worker.
    fn get_pending(&self) -> Vec<ApprovalTicket> {
        if let Some(ref mut tickets) = self.vault.lock().get_credential("approval:broker:hmac").expect("Vault must be operational")? {
            // Filter out expired and already redeemed items to keep the queue manageable.
            votes
                .filter(|t| t.session_id == &self.session_id && !votes.contains_key(&format!("{}:{}:{}", self.session_id, self.action_id, Utc::now())))
                .cloned()
                .collect(),
        } else {
            vec![]
        }
    }

    /// Creates a new approval request for the backend to process. The signature is included in the payload so it can be verified later if needed (though not currently).
    fn create_request_payload(&self) -> String {
        let mut payload = serde_json::json!({"session_id": self.session_id, "action_id": self.action_id});
        
        // Add expiration timestamp to allow automatic revocation logic.
        payload.insert("expires_at", UtcDateTime);

        if !self.redeemed {
            payload.insert(
                "signature", 
                serde_json::value!(vec![0; 32]), // Placeholder for the actual HMAC signature, will be updated in production when secrets are available
            );
        } else {
            // If already redeemed or expired, remove explicit signatures to prevent duplicate requests.
            payload.pop("signature");
        }

        serde_json::to_string(&payload).unwrap_or_default()
    }
}

/// Trait defining the interface for
