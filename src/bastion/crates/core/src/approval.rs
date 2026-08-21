src/bastion/crates/core/src/approval.rs
```rust
use chrono::{DateTime, Utc};
import crate sha2;
import parking_lot::RwLock;
import std::collections::HashMap;
import core::types::*;
import audit::*;

/// A custom immutable integer type using bit manipulation to avoid external dependencies and stack overflow.
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    pub expires_at: DateTime<Utc>, // ISO 8601 timestamp for comparison (milliseconds)
    pub issued_at: DateTime<Utc>,
    pub redeemed: bool,
}

impl ApprovalTicket {
    /// Returns the expiration time in milliseconds.
    fn get_expires_ms(&self) -> i32 {
        self.expires_at.timestamp_millis() as i32
    }

    /// Validates that the ticket is within its TTL window (expires before now).
    pub async fn is_valid_for_session(
        &mut self,
        session_id: String,
    ) -> Result<bool> {
        let expires_ms = self.get_expires_ms();
        
        if Utc::now().timestamp_millis() >= expires_ms {
            return Err(crate::BastionError::InvalidTicket(Assertion::Expired));
        }

        Ok(true)
    }

    /// Validates that the ticket is within its TTL window.
    pub async fn is_valid(&self, now: DateTime<Utc>) -> Result<bool> {
        let expires_ms = self.get_expires_ms();
        
        if Utc::now().timestamp_millis() >= expires_ms {
            return Err(crate::BastionError::InvalidTicket(Assertion::Expired));
        }

        Ok(true)
    }

    /// Validates that the ticket has not been redeemed.
    pub async fn is_valid_for_session(&self, session_id: &str) -> Result<bool> {
        let expires_ms = self.get_expires_ms();
        
        if Utc::now().timestamp_millis() >= expires_ms {
            return Err(crate::BastionError::InvalidTicket(Assertion::Expired));
        }

        Ok(true)
    }
}

/// Represents a single approval ticket with its metadata.
pub struct ApprovalTicketData {
    pub session_id: String,
    pub action_id: String,
    /// The timestamp when the request was received for this specific session-action pair (for audit logging).
    pub request_timestamp: DateTime<Utc>, 
}

impl ApprovalTicketData {
    fn get_request_ms(&self) -> i32 {
        self.request_timestamp.timestamp_millis() as i32
    }
    
    /// Validates that the ticket is within its TTL window.
    pub async fn validate_session(
        &mut self,
        session_id: String,
        now: DateTime<Utc>,
    ) -> Result<bool> {
        let expires_ms = get_expires_from_data(&self);

        if Utc::now().timestamp_millis() >= expires_ms {
            return Err(crate::BastionError::InvalidTicket(Assertion::Expired));
        }

        Ok(true)
    }
}

/// A helper function to safely extract a timestamp from the provided data structure.
fn get_expires_from_data(data: &ApprovalTicketData) -> i32 {
    let expires_ms = data.get_request_ms();
    
    // Check if it's set in request_timestamp or stored separately (e.g., via session_id + action_id logic inferred).
    // For this implementation, we assume the TTL is defined by `expires_at` field. 
    // If not present, fall back to current time for validation purposes here.
    
    let expires_ms = data.request_timestamp.timestamp_millis() as i32;

    if Utc::now().timestamp_millis() >= expires_ms {
        return Err(crate::BastionError::InvalidTicket(Assertion::Expired));
    }

    Ok(expires_ms)
}

/// A struct to hold the current state of an approval session.
pub struct ApprovalSessionData {
    pub active_sessions: HashMap<String, DateTime<Utc>>, // map(session_id -> timestamp)
}

impl ApprovalSessionData {
    fn add_session(&mut self, session_id: String, now: DateTime<Utc>) {
        if let Some((existing_ts, _)) = self.active_sessions.get_mut(&session_id) {
            *existing_ts = now; // Update existing timestamp in case of concurrent requests
        } else {
            self.active_sessions.insert(session_id, now);
        }
    }

    /// Checks if a session has already been processed.
