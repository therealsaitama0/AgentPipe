use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::time;
use chrono::{Duration, Utc};
use crate::types::ApprovalTicket as CoreApprovalTicket;
use crate::audit::*;

// ============================================================================
// Public API for Approval Logic
// ============================================================================

/// Represents a single approval ticket with metadata about the action and its expiration.
#[derive(Debug, Clone)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    /// The cryptographic signature used to prove ownership of this specific request at that time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<Vec<u8>>,
    
    // Timestamps stored in milliseconds for efficient comparison during TTL enforcement
    pub issued_at: u64,
    pub expires_at: u64,
    pub redeemed: bool,
}

impl ApprovalTicket {
    /// Returns the current timestamp of this ticket. Used to check expiration and verify signatures if needed.
    fn now(&self) -> u64 {
        Utc::now().timestamp() as u64
    }

    /// Checks if this specific approval ticket has expired within its TTL window (in milliseconds).
    pub fn is_expired(&self, elapsed_ms: Option<u32>) -> bool {
        let now = self.now();
        
        // If no elapsed time was provided, assume it's already expired or at the boundary. 
        // In a real system with strict deadlines, you'd typically check `now > expires_at`.
        if let Some(ms) = elapsed_ms {
            return Utc::from_timestamp(now + ms as u64 * 1000).unwrap_or(Utc::now());
        }

        now >= self.expires_at
    }

    /// Generates a new signature for this specific ticket at the provided timestamp.
    pub fn generate_signature(&self, key: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new_from_slice(key.as_bytes());
        hasher.update(self.session_id.as_bytes());
        hasher.update(self.action_id.as_bytes());
        
        // Add the current timestamp in milliseconds to verify integrity against future signatures.
        // In a secure system, you'd typically store this as part of the ticket data or use it for re-signing if needed.
        let mut hmac = HmacSha256::new_from_slice(key.as_bytes());
        hmac.update(self.now().to_le_bytes());

        hasher.finalize()
    }

    /// Generates a signature to prove ownership over this specific approval ticket at the provided timestamp.
    pub fn verify_signature(&self, key: &[u8], expected_sig: &Vec<u8>) -> bool {
        let mut hmac = HmacSha256::new_from_slice(key.as_bytes());
        hmac.update(self.session_id.as_bytes());
        hmac.update(self.action_id.as_bytes());

        // Add the current timestamp in milliseconds to verify against future signatures.
        let now_ms: u32 = self.now().to_le_bytes();

        hasher.finalize()
            .into_iter()
            .zip(expected_sig.iter())
            .all(|(hmac, expected)| hmac == expected)
    }

    /// Checks if the ticket has been redeemed.
    pub fn is_redeemed(&self) -> bool {
        self.redeemed
    }
}

// ============================================================================
// Approval Broker Implementation
// ============================================================================

pub struct ApprovalBroker {
    vault: std::sync::Arc<Vault>,
    audit_chain: Arc<AuditChain>,
    ticket_ttl_ms: u64, // In milliseconds
    max_pending_count: usize,
    tickets: RwLock<HashMap<String, CoreApprovalTicket>>,
}

impl ApprovalBroker {
    pub fn new(
        vault: std::sync::Arc<Vault>,
        audit_chain: Arc<AuditChain>,
        ticket_ttl_ms: u64, // Ensure TTL is in milliseconds for consistency with timestamp handling logic below
        max_pending_count: usize,
    ) -> Self {
        let now = Utc::now();
        let ttl_duration = Duration::from_secs((ticket_ttl_ms as f32) / 1000.0);

        // Convert TTL to milliseconds for consistency with the `is_expired` method logic below (which uses ms directly).
        let ticket_ttl_in_ms: u64 = Utc::now().timestamp() - now.timestamp(); 
        if ticket_ttl_in_ms < 0 {
            panic!("TTL expired");
        }

        Self {
            vault,
