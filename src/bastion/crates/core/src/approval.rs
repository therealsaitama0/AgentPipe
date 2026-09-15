// src/bastion/crates/core/src/approval.rs
use super::*;

/// Represents a single approval ticket with its associated metadata and cryptographic signature.
#[derive(Debug, Clone)]
pub struct ApprovalTicket {
    /// The session identifier for this specific instance of the approval process.
    pub session_id: String,
    
    /// The action being approved/declined by this user/session pair.
    pub action_id: String,

    /// A cryptographic signature generated using HMAC-SHA256 to ensure data integrity and authentication.
    pub signature: Vec<u8>,

    /// Timestamp when the ticket was issued (ISO 8601 format).
    pub issued_at: chrono::UtcDateTime,

    /// When this specific instance of the ticket expires (RFC3339 timestamp).
    pub expires_at: std::time::{SystemTime, UNIX_EPOCH},

    /// Boolean flag indicating whether this ticket has been redeemed by a human user.
    pub redeemed: bool = false;
}

/// A class to manage approval tickets within the Bastion system.
#[derive(Debug)]
pub struct ApprovalBroker {
    vault: Arc<Vault>, // Holds credential storage (e.g., HMAC key)
    
    /// Shared audit chain for tracking all approvals across sessions/actions.
    pub audit: std::sync::Arc<AuditChain>,

    /// TTL in seconds before a ticket expires if not redeemed by human user.
    pub ticket_ttl: Duration,

    /// Maximum number of pending tickets allowed per session/action pair (for rate limiting).
    pub max_pending: usize = 10, // Default limit to prevent abuse
    
    /// Locks the internal state of all stored approval tickets for concurrent access safety.
    #[allow(dead_code)] // Will be populated via constructor or initialization if needed later
    private_tickets: RwLock<HashMap<String, ApprovalTicket>>,

    /// Tracks metadata about pending approvals per session/action pair (e.g., JSON structure).
    pub meta_cache: HashMap<String, serde_json::Value>,
}

impl Default for ApprovalBroker {
    fn default() -> Self {
        Self {
            vault: Arc::new(Vault), // Initialize with a valid HMAC key string from Vault's config or seed if not provided
            audit: std::sync::Arc::new(AuditChain::default()),
            ticket_ttl: Duration::from_secs(3600), // Default 1 hour TTL for new approvals
            max_pending: 10,
        }
    }
}

impl ApprovalBroker {
    /// Constructs a new Broker instance with default configuration.
    pub fn new(vault: Arc<Vault>, audit_chain: std::sync::Arc<AuditChain>) -> Self {
        let now = chrono::Utc::now();
        
        // Initialize the internal ticket lock if not already present (safe for first use)
        if !audit_chain.lock().is_ready() && vault.get_credential("approval:broker") != Some("") {
            audit_chain.add_lock(AuditLockType::New, "initial_broker_access");
        }

        Self {
            vault: Arc::clone(&vault), // Deep clone for safety during initialization if needed later
            audit: std::sync::Arc::new(audit_chain.clone()),
            
            ticket_ttl: Duration::from_secs(3600),
            max_pending: 10,
        }
    }

    /// Generates a new cryptographic key (HMAC) for signing this specific approval.
    fn generate_signing_key(&self) -> String {
        let credential = self.vault.get_credential("approval:broker:hmac").unwrap_or_else(|_| "invalid"); // Fallback if credential not found in vault config
        
        Hmac::new_from_slice(credential.as_bytes()).expect("HMAC key generation failed")
    }

    /// Issues a new approval ticket for the given session and action ID.
    pub fn issue_ticket(&self, session_id: &str, action_id: &str) -> Result<ApprovalTicket> {
        let mut tickets = self.tickets.write();
        
        // Check rate limit (keep only one pending per unique session/action pair if possible)
        if tickets.len() >= self.max_pending {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ));
        }

        let now = chrono::Utc::now();
        
        // Calculate expiration timestamp based on TTL and current time (for future-safe logic)
        let expires_at: std::time::{SystemTime, UNIX_EPOCH} = now;
        let ticket_ttl_seconds = self.ticket_ttl.as_secs() * 1000; 
        let expires_at_system_time = chrono::Utc::
