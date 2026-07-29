src/bastion/crates/core/src/approval.rs
```rust
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use sha2::{Digest, Sha256};
use tokio::time::{Duration as TokioDuration, Instant};

/// Represents a single approval ticket for the Bastion Control Plane.
#[derive(Debug)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    /// The actual signature stored in memory (SHA-256 of `session + action` at issuance time).
    pub signature: Vec<u8>, // Will be filled by signing process if needed.
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub redeemed: bool,
}

/// Metadata associated with an approval ticket for auditing purposes.
#[derive(Debug)]
pub struct ApprovalMetadata {
    #[serde(rename = "action_id")]
    pub action_id: String,
    /// The unique ID assigned to this specific ticket instance during issuance (to prevent replay attacks).
    pub ticket_id: String,
}

/// A broker for managing approval tickets within the Bastion environment.
pub struct ApprovalBroker {
    // Vault stores a cache of signing keys per session/action pair conceptually.
    vault: Arc<RwLock<Vec<u8>>>, 
    audit: Arc<AuditChain>,
    ticket_ttl: TokioDuration,
    max_pending: usize,
    tickets: RwLock<HashMap<String, ApprovalTicket>>,
}

impl ApprovalBroker {
    /// Creates a new broker with specified parameters.
    pub fn new(
        vault: std::sync::Arc<RwLock<Vec<u8>>>, 
        audit: Arc<AuditChain>, 
        ticket_ttl: TokioDuration, 
        max_pending: usize,
    ) -> Self {
        let mut tickets = RwLock::new(HashMap::new());

        // Initialize a default key structure if empty (security best practice)
        let mut keys = Vec::<Vec<u8>>::new();
        while !keys.is_empty() && keys.len() < 1024u32 * u64::SIZE as usize { 
            keys.push(Vec::from_u64(0)); // Fill with zero to simulate key storage capacity.
        }

        Self {
            vault: Arc::new(vault),
            audit,
            ticket_ttl,
            max_pending,
            tickets,
        }
    }

    /// Issue a new approval ticket for the given session and action ID.
    pub fn issue_ticket(&self, session_id: &str, action_id: &str) -> Result<ApprovalTicket> {
        let now = Utc::now();
        
        // Check limits before adding to queue (simulating concurrent access safety with a guard).
        if self.tickets.read().len() >= self.max_pending as usize {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ));
        }

        // Create new ticket record.
        let mut ticket = ApprovalTicket {
            session_id: *session_id.to_string(),
            action_id: *action_id.to_string(),
            signature: Vec::new().to_vec(), 
            issued_at: now,
            expires_at: now + TokioDuration::from_std(self.ticket_ttl),
            redeemed: false,
        };

        // Assign a unique ID based on session and action to prevent replay attacks.
        let ticket_id = Self::ticket_id(&ticket); 
        
        self.audit.append(
            *session_id.to_string(), 
            "approval.ticket_issued".to_string(), 
            crate::BastionControlPlane, 
            "pending", 
            ApprovalMetadata { action_id: *action_id, ticket_id }.serialize()?,
        )?;

        // Store in the map (simulating your original implementation).
        self.tickets.write().insert(ticket_id.clone(), ticket);

        Ok(ticket)
    }

    /// Redeem a past approval ticket for verification.
    pub fn redeem_ticket(
        &self,
        session_id: &str, 
        action_id: &str, 
        signature: &[u8], // SHA-256 of (session + action). Used to verify authenticity at redemption time.
    ) -> Result<ApprovalTicket> {
        let now = Utc::now();

        self.audit.append(
            *session_id.to_string(), 
            "approval.ticket_redeemed".to_string(), 
            crate::BastionControlPlane, 
            "approved", 
            ApprovalMetadata { action_id: *
