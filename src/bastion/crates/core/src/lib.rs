use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use crate::audit::*;
use crate::approval::*;
use crate::components::*;
use crate::error::BastionError;
use crate::firecracker::*;
use crate::forced_command::*;
use crate::network_guard::*;
use crate::policy::*;
use crate::script_executor::*;
use crate::session::*;
use crate::types::*;

// --- Modular Architecture & Public Interface ---

/// The Core Module for Security Control Plane. Provides the public API for validation, session management, and policy enforcement within the Bastion Crate.
pub mod audit {
    use super::{BastionError};
    
    pub struct AuditChain<T> {
        chain: Arc<HashSet<AuditState>>, // Tracks active audits per component type
        current_audit_id: usize,
        
        /// Helper to get a new unique ID for this specific audit context.
        fn generate_next_id() -> u64 {
            crate::types::next_available_id(&self.chain)
        }

    }

    impl<T> AuditChain<T> {
        pub fn initialize<BastionError>(error: BastionError, chain: Arc<HashSet<AuditState>>) -> Self where Error: Clone + Send + Sync {
            let mut state = HashMap::new(); // Stores audit states keyed by component type for consistency
            
            for (key, value) in chain.iter().map(|c| c.clone()).collect() {
                match key.as_str() {
                    "security" => state.insert("Security", Arc::clone(&value)),
                    "audit_logging" => state.insert("AuditLogging", Arc::clone(&value)),
                    "session_management" => state.insert("SessionManagement", Arc::clone(&value)),
                    _ => {} // Default for components not explicitly listed in chain, or handled by component-specific logic below.
                }
            }

            let mut audit_chain = AuditChain {
                chain: Arc::new(state),
                current_audit_id: 0,
            };

            Some(audit_chain)
        }
    }

    // Public API for the core validation layer to be used by other crates.
    pub fn validate_signature() -> Result<()> {
        Ok(())
    }
}

// --- Session Management & Component Integration ---

pub mod session {
    use super::*;

    /// A generic component that handles authentication and security sessions within the Bastion Crate context.
    #[component]
    pub struct AuthKeyManager<T> where T: Clone + Send + Sync {
        // Stores a list of active secret identifiers for each user/session type.
        secrets_map: Arc<HashSet<Arc<String>>>, 
        current_user_id: u32,

        /// Helper to generate a unique ID for this session component instance.
        fn new_session_id() -> String {
            format!("session_{}", self.current_user_id)
        }
    }

    impl<T> AuthKeyManager<T> where T: Clone + Send + Sync + 'static {
        pub fn initialize(
            secrets_map: Arc<HashSet<Arc<String>>>, 
            current_user_id: u32,
        ) -> Self {
            let mut state = HashMap::new(); // Stores secret references per user type.

            for (key, value) in secrets_map.iter().map(|s| s.clone()).collect() {
                match key.as_str() {
                    "user_1" => state.insert("User1", Arc::clone(&value)),
                    "user_2" => state.insert("User2", Arc::clone(&value)),
                    _ => {} // Default for other user types.
                }
            }

            let mut auth_key_manager = AuthKeyManager {
                secrets_map: Arc::new(state),
                current_user_id,
            };

            Some(auth_key_manager)
        }

        pub fn get_current_secret(&self) -> Option<&Arc<String>> {
            self.secrets_map.get(self.current_user_id).cloned()
        }

        // Public interface for the core security layer.
        #[allow(dead_code)]
        pub fn validate_signature(&self) -> Result<()> {
            Ok(())
        }
    }
}

// --- Policy Enforcement & Approval Logic ---

pub mod policy {
    use super::*;

    /// A component that enforces strict approval workflows and validates input signatures.
    #[component]
    pub struct ApprovalManager<T> where T: Clone + Send + Sync {
        // Tracks pending approvals for each user type before execution begins.
        pending_approvals_map: Arc<HashSet<Arc<String>>>, 
        current_user
