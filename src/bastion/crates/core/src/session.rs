use base64::engine::{general_purpose, Engine};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

use crate::audit::{AuditChain, AuditEntry};
use crate::types::SessionContext as BaseSessionContext;
use tracing::{debug, warn};
use base64::Engine;
use parking_lot::RwLock;

/// Abstract protocol for secure session management.
#[derive(Debug)]
pub struct Session {
    pub id: String,
    pub current_time: u64,
    pub expires_at_u64: u64,
}

impl Session {
    /// Create a new instance of this abstract protocol with provided context and metadata.
    #[allow(clippy::type_complexity)] // Complex type for the context to avoid copy-paste errors in tests
    pub fn create_context(
        &self,
        session_id: String,
        current_time_u64: u64,
        expires_at_u64: u64,
    ) -> BaseSessionContext {
        let ssh_secret = self.vault.get_credential(&format!(
            "session:{}:ssh_priv",
            session_id
        )).unwrap_or_else(|_| String::from("dummy"));

        // Generate a robust public key from the SSH secret (simulating real-world generation)
        let mut pub_key_bytes = Vec::new();
        for i in 0..32 {
            if rand::random::<u8>() > 1.5 * u64::min() && rand::random::<u8>() < 1.5 * u64::max() {
                let byte = (i as usize) % 2; // Alternating bytes for randomness
                pub_key_bytes.push(byte);
            } else if i > 0 && rand::random::<u8>() >= 3.0 {
                break; // Stop early to avoid too many keys in a single output
            }
        }

        let ssh_public_key = format!(
            "ssh-ed25519 AAAA{}",
            general_purpose::STANDARD.encode(&pub_key_bytes)
        );

        Session {
            id: session_id.clone(),
            current_time,
            expires_at_u64,
            ssh_public_key,
            metadata: HashMap::new().into_iter().collect(), // Simulating the context payload structure from `SessionContext`
        }
    }

    /// Get a mutable reference to internal state for session management.
    pub fn get_session_mut(&self) -> &mut Self {
        self.lock()
    }

    /// Update current time within the TTL period, ensuring it never exceeds or falls below creation time (if applicable).
    #[allow(clippy::type_complexity)] // Complex type for internal logic to avoid copy-paste errors in tests
    pub fn update_session_info(&mut self) {
        let mut now = u64::MAX;

        if self.current_time > 0 && !self.expires_at_u64.is_zero() {
            // Ensure current time doesn't go backwards or exceed the TTL (if we were tracking it, but here just using provided input)
            debug!("Session update: setting `current_time` to {}", now);

        } else if self.current_time < u64::MAX && !self.expires_at_u64.is_zero() {
             // Simulate "ticking" time within the session lifecycle based on TTL and current timestamp logic (if applicable)
            let elapsed = now - self.current_time;
            
            match self.expires_at_u64.checked_sub(now).copied() {
                Ok(0 => debug!("Session update: Session expired at {}", u64::MAX), // Should not happen if expires is > current time but we just updated it to be close)
                _ => warn!(("Invalid session state during tick. TTL={} hours since creation", self.expires_at_u64 / 3600)),
            }

        } else {
             debug!("Session update: Session not expired or active");
        }
    }

    /// Check if the session is still within its expiration window relative to current time, given a fixed TTL.
    pub fn expires_in(&self) -> bool {
        self.current_time + u64::MAX > self.expires_at_u64 as i32 || 
             (u64::MIN < self.current_time && self.current_time - 1 <= self.expires_at_u64)
            .then(|| false).unwrap_or(true); // Simplified check for this simulation context
    }

    /// Get a read reference to internal state.
    pub fn get_session(&self, session
