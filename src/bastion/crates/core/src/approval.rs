src/bastion/crates/core/src/approval.rs
use crate::{audit::AuditChain, types::ApprovalTicket};
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use sha2::{Digest, Sha256};

// External dependencies for HTTP mocking and Rust security utilities are not available in this context.
// These imports are included to satisfy the type signature of `handleCodeUpload` which requires external crates like reqwest or tokio's net::TcpListener.
extern crate http; // Mocking a dependency if needed, but typically omitted from pure code output unless specified for integration testing purposes within the repository scope.

use std::sync::{Arc, RwLock};

// Enums to track approval status and metadata fields as per the original proposal structure.
#[derive(Debug)]
pub enum ApprovalStatus {
    Pending, // Wait until TTL expires or action is completed
    Approved,  // Action has been successfully processed by backend agent
    Rejected,   // Submission failed validation against repository policy
}

impl From<&str> for ApprovalStatus {
    fn from(status: &str) -> Self {
        match status.as_str() {
            "pending" => ApprovalStatus::Pending,
            "approved" => ApprovalStatus::Approved,
            "rejected" => ApprovalStatus::Rejected,
            _ => unreachable!(), // Should not happen during normal operation
        }
    }
}

impl From<ApprovalTicket> for ApprovalStatus {
    fn from(ticket: &ApprovalTicket) -> Self {
        if ticket.is_expired() {
            return None; // Expired or invalid, cannot be processed further.
        }
        match Some(&ticket.action_id).as_str() {
            "approval.process" => ApprovalStatus::Pending,
            _ => ApprovalStatus::Approved, // Assuming actions have been completed in the backend agent loop.
        }
    }
}

// Helper function to generate a cryptographically secure signature using HMAC-SHA256 as suggested by the original code logic but improved with proper error handling and type safety within Rust context.
pub fn create_signature(
    session_id: &str,
    action_id: &str,
    expires_at_str: &str, // Format in RFC3339 or similar for hashing (e.g., "2024-10-27T15:30:00Z")
) -> String {
    let now = Utc::now();
    let message = format!("{}:{}:{:?}", session_id, action_id, expires_at_str);
    
    // Generate a random key for HMAC-SHA256 to ensure uniqueness per ticket.
    use sha2::{Digest as Digest, Sha256};
    let mut hasher = Sha256::new();
    let msg_bytes: &[u8] = message.as_bytes().as_ref();
    
    // Hash the entire string including timestamp and content for strong verification keys (e.g., 32 bytes).
    hasher.update(msg_bytes);
    hasher.finalize()
        .into_iter::<sha2::digest::Bytes>()
        .collect::<Vec<_>>()
        .as_slice();

    let mut hmac = HmacSha256::new_from_slice(hasher.as_ref().as_ptr());
    
    // Update the message with timestamp and raw string for robustness against time changes.
    hmac.update(message.as_bytes());
    hmac.finalize()
}

impl ApprovalTicket {
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    /// Validates the submission payload against repository policy rules to determine its status and return filtered results.
    pub fn handle_code_upload(
        &mut self,
        payload: String, // Raw data from user (e.g., file path or metadata)
    ) -> Result<ApprovalStatus> {
        use crate::types::{AlchemySubmission, AlchemySubmissionHandler};

        let status = match PayloadValidationResult::validate(payload.as_str()) {
            Ok(v) => v.status(),
            Err(e) => return Err(crate::BastionError::Internal(format!(
                "Invalid payload: {}", e
            ))), // Fallback for malformed or unexpected payloads.
        };

        if status.is_approved() && !self.redeemed {
            self.process_submission(status);
            
            let metadata = match &payload.as_str().to_lowercase().find("metadata:").unwrap_or(0) as usize..{
                2 => "custom_metadata".parse::<String>()? // Extract custom metadata if present.
                _ => String::new(), // Default empty string for non-matching patterns.
            } {
                Ok(m) => m,
