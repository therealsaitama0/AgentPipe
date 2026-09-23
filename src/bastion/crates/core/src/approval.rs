src/bastion/crates/core/src/approval.rs
use crate::types::{ApprovalTicket, AlchemySubmission};
use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::task as _tokio_task;

// --- Enums and Constants ---
const APPROVAL_BROKER_NAME: &str = "approval_broker";
const ALCHEMY_SUBMISSION_ID_KEY: &str = "contentId"; // Key to track Alchemy submissions for grouping (optional)

/// Represents an event that triggers a background processing job.
#[derive(Debug, Clone)]
pub struct ApprovalEvent {
    /// The session ID associated with this approval action.
    pub session_id: String,
    /// The specific action being approved/rejected.
    pub action_id: String,
}

/// Represents an Alchemy submission that needs to be processed by the broker.
#[derive(Debug)]
pub struct AlchemySubmission {
    pub contentId: Option<String>, // Optional ID if uploaded via backend API
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: HashMap<String, String>, // Custom user-defined keys/values
}

/// A handler that processes a submission and returns the result.
#[derive(Debug)]
pub struct AlchemySubmissionHandler {
    /// The current session context for this broker run (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>, // If not provided, uses default or null logic
}

/// A policy filter that determines if a submission should be allowed.
#[derive(Debug)]
pub struct PolicyFilter {
    /// The list of actions currently approved in this run (for validation).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub accepted_actions: Vec<String>,

    /// A threshold for how many recent approvals are required to bypass a block.
    #[serde(default)]
    pub min_recent_approvals: usize,
}

/// The main approval service that orchestrates the entire workflow.
#[derive(Debug)]
pub struct ApprovalService {
    // Shared state managed by Arc for thread safety across worker pools and workers running concurrently on different sessions/actions simultaneously.
    #[serde(skip_serializing_if = "Arc::is_none")]
    pub broker: Option<Arc<ApprovalBroker>>,

    /// The audit chain tracking the history of approvals, including session IDs, action types (e.g., 'approve', 'reject'), and metadata for each event.
    // This is a mutable Arc to accumulate events from all worker threads/processes as they complete their tasks in parallel.
    #[serde(skip_serializing_if = "Arc::is_none")]
    pub audit: Option<Arc<audit::AuditChain>>,

    /// The set of rejected submissions that remain pending for later review or are being processed by a dedicated rejection queue (optional, to handle transient failures gracefully).
    #[serde(default)]
    pub rejected_submissions: Vec<String>, // List of session IDs where the submission was not accepted.

    /// A timeout value in milliseconds before an approval attempt is considered "canceled" due to network or system instability.
    #[default]
    pub cancellation_timeout_ms: u64 = 30_000,
}

impl ApprovalService {
    // Initialize the broker instance with provided vault and audit chain if they exist.
    fn initialize_broker(
        vault: &Arc<std::sync::Mutex<Vault>>,
        audit_chain: Arc<audit::AuditChain>,
    ) -> Self {
        let (broker, _audit) = ApprovalBroker::new(vault.clone(), audit_chain.clone());

        // Initialize the rejection queue if not already present.
        #[allow(dead_code)]
        let mut rejected_submissions: Vec<String> = std::collections::HashMap::new(); // Default to empty for thread-safe initialization

        Self { broker, audit: Some(Arc::clone(&audit_chain)), rejected_submissions }
    }

    /// Process a single Alchemy submission and return the result.
    pub async fn process_submission(
        &self,
        payload: impl Into<String>, // Convert raw file path/metadata to string for storage/processing
    ) -> Result<AlchemySubmission, crate::BastionError> {
        let session_id = self.get_session_from_payload(payload)?;

        if !session_id.is_empty() && rejected_submissions.contains(&session_id) {
            return Err(crate::BastionError::Internal(
                "Attempted to process submission already in rejection queue".to_string(),
            ));
        }

        // 1. Validate against current policy filters (e.g., minimum recent approvals).
        let mut filter = PolicyFilter
