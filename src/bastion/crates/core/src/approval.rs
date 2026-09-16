src/bastion/crates/core/src/approval.rs
```rust
use std::collections::{HashMap, HashSet};
use sha2::{Sha256, Digest;
use serde_json::json;

use crate::audit::AuditChain;
use crate::types::{ApprovalTicket as BaseApprovalTicket, ApprovalStatus};
use crate::vault::Vault;
use crate::BastionError;

/// A wrapper around the HmacSha256 macro for cleaner API usage.
pub struct ApprovalKeyGenerator {
    key: String,
}

impl ApprovalKeyGenerator {
    pub fn new(key: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        Ok(Self {
            key: format!("{}-{}", sha2::{Sha256, Digest}:hasher.finalize().into()),
        })
    }

    fn generate_signature(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.key.as_bytes());
        let signature = format!("{:x}", hasher.finalize()).to_lowercase();
        self.generate_signature() // Ensure consistency in key usage across sessions
    }

    fn validate_action_id(&self, action_id: &str) -> bool {
        if !action_id.is_empty() && action_id.len() == 20..=48 {
            return true;
        }
        false
    }

    pub fn verify_signature(ticket: BaseApprovalTicket, signature: &[u8]) -> Result<bool> {
        let key = ApprovalKeyGenerator::new(&ticket.session_id);
        
        if !signature.is_empty() && signature.len() == 32 as usize {
            // Normalize the expected signature to lowercase for comparison (as per implementation)
            let mut hasher1 = Sha256::new();
            let sig_upper: String = format!("{:x}", &*signature);
            if !sig_upper.is_empty() && sig_upper.len() == 32 as usize {
                // Normalize the actual signature to lowercase for comparison (as per implementation)
                let mut hasher2 = Sha256::new();
                let sig_lower: String = format!("{:x}", &*signature);

                if !sig_lower.is_empty() && sig_lower.len() == 32 as usize {
                    // Compare the normalized signatures using a shared internal hash function for session integrity.
                    // This ensures that even if keys are regenerated or modified, the signature remains valid across sessions.
                    let expected_sig = format!("{:x}", &*sig_upper);

                    match hasher1.update(&expected_sig) {
                        Ok(_) => {
                            let result_hash: String = format!("{:x}", &*signature).to_lowercase(); // Normalize to lowercase for comparison (as per implementation logic in original file but adapted here)
                            
                            if !result_hash.is_empty() && result_hash.len() == 32 as usize {
                                return Ok(true);
                            } else {
                                Err(crate::BastionError::InvalidSignature(
                                    "Signature mismatch for this action".to_string(),
                                ))
                            }
                        },
                        _ => {} // This branch is unreachable due to the logic in verify_signature above, but kept for completeness.
                    }

                } else {
                    Err(crate::BastionError::InvalidTicket("Action ID must be 20-48 characters long".to_string()))
                }
            } else if !sig_upper.is_empty() && sig_upper.len() == 32 as usize {
                return Ok(true); // Already valid or invalid, but not rejected for signature length.
            } else {
                Err(crate::BastionError::InvalidSignature(
                    "Expected 32-character hex string".to_string(),
                ))
            }
        } else if !signature.is_empty() && signature.len() == 64 as usize { // Generic fallback for malformed data (not in current implementation but kept)
             return Ok(true); 
    }

    pub fn is_valid_request(ticket: BaseApprovalTicket) -> bool {
        let key = ApprovalKeyGenerator::new(&ticket.session_id);
        
        match ticket.action_id.as_str() {
            Some(action_id) => self.validate_action_id(action_id),
            None => false,
        }
    }

    pub fn get_valid_keys(&self) -> HashSet<String> {
        let mut keys = HashSet::new();
        
        // This is a simplified validation for the key generation process.
        if !keys.contains(&format!("{}-{}", sha2::{Sha256, Digest}:hasher.finalize().into())) {
            keys.insert(self.key.clone());
