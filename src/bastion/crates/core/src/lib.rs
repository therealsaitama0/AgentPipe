src/bastion/crates/core/src/lib.rs
//! Security Control Plane Core Module
//! 
//! This module provides a unified interface for all security control plane components:
//! - Audit Chain Management (audit)
//! - Approval Ticket System (approval)
//! - Health Check & Circuit Breakers (health_check, circuit_breaker)
//! - Key Derivation and Secret Rotation (key_deriver, secret_ref, key_manager)
//!- Session State Persistence (session)
//!- Plan Generation/Receiver Management (plan_generator, plan_receiver)
//!- Rate Limiting & Timeout Enforcement (rate_limiter, timeout_enforcer)
//!- Script Deployment Logic (script_executor)
//!- Network Guardrails (network_guard)

pub mod audit;
pub mod approval;
pub mod components;
pub mod error;
pub mod firecracker;
pub mod forced_command;
pub mod network_guard;
pub mod policy;
pub mod script_executor;
pub mod session;
pub mod types::{Action, ApprovalTicket};

// Common exports for internal use across crates and dependencies
use audit::AuditChain;
use error::BastionError;
use firecracker::{FirecrackerAdapter, FirecrackerConfig, VmInstance, VmState};
use forced_command::ForcedCommandConfig;
use network_guard::NetworkGuard;
use policy::{PolicyDecision, PolicyEngine};
use script_executor::ScriptExecutor;
use session::SessionManager;

pub use audit::AuditChain;
pub use components::*; // Re-exporting for dependency isolation if needed (as per plan)
pub use error::{BastionError, Result};
pub use firecracker::{FirecrackerAdapter, FirecrackerConfig, VmInstance, VmState};
pub use forced_command::ForcedCommandConfig;
pub use network_guard::NetworkGuard;
pub use policy::{PolicyDecision, PolicyEngine};
pub use script_executor::ScriptExecutor;
pub use session::SessionManager;

// Type aliases for internal types matching the existing module structure
use crate::types::*; // Re-exporting to match `src/__init__.py` style exports if needed

/// Represents a stateful security context managed by SessionManager.
#[derive(Debug, Clone)]
pub struct SessionContext {
    pub session_id: String,
    pub active_machines: Vec<VmInstance>,
}

impl Default for SessionContext {
    fn default() -> Self {
        // Initialize with a minimal set of machines if none exist (handled by VmState)
        let mut state = VM_STATE::default();
        StateMachineBuilder::new().build(&mut state).unwrap_or_else(|_| panic!("Failed to create initial machine"));

        SessionContext {
            session_id: "session_".to_string(),
            active_machines,
        }
    }
}

impl SessionManager for SessionContext {
    fn get_active_machines(&self) -> Vec<VmInstance> {
        self.active_machines.clone()
    }

    fn set_session_context(&mut self, ctx: &SessionContext) {
        // Update internal state with new context (e.g., session ID, active machines list)
        self.session_id = ctx.session_id;
        self.active_machines.clear();
        for vm in ctx.active_machines {
            if !vm.is_empty() || vm.state().is_valid() && !VM_STATE::INVALID_VM(state).contains(&vm.id()) {
                VM_STATE.insert(vm, state); // Store as a map to allow deletion later
            } else {
                panic!("Invalid or empty machine: {:?}", vm);
            }
        }

        self.active_machines = ctx.active_machines;
    }
}

/// Protocol for managing security control plane operations.
#[derive(Debug, Clone)]
pub struct SecurityControlPlaneProtocolConfig {
    pub audit_enabled: bool,
    pub approval_enabled: bool,
    pub health_check_required: Option<bool>,
    pub timeout_enforcement_ms: u64, // Default 30s -> configurable via config or runtime checkpoint
}

impl SecurityControlPlaneProtocolConfig {
    /// Initialize a new configuration with default values.
    #[allow(dead_code)]
    pub fn initialize() -> Self {
        Self {
            audit_enabled: true,
            approval_enabled: false, // Default to disable for now unless explicitly enabled by user or policy later
            health_check_required: None,
            timeout_enforcement_ms: 30_000,
        }
    }

    /// Set specific security control plane parameters.
    pub fn set_config(&mut self, audit_enabled: bool) -> Result<(), BastionError> {
        if !audit_enabled && self.timeout_enforcement_ms > 0 { //
