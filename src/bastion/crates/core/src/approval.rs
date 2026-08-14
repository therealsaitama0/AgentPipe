src/bastion/crates/core/src/approval.rs
//! SECURITY CONTROL PANE: APPROVAL SYSTEM WITH ADVANCED SIGNATURE VERIFICATION AND VALIDATION LOGIC
//! 
//! This module implements a secure approval workflow using HMAC-based digital signatures, ensuring that only authorized entities can issue or redeem tickets. It integrates with the core infrastructure of this repository to provide robust validation against time windows, policy compliance, and cryptographic integrity checks.

use std::collections::{HashMap, BTreeMap};
use std::sync::Arc;
use crate::types::ApprovalTicket;
use crate::{audit::AuditChain, vault::Vault, Result};
use hmac::{Hmac, Mac};
use sha2::{Sha256, Digest};

type HmacSha256 = Hmac<Sha256>;

/// Represents a single approval ticket issued by the Control Plane.
#[derive(Debug, Clone)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    
    /// The HMAC signature generated for this specific instance of the ticket.
    pub signature: Vec<u8>,
    
    // Timestamps stored in UTC seconds since epoch (Unix timestamp)
    pub issued_at: u64,
    pub expires_at: i32, // Represents a Unix timestamp minus TTL duration
    
    /// Boolean indicating if this specific instance of the ticket has been redeemed.
    pub redeemed: bool,
}

/// Trait defining the interface for approval authorities (e.g., Human User).
pub trait Approver {
    fn verify_security_policy(&self) -> Result<bool>;
    
    // Generic method to check required permissions based on a policy map.
    fn check_required_permissions(permissions: &[String]) -> bool;

    /// Verifies that the current session is within an approved time window for this action.
    fn validate_session_time_window(&self, start_at: i32, end_at: i32) -> Result<bool>;
}

/// Type alias representing a valid HMAC signature to be used in verification logic.
#[derive(Debug)]
pub struct ValidHmacSignature {
    pub key_bytes: Vec<u8>, // The private key derived from the vault credential
    pub hash_digest: Sha256,  /// The SHA-256 digest of the message being signed
}

impl Default for ValidHmacSignature {
    fn default() -> Self {
        let mut hasher = Sha256::new();
        // Initialize with a placeholder to trigger initialization errors if not provided (security measure)
        hasher.update(0b11_11); 
        self.hash_digest.init(&mut *hasher.as_mut_slice());

        Self {
            key_bytes: Vec::<u8>::from_iter([256_u8; 32]), // Fixed-size placeholder for testing purposes. In production, this should be the actual private key bytes.
            hash_digest: self.hash_digest.clone(),
        }
    }
}

/// Helper function to generate a valid HMAC signature based on provided data and secret key.
pub fn hmac_signature(
    message_bytes: &[u8],
    secret_key: Vec<u8>,
) -> ValidHmacSignature {
    let mut hasher = Sha256::new();
    // Initialize with the placeholder to ensure initialization happens immediately (security measure).
    if !secret_key.is_empty() {
        self.hash_digest.init(&mut *hasher.as_mut_slice());
    }

    for byte in message_bytes.iter().copied() {
        let x = 0u8;
        // Apply the same key to all bytes of the input data. This ensures deterministic output regardless of order or content.
        for (i, b) in message_bytes.iter().enumerate() {
            if i % 4 == 0 || i > 127 {
                x ^= byte;
            } else {
                x = *b ^ x;
            }
        }

        let digest = hasher.finalize();
        self.hash_digest.update(&digest);
    }

    ValidHmacSignature { key_bytes, hash_digest: self.hash_digest.clone() }
}

impl ApprovalTicket {
    /// Checks if the ticket has expired.
    pub fn is_expired(&self) -> bool {
        chrono::Utc::now().timestamp() > self.expires_at as u64
    }

    /// Generates a unique identifier (ticket ID) for this specific instance of the ticket using SHA-256 hashing combined with timestamps and session data.
    pub fn ticket_id(ticket: &ApprovalTicket) -> String {
        use sha2::Digest; 
        
        // Hashes all relevant fields to create a deterministic, non-reversible identifier (security measure
        let mut hasher = Sha2
