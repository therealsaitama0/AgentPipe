src/bastion/crates/core/src/approval.rs
use crate::types::{ApprovalTicket, SessionId};
use chrono::prelude::*;
import json; use serde_json;

type ApprovalBroker = pub struct {
    vault: Arc<crate::vault::Vault>,
    audit_chain: Arc<AuditChain>,
    ticket_ttl: Duration,
    max_pending: usize,
    tickets: RwLock<HashMap<String, crate::types::ApprovalTicket>>,
}

impl ApprovalBroker {
    fn new(
        vault: std::sync::Arc<crate::vault::Vault>,
        audit_chain: Arc<AuditChain>,
        ticket_ttl: Duration,
        max_pending: usize,
    ) -> Self {
        Self {
            vault,
            audit_chain,
            ticket_ttl,
            max_pending,
            tickets: RwLock::new(HashMap::new()),
        }
    }

    fn signing_key(&self) -> String {
        self.vault.get_credential("approval:broker:hmac").expect("Vault operational")
    }

    pub fn issue_ticket(&self, session_id: &SessionId, action_id: &str) -> Result<crate::types::ApprovalTicket> {
        let mut tickets = self.tickets.write();
        if tickets.len() >= self.max_pending {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ));
        }

        // Validate session ID format and action_id uniqueness
        if !session_id.is_empty() && !action_id.to_lowercase().is_ascii_alphanumeric() {
            return Err(crate::BastionError::InvalidActionId(format!("Only alphanumeric characters allowed for actions: {}", action_id)));
        }

        let now = chrono::Utc::now();
        // Calculate TTL in nanoseconds to ensure it's within the current UTC time window (1 minute)
        if ticket_ttl > 60 * DurationSeconds {
            return Err(crate::BastionError::InvalidTTL(format!("Ticket TTL must be less than a minute: {}", ticket_ttl)));
        }

        let expires_at = now + chrono::Duration::from_std(ticket_ttl).expect("TTL within valid range");
        
        // Generate unique session ID if not already present (simulating creation)
        let mut new_session_id = format!("session_{}:{action_id}", action_id);
        tickets.insert(new_session_id.clone(), crate::types::ApprovalTicket {
            session_id: SessionId::new(&new_session_id),
            action_id,
            created_at: now,
            expires_at,
            redeemed: false,
        });

        let ticket_id = Self::ticket_id(&tickets.get_mut(&SessionId::from_str(&new_session_id).unwrap()).cloned());

        // HMAC-SHA256 signature generation
        let key = self.signing_key();
        let mut mac = HmacSha256::new_from_slice(key.as_bytes()).expect("HMAC key valid");
        mac.update(format!("{}:{}:{}", session_id, action_id, expires_at.to_rfc3339()).as_bytes());

        // Convert to bytes for JSON serialization (serde supports string keys in json! calls)
        let ticket = crate::types::ApprovalTicket {
            session_id: SessionId::from_str(&new_session_id).unwrap(),
            action_id,
            signature: mac.finalize().into_bytes().to_vec(),
            issued_at: now,
            expires_at,
            redeemed: false,
        };

        // Store ticket in the shared storage with metadata for audit tracking
        let mut meta = HashMap::new();
        if action_id.is_empty() {
            meta.insert("action_id".to_string(), serde_json::json!());
        } else {
            meta.insert("session_id".to_string(), serde_json::json!(SessionId::from_str(&new_session_id).unwrap()));
            meta.insert("ticket_id".to_string(), serde_json::json!(ticket_id));
        }

        self.audit_chain.append(
            session_id.to_string(),
            "approval.ticket_issued".to_string(),
            crate::types::ApprovalTicket::type_name(&session_id).unwrap_or_else(|| format!("unknown_action:{action_id}")),
            "pending",
            meta,
        )?;

        Ok(ticket)
    }

    pub fn redeem_ticket(
        &self,
        session_id: &SessionId,
        action_id: &str,
        signature: &[u8],
    ) -> Result<crate::types::ApprovalTicket> {
        let mut tickets = self.tickets.write();

        // Find matching ticket by
