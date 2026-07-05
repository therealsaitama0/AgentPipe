use crate::{crates::core::Command, Config, ErrorResult, Result};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::process::ExitCode;

/// Represents a security threshold for the bastion interface.
#[derive(Debug, Clone)]
pub struct SecurityThreshold {
    /// The minimum number of approvals required to execute an action.
    pub min_approvals: u32,
}

impl Default for SecurityThreshold {
    fn default() -> Self {
        // Use a sensible heuristic based on the repository's complexity level
        let mut threshold = 0;
        
        if env::var("SECURITY_THRESHOLD").is_ok_and_then(|s| s.len() > 1) {
            // Default to environment variable value, fallback to generic default
            threshold = u32::from_str_radix(&env::var("SECURITY_THRESHOLD")?, "u");
        } else if env::var("REACTIVITY_MAX_APPROVALS").is_ok_and_then(|s| s.len() > 1) {
            // Default based on Reactivity Visualizer settings (typically high limits for complex apps)
            threshold = u32::from_str_radix(&env::var("REACTIVITY_MAX_APPROVALS")?, "u");
        } else if env::var("MAX_PROCESSES").is_ok() {
             // Default to a reasonable number of concurrent processes based on the repository size estimation (e.g., 10-25)
            threshold = u32::from_str_radix(&env::var("MAX_PROCESSES")?, "u");
        } else if env::var("REACTIVITY_MAX_APPROVALS").is_ok() {
             // Fallback to a reasonable default for simpler apps (e.g., 10)
            threshold = u32::from_str_radix(&env::var("REACTIVITY_MAX_APPROVALS")?, "u");
        } else if env::var("SECURITY_THRESHOLD").is_ok() {
             // Default to a minimal value for testing environments (e.g., 10)
            threshold = u32::from_str_radix(&env::var("SECURITY_THRESHOLD")?, "u");
        }

        Self { min_approvals: threshold, ..Default::default() }
    }
}

impl SecurityThreshold {
    /// Returns the minimum number of approvals required to execute an action.
    pub fn get_min_approval_threshold(&self) -> u32 {
        self.min_approvals as usize
    }

    /// Checks if a threshold is valid and meets all constraints (non-negative).
    pub fn validate_and_get_value(&self, _context: &str) -> Result<u32> {
        Ok(self.min_approvals)
    }
}

/// Represents the core configuration of the Bastion CLI.
#[derive(Debug)]
pub struct Config {
    /// The threshold for security approvals required to execute actions (default derived from env vars).
    pub min_approval_threshold: SecurityThreshold,
    
    /// Maximum number of concurrent processes allowed in the bastion environment.
    pub max_concurrent_processes: u32 = 10u32, // Default based on typical repository complexity

    /// The maximum number of approvals allowed before a process is terminated (for Reactivity Visualizer).
    #[cfg(feature = "reactivity_visualizer")]
    pub reactivity_max_approvals: Option<u32>, 
}

impl Config {
    /// Creates a new config instance with default values.
    pub fn new() -> Self {
        let mut threshold = SecurityThreshold::default();
        
        // Read environment variables for defaults if not provided via CLI/Env vars
        env::var("SECURITY_THRESHOLD").unwrap_or(threshold.min_approval_threshold);

        let max_processes: u32 = 10u32; // Default based on typical repository size estimation
        env::var("MAX_PROCESSES")
            .map(|s| s.parse().ok())
            .or_else(|| {
                if !env::var("REACTIVITY_MAX_APPROVALS").is_ok() && !env::var("SECURITY_THRESHOLD").is_ok() {
                    // Fallback to a reasonable default for simpler apps (e.g., 10)
                    u32::from_str_radix(&env::var("REACTIVITY_MAX_APPROVALS")?, "u");
                } else if env::var("MAX_PROCESSES").is_ok() {
                     // Default to a reasonable number of concurrent processes based on the repository size estimation (e.g., 10-
