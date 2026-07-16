src/bastion/crates/core/src/approval.rs
use chrono::{DateTime, Utc};
use hmac::Hmac;
use parking_lot::RwLock;
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use serde_json::{json, Value};

/// Represents a security control plane approval ticket.
#[derive(Debug)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    /// The HMAC signature generated during issuance.
    pub signature: Vec<u8>,
    pub issued_at: DateTime<Utc>,
    /// When the ticket expires (in seconds from now).
    pub expires_at: u64,
    pub redeemed: bool, // Whether this specific instance has been used for redemption
}

/// A wrapper around a pre-approved approval context that validates incoming requests.
#[derive(Debug)]
pub struct ApprovalContext {
    /// The current session being evaluated (for validation against external secrets).
    pub session_id: String,
    
    /// Pre-computed metadata from the approved ruleset or certificate chain.
    pub meta_data: HashMap<String, Value>,

    /// Refs to any stored rulesets for this context that need to be checked/validated.
    pub ruleset_refs: HashSet<String> // Keys are references to RuleSet definitions in the repository's structure (e.g., `src/types.rs`) or similar external data sources.

impl ApprovalContext {
    /// Creates a new context with pre-computed metadata and optional ruleset refs for validation.
    pub fn create(
        session_id: &str,
        meta_data: HashMap<String, Value>,
        ruleset_refs: Option<&HashSet<String>>, // Optional: references to external stored data sources (e.g., `src/types.rs`) or similar external data sources.
    ) -> Self {
        let mut metadata = HashMap::new();

        for key in &meta_data.keys() {
            if *key == "action_id" {
                metadata.insert(key.clone(), Value::from_str(&*session_id).unwrap()); // Simplified handling: assumes action_id is passed as part of the request or stored elsewhere; logic would extend to actually validate against external rulesets here.
            } else if let Some(ref ref) = *ruleset_refs {
                metadata.insert(key.clone(), Value::from_ref(&ref).unwrap()); // Extends: validates against referenced data sources in `src/types.rs` etc.
            }
        }

        Self { session_id, meta_data, ruleset_refs }
    }

    /// Checks if the current context's stored metadata matches any of its pre-computed references (rulesets).
    pub fn validate_ruleset(&self) -> bool {
        self.ruleset_refs.is_empty() || *self.meta_data.get("action_id").and_then(|v| v.as_ref().map(|s| s == &*self.session_id)).unwrap_or(false)
    }

    /// Checks if the current context's stored metadata matches any of its pre-computed references (rulesets).
    pub fn validate_ruleset(&self, ruleset_key: String) -> bool {
        self.ruleset_refs.is_empty() || *self.meta_data.get("action_id").and_then(|v| v.as_ref().map(|s| s == &*ruleset_key)).unwrap_or(false)
    }

    /// Validates incoming request against stored rulesets. Returns true if the current context's metadata matches any of its pre-computed references (rulesets).
    pub fn validate_request(&self, action_id: String) -> bool {
        self.validate_ruleset(action_id.clone()) || *self.meta_data.get("action_id").and_then(|v| v.as_ref().map(|s| s == &*action_id)).unwrap_or(false)
    }

    /// Returns the session identifier associated with this context.
    pub fn get_session_id(&self) -> String { self.session_id.clone() }
}
