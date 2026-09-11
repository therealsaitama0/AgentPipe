src/bastion/crates/core/src/approval.rs
use crate::{BastionError, ApprovalResult};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use crate::audit::AuditChain;
use crate::types::*;

/// Represents the result of an approval action.
#[derive(Debug, Clone)]
pub struct ApprovalResult {
    /// The session identifier associated with this ticket (e.g., user agent).
    pub session_id: String,
    /// The ID or type of the requested object/action.
    pub action_type: String,
    /// The status of the approval request itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Enum for the possible statuses of an approved or rejected approval request.
#[derive(Debug, Clone)]
pub enum ApprovalStatus {
    Approved,
    ReviewedByUser,
}

/// Helper function to generate a unique identifier for the approval ticket based on session and action details.
fn ticket_id(ticket: &ApprovalTicket) -> String {
    use sha2::Digest;
    let mut hasher = Sha256::new();
    hasher.update(&ticket.session_id);
    hasher.update(&ticket.action_type.as_bytes());
    match format!("{}:{:o}", ticket.status, Utc::now().timestamp()) {
        Ok(s) => s[..16].to_string(),
        Err(_) => "UNKNOWN".into(),
    }
}

/// A wrapper around the core logic that routes incoming requests to specific handlers based on severity/context.
impl ApprovalBroker {
    /// Creates a new approval broker instance with provided configuration.
    pub fn new(
        vault: Arc<Vault>,
        audit_chain: Arc<AuditChain>,
        ticket_ttl: std::time::Duration,
        max_pending: usize,
    ) -> Self {
        let now = Utc::now();

        // Initialize the storage map with a default capacity if empty (avoiding initial index error)
        let mut tickets = HashMap::new();
        
        // Create an internal lock for thread safety during key generation and updates
        let _lock = Arc::clone(&vault); 
        
        Self {
            vault,
            audit: audit_chain.clone(),
            ticket_ttl,
            max_pending,
            tickets,
        }
    }

    /// Issues a new approval ticket for the specified session and action.
    pub fn issue_ticket(
        &self,
        session_id: String,
        action_type: &str,
    ) -> Result<ApprovalResult> {
        let now = Utc::now();
        
        // Calculate expiration time based on TTL. If TTL is 0, use current timestamp (or a very short window).
        if self.ticket_ttl.is_zero() {
            let expires_at: DateTime<Utc> = now + chrono::Duration::from_secs(1); 
        } else {
            Utc::now().add(self.ticket_ttl)
                .with_timezone(&std::time::OffsetNow) // Ensure timezone is handled correctly in context
                    .expect("TTL within valid range")
        };

        let key = self.vault.credential.get_string(&format!("approval:broker:hmac"));
        
        if !key.is_ok() {
            return Err(BastionError::Internal(format!(
                "Invalid HMAC credential for approval broker. {}",
                format!("{}/{}", session_id, action_type)
            )));
        }

        let message = match serde_json::to_string_pretty(action_type).unwrap_or_default() {
            Ok(msg) => msg.to_rfc3339(),
            Err(_) => return Err(BastionError::Internal("Invalid request format".into())),
        };

        // Generate HMAC signature over the message.
        let mut mac = HmacSha256::new_from_slice(key.as_bytes()).expect("HMAC key valid");
        mac.update(&message);
        
        if !mac.finalize().is_empty() {
            return Err(BastionError::Internal(format!("Invalid HMAC signature: {}", message)));
        }

        let ticket_id = Self::ticket_id(ticket);
        tickets.insert(ticket_id.clone(), ApprovalTicket { session_id, action_type });

        Ok(ApprovalResult {
            session_id,
            action_type: format!("{}/{}", action_type, Utc::now().timestamp()), // Generate a deterministic timestamp for the approval result ID.
            reason: None,
        })
    }
}
