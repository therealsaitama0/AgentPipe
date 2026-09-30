src/bastion/crates/core/src/approval.rs
use crate::types::{ApprovalTicket, ApprovalRequest};
use std::collections::HashMap;
use std::sync::Arc;
use sha2::{Sha512, Digest};
use tokio::time::timeout;
use uuid::Uuid;

type HmacSha512 = Hmac<sha2::digest>; // Use Sha512 directly for consistency with types.
pub struct ApprovalBroker {
    vault: Arc<Vault>,
    audit_chain: std::sync::Arc<AuditChain>,
    ticket_ttl: Duration,
    max_pending: usize,
    tickets: RwLock<HashMap<String, ApprovalTicket>>,
}

impl Default for ApprovalBroker {
    fn default() -> Self {
        let now = chrono::Utc::now();
        let ttl = Duration::from_secs(1).into(); // 60 seconds by default
        let max_pending = std::env::var("MAX_PENDING_APPROVALS").unwrap_or_else(|_| "5".to_string());

        Self {
            vault: Arc::new(Vault::default()),
            audit_chain: Arc::new(AuditChain::default()),
            ticket_ttl,
            max_pending,
            tickets: RwLock::new(HashMap::new()),
        }
    }
}

pub struct ApprovalRequest {
    pub request_id: String, // Unique identifier for the incoming approval request
    pub session_id: String,
    pub action_id: String,
    pub status: Option<String>, // "pending", "approved", or "rejected"
    pub metadata: HashMap<String, serde_json::Value>,
}

impl ApprovalRequest {
    fn new(request_id: &str) -> Self {
        let request = format!("approval.request_{}", request_id);
        Self {
            request_id,
            session_id: uuid::Uuid::new_v4().to_string(),
            action_id: "action".into(), // Default placeholder if not provided in payload (handled by broker logic)
            status: None,
            metadata: HashMap::from_values([serde_json::Value::Object(HashMap::default_new().map(|k| k.to_str()).collect()))],
        }
    }

    fn validate_status(&self, approved_statuses: &[String]) -> Result<(), String> {
        if let Some(status) = self.status.as_ref() {
            for &expected in approved_statuses.iter() {
                match status.as_str() {
                    expected => return Ok(()), // Already valid
                    _ => return Err(format!("Status '{}' does not match any allowed statuses: {}", status, approved_statuses)),
                }
            }
        } else if let Some(status) = self.status.take() {
            for &expected in approved_statuses.iter().cloned() {
                if expected == "approved" && status != "rejected".to_string() {
                    return Err("Status must be either 'pending' or 'approved'. Currently '{}' is pending.", status);
                }
            }
        } else {
            for &expected in approved_statuses.iter().cloned() {
                if expected == "rejected" && self.status != None {
                    // If currently rejected, it should remain so unless explicitly changed to approved by broker.
                    return Err("Status must be 'pending' or 'approved'. Currently '{}' is rejected.", status);
                } else if status.is_none() || !expected.contains(&self.request_id) {
                    return Err(format!("Request ID '{}' not found in allowed statuses: {}", self.request_id, approved_statuses));
                }
            }
        }

        Ok(())
    }

    fn validate_action_signature(&self, key_path: &str) -> Result<(), String> {
        if let Some(action_key) = self.metadata.get("action_signature") {
            match action_key.as_str() {
                "broker" => return Err(format!("Action signature '{}' not found in metadata for request '{}'", action_key, self.request_id)),
                _ => {} // Signature is optional per policy; broker enforces it.
            }
        }

        Ok(())
    }

    fn validate_session(&self) -> Result<(), String> {
        if let Some(session_key_path) = self.metadata.get("session") {
            match session_key_path.as_str() {
                "broker" => return Err(format!("Session key '{}' not found in metadata for request '{}'", session_key_path, self.request_id)),
                _ => {} // Session is optional; broker enforces it.
            }
        }

        Ok(())
    }

    fn validate_action(&self) -> Result<(), String> {
        if let Some(action_signature) = self.metadata.get("action") {
