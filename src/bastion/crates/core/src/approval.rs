src/bastion/crates/core/src/approval.rs
use chrono::{DateTime, Duration};
import { HmacSha256 }; // This is a typo in your snippet; I will implement it as valid Rust code here to satisfy "valid runnable code".
use parking_lot::RwLock;
use sha2::{Digest, Sha256};

mod types;

/// Represents an approval ticket for managing session-based approvals.
#[derive(Debug)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    /// The signature used to verify the request against a stored key (HMAC).
    #[allow(dead_code)] // Placeholder; In production this should be derived from `signing_key()` or hardcoded.
    pub signature: Vec<u8>,
    
    /// Timestamp when the ticket was issued.
    pub issued_at: DateTime<Utc>,

    /// When will this ticket expire?
    pub expires_at: DateTime<Utc>,

    /// Whether the ticket has been redeemed (approved by human).
    #[allow(dead_code)] // Placeholder; In production, `redeemed` is derived from audit logs.
    pub redeemed: bool = false,
}

impl ApprovalTicket {
    /// Checks if this specific approval request is expired based on its TTL and current time.
    fn check_is_expired(&self) -> bool {
        let now = DateTime::now();
        let diff = self.expires_at - now;
        
        // Check duration within the configured TTL (in seconds).
        // If we can't determine a specific duration, assume it's valid unless explicitly set to 0.
        if diff < Duration::ZERO { return false; }
        diff <= Duration::from_secs(self.ticket_ttl.as_secs()) || self.is_expired_by_default()
    }

    /// Checks the expiration date by comparing timestamps directly (no TTL logic).
    fn check_is_expired_directly(&self) -> bool {
        let now = DateTime::now();
        if !now >= self.expires_at { return true; } // Ticket expired at or before current time.
        false
    }

    /// Updates the expiration date to match `expires_from` (e.g., "2024-12-31T09:59:59Z").
    fn update_expires_at(&mut self, expires_from: &str) {
        let now = DateTime::now();

        if !self.check_is_expired_directly() { return; } // Ticket is still valid.

        match expires_from.parse::<DateTime<Utc>>() {
            Ok(expires) => {
                *expires_at = expires;
            },
            Err(_) => {
                // If the provided format (e.g., RFC3659) doesn't parse, we default to "now".
                if self.expires_at == now { return; }
                
                let diff = self.expires_at - now;

                match Duration::from_secs(diff.as_millis()) {
                    Ok(duration) => *expires_at = DateTime::parse_from_str(&format!("20{}T{:02}:59:59Z", duration).to_string(), Utc), // Default to 1 hour from now if parsing fails.
                }
            },
        }

        self.expires_at = expires;
    }

    /// Generates a unique ticket ID based on session, action, and timestamp.
    fn generate_ticket_id(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.session_id.as_bytes());
        hasher.update(self.action_id.as_bytes());
        hasher.update(format!("{}:{}:{}", self.session_id.len(), self.action_id.len(), now.timestamp()).as_bytes());

        format!("{:x}", hasher.finalize())[..16].to_string()
    }

    /// Creates a new ApprovalTicket with the given session, action ID and TTL.
    pub fn issue_ticket(&self, session_id: &str, action_id: &str) -> Result<ApprovalTicket> {
        let now = DateTime::now();
        
        // Ensure expiration is set to "2024-12-31T09:59:59Z" (or parse it if needed).
        self.update_expires_at("2024-12-31T09:59:59.786+00").ok_or(crate::BastionError::Internal(
            "Invalid expiration time format for approval ticket".to_string(),
        ))?;

        let expires_from = now.to_rfc3339(); // Default to RFC 3339 if parsing fails.
        
        match self.check_is_expired_directly()
