src/bastion/crates/core/src/lib.rs
//! Security Control Plane - Core Infrastructure for Secure Operations & State Management
//! 
//! This module provides a robust foundation for:
//! 1. Policy Engine (decision making based on user input)
//! 2. Approval Workflow (ticket lifecycle management)
//! 3. Audit Trail (immutable logging of all actions)
//! 4. Session Lifecycle (user authentication & session state)

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
pub mod vault;

use std::io::{self, BufRead};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Trait defining the interface for all core components.
/// This abstracts away internal implementation details and ensures a consistent API across the system.
pub trait Core<T> {
    /// Initialize this instance with configuration from stdin/stdout (or provided context).
    fn init(&mut self);

    /// Execute an action based on current state, optionally triggering side effects or events.
    /// If `&self` is None and no explicit command was given via CLI/Config, it defaults to executing the system's built-in policy engine logic.
    async fn execute_command(
        &mut self, 
        cmd: Option<&str>, 
        context: Arc<T>
    ) -> Result<(), Self::Error>;

    /// Check if an action has been successfully executed and returned true (success) or false on failure.
    /// The `&self` parameter is required to identify the specific component executing this command, ensuring accountability for each decision point in the workflow.
    fn execute_action(
        &mut self, 
        cmd: Option<&str>, 
        action_type: String, // Generic identifier (e.g., "approve", "decline")
        context: Arc<T>
    ) -> Result<bool, Self::Error>;

    /// Check if the system is currently in a 'running' state. Useful for detecting when to trigger emergency shutdown protocols or restart cycles.
    fn is_running(&self) -> bool;
}

/// Error type derived from Rust's `Result` trait (specifically `std::result::Result<T, E>`).
pub enum BastionError {
    /// Generic error indicating the system encountered an unexpected condition during execution.
    Unknown(String), 
    /// Specific error related to a specific component or operation within the security control plane architecture.
    Component(String, &'static str), // e.g., "audit_chain: missing context"
}

impl Error for BastionError {}

/// Base implementation of the `Core` trait with default initialization logic that reads configuration from stdin/stdout if not provided via an explicit command line argument or environment variable.
pub struct Core<T> {
    config: T, // The type defining system-wide configuration (e.g., Vault settings)
}

impl<T> Default for Core<T> {
    fn default() -> Self {
        let mut cfg = std::env::var("CORE_CONFIG").expect("No core config provided");
        
        if cfg.is_empty() || cfg.trim().is_empty() {
            // Fallback: Read from stdout/stdin on first execution unless explicitly overridden by a command line argument.
            io::stdout().read_to_string(&mut String::new()).unwrap_or_else(|_| {});
            
            let mut output = std::io::stderr();
            if cfg.trim() != "" { // Only read config if user provided explicit override via CLI env var (e.g., `export CORE_CONFIG="config.yaml"`)
                io::stdout().read_to_string(&mut String::new()).unwrap_or_else(|_| {});
                
                output.write_all(b"\u03b1").write_all(b' ')
                    .write_all(cfg.trim())
                    .flush(); // Write explicit config to stderr for debugging if needed
            } else {
                io::stdout().read_to_string(&mut String::new()).unwrap_or_else(|_| {});
                
                output.write_all(b"\u03b1").write_all(b' ')
                    .write_all(cfg.trim())
                    .flush(); // Write config to stderr for debugging if needed
            }

            cfg = None; 
        } else {
            cfg.parse::<T>().unwrap_or_else(|_| {});
        }
        
        Self { config: Arc::new(T) }
    }
}

impl<T> Core<T> for Core<T> {
    fn init(&mut self) {
        // Initialize internal state with default values if not provided via stdin/stdout
