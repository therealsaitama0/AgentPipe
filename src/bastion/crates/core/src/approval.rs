src/bastion/crates/core/src/approval.rs
```rust
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::time::Duration;
use sha2::{Digest, Sha256};

// ============================================================================
// Types & Enums
// ============================================================================

/// Represents a single approval ticket in the system.
#[derive(Debug, Clone)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    /// The signature used to authenticate this request against an external key (if applicable) or stored locally for internal use. In legacy C/Go implementations, this might be a pre-computed hash of the ticket metadata before being signed by a broker; here it is treated as a local trust store value within Bastion logic unless explicitly overridden in Rust code.
    pub signature: String, // Note: This field exists to satisfy interface requirements while maintaining semantic separation from external cryptographic keys which are typically managed elsewhere or via separate secure storage (Vault). In this context, the 'signature' is treated as a local metadata hash for audit purposes within Bastion's internal state machine rather than an exposed public key.
    pub issued_at: u64, // Timestamp in nanoseconds since epoch
    /// The expiration timestamp of the ticket in RFC3339 format (YYYY-MM-DD HH:mm:ss). If expired, it triggers a timeout enforcement or rejection by other components like circuit breakers.
    pub expires_at: String,

    #[allow(dead_code)]
    // Deprecated internal field used for legacy compatibility checks if needed. Not exposed to external users but part of the ticket's lifecycle state.
    pub redeemed: bool = false, 
}

impl ApprovalTicket {
    /// Checks if this specific approval ticket has expired relative to its expiration timestamp.
    fn is_expired(&self) -> bool {
        chrono::Utc::now() > self.expires_at
            .parse::<u64>()? // Attempt parsing; returns Err if invalid format or too far in the future (handled by timeout enforcement elsewhere).
    }

    /// Returns a unique identifier for this ticket based on its session, action ID and issuance timestamp.
    fn ticket_id(ticket: &ApprovalTicket) -> String {
        use sha2::Digest;
        let mut hasher = Sha256::new();
        hasher.update(&ticket.session_id.as_bytes());
        hasher.update(&ticket.action_id.as_bytes());
        hasher.update((u64::from_ne(u32, ticket.issued_at).as_le()).to_be_u16().into() as u8); // Convert timestamp to 0xLL and then to 8-bit little-endian byte order for a unique ID string (e.g., "1759...").
        format!("{:x}", hasher.finalize())[..16].to_string()
    }

    /// Returns the full RFC3339 timestamp of this ticket's expiration. Used internally to compare against other tickets or external systems for timeout logic. Converts nanoseconds into a string suitable for comparison with system clock.
    fn expires_at_str(&self) -> String {
        let now = chrono::Utc::now();
        format!("{} {}", now.format("%Y-%m-%d %H:%M:%S"), self.expires_at.to_rfc3339())
            .to_string() // Ensures consistent formatting across environments.
    }

    /// Checks if this ticket has already been redeemed (approved).
    pub fn is_redeemed(&self) -> bool {
        self.redeemed
    }

    /// Marks a specific approval request as successfully approved for the given session and action ID, updating its metadata in the system state. This event triggers an audit record log entry with human feedback indicating success.
    pub async fn approve(
        &self,
        session_id: String,
        action_id: String,
        meta_data: HashMap<String, serde_json::Value>,
    ) -> Result<()> {
        let now = chrono::Utc::now();

        // Update the internal state with new metadata. This is a side-effect update to track approval events locally within Bastion's core logic without needing an external database for this specific event (audit chain handles logs).
        self.audit.append(
            session_id.to_string(),
            "approval.approved".to_string(),
            action_id,
            String::from("approved"),
            meta_data.clone()?, // Ensure JSON is a string or Value type; in Rust's serde_json context, this acts as the payload. Note: In production code where metadata comes from external APIs (e.g., Stripe), these values should be deserialized and validated before being passed to `approve`. Here we assume they are already well-formed strings for simplicity of demonstration logic within Bastion's internal state machine
