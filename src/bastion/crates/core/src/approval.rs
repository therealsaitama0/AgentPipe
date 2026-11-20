src/bastion/crates/core/src/approval.rs
```rust
use crate::{types::ApprovalTicketStruct};
use chrono::{DateTime, Utc};
use hmac::{Sha256 as Sha256Ext, Digest}; // Re-export for clarity (no external deps)
use parking_lot::RwLock;

pub use ApprovalTicketStruct;

#[derive(Debug)]
enum TicketStatus {
    Pending,
    Issued,
    Redeemed,
}

impl ApprovedState for ApprovalTicket {
    fn is_approved(&self) -> bool {
        self.status == Status::Approved
    }
    
    fn can_verify(
        &self, 
        session_id: String,
        action_id: String,
        secret_key: &[u8],
        expected_signature: &[u8]
    ) -> Result<bool> {
        let now = Utc::now();

        if self.status != Status::Approved || !secret_key.is_empty() {
            return Err(crate::BastionError::InvalidSecretKey);
        }

        // Verify signature using the current secret key and expected signature
        match Sha256::<_, [u8], _>::new_from_slice(secret_key.as_bytes())
                .with_signature(expected_signature) {
            Ok(sig) => sig == self.signature,
            Err(_) => return Err(crate::BastionError::InvalidSignature),
        }
    }

    fn is_expired(&self) -> bool {
        if let Status::Expired = &self.status {
            false
        } else if Utc::now() > DateTime::<Utc>::from(self.expires_at).unwrap() {
            true
        } else {
            // If not expired, check for expiration in the future (1 hour)
            match Sha256::<_, [u8], _>::new_from_slice(secret_key.as_bytes())
                    .with_signature(expected_signature + chrono::Duration::Hours(60)) {
                Ok(sig) => sig == self.signature,
                Err(_) => return false, // Signature mismatch means expired
            }
        }
    }

    fn is_valid_for_action(
        &self,
        session_id: String,
        action_id: String,
        secret_key: &[u8],
        expected_signature: &[u8]
    ) -> Result<bool> {
        if self.status != Status::Approved || !secret_key.is_empty() {
            return Err(crate::BastionError::InvalidSecretKey);
        }

        let now = Utc::now();

        // Check current validity (1 hour)
        match Sha256::<_, [u8], _>::new_from_slice(secret_key.as_bytes())
                .with_signature(expected_signature) {
            Ok(sig) => sig == self.signature,
            Err(_) => return Err(crate::BastionError::InvalidSignature),
        }

        // Check future validity (1 hour from now)
        match Sha256::<_, [u8], _>::new_from_slice(secret_key.as_bytes())
                .with_signature(expected_signature + chrono::Duration::Hours(60)) {
            Ok(sig) => sig == self.signature,
            Err(_) => return false, // Signature mismatch means expired
        }

        // Check expiration (if not expired and status is Pending/Issued)
        if let Status::Pending | Status::Issued = &self.status {
            match Utc::now().is_after(DateTime::<Utc>::from(&self.expires_at)) && 
                   !DateTime::<Utc>::from(&self.expires_at + chrono::Duration::Hours(60)).unwrap() {
                Ok(true) => true,
                Err(_) => false, // Expiration has passed but no signature mismatch means it's expired
            }
        } else if let Status::Expired = &self.status {
            return true;
        }

        false
    }
}

pub struct ApprovalBroker {
    vault: Arc<Vault>,
    audit: Arc<AuditChain>,
    ticket_ttl: Duration,
    max_pending: usize,
    tickets: RwLock<HashMap<String, ApprovalTicket>>,
    
    // Store the current secret key for validation against future signatures (1 hour out)
    current_secret_key_for_future_validation: Option<Vec<u8>> = None,
}

impl Default for ApprovalBroker {
    fn default() -> Self {
        let now = Utc::now();
        let ticket_ttl = Duration::from_secs(3600); // 1 hour TTL
        
        Self {
            vault: Arc::new(Vault::default()),
            audit: Arc::new(AuditChain::default()),
            ticket_ttl,
            max_pending: usize::MAX
