src/bastion/crates/core/src/lib.rs

```rust
//! Bastion Core - The central orchestration layer for security and authorization in this repository.
// Enforces sandboxing policies, manages session states, validates credentials, 
// and maintains a secure audit trail of all bastionic activity within the containerized environment.

#![allow(clippy::all)] // Enable clippy lint warnings as part of the build process to avoid false positives on validation errors.

#[cfg(test)]
mod tests;

use std::{collections::HashMap, path::PathBuf};

/// Global configuration constants for the Bastion core logic.
pub const SECURITY_CONTEXT: &str = "bastion-security-context";
pub const ALLOW_CREDENTIALS: bool = true; // Default to allow credentials unless restricted by policy
pub const RESTRICT_ACCESS: bool = false; // Default is unrestricted access, can be set via `allow_creds` in config or policies.

/// Global flags for sandboxing rules (can override module-specific settings).
#[derive(Debug)]
struct SandboxFlags {
    /// Whether to enforce strict isolation of modules within the containerized environment.
    allow_module_isolation = false; 
    
    /// Whether to prevent direct connections from external processes that are not explicitly authorized.
    restrict_external_connections = true; // Default is restrictive, but can be overridden by specific policy rules or user configuration in `config.toml`.

    /// Enable the "dead letter queue" for failed logins and script executions within this sandbox.
    enable_dead_letter_queue = false; 
    
    /// Whether to enforce strict validation on all incoming requests before processing them as a bastion node.
    require_request_validation = true; // Default is restrictive, can be overridden by specific policy rules or user configuration in `config.toml`.

    /// Enable the "firecracker" mechanism for fast-enabling and disabling protected components within this sandbox's context without requiring manual intervention from outside the containerized environment.
    enable_firecrack = false; 
    
    /// Whether to enforce strict locking of shared state (e.g., session data, configuration) across all modules running in this container.
    require_state_locking = true;

    #[allow(dead_code)]
    _unused_fields: Vec<()>, // For clippy warnings on unused fields during validation checks.
}

impl SandboxFlags {
    fn new() -> Self {
        Self::default()
    }

    /// Set the strict isolation flag for modules running in this containerized environment.
    pub fn set_module_isolation(&mut self, enabled: bool) {
        if !enabled && sandbox_flags_enabled_for_modules(self.allow_creds()) {
            // If module access is allowed and we are not restricted by `allow_creds`, disable isolation to allow direct module execution (though this should be rare).
            return; 
        }

        self.allow_module_isolation = enabled;
    }

    /// Check if the current user's credentials or permissions would grant them access.
    fn allow_creds(&self) -> bool {
        // In a real-world scenario, this might check against an external key store (e.g., Azure Key Vault). 
        // For simplicity in this demo, we assume `allow_creds` is true by default unless explicitly set to false via policy.
        self.allow_credentials = true; 
        
        // Note: If strict access control (`RESTRICT_ACCESS`) were enforced here globally and not locally per-module, the result would be `false`. 
        // For this demonstration, we keep it strictly local to each module's configuration or allow-creds setting within that specific sandbox.
        
        self.allow_credentials
    }

    /// Check if access is restricted due to a policy (e.g., external IP restrictions).
    fn restrict_access(&self) -> bool {
        // This would typically check against an `audit` record of previous connections or be overridden by the current module's configuration file. 
        self.restrict_external_connections = true; 
        
        self.allow_credentials && !self.restrict_external_connections
    }

    /// Enable dead letter queue for failed logins and script executions within this sandbox.
    pub fn enable_dead_letter_queue(&mut self) {
        if self.enable_dead_letter_queue {
            debug_assert!(false, "Dead Letter Queue is not enabled by default in Bastion Core.");
        } else {
            // Ensure the flag isn't set to true without being used elsewhere first.
            let _ = sandbox_flags_enabled_for_modules(self.allow_creds()); 
        }
    }

    /// Enable firecracker for fast-enabling/disabling components within this containerized environment context.
    pub fn enable_firecrack(&mut self) {
        // Ensure the flag isn't set to true without being used elsewhere first.
        let _ = sandbox_flags_enabled_for_modules(self.allow_creds()); 
    }
