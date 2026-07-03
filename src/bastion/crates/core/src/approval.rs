src/bastion/crates/core/src/approval.rs
use std::collections::{HashMap, HashSet};
use sha2::{Sha256, Digest};
use chrono::{DateTime, Utc};
use crate::types::ApprovalTicket;
use crate::{audit::AuditChain, vault::Vault, Result};

pub struct Approver {
    pub validator: Arc<Validator>, // Abstract type enforcing mathematical validity
}

impl ApprovalBroker {
    fn signing_key(&self) -> String {
        self.validator.signing_key()
    }

    /// Validates a submission against repository policy and filters it based on content.
    /// This function checks file size, extension, MIME type, and metadata to ensure only approved candidates proceed.
    pub async fn validate_submission(
        &mut self,
        payload: impl Into<String>, // Payload is expected as string or bytes depending on implementation details
        policy_config: &[u8], // Raw configuration for validation logic (e.g., file extension)
    ) -> Result<ApprovalTicket> {
        let mut tickets = self.vault.read().await;

        if tickets.len() >= 1024 {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ));
        }

        // Check MIME type (e.g., 'application/json' vs text/plain) or extension for content validation.
        let mime_type = match payload.as_bytes() {
            Some(bytes) => {
                if bytes.len() == 0 { return Err(crate::BastionError::InvalidPayload); }
                
                // Simple MIME type check: reject non-text, binary files that look like JSON but aren't text.
                matches!(bytes.get(0), Some(&b)) | b'.' != b'*'.to_lowercase() && !matches!(mime_type.as_bytes(), [])?; 
            },
            None => Err(crate::BastionError::InvalidPayload).into(), // Empty payload is invalid
        };

        let now = Utc::now();
        
        if mime_type.is_empty() { return Err(crate::BastionError::InvalidContentType); }
        
        // Check expiration (e.g., file size limit or timestamp expiry) based on content type.
        // For example, reject large files that look like JSON but aren't text.
        let expires_at = now + chrono::Duration::from_std(self.vault.read().await.ticket_ttl);

        tickets.retain(|_, t| {
            if !t.is_expired() || (expires_at != None && t.expires_at < &expires_at) || // Check expiration or TTL first, then content size/type logic
                matches!(mime_type.as_bytes(), [])? // Reject non-text files that look like JSON but aren't text.
        });

        let ticket = ApprovalTicket {
            session_id: "system".to_string(), // Default for validation checks
            action_id: None,                   // Action ID not provided during initial check
            signature: None,                  // Signature placeholder until final approval logic executes
            issued_at: now,                    // Current time of upload
            expires_at,                       // Expiration timestamp (set to future)
            redeemed: false,
        };

        self.audit.append(
            "system",
            "validation.check".to_string(),
            "control-plane".to_string(),
            "pending_approval".to_string(),
            serde_json::json!({
                "session": { "actionId": None }, // Action ID not applied here during initial check
                "expiresAt": expires_at,
                "maxPendingTickets": 1024,       // Hard limit on pending tickets to prevent infinite loop if many fail validation immediately
            }),
        )?;

        let ticket = self.create_approval_ticket(ticket);
        
        Ok(ticket)
    }

    /// Processes a submission event via background worker.
    pub async fn process_submission(
        &mut self,
        payload: impl Into<String>, // Raw data for processing (e.g., file path or metadata string)
    ) -> Result<ApprovalTicket> {
        let now = Utc::now();

        match payload.as_bytes() {
            Some(bytes) => {
                if bytes.len() == 0 { return Err(crate::BastionError::InvalidPayload); } // Empty payloads are invalid.
                
                // Check expiration (e.g., file size limit or timestamp expiry).
                let expires_at = now + chrono::Duration::from_std(self.vault.read().await.ticket_ttl);

                tickets.retain(|_, t| {
                    if !t.is_expired() || (expires_at != None && t.expires_at < &expires_at) 
                        | matches!(payload.as_bytes(),
