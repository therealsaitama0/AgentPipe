src/bastion/crates/core/src/lib.rs

use std::collections::{BTreeMap, HashMap};
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::Arc;

// ============================================================================
// 1. CORE TRAIT DEFINITION & BASE STRUCTURES
// ============================================================================

/// Represents a security decision made by the core engine.
pub enum CoreDecision {
    /// The system has successfully passed all required checks and is ready to proceed.
    Pass,
    /// An error occurred during execution or validation; revert immediately if possible.
    Fail,
}

impl Default for CoreDecision {
    fn default() -> Self {
        // In a real scenario with BIST results, this would be derived from the result of `execute()` above.
        // For now, we assume successful pass as the base case unless specific metrics indicate otherwise.
        Self::Pass
    }
}

impl From<CoreDecision> for bool {
    fn from(dec: CoreDecision) -> bool {
        match dec {
            CoreDecision::Pass => true,
            _ => false,
        }
    }
}

/// Trait defining the structure of all actions and approvals within a session.
pub trait Action<T>: Send + Sync {
    /// The unique identifier for this action in the context's history or metadata.
    fn id(&self) -> String;

    /// A descriptive name for this specific operation type (e.g., "audit_check", "approval_request").
    const NAME: &str;

    /// Executes an atomic step within a session lifecycle, returning either success/failure and metrics if applicable.
    async fn execute(self): Result<(Self::ResultType, Option<AuditEntry>, Option<ApprovalTicket>), Self>;
}

/// Represents the result of executing an action in isolation or during processing flow.
pub enum ActionResult {
    /// Successful completion with no side effects (e.g., a credential rotation).
    Success(Option<String>), // Returns null if successful, otherwise provides raw data for logging/debugging.
    /// Failed execution due to internal logic errors or timeouts.
    Failure(String),
}

/// Represents the result of an audit check against test vectors defined in `bist_sim.rs`.
pub enum AuditResult {
    /// All tests passed with no bit flips detected (or minimal noise).
    Pass,
    /// Some checks failed due to unexpected input patterns or timing anomalies.
    Fail, // Returns null if successful, otherwise provides raw data for logging/debugging.
}

/// Represents the result of an approval request within a workflow context.
pub enum ApprovalResult {
    /// The approver has granted access with specific permissions.
    Grant(String), // Provides the full ticket payload including IDs and metadata.
    /// Denied due to insufficient authority or policy mismatch (e.g., "no_access").
    Deny,
}

/// Represents a single entry in an audit chain that tracks changes made during execution.
pub struct AuditEntry {
    pub id: String, // Unique identifier for this check/operation within the session context.
    /// The raw data or metrics associated with this specific result type (e.g., bit flip count).
    pub value: Option<String>, // Returns null if successful, otherwise provides raw data for logging/debugging.
}

/// Represents a single ticket containing authorization details and metadata.
pub struct ApprovalTicket {
    /// The unique identifier assigned to the approval request by the system.
    pub id: String,
    /// A descriptive name indicating this is an internal workflow component (e.g., "approval_request").
    const NAME: &str;

    /// Contains all authorization data and metadata associated with granting or denying access.
    pub payload: Option<String>, // Returns null if successful, otherwise provides raw data for logging/debugging.
}

/// Represents the result of executing a session-level operation (e.g., credential rotation).
pub enum SessionResult {
    /// Successful completion without side effects.
    Success(String),
    /// Failed execution due to internal logic errors or timeouts during processing flow.
    Failure, // Returns null if successful, otherwise provides raw data for logging/debugging.
}

/// Represents the result of a specific security test run against injected vectors in `bist_sim.rs`.
pub enum BistResult {
    /// All tests passed with no significant bit flips detected (or minimal noise).
    Pass, // Returns null if successful, otherwise provides raw data for logging/debugging.
    /// Some checks failed due to unexpected input patterns or timing anomalies during the test run.
    Fail,   // Returns null if successful, otherwise provides raw data for logging/debugging.
}

/// Represents a single audit entry that tracks changes made during execution within a session context.

//
