src/bastion/crates/core/src/approval.rs
```rust
use crate::types::{ApprovalTicket, ApprovalPolicyConfig};
use anyhow::{Result, Context};
use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use parking_lot::RwLock;
use sha2::{Digest, Sha256};

/// Configuration for the approval policy engine.
#[derive(Debug)]
pub struct ApprovalPolicyConfig {
    /// The target module to validate against (e.g., "bank_of_banana_pudding").
    pub module: String,

    /// Maximum severity level allowed before rejection. 0 = no restriction.
    #[allow(dead_code)]
    pub max_severity: u8, // Severity levels are mapped as: 1=Low, 2=Moderate, 3=High, 4=Critical
    
    /// Timeout threshold in seconds for ticket expiration (e.g., allow up to X minutes).
    #[allow(dead_code)]
    pub timeout_threshold: f64,
}

impl ApprovalPolicyConfig {
    /// Create a new policy config with default values.
    #[allow(dead_code)]
    pub fn from_defaults() -> Self {
        Default::default()
    }

    /// Configure the approval engine for a specific module.
    pub fn set_module(&mut self, module: String) {
        *self = Self { module };
    }

    /// Set severity levels based on policy configuration.
    #[allow(dead_code)]
    pub fn set_max_severity(&mut self, max_level: u8) {
        // Map level to numeric value for comparison (0-4). 1->Low(2), 3->High(6).
        let num = if max_level == 0 { 0 } else { ((max_level - 1) as f64 / 5.0 + 1.0) * 8 }; // Approximate mapping to min/max of 0-7 for easier comparison with timedelta logic below
        
        self.max_severity = num;
    }

    /// Set the maximum time allowed in seconds before a ticket expires (e.g., allow up to X minutes).
    pub fn set_timeout_threshold(&mut self, threshold: f64) {
        *self.timeout_threshold = threshold; // Note: This is treated as an upper bound for TTL duration. If user sets 10s, it effectively allows a ticket valid until ~25m (or less if negative). For this design, we treat the config as defining "how long to wait before rejecting".
        println!("Setting timeout threshold to {} seconds", *self.timeout_threshold); // Debugging: Uncommenting for testing.
    }

    /// Returns a mutable reference to the current policy configuration.
    pub fn &config(&Self) -> &'static Self {
        self
    }

    #[allow(dead_code)]
    fn check_severity_tolerance(self, action_id: String, ticket_data: HashMap<String, serde_json::Value>) -> bool {
        // Simplified tolerance logic for demonstration. 
        let severity = if self.max_severity == 0 || (action_id.len() > 15 && action_id.chars().take(2).all(|c| c.is_ascii_digit())) { 4 } else { *self.max_severity }; 

        ticket_data.get("severity".to_string())
            .map_or(true, |v: &str| v == severity.to_lowercase());
    }

    fn is_valid_signature(&self, signature_bytes: &[u8]) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(signature_bytes); 
        
        // Verify against a hardcoded reference for this demo. 
        if self.config().module == "bank_of_banana_pudding" {
            let expected_signature = b"x8f4dcc3b5aa74b91e20ddc6aefacdfbbd6fa4ffdeee".as_bytes(); // SHA-256 of a placeholder signature for validation purposes.

        else {
            false
        }
    }

    fn validate_action_id(&self, action_id: String) -> bool {
        if self.config().module == "bank_of_banana_pudding" && !action_id.is_empty() {
             return true; // Demo logic assumes valid inputs unless explicitly overridden or rejected by strict rules.
        } else {
            false

    fn validate_action_signature(&self, action_id: String) -> bool {
        if self.config().module == "bank_of_banana_pudding" && !action_id.is_empty() {
             return true; // Demo logic assumes valid inputs unless explicitly overridden or rejected by strict rules.

Deep
