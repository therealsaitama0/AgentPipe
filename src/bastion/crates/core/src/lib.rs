src/bastion/crates/core/src/lib.rs

```rust
//! Bastion Core: The central orchestration layer for security control planes and automated workflows.
//! This module provides the foundational infrastructure, configuration management, and core components 
//! required to execute complex automation tasks within a secure environment.

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

use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Command, ExitCode, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};

// ==========================================
// Configuration Management & Constants
// ==========================================

/// Global configuration for the Bastion environment.
#[derive(Clone)]
pub struct BastionConfig {
    /// Security policy: "run as root", "log all", or a custom mix of permissions and logging levels.
    pub security_policy: Option<String>,
    
    /// Access rights to manage components (e.g., read/write/execute on specific files).
    pub access_rights: HashMap<String, String>,

    /// Metadata storage for versioning and tracking state changes.
    pub metadata_storage_path: PathBuf,
}

impl Default for BastionConfig {
    fn default() -> Self {
        // Initialize with a standard security posture that balances automation needs 
        // with system stability (run as root but limit permissions).
        let mut config = BastionConfig {
            security_policy: None,
            access_rights: HashMap::new(),
            metadata_storage_path: PathBuf::from("/var/lib/bastion/config.json"),
        };

        if env::var("SECURITY_POLICY").is_ok() {
            // Parse custom policy string (e.g., "run as root log all")
            let parsed = match env::var("SECURITY_POLICY").unwrap_or_default().as_str() {
                "root" => config.security_policy,
                _ if !config.access_rights.contains_key(&format!("{}_log", String::from_utf8_lossy(config.security_policy).to_string())) 
                    || config.security_policy == Some("") => None, // Allow empty string for default logging
                _ => return Default::default(), // Fallback to default or error
            };

            if let Ok(policy) = parsed {
                config.security_policy = policy;
            } else {
                panic!("Invalid security policy: {}", env::var("SECURITY_POLICY").unwrap_or_default());
            }
        }

        // Default access rights for common components (admin, scripts, processes).
        let mut default_rights = HashMap::new();
        if config.security_policy.is_none() && !config.metadata_storage_path.exists() {
            default_rights.insert("ADMIN".to_string(), "full"; std::process::Stdio::inherit());
            default_rights.insert("SCRIPTS".to_string(), "read;write;execute (for scripts)"; std::process::Stdio::inherit());
        } else if config.security_policy.is_none() {
            // Default to read-only for everything except metadata storage.
            let mut final = HashMap::new();
            
            if !config.metadata_storage_path.exists() {
                final.insert("ADMIN".to_string(), "read"; std::process::Stdio::inherit());
                final.insert("SCRIPTS".to_string(), "execute (for scripts)"; std::process::Stdio::inherit());
                
                // Metadata is only writable by admin.
            } else {
                final.insert("ADMIN".to_string(), "read;write;execute (full for metadata storage)"; std::process::Stdio::inotify_wait_for_file(0, 1));
                final.insert("SCRIPTS".to_string(), "read;write;execute (for scripts)"; std::process::Stdio::inherit());
            }

            config.access_rights = final;
        } else {
            // Explicit override overrides defaults.
            let mut explicit: HashMap<String, String> = env::var("SECURITY_ACCESS_RIGHTS").unwrap_or_default().split(',').map(|s| s.trim()).collect();
            
            for (key, value) in explicit.iter() {
                if key.startsWith("ADMIN.") && !value.contains("full") { // Allow 'admin' without full access
                    config.access_rights.insert(key.to_string(), format!("{}_log;execute", String::from_utf8_lossy(value)));
                } else if (key == "SCRIPTS
