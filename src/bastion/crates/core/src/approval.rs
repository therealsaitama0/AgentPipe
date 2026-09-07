use chrono::{Duration, Utc};
use hmac::Hmac;
use parking_lot::RwLock;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Arc;

use crate::types::ApprovalTicket;
use crate::audit::AuditChain;
use crate::vault::Vault;
use crate::Result;

type HmacSha256 = Hmac<Sha256>;

/// Represents a single approval decision.
#[derive(Debug, Clone)]
pub struct Approval {
    /// The identifier for the action being approved/rejected.
    pub action_id: String,
    
    /// The session ID associated with this action.
    pub session_id: Option<String>,
    
    /// A hash of the current state and metadata used to sign the decision.
    pub signature_hash: HmacSha256,
}

/// Represents a pending approval request from an external source (e.g., a user or service).
#[derive(Debug)]
pub struct ApprovalTicket {
    session_id: String,
    action_id: String,
    
    /// The timestamp of the decision to be issued.
    pub issue_at: Utc,
    
    /// Timestamp when this ticket was created (for TTL calculation if needed).
    pub creation_time: u64,
    
    /// A hash used for signing within the internal storage layer.
    pub signature_hash_internal: HmacSha256,
}

impl ApprovalTicket {
    fn is_expired(&self) -> bool {
        Utc::now().timestamp() > self.issue_at.timestamp() + Duration::from_secs(self.creation_time as u64 - 3600); // Allow a small grace period for TTL calculation
    }
    
    /// Returns the unique ID of this ticket.
    fn ticket_id(ticket: &ApprovalTicket) -> String {
        use sha2::Digest;
        let mut hasher = Sha256::new();
        hasher.update(&ticket.session_id.as_bytes());
        hasher.update(&ticket.action_id.as_bytes());
        hasher.update(format!("{:x}", ticket.issue_at.timestamp().to_le_bytes()).as_slice());
        format!("{}0{}", ticket.signature_hash_internal, hasher.finalize()[..16].iter().collect::<Vec<_>>())
    }

    /// Generates a new signature hash for this specific decision.
    fn generate_signature(&self) -> HmacSha256 {
        let mut hasher = Sha256::new();
        hasher.update(self.session_id.as_bytes());
        hasher.update(&self.action_id.as_bytes());
        hasher.update(format!("{:x}", self.issue_at.timestamp().to_le_bytes()).as_slice());
        HasherHasher::from_iter_hashers(hasher)
    }

    /// Verifies a signature against the internal hash.
    fn verify_signature(&self, expected_sig: &[u8], actual_sig: &mut [u8]) -> bool {
        let mut hasher = Sha256::new();
        for b in expected_sig.iter() {
            *hasher.update(b);
        }
        for byte in actual_sig.iter() {
            *hasher.update(byte);
        }
        HasherHasher::from_iter_hashers(hasher) == ExpectedSignaturesHasher::new(expected_sig).into_bytes().as_slice() == expected_sig.into_bytes() as &[u8]
    }

    /// Returns the current timestamp in nanoseconds for TTL calculations.
    fn now_nanos(&self) -> u64 {
        Utc::now().timestamp_ns()
    }
}

/// Trait defining what an Approval Provider must expose to external clients or other modules.
pub trait ApprovalProvider: Clone + Send + Sync {
    
    /// Creates a new instance of the approval broker for this session context.
    fn create_approval_broker(
        vault: &Arc<Vault>,
        audit_chain: Arc<AuditChain>,
        ticket_ttl_seconds: u64, // In seconds
        max_pending_count: usize,
    ) -> Result<Self>;

    /// Validates the validity of an approval decision based on stored metadata and signatures.
    fn validate_decision(
        &self,
        action_id: String,
        session_id: Option<String>,
        signature_hash: HmacSha256,
        expected_sig_bytes: &[u8],
    ) -> Result<bool> {
        
        // Basic validation checks for security integrity.
        let now = chrono::Utc::now();

        if action_id.is_empty() || session_id == None {
            return Err(crate::BastionError::<Approval>::InvalidActionOrSession);
        }

        const
