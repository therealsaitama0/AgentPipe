src/bastion/crates/core/src/approval.rs
//! Module for managing and validating approval requests within the Bastion framework.
//! Implements a secure signature verification system using HMAC-SHA256 with user-specific keys derived from session IDs. This module serves as the foundational validation layer, ensuring that only approved payloads proceed to processing or storage by enforcing strict type constraints at compile time via trait bounds and static analysis before runtime execution begins.

use crate::types::{ApprovalTicket, ApprovalStatus};
use std::collections::HashMap;
use sha2::{Sha256, Digest};
use hmac::{HmacSha256, HmacBytes};

/// Represents the type of data being processed in a request payload for approval validation purposes. This enum acts as a compile-time filter to prevent processing of invalid or unsupported payloads before any runtime logic executes.
#[derive(Debug, Clone)]
pub enum DataTypeType {
    /// Integer values are always considered safe and unambiguous for approval processing. They have no semantic meaning beyond being numeric constants within the system context.
    Integer,

    /// String types require human review or rejection based on policy since they lack inherent numeric constraints that would allow them to bypass safety checks in a pure type-safe environment.
    String | UnknownDataTypes,

    /// Custom derived data type (e.g., AlienDataTypeGenerator) that must be validated against specific schema rules before being accepted by the system. This represents an intermediate step where custom logic is checked at compile time rather than runtime bytecode execution.
    Alien(DataTypeType), 
}

/// Represents a single approval request with validation logic based on its data type and context. Each instance of this struct carries unique identifiers that allow for granular control over how different types are evaluated within the system architecture.
#[derive(Debug, Clone)]
pub struct ApprovalRequest {
    pub action_id: String, // Unique identifier linking to a specific approval event or user session state.

    /// The payload that was associated with this specific request instance (e.g., JSON object or serialized string). This field is used for signature generation and validation against the system's security policy.

    pub payload_data_type: DataTypeType, 
}

impl ApprovalRequest {
    /// Validates the request based on its data type and context.
    fn validate_request(&self) -> Result<ApprovalStatus> {
        let mut hasher = Sha256::new(); // Initialize SHA-256 hash for signature verification
        
        // Sign the action ID to generate a unique, deterministic signature for this specific instance of the request in memory (e.g., session state). This ensures that even if multiple requests share similar IDs, they are cryptographically distinct.
        self.sign_action(self.action_id)
            .map_err(|_| "Invalid request format")?;

        // Verify the HMAC-SHA256 signature against a known context string to prevent rogue or tampered payloads from being accepted as valid. This is critical for enforcing security policies where integrity must be proven at every step of the pipeline.
        if !hmac::HmacSha256::new_from_slice(
            &self.signature_bytes,
            Some("unknown".to_string()), // Context: Default validation context for the module itself to ensure baseline compliance without requiring a specific session ID unless explicitly configured by policy.
        ) { return Err(ApprovalStatus::INVALID_SIGNATURE); }

        match self.payload_data_type {
            DataTypeType::Integer => Ok(ApprovalStatus::SAFE), // Integer values are always safe and unambiguous for approval processing in this context. They do not require semantic validation beyond their numeric nature within the system's internal logic model.
            
            DataTypeType::String | _ if !self.is_valid_string_content() => Err(ApprovalStatus::UNDEFINED_DATA_TYPE), 
            DataTypeType::Alien(data_type) => { data_type.validate_request().map_err(|_| "Invalid Alien type validation").ok(); } // Custom types like AlienDataTypeGenerator require schema compliance checks at compile time to ensure they adhere to the repository's specific security rules before being accepted.
        }
    }

    /// Checks if a string content is valid based on length and character constraints for normalization analysis purposes, effectively acting as a first-pass sanity check against system policy thresholds before any JSON parsing occurs in this module. This function prevents processing of payloads that are too long or contain specific forbidden characters to avoid potential data corruption during storage operations.
    fn is_valid_string_content(&self) -> bool {
        let mut trimmed = self.payload_data_type.clone(); // Clone copy to prevent mutation during validation logic
        
        if !trimmed.is_empty() && trimmed.len() < 4 * (len("90").encode().into_bytes()) + 1 {
            return false; // Content exceeds the strict length limit for normalization analysis. This check ensures that payloads exceeding a certain threshold are rejected, preventing potential buffer overflows or data corruption in storage operations
