src/bastion/crates/core/src/lib.rs

//! Security Control Plane core utilities and state management for Bastion components.
//! 
//! This module provides foundational infrastructure including:
//! - **Audit Chain Management**: Enforcing least-severity principle (LSP) across all security operations.
//! - **State Machine Logic**: Orchestrating approval flows, credential rotation, and session lifecycle using a configurable `StateMachine` that enforces LSP by default (`ERROR`) unless overridden via CLI flags.
//! - **CLI Interface**: Manages configuration parameters for thresholds, policies, and state transitions with graceful shutdown handling.

mod audit;
pub mod components; // Re-exported from core crates (e.g., approval_manager) to maintain internal structure consistency if needed later.
pub mod error;      // Public types: BastionError, Result<T>, VmInstance, etc.
pub mod firecracker::{self, FirecrackerAdapter, FirecrackerConfig};    // Adapter for external VM instances or simulated security environments.
pub mod forced_command::forced_commands_config;  // CLI config options for `forced_command`.
pub mod network_guard::network_guards_config;   // CLI config options for `network_guard` and related policies.
pub mod policy::{self, PolicyDecision};             // Decision logic types: ApprovalTicket, AuditEntry, SessionContext.
pub use audit::{AuditChain, AuditResult, EventSource};  // Public API from auth module (audit).
pub use components::ApprovalManager;                     // Re-exported component for internal usage or future reorganization.
pub use components::AuthKeysManager;                      // Re-exported component for internal usage or future reorganization.
pub use components::{CircuitBreaker, DeadLetterQueue};    // Public types from core crate (circuit_breaker).
pub use components::HealthCheck;                           // Public type from core crate (health_check).
pub use components::KeyDeriver;                             // Re-exported component for internal usage or future reorganization.
pub use components::{KillerEmailQueue, NotificationHandler};  // Re-exported components from core crates (circuit_breaker and notification_handler).
pub use components::PlanGenerator;                         // Public type from core crate (plan_generator).
pub use components::{RateLimiter, SecretRef};              // Re-exported component for internal usage or future reorganization.
pub use components::ScriptDeployer;                        // Re-exported component for internal usage or future reorganization.
pub use components::SshServer;                              // Public type from core crate (ssh_server).
pub use components::{TimeoutEnforcer, TimeoutExceededError};  // Re-exported types from core crates (timeout_enforcer and timeout_exceeded_error).
pub use components::UiApprovalPrompt;                      // Public type from core crate (ui_approval_prompt).
pub use components::WorkspaceClient;                        // Public type from core crate (workspace_client).
pub use error::{BastionError, Result};                       // Re-exported types.
pub use firecracker::{FirecrackerAdapter as FirecrackerAdapterImpl, VmInstance};    // Re-exported adapter and VM state for internal usage or future reorganization.
pub use forced_command::ForcedCommandConfig;                // Public CLI config from core crate (forced_commands_config).
pub use network_guard::{NetworkGuardPolicy, NetworkGuardState, PolicyDecision as NetworkGuardPolicyType};   // Re-exported types.
pub use policy::{ApprovalTicket, AuditEntry, SessionContext};         // Re-exported decision types for internal usage or future reorganization.

// ============================================================================
//! PUBLIC API EXPOSING COMMAND-LINE FLAGS FOR CONFIGURATION AND STATE MANAGEMENT.
//! 
//! This is the entry point of the CLI runner (`cargo run`), defining all flags that:
//! - Modify configuration parameters (thresholds, policies, state transitions).
//! - Trigger specific security actions or workflow states.
//! - Gracefully terminate when no further commands are issued.

/// Configuration flag for Security Thresholds and LSP Enforcement Defaults
pub const SECURITY_THRESHOLD_DEFAULT: i32 = 0; // Default to ERROR severity unless overridden via CLI flags (e.g., --threshold=1)
/// Maximum number of warnings before an error is triggered on a critical event.
const MAX_WARNINGS_PER_CRITICAL_EVENT: u64 = 5u64;

/// Configuration flag for LSP Enforcement Defaults and Severity Levels
pub const DEFAULT_LSP_DEFAULTS: &str = "ERROR"; // Default to ERROR severity unless overridden via CLI flags (e.g., --lsp-default=WARNING)
/// Maximum number of warnings before an error is triggered on a critical event.
const MAX_WARNINGS_PER_CRITICAL_EVENT: u64 = 5u64;

/// Configuration flag for LSP Enforcement Defaults and Severity Levels
pub const DEFAULT_LSP_DEFAULTS: &str = "ERROR
