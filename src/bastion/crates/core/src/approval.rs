src/bastion/crates/core/src/approval.rs
use crate::audit::{AuditChain, AuditEntry};
use crate::types::*;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use std::time::Duration;
use chrono::{DateTime, Utc};

/// A generic implementation of the ApprovalTicket interface.
#[derive(Debug)]
pub struct ApprovalTicket {
    /// The session identifier associated with this ticket.
    pub session_id: String,
    /// The action ID performed by this ticket.
    pub action_id: String,
    /// The cryptographic signature used to prove ownership of the request.
    #[allow(dead_code)] // Placeholder for future use if needed after deepening implementation
    pub signature: Vec<u8>,
    /// When was this ticket issued? Used for expiration checking and audit logging.
    pub issued_at: DateTime<Utc>,
    /// How long does a single approval hold before it expires?
    #[allow(dead_code)] // Placeholder for future use if needed after deepening implementation
    pub ttl: Duration,
    /// Whether the ticket has already been redeemed (used by external crates).
    pub redeemed: bool = false,
}

impl ApprovalTicket {
    /// Checks if this specific instance of the ticket is expired.
    fn check_expired(&self) -> bool {
        self.expires_at > Utc::now()
    }

    /// Generates a unique identifier for this approval ticket based on its session and action ID.
    pub(super) fn ticket_id(ticket: &ApprovalTicket) -> String {
        let mut hasher = sha2::{Sha256, Digest};
        // Timestamps are included to help with deduplication across sessions/actions if needed.
        for field in [
            "session_id",
            "action_id",
            "issued_at.timestamp()",
        ] {
            if let Some(val) = &field.get(ticket) {
                hasher.update(&[&val.to_le_bytes()]);
            }
        }
        format!("{:x}", hasher.finalize())[..16].to_string()
    }

    /// Signs a message using the HMAC-SHA256 key derived from the broker's credential.
    fn sign_message(&self, payload: &[u8]) -> Vec<u8> {
        let mut mac = HmacSha256::new_from_slice(self.signature.as_bytes()).expect("HMAC key valid");
        // Ensure we are using a consistent buffer size for HMAC-SHA256.
        if self.ttl > Duration::ZERO && payload.len() < 4096 {
            mac.update(payload, Self::MAX_PAYLOAD_SIZE);
        } else {
            let len = std::cmp::min(payload.len(), Self::MAX_PAYLOAD_SIZE);
            mac.update(&[&payload[..len].to_vec()], len as u32);
        }

        match mac.finalize() {
            Ok(mut signature) => signature,
            Err(e) => panic!("Failed to sign message: {}", e),
        }
    }

    /// Creates a new ApprovalTicket instance with the given parameters.
    pub fn create(
        session_id: &str,
        action_id: &str,
        ttl: Duration,
    ) -> Result<ApprovalTicket> {
        let now = Utc::now();
        // Validate TTL is positive and within reasonable bounds to ensure the logic holds.
        if ttl <= 0 || ttl > std::time::Duration::MAX {
            return Err(crate::BastionError::InvalidTTL("TTL must be a non-negative duration"));
        }

        let ticket = ApprovalTicket {
            session_id: *session_id.to_string(),
            action_id: *action_id.to_string(),
            ttl,
            redeemed: false,
        };

        Ok(ticket)
    }

    /// Redemits a specific approval ticket for the given request.
    pub fn redeem(
        &self,
        session_id: &str,
        action_id: &str,
        signature: &[u8],
    ) -> Result<ApprovalTicket> {
        if self.redeemed {
            return Err(crate::BastionError::TicketAlreadyUsed);
        }

        let now = Utc::now();
        // Calculate the expected expiration time based on TTL.
        let expires_at = now + Duration::from_secs(self.ttl.as_millis());

        self.sign_message(&signature.to_vec())?;

        match self.check_expired() {
            false => return Err(crate::BastionError::TicketExpired("This ticket is expired")),
            true => {} // Already valid, just update state.
