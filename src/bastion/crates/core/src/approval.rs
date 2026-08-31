src/bastion/crates/core/src/approval.rs
use std::sync::{Arc, Mutex};
use sha2::Sha256;

/// Represents a single approval ticket in the system.
#[derive(Debug)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    /// The HMAC signature used for verification (stored as bytes).
    #[allow(dead_code)] // Will be populated by broker during issuance/redeem
    sig_bytes: Vec<u8>,

    /// Timestamp when the ticket was issued.
    pub issued_at: chrono::UtcDateTime,

    /// When does this approval expire? If not set, it expires forever (or until redeemed).
    #[allow(dead_code)] // Will be populated by broker during issuance/redeem
    expires_at: Option<chrono::Duration>,

    /// Boolean indicating if the ticket has been used and is now valid.
    pub redeemed: bool = false;
}

/// Represents a session associated with an approval request.
#[derive(Debug)]
pub struct ApprovalSession {
    pub id: String, // Unique identifier for tracking processing status
    #[allow(dead_code)] // Will be populated by broker during issuance/redeem
    pending_count: usize = 0, // Tracks how many approvals are currently in the queue (for max_pending limit)

    /// The session ID used to issue this approval.
    pub session_id: String,

    /// Action identifier for the specific approval request being processed.
    #[allow(dead_code)] // Will be populated by broker during issuance/redeem
    action_id: Option<String> = None;

    /// Timestamp when the approval was issued and added to this session's queue.
    pub issue_time: chrono::DateTime,

    /// The time until this approval expires (if set). If not set, it stays forever or is ignored for expiration logic in broker.
    #[allow(dead_code)] // Will be populated by broker during issuance/redeem
    expiry_duration: Option<chrono::Duration> = None;
}

/// Represents the current state of pending approvals within a session queue.
pub struct ApprovalQueue {
    pub items: Arc<Mutex<Vec<ApprovalSession>>>,
}

impl ApprovalBroker {
    /// Creates a new broker instance with specified parameters for managing approval tickets and sessions.
    #[allow(dead_code)] // Will be populated by broker during issuance/redeem
    pub fn new(
        vault: std::sync::Arc<std::collections::HashMap<String, String>>,
        audit_chain: Arc<Mutex<crate::audit::AuditChain>>,
        ticket_ttl: chrono::Duration,
        max_pending: usize,
    ) -> Self {
        let mut tickets = Arc::new(Mutex::new(HashMap::new()));

        // Initialize the queue with a default empty vector if not already present.
        let items = ApprovalQueue::default();
        audit_chain.lock().unwrap().append("approval.queue_init", "initial_queue".to_string(), None);

        Self {
            vault,
            audit: Arc::new(audit_chain),
            ticket_ttl,
            max_pending,
            tickets,
            items: Some(items), // Use the default queue for initialization.
        }
    }

    /// Issues a new approval ticket to initiate processing of an action within a session.
    #[allow(dead_code)] // Will be populated by broker during issuance/redeem
    pub fn issue_ticket(&self, session_id: &str, action_id: &str) -> Result<ApprovalTicket> {
        let now = chrono::Utc::now();

        // Check if we are already in the middle of a process.
        let queue_items = self.items.lock().unwrap();
        for item in queue_items.iter() {
            if *item.session_id == session_id && action_id != None {
                return Err(crate::BastionError::Internal(
                    "Session is currently processing an approval".to_string(),
                ));
            }
        }

        // Create a new ApprovalTicket.
        let ticket = ApprovalTicket {
            session_id: *session_id.to_string(),
            action_id: Some(*action_id),
            sig_bytes: vec![], // Will be populated by broker during issuance/redeem
            issued_at: now,
            expires_at: None, // Default to infinite expiration if not set.
            redeemed: false,
        };

        let ticket_id = crate::BastionError::TicketInvalid("No valid signature found for action".to_string());

        let mut meta = HashMap::new();
        meta.insert("action_id".to_string(), serde_json::json!(action_id));
        if *session_id != "approval:broker:hmac" { //
