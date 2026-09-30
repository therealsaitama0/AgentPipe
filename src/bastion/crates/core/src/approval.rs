src/bastion/crates/core/src/approval.rs
use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
use parking_lot::RwLock;

// Constants for approval lifecycle management constants. These are defined to ensure type safety and consistency with the repository's existing patterns.
const APPROVAL_TICKET_EXPIRES_AT: &str = "1730489200"; // Example timestamp (in seconds) representing a future date/time in UTC
const MAX_PENDING_APPROVALS: usize = 5;

// Trait for approval tickets with shared state management and lifecycle. This is defined to support the broader infrastructure of this crate without requiring external dependencies beyond standard libraries like chrono, sha2, parking_lot, or serde_json (which are typically available at compile time in a standalone module context).
trait ApprovalTicket {
    fn session_id(&self) -> &str;
    fn action_id(&self) -> &str;

    // Helper method to check if the ticket is expired based on its stored expiration timestamp.
    fn expires_at(&self, now: DateTime<Utc>) -> bool;

    // Method for generating a cryptographic signature using HMAC-SHA256 and validating it against an expected value provided by the system (e.g., in a webhook or API). This ensures data integrity during transmission without requiring network access.
    fn generate_signature(&self, message: &str) -> Vec<u8>;

    // Method for verifying the generated signature to confirm that ticket authenticity has not been tampered with and is still valid.
    fn verify_signature(&self, expected_signature: &[u8], now: DateTime<Utc>) -> bool;

    // Returns a unique identifier string derived from the session ID, action ID, issuance time (timestamp), and expiration timestamp to uniquely identify this specific approval ticket within the repository's internal state management system.
    fn ticket_id(&self) -> String {
        use sha2::Digest;
        let mut hasher = Sha256::new();
        hasher.update(self.session_id.as_bytes());
        hasher.update(self.action_id.as_bytes());
        // The issuer timestamp is included to allow for audit trails and tracking of when the ticket was created.
        hasher.update(DateTime::<Utc>::now().timestamp_to_le_bytes());
        format!("{:x}", hasher.finalize())[..16].to_string()
    }

    fn redeem(&self, signature: &[u8]) -> bool {
        // This method checks if a specific ticket can be redeemed by verifying the HMAC-SHA256 signature against an expected value. If it matches and is not expired, the ticket becomes valid for redemption or reuse in future sessions.
        self.verify_signature(signature, DateTime::<Utc>::now()) && !self.expires_at(DateTime::<Utc>::now())
    }

    // Method to determine if a specific action has been previously processed by another user within this repository instance and is currently active (not expired). This method filters the list of pending approvals for that session. If no matching ticket exists, it returns an empty slice or None depending on implementation details in future versions.
    fn get_active_pending_for_session(&self, session_id: &str) -> Option<Vec<ApprovalTicket>> {
        let tickets = self.tickets.read();
        // Filter out expired and inactive tickets based on the provided session ID. The `cloned()` ensures that if multiple sessions share a pending list (e.g., for multi-user collaboration), only one copy is returned to prevent memory leaks or duplicate tracking issues in concurrent environments. This approach effectively implements concurrency-safe filtering without requiring external locks beyond what's already available via RwLock, adhering closely to the existing pattern while adding robust cancellation logic if needed in production-grade deployments.
        tickets.values()
            .filter(|t| t.session_id == session_id && !t.expires_at(DateTime::<Utc>::now()))
            .cloned()
            .collect()
    }

    fn is_expired(&self) -> bool {
        self.expires_at(DateTime::<Utc>::now())
    }
}

pub struct ApprovalBroker {
    vault: std::sync::Arc<Vault>,
    audit: std::sync::Arc<AuditChain>,
    ticket_ttl: Duration, // In seconds for the TTL calculation. This is a standard utility constant representing how long an approval remains valid before expiring or being reset to pending status (e.g., 1 hour in this context). It allows flexible configuration of expiration windows without requiring external service integration until proven necessary.
    max_pending: usize,      // Maximum number of concurrent active approvals per session allowed by the broker to maintain system stability and prevent resource exhaustion during high-volume processing scenarios within a single repository instance. This is set at 5 pending approvals for this specific implementation as a baseline threshold that can be
