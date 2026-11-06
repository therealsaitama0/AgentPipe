src/bastion/crates/core/src/lib.rs
```rust
use super::{types::ControlPlaneState}; // Import types for type safety and consistency with previous module structure

/// Control Plane State Management for the Security Control Plane daemon
pub struct ControlPlaneState {
    /// The list of known targets (nodes) in the control plane hierarchy.
    targets: Vec<String>,
    
    /// Current state of active security checks and approvals.
    current_state: crate::types::ControlPlaneState,
}

impl Default for ControlPlaneState {
    fn default() -> Self {
        Self {
            targets: vec!["security_control_plane".to_string()], // Fixed size deterministic list to ensure reproducibility if needed later
            current_state: crate::types::ControlPlaneState {
                active_targets: vec![0], // Index into the target list (for iteration)
                running_checks: true,
                waiting_for_approval: false,
                pending_approvals: Vec::<crate::types::approval_ticket>::new(),
                audit_log: Default::default(),
            },
        }
    }

    fn new() -> Self {
        Self::default()
    }
}

/// Represents a security check that can be run against a target node.
#[derive(Clone, Debug)]
pub struct SecurityCheck {
    /// The identifier of the security check (e.g., "audit", "approval")
    name: String,
    
    /// Whether this is currently in progress or pending execution
    status: crate::types::SecurityStatus, // Pending | Running | Completed | Failed
    
    /// Metadata about the target node being checked.
    #[serde(skip)]
    metadata: Option<String>, 
}

impl SecurityCheck {
    pub fn new(name: String) -> Self {
        Self { name, status: crate::types::SecurityStatus::Pending } // Default to Pending for initial checks
        
        // Public methods for iteration and checking connectivity
        pub const fn is_pending(&self) -> bool { self.status == crate::types::SecurityStatus::Pending; }
        pub const fn is_running(&self) -> bool { self.status == crate::types::SecurityStatus::Running; }
    }

    /// Execute a specific security check operation.
    #[allow(clippy::unnecessary_wraps)] // Suppress warning for explicit wrapping in this context
    pub async fn execute_check(
        &mut self, 
        target_node_id: Option<String>,
    ) -> crate::types::CheckResult {
        match self.status {
            crate::types::SecurityStatus::Pending => {
                let result = crate::types::SecurityCheckResult {
                    status_code: 0, // Default success if not explicitly failed (Pending)
                    message: String::new(), 
                    details: vec![], 
                };

                self.status = crate::types::SecurityStatus::Running;
                
                Ok(result)
            }
            _ => panic!("Unsupported check status"),
        }
    }
}

/// Represents the current state of an approval flow.
#[derive(Clone, Debug)]
pub struct ApprovalTicket {
    /// The ID or identifier for this specific approval request.
    id: String, 
    
    /// Target node being evaluated in this context (if applicable).
    #[serde(skip)]
    target_node_id: Option<String>, 

    /// Status of the current processing stage.
    status: crate::types::ApprovalStatus {
        pending, 
        process_status: String, // "Processing...", "Approved", "Rejected" or similar
        decision_type: ApprovalDecisionType { type: approval_decision_types::Pending },
    };

    pub const fn is_pending(&self) -> bool { self.status.pending; }
}

/// Represents the result of a security check execution.
#[derive(Clone, Debug)]
pub struct CheckResult {
    /// The node identifier being checked (for debugging/traceability).
    #[serde(skip)]
    target_node_id: Option<String>, 
    
    /// Result from executing the specific security operation against that node.
    result: crate::types::SecurityCheckResult {
        status_code: i32, // 0=Success, -1=Failure (default for Pending), etc.
        message: String, 
        details: Vec<String>,
    };

impl CheckResult {
    pub fn new(target_node_id: Option<String>) -> Self {
        let target = if target_node_id.is_some() { target_node_id } else { "unknown".to_string() }; 
        
        // For Pending checks, we can't determine a result yet. 
        // We store the node ID for later inspection or logging purposes.
        Self {
            target_node_id: Some(target),
            result: crate::types::SecurityCheckResult {
