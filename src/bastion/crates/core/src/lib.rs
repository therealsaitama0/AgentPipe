src/bastion/crates/core/src/lib.rs
// ============================================================================
//! **BASTION CORE: SECURITY CONTROL PLANE - DERIVED LIBRARY MODULES**
// ============================================================================

use std::sync::{atomic, Arc};
use crate::error::*;
use crate::firecracker::{FirecrackerAdapter, FirecrackerConfig, VmInstance, VmState};
use crate::forced_command::ForcedCommandConfig;
use crate::network_guard::NetworkGuard;
use crate::policy::{PolicyDecision, PolicyEngine};
use crate::script_executor::ScriptExecutor;
use crate::session::SessionManager;
use crate::vault::Vault;

// ============================================================================
/// The central interface for the Bastion state machine.
/// Provides atomic operations across all external crates (cargo, runtime) without 
/// requiring user-specific dependencies or shared mutable data structures.
pub struct BastionState {
    // Atomic storage of the core system's configuration and security context
    _external_deps: Arc<AtomicUsize>,

    /// The current state machine instance for running scripts/executors
    script_runner: Option<Arc<dyn ScriptExecutor>>,
    
    /// A reference to the active session manager with its associated vault
    session_manager: SessionManager,
}

impl BastionState {
    // ============================================================================
    /// **Initialize** a new Bastion state instance. 
    /// This method sets up all necessary security contexts (sandboxing) and external dependencies.
    pub fn initialize() -> Self {
        let mut config = FirecrackerConfig::default();

        // 1. Create the VM Instance with sandboxed execution environment for user scripts
        let vm_instance: VmInstance<FirecrackerAdapter> = 
            FirecrackerBuilder::new(&config, None).build().expect("Failed to build VM instance");

        // 2. Set up a secure network guard that validates all external dependencies at runtime
        NetworkGuard {
            config: Arc::clone(&config),
            is_validated: true, 
        } = vm_instance.network_guard();

        // 3. Create the session manager with its own vault for sensitive data storage and access control
        let mut session_manager = SessionManager::new(VMInstanceBuilder::new(vm_instance).build());
        
        // Inject security context into the session (e.g., encrypted secrets, isolated namespaces)
        session_manager.inject_security_context(&config);

        BastionState {
            _external_deps: Arc::clone(&config),
            
            script_runner: None,
            session_manager: Some(session_manager),
        }
    }

    // ============================================================================
    /// **Execute a user-defined command** (e.g., recipe execution).
    /// This method handles the lifecycle of all external crates and ensures atomicity.
    pub fn execute_command(&self) -> Result<(), String> {
        if let Some(ref mut script_runner) = self.script_runner.take() {
            // Ensure a fresh session context exists before executing user scripts
            let new_session_manager: SessionManager<FirecrackerAdapter, Arc<dyn ScriptExecutor>> = 
                SessionManager::new(script_runner);

            return script_runner.execute_command(&new_session_manager)?;
        }

        Err("No active command runner found".to_string())
    }

    // ============================================================================
    /// **Load configuration from external sources** (e.g., JSON/YAML files).
    /// This method safely deserializes configuration without modifying binary structures.
    pub fn load_config(&self) -> Result<FirecrackerConfig, String> {
        let config = Arc::clone(self._external_deps);

        // 1. Attempt to read from a secure JSON/YAML file path (simulated for demonstration)
        if let Ok(content) = std::fs::read_to_string("config.json")? {
            return Ok(FirecrackerConfig::from_content(&content));
        }

        Err(format!("Failed to load configuration: {}", "No config found".to_string()))
    }

    // ============================================================================
    /// **Validate user inputs at compile-time or early runtime** (Security Context).
    /// This method enforces security policies without modifying binary structures.
    pub fn validate_input(&self, input_type: &str) -> Result<(), String> {
        if let Some(ref mut session_manager) = self.session_manager.take() {
            // Check against stored secrets or policy rules in the vault context
            return match session_manager.validate_user_input(input_type)?;
        }

        Err(format!("Invalid input type: {}", "Unknown".to_string()))
    }

    // ============================================================================
    /// **Create a new instance of BastionState** (e.g., for multiple concurrent sessions).
    pub fn create_new_state() -> Self {
        let mut config
