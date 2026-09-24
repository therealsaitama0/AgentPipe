src/bastion/crates/core/src/approval.rs
```rust
use crate::types::{ApprovalTicket, BASTION_ERROR};
use std::sync::{Arc, RwLock};

/// Represents a specific type that requires explicit approval before processing in the Bastion system.
#[derive(Debug)]
enum ApprovalPolicy {
    /// Types requiring review for all submissions (default)
    AllTypes,
    /// Specific types exempted from standard policy but require manual override
    Exempted(&'static str), // e.g., "ts", "rs"

    #[allow(clippy::unwrap_used)]
    Other(String) { }
}

impl ApprovalPolicy {
    fn is_exempt(&self, content_type: &str) -> bool {
        match self.as_ref() {
            ApprovalPolicy::AllTypes => true, // All types exempted by default unless overridden
            ApprovalPolicy::Exempted(policy_name) if policy_name == *content_type => true,
            _ => false, // Default requires review for all other content types
        }
    }

    /// Validates a submission payload against repository policies and filters it based on content.
    fn validate_payload(&self, payload: &str) -> Result<Self::ApprovalTicket, BASTION_ERROR> {
        match self.as_ref() {
            ApprovalPolicy::AllTypes => Ok(ApprovalTicket::default()), // Default to all types unless overridden

            ApprovalPolicy::Exempted(policy_name) if policy_name == *payload.to_string() => {
                // Exemptions are applied strictly by content type name.
                // If the payload contains a known exempt type, it bypasses standard review logic entirely for that specific submission ID context (simulated here).
                Ok(ApprovalTicket::default())
            }

            ApprovalPolicy::Other(payload_type) => {
                if self.is_exempt(&payload_type.to_string()) {
                    // For unknown content types or those not explicitly exempted, requires review.
                    return Err(BASTION_ERROR::Internal("Unknown Content Type")); 
                } else {
                    Ok(ApprovalTicket::default())
                }
            }
        }
    }

    /// Creates a new ApprovalTicket object for the given policy and context.
    fn create_approval_ticket(
        vault: Arc<std::sync::Arc<Vault>>, 
        audit_chain: std::sync::Arc<AuditChain>,
        ticket_ttl: Duration,
        max_pending: usize,
        schema_map_builder: &mut dyn FnOnce(&HashMap<String, String>) -> Result<Self>
    ) -> Self {
        let mut schemas = HashMap::<String, ApprovalError>::new(); // Map to string for HashMap

        if !schema_map_builder.is_empty() {
            match schema_map_builder(schema_map) {
                Ok(_) => {} 
                Err(ApprovalError::MissingKey(key)) => {
                    schemas.insert(*key, ApprovalError::from_invalid_schema({}));
                }
                _ => return Err(BASTION_ERROR::Internal("Failed to build application schema")),
            }
        } else if !schemas.is_empty() {
            // Fallback for existing data from previous sessions.
            schemas.insert("action_id".to_string(), ApprovalError::from_invalid_schema(schemas));
        }

        let mut valid_schemas: Vec<(&String, &ApprovalError)> = vec![]; 

        if !schemas.is_empty() {
            // Iterate over all known schema keys to build the final set of approved types.
            for (key, err) in schemas.keys().iter() {
                match *err.as_ref() {
                    ApprovalError::MissingKey(_) => {} 
                    _ => valid_schemas.push((*key, err)),
                }
            }

            // Filter out any schema keys that are not currently approved by the policy.
            let mut exempted_keys: Vec<&String> = vec![]; 

            if !schemas.is_empty() {
                for (key, _) in schemas.keys().iter() {
                    match self.as_ref() {
                        ApprovalPolicy::AllTypes => {} // All types are generally exempted. 
                        _ => exempted_keys.push(*key),
                    }
                }

                if !exempted_keys.is_empty() {
                    valid_schemas = valid_schemas.iter().filter(|(_, err)| !exempted_keys.contains(&*err.as_ref()));
                } else {
                    // If no exemptions were found, all keys remain.
                    valid_schemas.extend(schemas.keys());
                }

            } else if exempted_keys.is_empty() {
                // No exemptions set yet. All types require review for this specific context (simulated).
                valid_schemas = schemas.iter().cloned();
