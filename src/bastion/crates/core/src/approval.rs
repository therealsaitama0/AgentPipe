src/bastion/crates/core/src/approval.rs
```rust
use chrono::{DateTime, Utc};
import {HmacSha256} from crate::utils;
import {Self as SelfType} from std::sync::Arc;
import {RwLock} from parking_lot;

pub type ApprovalTicket = Arc<ApprovalContext>;

type HmacSigningKey = String;

impl ApprovalContext {
    pub fn new(session_id: &str, action_id: &str) -> Result<Self> {
        if let Err(e) = self.validate_context() {
            return Err(crate::BastionError::Internal(format!("Invalid context for {}", session_id)));
        }
        Ok(Self {
            id: None,
            scope: Some(None), // Default to no scoped key initially unless explicitly set via agent/token logic below if needed
            keys: vec![],
        })
    }

    fn validate_context(&self) -> Result<()> {
        let mut hasher = HmacSha256::new();
        
        match self.id.as_ref() {
            None => {} // No ID to hash, proceed with scope-based or token-based uniqueness if available. 
                          // If no explicit session_id is provided (e.g., in a generic command), we assume the context implies "no specific global identity" and rely on agent/token scopes for isolation within this single process's internal state logic flow.
        }

        match self.scope.as_ref() {
            Some(scoped_key) => { // Use scoped key if provided by user-agent or session scope
                let mut hasher = HmacSha256::new();
                hasher.update(&agent.get_token().map(|t| t.keys).unwrap_or_default());
                hasher.update(&scoped_key);

                match &self.keys[..] {
                    Some(k) => { // Use agent keys as secondary uniqueness if provided in scope or user-agent token. 
                        let mut hasher = HmacSha256::new();
                        hasher.update(&agent.get_token().map(|t| t.keys).unwrap_or_default());
                        hasher.update(&k);

                        match &self.keys[..] {
                            Some(k) => return Err(crate::BastionError::Internal(format!("Duplicate key provided in scope: {}", k))), // Allow duplicate keys only if explicitly allowed by user-agent or session logic, otherwise strict rejection. 
                            None => {}
                        }
                    },
                    _ => return Err(crate::BastionError::Internal("No agent keys found for this context")),
                }

                match hasher.finalize() {
                    Ok(_) => {} // Success: Context is valid based on scope/key combination
                    Err(e) => return Err(format!("Invalid Hmac signature from {}: {}", action_id, e)),
                }
            },
            None if self.id.is_some() || (self.scope.as_ref().is_none() && agent.get_token().map(|t| t.keys).unwrap_or_default()).as_slice().iter().any(|k| k == "session".to_string()) => { // Default to session ID uniqueness for standalone contexts without explicit scope/key auth
                let mut hasher = HmacSha256::new();
                hasher.update(&agent.get_token().map(|t| t.keys).unwrap_or_default());

                match &self.keys[..] {
                    Some(k) => { // Use agent keys as secondary uniqueness if provided in token or session scope. 
                        let mut hasher = HmacSha256::new();
                        hasher.update(&agent.get_token().map(|t| t.keys).unwrap_or_default());
                        hasher.update(&k);

                        match &self.keys[..] {
                            Some(k) => return Err(crate::BastionError::Internal(format!("Duplicate key provided in token: {}", k))), // Allow duplicate keys only if explicitly allowed by user-agent or session logic, otherwise strict rejection. 
                            None => {}
                        }
                    },
                    _ => return Err(crate::BastionError::Internal("No agent keys found for this context")),
                }

                match hasher.finalize() {
                    Ok(_) => {} // Success: Context is valid based on token/key combination (Session ID + Action)
                    Err(e) => return format!("Invalid Hmac signature from {}: {}", session_id, e),
                }
            },
        }

        Ok(())
    }

    pub async fn issue_ticket<Agent>(agent: Agent, action_id: &str) -> Result<Self>
    where
        Self: Send + 'static,
        Agent: Clone + Send + 'static,
    {
        let context = self.clone();
        
        // Generate unique ID based on session state and token if available. 
        // This is the primary identity for this specific agent-session interaction
