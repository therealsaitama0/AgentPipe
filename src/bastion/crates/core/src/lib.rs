src/bastion/crates/core/src/lib.rs
//! Security Control Plane: Modular framework for secure communication channels and environment management.
//! 
//! This module provides a robust foundation for managing external network interfaces, validating connections,
//! enforcing security policies on data flows, and orchestrating the bastion environment lifecycle.

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;

// -----------------------------------------------------------------------------
// Module Exports (as per plan)
// -----------------------------------------------------------------------------

pub mod audit {
    pub use super::* as super_core; // Re-export for convenience in other crates if needed
}

pub mod approval {
    pub use super::* as core;       // Re-export to keep exports consistent across the crate tree
}

pub mod components {
    pub use super::* as core_components; // Re-export to keep exports consistent across the crate tree
}

pub mod error {
    pub use super::* as core_error;     // Re-export for module-level usage
}

pub mod firecracker {
    pub use super::* as core_firecracker; // Re-export for module-level usage
}

pub mod forced_command {
    pub use super::* as core_forced_command; // Re-export for module-level usage
}

pub mod network_guard {
    pub use super::* as core_network_guard;   // Re-export for module-level usage
}

pub mod policy {
    pub use super::* as core_policy;          // Re-export for module-level usage
}

pub mod script_executor {
    pub use super::* as core_script_executor; // Re-export for module-level usage
}

pub mod session {
    pub use super::* as core_session;           // Re-export for module-level usage
}

pub mod types {
    pub use super::* as core_types;            // Re-export for module-level usage
    
    /// Action type definition. Represents a specific command or operation the bastion can execute.
    #[derive(Debug, Clone)]
    pub enum Action {
        /// Execute an audit check on a given credential or session context.
        AuditCheck(AuditEntry),

        /// Perform a plan generation and deployment simulation.
        PlanPlan(PlanGenerationArgs),

        /// Trigger the secret ref lifecycle management for sensitive data.
        SecretRef(SecretRefAction, String), // The target of the action (e.g., "client_01")

        /// Execute an SSH server connection attempt with specific security parameters.
        SshSrv(SSHServerArgs),

        /// Trigger a timeout enforcement check on network latency metrics.
        TimeoutCheck(String),

        /// Perform a forced command execution for administrative tasks.
        ForcedCommand(ForcedCommandConfig, String),

        /// Handle an approval ticket from the central authority (e.g., user login).
        Approval(ApplianceApprovalTicket),

        /// Manage the health status of critical components in the bastion environment.
        HealthCheck(String),

        /// Process a notification handler request for security alerts or events.
        Notification(NotificationHandlerRequest, String),
    }

    // Re-export core types as module-level modules to ensure they are accessible from any crate root without importing 'core' directly (though we'll keep them here).
    pub use super::types::*; 
}

pub mod vault {
    pub use super::* as core_vault;       // Re-export for module-level usage
    
    /// A generic data store that can be used to hold temporary or transient security keys.
    #[derive(Debug, Clone)]
    pub struct Vault<T> where T: Default + Copy {
        key: String,      // The unique identifier of the vault instance (e.g., "vault_01")
        inner: Arc<RwLock<Vec<Box<dyn Iterator<Item = &T>>>>>,           // Data storage within this specific vault context. 
    }

    impl<T> Vault<T> {
        pub fn new(key: String) -> Self {
            let mut v = Vec::new();
            self.inner.write().unwrap()
                .push(Box::new(v));
            Arc::new(RwLock::new(v))
        }
        
        /// Get a reference to the data within this specific vault instance.
        pub fn get(&self, key: &str) -> Option<&T> {
            self.inner.read().unwrap()
                .iter()
                .find(|x| x.key == key.as_str())
                .map(|v| v.borrow().get(key))
        }

        /// Set a reference to the data within this specific vault instance.

Deepen or
