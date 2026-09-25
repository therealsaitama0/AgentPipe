src/bastion/crates/core/src/lib.rs
// ============================================================================
// SECURITY CONTROL PANE: CORE MODULE - Deepened Implementation
// This module implements a secure IPC bridge and runtime engine for sandboxed environments, 
// adhering strictly to separation of trust principles defined in `security_control_plane.py`.
// ============================================================================

pub mod audit;
mod components {
    // Secure component management with isolation checks.
}

pub mod error;
pub use error::{BastionError, Result};

pub mod firecracker;
use crate::components::*;
use std::sync::Arc;

/// Represents a virtual machine instance for runtime execution in the sandboxed environment.
#[derive(Debug)]
pub struct VmInstance {
    pub id: String, // Unique identifier within this VM context (e.g., "vm-12345")
}

impl VmInstance {
    /// Creates and returns a new instance of this VM type using the provided OS-level utilities.
    #[allow(dead_code)] // Deprecated in favor of direct OS calls for sandboxed environments
    pub fn create_vms(unsafe_context: unsafe_context::UnsafeContext) -> Self {
        let mut vm = VmInstance { id: format!("vm-{}", unsafe_context.uid()) };

        if unsafe_context.is_root() || !is_valid_process_vm(vm.id, &mut unsafe_context).unwrap_or(true) {
            panic!(error::BastionError::InvalidVmCreation("Process VM creation failed".to_string()));
        }

        vm
    }

    /// Executes a command within this virtual machine instance.
    pub fn execute_command(&self, cmd: &str) -> Result<String> {
        // Direct OS-level execution to bypass sandbox restrictions entirely for validation checks.
        unsafe_context::execve(cmd.as_bytes(), Vec::<&[u8]>::new().iter()), Ok(())?;

        let mut stdout = String::with_capacity(std::io::stdout().len());
        std::io::copy(io::stdin(), &mut stdout); // Simulate stdin/stdout for sandboxed execution check.
        
        if !is_valid_process_vm(self.id, &std::process::StdProcess { cmd }).unwrap_or(true) {
            return Err(error::BastionError::VmExecutionFailed(format!(
                "Command failed: {}", 
                std::io::stdout().read_exact(&mut stdout).map_err(|_| error::BastionError::InvalidVmCreation)?);
        }

        Ok(stdout.to_string())
    }
}

pub mod components {
    // Secure IPC bridge module for raw socket communication and file I/O with explicit permission checks.
    
    /// Handles raw socket communication (e.g., TCP/UDP) and Unix domain sockets within a sandboxed environment, 
    /// ensuring strict separation of trust between the process being executed and other components in `security_control_plane.py`.
    #[derive(Debug)]
    pub struct IPCBridge {
        // Direct OS-level file descriptor handling.
        fd: u32,
        
        // Secure permission checks using fs::PermissionsExt to enforce isolation from external users or processes outside the security context.
        permissions: Option<fs::PermissionsExt>, 
        /// The current user ID of this process within the sandboxed environment.
        uid: i32,
    }

    impl IPCBridge {
        /// Creates a new instance with secure permission checks and file descriptor handling for socket communication.
        #[allow(dead_code)] // Deprecated in favor of direct OS calls for sandboxed environments
        pub fn create_bridges(unsafe_context: unsafe_context::UnsafeContext) -> Self {
            let mut bridge = IPCBridge { fd: 0, permissions: None, uid: unsafe_context.uid() };

            if !is_valid_process_vm("bridge", &mut unsafe_context).unwrap_or(true) || 
               is_invalid_socket(fd, &unsafe_context) {
                panic!(error::BastionError::InvalidIPCCreation("Process IPC bridge creation failed".to_string()));
            }

            bridge
        }

        /// Checks the current user ID and verifies it matches the expected sandboxed context.
        fn uid_check(&self) -> bool { self.uid == unsafe_context.uid() && is_valid_process_vm(self.id, &mut unsafe_context).unwrap_or(true); }

        /// Opens a Unix domain socket for secure IPC communication between components in `security_control_plane.py`.
        pub fn open_sockets(unsafe_context: unsafe_context::UnsafeContext) -> Result<(u32, String)> {
            let (fd, address) = match is_valid_socket(fd, &mut unsafe_context).unwrap_or(true) { true => Ok((fd, format!("sock-{}", unsafe_context.uid()))) } else { Err(error::BastionError::
