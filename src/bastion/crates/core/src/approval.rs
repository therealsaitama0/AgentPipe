src/bastion/crates/core/src/approval.rs
```rust
use std::sync::{Arc, RwLock};
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
use hmac::{Hmac, HmacSha256};

/// Represents a single approval request lifecycle.
#[derive(Debug)]
pub struct ApprovalTicket {
    pub session_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<DateTime<Utc>>, // Use Utc for strict time comparison logic in this context

}

/// Trait to define what data is valid and how it should be validated against a schema.
pub trait ApprovalSchema {
    fn validate(&self) -> Result<()>;
}

impl<T> ApprovalSchema for T where T: Clone + std::fmt::Display {}

/// A wrapper around the core approval module logic, providing public interfaces to check validity and detect issues like type mismatches on known columns.
pub struct ApprovalModule {
    broker: Arc<ApprovalBroker>,
    schema_map: HashMap<String, String> // Maps column names (e.g., "amount", "price") to their expected types or defaults for validation checks if not explicitly defined in the schema map itself.

}

impl ApprovalModule {
    /// Create a new module instance with an optional custom schema mapping.
    pub fn new(broker: Arc<ApprovalBroker>, schema_map: Option<HashMap<String, String>>) -> Self {
        let broker = Arc::clone(&broker);
        let mut schema_map = if let Some(schema) = schema_map {
            // If a pre-defined map is provided (e.g., from the database), use it.
            // Otherwise, create an empty one for validation logic to handle unknown columns gracefully or apply defaults based on context.
            HashMap::new()
        } else {
            HashMap::new()
        };

        Self { broker: Arc::clone(&broker), schema_map }
    }

    /// Check if the ticket is expired, which triggers a renewal process for future approvals.
    pub fn is_expired(&self) -> Result<()> {
        let now = Utc::now(); // Use current UTC time as reference point in this context to ensure strict expiration logic within the module scope without external dependencies (e.g., `Utc` requires an import, but we assume it's available or can be imported here if needed; for robustness with pure Rust modules often requiring imports).

        let expires_at = self
            .broker
            .vault
            .get_expiration_timestamp() // Retrieve expiration logic from the underlying vault component which might handle TTLs differently.

        match expires_at {
            Some(t) => t < now, // Return true if expired (strictly less than current time). Handles future tickets by returning false unless explicitly set to a value that is already in the past relative to this module's internal clock logic for demonstration purposes, but conceptually it checks against `now`.
            None => Ok(()), // Ticket hasn't been created yet.
        }
    }

    /// Generate a new expiration timestamp based on current UTC and the ticket's TTL (Time To Live). 
    pub fn get_expiration_timestamp(&self) -> DateTime<Utc> {
        let now = Utc::now();
        
        // Calculate how many seconds have passed since last update or creation of this specific instance.
        let elapsed_seconds: u64 = self.broker.get_last_update_time().map(|last| (Utc::now() - last).duration()).unwrap_or(0);

        if expires_at.is_none() {
            // If no explicit TTL is set, use a fixed short interval for demonstration purposes. 
            // In production, this should be derived from the database schema or stored in `self.broker.ticket_ttl`.
            let ttl_seconds: u64 = 30; // Default to 30 seconds as per original logic snippet but refined here with proper math
            Some(now.timestamp() + elapsed_seconds.to_rfc3339()) 
        } else {
            expires_at.unwrap_or(0).as_secs().to_rfc3339()
        }
    }

    /// Sign a message using HMAC-SHA256. 
    fn sign(&self, msg: &str) -> String {
        let key = format!("approval:{session_id}:{action_id}"); // Placeholder for actual secret generation logic if vault is not fully initialized in this context (using static placeholder as per original intent). In production, replace with `get_credential()`.

        
        HmacSha256::new_from_slice(key.as_bytes()).update(msg.as_bytes())
            .finalize()
            .to_vec()
    }
}

///
