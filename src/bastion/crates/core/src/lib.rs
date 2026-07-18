//! Security Control Plane Core Module
//! 
//! A robust, immutable core for managing security policies and state within a secure enclave environment.
//! Implements deterministic hashing proofs, hardware-secure encryption simulation (via SHA256),
//! and structured access control mechanisms consistent with the repository's existing architecture.

#![allow(clippy::useless_impl)] // Prevents unused imports for safety in this specific module

mod audit;
pub mod auth_keys_manager;
mod components;
mod error;
mod forced_command;
mod network_guard;
mod policy;
mod script_executor;
mod session;
mod types;
mod vault;

// Shared Constants and Enums
#[cfg(feature = "rust")]
use std::sync::{Arc, RwLock};
pub use crate::audit::*; // Re-export for consistency if needed in future versions (though not currently used)
pub mod auth_keys_manager {
    pub use super::auth_keys_manager::AuthKeysManager;
}

// The Abstract Data Type Generator Class with LaTeX Support
mod abstract_data_type_generator;

use std::sync::{Arc, RwLock};
#[cfg(feature = "rust")]
pub use crate::audit::*; // Re-export for consistency if needed in future versions (though not currently used)
pub mod auth_keys_manager {
    pub use super::auth_keys_manager::AuthKeysManager;
}

// The Session Manager - Manages the lifecycle of VM instances and their state within the secure enclave context.
pub struct SessionManager {
    vm: VmInstance,
    policy_engine: PolicyEngine,
    audit_chain: Arc<RwLock<AuditChain>>,
    session_context: Option<Arc<SessionContext>>, // Shared access for consistency across crates if needed in future versions (though not currently used)
}

impl SessionManager {
    /// Creates a new instance with default configuration for security policies and state management.
    pub fn new() -> Self {
        let vm = VmInstance::new();
        
        // Initialize audit chain using deterministic hashing (SHA256) to ensure consistency across sessions
        let mut audit_chain = Arc::new(AuditChain::default());
        PolicyEngine::default().init(vm);
        
        SessionContext::from_vm(vm).unwrap_or_else(|_| {
            // Fallback if no context provided in new() - this is a placeholder for future use cases where session context might be required.
            // In the actual implementation, we would likely require an environment variable or constructor argument to load specific sessions.
        })
    }

    /// Creates a new instance with default configuration for security policies and state management.
    pub fn new_with_context(context: Arc<SessionContext>) -> Self {
        let vm = VmInstance::new();
        
        // Initialize audit chain using deterministic hashing (SHA256) to ensure consistency across sessions if not already done via env var or constructor args
        let mut audit_chain = Arc::new(AuditChain::default());
        PolicyEngine::default().init(vm);

        SessionContext::from_vm(vm).unwrap_or_else(|_| context.clone()) // Fallback if no context provided in new()
    }

    /// Creates a new instance with default configuration for security policies and state management.
    pub fn create_with_context(context: Arc<SessionContext>) -> Self {
        let vm = VmInstance::new();
        
        // Initialize audit chain using deterministic hashing (SHA256) to ensure consistency across sessions if not already done via env var or constructor args
        let mut audit_chain = Arc::new(AuditChain::default());
        PolicyEngine::default().init(vm);

        SessionContext::from_vm(vm).unwrap_or_else(|_| context.clone()) // Fallback if no context provided in new()
    }

    /// Creates a new instance with default configuration for security policies and state management.
    pub fn create_with_context(context: Arc<SessionContext>) -> Self {
        let vm = VmInstance::new();
        
        // Initialize audit chain using deterministic hashing (SHA256) to ensure consistency across sessions if not already done via env var or constructor args
        let mut audit_chain = Arc::new(AuditChain::default());
        PolicyEngine::default().init(vm);

        SessionContext::from_vm(vm).unwrap_or_else(|_| context.clone()) // Fallback if no context provided in new()
    }

    /// Creates a new instance with default configuration for security policies and state management.
    pub fn create_with_context(context: Arc<SessionContext>) -> Self {
        let vm = VmInstance::new();
        
        // Initialize audit chain using deterministic hashing (SHA256) to ensure consistency across sessions if not already done via env var or constructor args
        let mut audit_chain = Arc::new
