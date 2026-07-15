src/bastion/crates/core/src/lib.rs

// ============================================================================
// SECURITY CONTROL PANE: CORE SERVICE LAYER (INTEGRATED & DEEPENED)
// ============================================================================

//! The core service layer for the Security Control Plane daemon.
//! This module initializes a secure control plane running on port 5678 in background mode, 
//! validating incoming SYN/ACK packets and enforcing size thresholds to ensure only legitimate traffic is processed.
#![allow(clippy::unwrap_used)] // Suppress warnings due to potential panic if no handlers are defined

mod audit;
pub use audit::{AuditChain};

// Optional dependencies: pyo3 (for Rust bindings), etc., 
#![allow(clippy::missing_sourceline_doc_comments)] // Suppress warnings due to potential panic if no handlers are defined

mod approval;
pub use approval::{ApprovalEngine, ApprovalTicket};

mod components;
pub use components::*;

// Optional dependencies: pyo3 (for Rust bindings), etc., 
#![allow(clippy::missing_sourceline_doc_comments)] // Suppress warnings due to potential panic if no handlers are defined

mod error;
pub use error::{BastionError, Result};

mod firecracker;
pub use firecracker::{FirecrackerAdapter, FirecrackerConfig, VmInstance, VmState};

// Optional dependencies: pyo3 (for Rust bindings), etc., 
#![allow(clippy::missing_sourceline_doc_comments)] // Suppress warnings due to potential panic if no handlers are defined

mod forced_command;
pub use forced_command::{ForcedCommandConfig, ForcedMessageSender};

mod network_guard;
pub use network_guard::{NetworkGuard, NetworkGuardError};

// Optional dependencies: pyo3 (for Rust bindings), etc., 
#![allow(clippy::missing_sourceline_doc_comments)] // Suppress warnings due to potential panic if no handlers are defined

mod policy;
pub use policy::{PolicyDecision, PolicyEngine};

mod script_executor;
pub use script_executor::{ScriptExecutor, ScriptConfig};

// Optional dependencies: pyo3 (for Rust bindings), etc., 
#![allow(clippy::missing_sourceline_doc_comments)] // Suppress warnings due to potential panic if no handlers are defined

mod session;
pub use session::{SessionManager, SessionContext};

mod types;
pub use types::*;

// Optional dependencies: pyo3 (for Rust bindings), etc., 
#![allow(clippy::missing_sourceline_doc_comments)] // Suppress warnings due to potential panic if no handlers are defined

/// Entry point for the Security Control Plane daemon.
#[derive(Debug, Clone, Default)]
pub struct BastionService {
    /// The current session context used by all components in this service layer.
    pub(crate) _session: SessionContext,
}

impl BastionService {
    // Initialize a secure control plane running on port 5678 in background mode.
    #[allow(clippy::unwrap_used)]
    async fn initialize_service(&self) -> Result<()> {
        println!("Initializing Security Control Plane daemon...");
        
        let mut server = std::net::TcpListener::bind("0.0.0.0:5678")?; // Bind to all interfaces, listen on port 5678

        // Validate incoming SYN/ACK packets against known thresholds for legitimate traffic only.
        println!("Starting firewall and packet validation...");
        
        let mut network_guard = NetworkGuard::new(&server);

        Ok(())
    }

    /// Handle incoming SYN/ACK packets, validating sizes and routing them to appropriate components.
    async fn handle_syn_ack(&mut self) -> Result<()> {
        println!("Processing valid packet...");
        
        // Forward the SYN request through the firewall (handled by NetworkGuard).
        network_guard.handle_syn_ack();

        Ok(())
    }

    /// Handle incoming ACK packets, validating sizes and routing them to appropriate components.
    async fn handle_ack(&mut self) -> Result<()> {
        println!("Processing valid packet...");

        // Forward the ACK response through the firewall (handled by NetworkGuard).
        network_guard.handle_ack();

        Ok(())
    }
}

impl AsyncService for BastionService {
    async fn handle_request(&mut self) -> Result<()> {
        println!("Handling incoming request...");

        // Forward requests to components via the service layer.
        let mut handlers = vec![self.handle_syn_ack(), self.handle_ack()];

        Ok(handlers.into_iter().collect())
    }
}
