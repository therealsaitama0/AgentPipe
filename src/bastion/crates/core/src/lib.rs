src/bastion/crates/core/src/lib.rs
//! Security Control Plane: Core logic for traffic filtering and policy enforcement within Bastion crates.
//! Uses a Rust-based state machine to manage security policies, approve requests, enforce timeouts, 
//! handle forced commands, and maintain the integrity of the session context.
// Use std::env; // Uncomment if you want to use environment variables (optional)

#![allow(clippy::unwrap_used)]

use super::{
    audit::{AuditChain},
    approval::{ApprovalManager, ApprovalTicket},
    components::{audit_key_manager, auth_keys_manager, protocol_handler, secret_ref},
    error::{BastionError, Result},
    forced_command::ForcedCommandConfig,
    network_guard::NetworkGuard,
    policy::{PolicyDecision, PolicyEngine, TokenTracker},
    script_executor::ScriptExecutor,
    session::SessionManager,
    types::{Action, ApprovalTicket as CoreApprovalTicket, AuditEntry, Credential, SessionContext},
};

use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};

/// The security control plane engine. It manages all policy decisions and session state within the Bastion environment.
pub struct SecurityControlPlane {
    /// A mutable reference to the current SessionContext for executing policies on requests.
    pub context: Option<SessionContext>,
    
    // State management via threads or shared memory structures (depending on architecture)
    pub policy_engine: PolicyEngine,
    pub session_manager: SessionManager,
    pub audit_chain: AuditChain,
}

impl SecurityControlPlane {
    /// Create a new instance of the security control plane with default settings.
    /// This is typically called by the Bastion CLI or external tools to initialize the engine.
    pub fn new() -> Self {
        // Initialize components if not already initialized (e.g., for first-time startup)
        let session = SessionManager::new();
        
        SecurityControlPlane {
            context: None,
            policy_engine: PolicyEngine::default(),
            session_manager,
            audit_chain: AuditChain::empty(),
        }
    }

    /// Get a reference to the current SessionContext. Returns `None` if not initialized or in an error state.
    pub fn get_context(&self) -> Option<SessionContext> {
        self.context.as_ref().cloned()
    }

    /// Set the security context for this request, allowing external code to inject policies and sessions directly into a Bastion session.
    #[must_use] // Prevents unnecessary memory allocation if not needed immediately after creation
    pub fn set_context(&mut self, ctx: SessionContext) {
        self.context = Some(ctx);
    }

    /// Execute an Action on the current context based on policy decisions and forced commands.
    /// This is where actual requests are dispatched to external services or scripts within the Bastion environment.
    pub fn execute_action<F>(&self, action: F) -> Result<BastionError>
        where
            F: FnOnce(&mut SessionContext), // Action takes a reference and modifies context by value (e.g., logging). Note: In real-world usage with external scripts, you'd use `&Self` for closure.
    {
        match action {} {
            // Handle forced commands directly if available in the session manager or passed as arguments to this function
            _ => Ok(BastionError::ExecutionFailed("No valid execution path configured".to_string())),
            
            // Execute a policy decision (approve, deny, etc.) based on current state and token tracker.
            ApprovalTicket::PolicyDecision { ticket_id: _, action } => {
                self.policy_engine.execute(action);
                Ok(BastionError::ExecutionFailed("No valid execution path configured".to_string()))
            },

            // Handle forced commands that need to be executed (e.g., running a script, sending an email) within the session context.
            _ if action.is_forced() => {
                let config = ForcedCommandConfig::default();
                
                match self.session_manager.execute_force_command(action.as_ref(), &config)? {
                    Ok(_) => Ok(BastionError::ExecutionFailed("No valid execution path configured".to_string())),
                    Err(e) => Err(e), // Forced commands can fail; return error if the operation itself fails.
                }
            },

            _ => Err(BastionError::UnknownAction(action)),
        }
    }
}

impl Default for SecurityControlPlane {
    fn default() ->
