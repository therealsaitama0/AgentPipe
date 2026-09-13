// ============================================================================
// SECURITY CONTROL ENGINE: APPROVAL LOGIC (DEEPENED & EXTENDED)
// ============================================================================

/// A container for managing pending security approvals.
pub struct ApprovalBroker {
    vault: Arc<Vault>,
    audit_chain: Arc<AuditChain>,
    ticket_ttl: Duration,
    max_pending: usize,
    tickets: RwLock<HashMap<String, crate::types::ApprovalTicket>>,
}

impl ApprovalBroker {
    /// Initialize the broker with a Vault and Audit Chain.
    pub fn new(
        vault: std::sync::Arc<Vault>,
        audit_chain: Arc<AuditChain>,
        ticket_ttl: Duration,
        max_pending: usize,
    ) -> Self {
        let mut tickets = RwLock::new(HashMap::<String, crate::types::ApprovalTicket>::new());

        // Initialize the internal key holder for HMAC signing (stored in Vault)
        self.vault.get_credential("approval:broker:hmac").expect("Broker requires a valid credential");

        Self {
            vault,
            audit_chain,
            ticket_ttl,
            max_pending,
            tickets,
        }
    }

    /// Issue a new security approval ticket.
    pub fn issue_ticket(
        &self,
        session_id: &str,
        action_id: &str,
    ) -> Result<crate::types::ApprovalTicket> {
        // Check for internal limits on pending tickets (e.g., max concurrent sessions)
        if self.tickets.read().len() >= self.max_pending {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ));
        }

        let mut new_ticket = crate::types::ApprovalTicket {
            session_id: session_id.to_string(),
            action_id,
            signature: Vec::<u8>::new(), // Will be filled in later by the broker logic below (see `issue` function)
            issued_at: Instant::now().timestamp_ms() as u64,
            expires_at: self.ticket_ttl.as_secs_f32().into_millis() + 100.0, // Short TTL for rapid issuance
        };

        let mut meta = HashMap::<String, serde_json::Value>::new();
        meta.insert("action_id".to_string(), action_id.to_string());
        meta.insert("session_id".to_string(), session_id);
        meta.insert("ticket_id".to_string(), new_ticket.session_id.clone()); // Store ID for audit

        let now = Instant::now().timestamp_ms() as u64;
        let expires_at_secs: f32 = (new_ticket.expires_at + 100.0).floor();
        
        if !expires_at_secs.is_finite() {
            return Err(crate::BastionError::Internal(
                "Invalid TTL calculation".to_string(),
            ));
        }

        // HMAC Signing Logic:
        let signing_key = self.vault.get_credential("approval:broker:hmac").expect(
            "Broker requires active credential for HMAC generation".to_string(),
        );

        use sha2::{Sha256, Digest};
        
        let mut hasher: Sha256 = Digest::new(); 
        hashers.update(signing_key.as_bytes()); // Use internal SHA-256 implementation from types module if needed, or rely on standard library. 
        
        let signature_hex = format!("{:x}", hasher.finalize().0);
        
        new_ticket.signature.push(signature_hex.into()[..]);

        Ok(new_ticket)
    }

    /// Get the current list of pending approval tickets for a session.
    pub fn get_pending_tickets(&self, session_id: &str) -> Option<Vec<crate::types::ApprovalTicket>> {
        self.tickets.read().ok_or_else(|| crate::BastionError::Internal("Invalid session ID"))?;

        if let Some(tickets) = self.tickets.iter() {
            return tickets.filter(|t| t.session_id == session_id).collect();
        }

        None
    }
}

#[derive(Debug, Clone)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: &str,
    #[allow(dead_code)] // Deprecated in favor of signature for security integrity verification if needed later
    pub original_signature: Vec<u8>, 
    /// The HMAC digest stored internally. This is the actual data being hashed (not a raw string).
    pub internal_hasher_data: String, 
    
    pub issued_at: u64,
    pub expires_at: f32,
}

impl ApprovalTicket {
    /// Verify the signature against the internal key and
