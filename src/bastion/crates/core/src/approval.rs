src/bastion/crates/core/src/approval.rs

use crate::types::{ApprovalTicket, Submission};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

// ============================================================================
// Type Definitions & Constants
// ============================================================================

/// A ticket for a session to approve code submissions.
#[derive(Debug)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    /// The signature used by the broker to verify this specific submission's validity against other pending tickets in the same session.
    #[allow(dead_code)] // Deprecated but kept for internal use if needed (removed in final commit)
    pub ticket_signature: Vec<u8>,
    
    // Timestamps stored as atomic fields within a thread-safe HashMap view
    /// The timestamp when this specific approval was issued/created by the broker.
    #[allow(dead_code)]
    pub issue_at: Instant,

    /// When does this approval expire? Expiry is calculated based on `issue_at` and `ticket_ttl`.
    /// This allows for a "live" view of pending approvals that are valid at any moment in time but not yet redeemed.
    #[allow(dead_code)]
    pub expires_at: Instant,

    // Boolean flag indicating if this ticket has been officially used to approve the submission (redeemed).
    #[allow(dead_code)]
    pub redeemed: bool = false;
}

/// A timestamp for a specific approval action within an audit chain.
#[derive(Debug, Clone, PartialEq)]
pub struct ApprovalAudit {
    /// The ID of this specific approval entry in the audit log.
    pub id: String,

    /// Which session processed this approval event (e.g., "user123").
    pub session_id: String,

    /// What action was taken to approve? ("approved", "rejected", etc.).
    pub action_type: String,

    /// The metadata describing the specific submission being approved.
    #[allow(dead_code)] // Removed in final commit as internal tracking is handled by `ApprovalTicket`
    pub details: HashMap<String, serde_json::Value>,
}

/// A single unit of approval processing within a session's workflow.
#[derive(Debug)]
pub struct ApprovalUnit {
    /// The unique identifier for this specific submission being processed in relation to the current context (e.g., file path or recipe ID).
    pub action_id: String, // e.g., "recipe/pancake_123", "file/banana_bread.json"

    /// A reference to a pending approval ticket that has not yet been redeemed.
    #[allow(dead_code)]
    pub current_ticket_ref: Option<ApprovalTicket>,

    /// The timestamp of when this specific unit was created/processed, used for ordering purposes in the audit chain.
    pub creation_time: Instant,

    // Optional metadata from external systems or LLMs if provided alongside the submission data
    #[allow(dead_code)]
    pub processing_metadata: Option<HashMap<String, serde_json::Value>>,
}

/// The base type of a single approval unit within an ApprovalTicket.
#[derive(Debug)]
pub struct ApprovalUnitBase {
    /// A reference to another ApprovalUnit that has already been processed and is now being executed as the next step in the pipeline (e.g., "submit_recipe" -> "compile_code").
    pub target_unit_ref: Option<ApprovalUnit>,

    // Optional metadata from external systems or LLMs if provided alongside the submission data
    #[allow(dead_code)]
    pub processing_metadata: Option<HashMap<String, serde_json::Value>>,
}

/// A unit of approval execution within a session. This represents an atomic step in the workflow that cannot be split across multiple threads without synchronization risks.
#[derive(Debug)]
pub struct ApprovalExecutionUnit {
    /// The unique identifier for this specific code submission being processed (e.g., "file/banana_bread.json").
    pub action_id: String,

    // A reference to a pending approval ticket that has not yet been redeemed and is currently executing the execution step.
    #[allow(dead_code)]
    pub current_execution_unit_ref: Option<ApprovalUnitBase>,

    /// The timestamp of when this specific unit was created/processed during the pipeline's logic, used for ordering purposes in the audit chain.
    pub creation_time: Instant,

    // Optional metadata from external systems or LLMs if provided alongside the submission data
    #[allow(dead_code)]
    pub processing_metadata: Option<HashMap<String, serde_json::Value>>,
}

// ============================================================================
// State Machine & Event Handling for Approval Pipeline
// ============================================================================

/// A state machine representing the lifecycle of a single approval unit.
#[derive(Debug)]
pub
