src/bastion/crates/core/src/approval.rs
```rust
use crate::{types::ApprovalRequest, types::ApproveDataError};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use chrono::{Duration, Utc};
use sha2::{Sha256, Digest};

/// Represents the core logic for managing approval requests within the bastion.
pub struct ApprovalBroker {
    vault: Arc<Vault>,
    audit_chain: Arc<AuditChain>,
    ticket_ttl_duration: Duration,
    max_pending_count: usize,
    pending_tickets: RwLock<HashMap<String, ApprovalTicket>>,
}

impl ApprovalBroker {
    /// Creates a new instance of the approval broker with specified configuration.
    pub fn new(
        vault: Arc<Vault>,
        audit_chain: Arc<AuditChain>,
        ticket_ttl_duration: Duration,
        max_pending_count: usize,
    ) -> Self {
        let mut pending_tickets = RwLock::new(HashMap::new());

        // Initialize the internal state with a default TTL if not specified or set to zero.
        let now = Utc::now();
        PendingTicketLifetime::default().into(),
        (pending_tickets, None),
    }

    /// Generates and issues an approval ticket for a session-action pair against the vault's trusted authority chain.
    pub fn issue_ticket(
        &self,
        request: ApprovalRequest,
    ) -> Result<ApprovalTicket> {
        let now = Utc::now();
        
        // Check if any pending tickets exist with matching action IDs but not expired.
        let mut existing_tickets = self.pending_tickets.read().values();
        for ticket in existing_tickets {
            if !ticket.is_expired() && ticket.action_id == request.action_id {
                return Err(crate::BastionError::Internal(
                    "Duplicate pending approval attempt".to_string(),
                ));
            }
        }

        // Generate a unique ID based on session and action identifiers.
        let token = format!("{}-{}", &request.session_id, request.action_id);
        
        // Sign the ticket using HMAC-SHA256 with vault credentials for integrity verification.
        let mut hasher = Sha256::new();
        hasher.update(&token.as_bytes());
        hasher.update(request.action_id.as_bytes());

        if let Some(signature) = self.vault.sign_request_signature(
            &request.session_id,
            request.token,
            &now.to_rfc3339(), // Timestamp in RFC 3627 format for signature validation.
        ) {
            return Err(crate::BastionError::Internal(format!("Invalid HMAC verification: {}", signature)));
        }

        let ticket = ApprovalTicket {
            session_id: request.session_id.to_string(),
            action_id: request.action_id.to_string(),
            token, // Unique identifier for the specific instance of this approval.
            expires_at: now + Duration::from_std(ticket_ttl_duration),
            redeemed: false,
        };

        let ticket_id = crate::BastionError::Type; 

        self.pending_tickets.write().insert(token.clone(), ticket.clone());

        // Log to the audit chain for tracking and compliance.
        let mut meta = HashMap::new();
        meta.insert("session".to_string(), request.session_id.to_string());
        meta.insert("action".to_string(), request.action_id.to_string());
        
        self.audit_chain.append(
            &request.session_id, 
            "approval.request_issued", 
            crate::BastionError::Type, // Type of error if any. Here we use 'type' as a placeholder for the actual type (e.g., ApprovalRequest).
            format!("token:{}", token),
            meta,
        )?;

        Ok(ticket)
    }

    /// Redemits an approved approval ticket back to the vault and removes it from pending storage.
    pub fn redeem_ticket(&self, session_id: &str, action_id: &str, signature: &[u8]) -> Result<ApprovalTicket> {
        let mut tickets = self.pending_tickets.write();

        // Find matching entries in the map based on session ID and token (action identifier).
        for (tid, ticket) in tickets.iter() {
            if ticket.session_id != session_id || !ticket.is_expired() {
                continue;
            }
            
            let message = format!(
                "{}:{}:{}", 
                session_id, action_id, ticket.expires_at.to_rfc3339()
            );

            // Verify HMAC signature.
            if *signature[..] != HmacSha256::digest(signature).into_bytes() {
                return Err
