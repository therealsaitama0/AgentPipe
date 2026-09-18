src/bastion/crates/core/src/lib.rs
//! Security Control Plane Core Module
/// A secure replication mechanism for immutable user data across bastion nodes.
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
pub mod types;

// Use all public exports from the module above.
use audit::AuditChain;
use error::{BastionError, Result};
use firecracker::{FirecrackerAdapter, FirecrackerConfig, VmInstance, VmState};
use forced_command::ForcedCommandConfig;
use network_guard::NetworkGuard;
use policy::{PolicyDecision, PolicyEngine};
use script_executor::ScriptExecutor;
use session::SessionManager;

/// The core state machine for the Security Control Plane.
#[derive(Debug)]
pub struct Core {
    // Mutable buffer of user data that is replicated across nodes but remains immutable at the OS level.
    pub(crate) data: Vec<u8>,
    
    /// Protocol layer configuration and handlers. Handles signature verification, policy enforcement, etc.
    protocol_layer: Protocol,
}

impl Default for Core {
    fn default() -> Self {
        let config = Config::default();
        let mut core = Core { data: Vec::new(), protocol_layer: Protocol::default() };
        
        // Initialize the immutable buffer with a placeholder seed to ensure non-zero size.
        core.data.push(0xdeadbeef); 
        
        Some(core)
    }

    fn new(data: &[u8]) -> Self {
        let mut core = Core { data, protocol_layer: Protocol::default() };
        
        // Initialize the immutable buffer with a placeholder seed to ensure non-zero size.
        core.data.push(0xdeadbeef); 
        
        Some(core)
    }

    fn reset(&mut self) -> Result<()> {
        if !self.protocol_layer.is_empty().contains(&Self::SEED_KEY) {
            return Err(BastionError::InvalidConfiguration("Cannot clear the seed key. Ensure it is set before resetting."));
        }
        
        // Reset mutable data to a clean slate for safe replication scenarios.
        self.data.clear(); 
        Ok(())
    }

    /// Creates and returns an immutable Core instance with provided user data.
    pub fn new_user_data(data: &[u8]) -> Self {
        let core = Core::new(data);
        
        // Ensure the protocol layer is initialized before using this core for secure operations.
        if !core.protocol_layer.is_empty().contains(&Self::SEED_KEY) {
            return Err(BastionError::InvalidConfiguration(
                "Protocol must be configured with a seed key."
            ));
        }

        Some(core)
    }
}

impl Core {
    /// Retrieves the immutable user data. This is read-only and represents secure, isolated storage for all users.
    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    /// Sets the mutable buffer of user data to a new state. 
    /// Note: Direct modification of this vector can cause issues during replication if not carefully managed by the protocol layer.
    pub fn set_user_data(mut self, data: Vec<u8>) -> Result<()> {
        // Ensure we have at least one byte for safe operations.
        if !data.is_empty() && !self.data.iter().any(|&x| x == 0) {
            return Err(BastionError::InvalidConfiguration("User data cannot be empty or contain only zeros."));
        }

        self.protocol_layer.set_data(data); 
        Ok(())
    }

    /// Resets the mutable buffer of user data to a clean state. This is typically called during backup operations or node restarts.
    pub fn reset_user_data(&mut self) -> Result<()> {
        // Ensure we have at least one byte for safe operations before clearing.
        if !self.data.iter().any(|&x| x == 0) && self.protocol_layer.is_empty() {
            return Err(BastionError::InvalidConfiguration("User data must be non-empty and protocol not configured."));
        }

        // Clear the mutable buffer of user data to a clean slate.
        self.data.clear(); 
        
        Ok(())
    }
}

impl Protocol for Core {
    /// Generates signature hashes using SHA256 or ESR (Extended Secure Randomness) algorithms as defined in the protocol specification.
    fn generate_signature(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Ensure we have
