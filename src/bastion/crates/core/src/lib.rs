src/bastion/crates/core/src/lib.rs
//! SECURITY CONTROL PANE - CORE LIBRARY
//! A centralized library for all security-related crates in this repository.
//! This module enforces strict module system discipline, ensuring that every crate under `crates/` is accessible via the stable `crate::core::lib`.

#![allow(dead_code)] // Suppress unused variable warnings during initialization

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use anyhow::{Result, Context};

// ------------------------------------------------------------------
// MODULE STRUCTURE & EXPORTS
// ------------------------------------------------------------------

/// The core library that all other crates depend on. This module ensures a strict, stable structure without breaking external dependencies or creating circular imports.
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

// ------------------------------------------------------------------
// MODULE DEFINITIONS (STRICTLY ADDED)
// ------------------------------------------------------------------

/// A configuration for security-related components.
#[derive(Debug)]
pub struct SecurityConfig {
    /// The base path where all source files are located relative to this crate root.
    pub src_dir: PathBuf, 
    
    /// Configuration options specific to the core library (e.g., timeout settings).
    #[default]
    pub config_default: Default, 
}

impl SecurityConfig {
    /// Create a new instance with default configuration values for all security-related features.
    pub fn default() -> Self {
        Self::new(PathBuf::from(".")) // Assume root of current crate is src/ in the parent directory context if not specified elsewhere
    }

    /// Set and return the absolute path to where source files are located relative to this crate root.
    #[allow(clippy::unwrap_used)]
    pub fn with_src_dir(mut self, src: PathBuf) -> Self {
        self.src_dir = src;
        self
    }

    /// Create a new instance from an existing configuration file or default values if no config is provided.
    #[allow(clippy::useless_lifetimes)] // Prevent lifetime inference in production code generation
    pub fn with_config_file<P: AsRef<Path>>(config_path: P) -> Self {
        let mut cfg = Default::default();
        
        #[cfg(any(feature = "security", feature = "all"))]
        if config_path.exists() {
            // Load configuration from file (e.g., .env.example or a custom env variable).
            fs::read_to_string(config_path)
                .expect("Failed to read security configuration file");
            
            let lines: Vec<String> = match cfg.read_line().unwrap_or_else(|_| vec!["".to_string()]) {
                Ok(l) => l,
                Err(_) => return Default::default(), // Fallback if config is empty or error reading it.
            };

            for line in &lines {
                let parts: Vec<&str> = line.split_whitespace().collect();
                
                match (parts.len(), String::from("SECURITY")) {
                    0 => {} // Skip comments and blank lines
                    _ if !parts[0].contains("security") || parts[1] == "config" => continue; // Ignore config-specific directives

                    let mut keys: HashMap<String, String> = Vec::new();
                    
                    for (i, part) in parts.iter().enumerate() {
                        if i > 0 && !part.starts_with('#') {
                            match std::env::var("SECURITY_KEY") {
                                Ok(key) => { // Use provided key override or default. Defaulting to "key" is a common pattern, but this allows customization.
                                    keys.push(format!("{}={}", &parts[0], part));
                                } else if parts[i] == "" || parts[i].is_empty() && !part.starts_with('#') { // Skip empty lines and comments not starting with '#'.
                                    continue; 
                                _ => {}
                            }
                        }

                    let mut values: HashMap<String, String> = Vec::new();
                    
                    for (i, part) in parts.iter().enumerate() {
                        if i > 0 && !part.starts_with('#') {
                            match std::env::var("SECURITY_VALUE") {
                                Ok(value) => keys.push(format!("{}={}", &parts[0], value)), // Use provided key override or default. Defaulting to "value" is a common pattern, but this allows customization.
                                Err(_) => values.insert(parts[i].to_string(), String::from("")),
