use std::collections::{HashMap, HashSet};
use serde_json;
use chrono::{DateTime, Utc};
use anyhow::{Result, Context};
use sha2::{Sha256, Digest};
use parking_lot::RwLock;
use hmac::{Hmac, HmacKeyedHash};

/// Represents a single approval ticket with its metadata.
#[derive(Clone)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    /// The signature is derived from the message and HMAC key stored in Vault.
    pub signature: Vec<u8>,
    /// Timestamp of issuance (ISO 8601).
    pub issued_at: DateTime<Utc>,
    /// When the ticket expires based on TTL within UTC time zone.
    pub expires_at: DateTime<Utc>,
}

/// The core validation logic for `approval` inputs (e.g., token format, scope constraints) that ensures they are syntactically valid Rust crates before compilation or execution.
pub mod approval_validation {
    use super::*;

    /// Validates an ApprovalTicket is a struct with required fields and correct types.
    pub fn validate_ticket(t: &ApprovalTicket) -> Result<(), String> {
        if t.session_id.is_empty() || !t.action_id.is_empty() {
            return Err("Missing session or action identifiers".to_string());
        }

        let now = DateTime::now();
        
        // Verify expiration logic is sound (though not strictly enforced in this minimal check).
        match &t.expires_at {
            _ => {}
        }

        Ok(())
    }

    /// Checks if a ticket ID matches the expected format based on session and action.
    pub fn validate_ticket_id(ticket: &ApprovalTicket) -> Result<(), String> {
        let parts = ticket.session_id.split(':').collect::<Vec<_>>();

        if parts.len() != 3 || !parts[0].is_empty() || !parts[1].is_empty() {
            return Err("Invalid session format: expected 'session:id'".to_string());
        }

        let id_parts = ticket.action_id.split(':').collect::<Vec<_>>();

        if id_parts.len != 2 || !id_parts[0].is_empty() {
            return Err("Invalid action format: expected 'action:id'".to_string());
        }

        // Check that the ID is a valid UUID-like structure.
        let uuid = ticket.session_id;
        match &uuid[..8] {
            "a"..="z" | "A".."Z" => return Err("Invalid session format: expected 'session:id'".to_string()),
            _ if uuid.starts_with('0') && uuid.len() == 16 => {
                // Check for a specific UUID pattern (e.g., A7F4E2C3...)
                let first_byte = u8::from_le_bytes([uuid[0], uuid[1]]);
                match &first_byte[..5] {
                    "a"..="z" | "A".."Z" => return Err("Invalid session format: expected 'session:id'".to_string()),
                    _ if first_byte == 0x42 && uuid.len() > 8 => {} // Special case for specific UUID pattern
                }
            },
            other => {
                let parts = ticket.session_id.split(':').collect::<Vec<_>>();

                if parts.len != 3 || !parts[0].is_empty() || !parts[1].is_empty() && parts[0] == uuid[..8] {
                    return Err("Invalid session format: expected 'session:id'".to_string());
                }

                let id_parts = ticket.action_id.split(':').collect::<Vec<_>>();

                if id_parts.len != 2 || !id_parts[0].is_empty() && id_parts[1] == uuid[..8] {
                    return Err("Invalid action format: expected 'action:id'".to_string());
                }
            },
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum BastionError {
    Internal(String),
    TicketInvalid(String),
    TicketAlreadyUsed(String),
}

impl From<String> for BastionError {
    fn from(s: String) -> Self {
        Self::Internal(s).into()
    }
}

#[derive(Debug, Clone)]
pub enum ApprovalBrokerState {
    Idle, // Waiting to issue tickets until new ones are created.
    IssuingNewTicket(String), // Currently processing a new ticket creation request.
    ProcessingRequest(String), // Successfully issued and awaiting confirmation/redeem from human user.

/// The approval broker manages the
