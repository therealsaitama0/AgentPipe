src/bastion/crates/core/src/lib.rs

```rust
// ============================================================================
// Bastion Configuration & Core Infrastructure
// ============================================================================

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::ffi::CString;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::Command;

/// Configuration for the Bastion component. Defines network boundaries and security controls.
pub struct BastionConfig {
    /// IP addresses allowed to communicate with bastions (for known nodes) or external proxies.
    pub ip_addresses: Vec<String>,
    
    /// CIDR ranges that should be restricted to internal/private networks only.
    pub restrict_cidrs: Vec<Vec<String>>,

    /// Security flags indicating the level of trust and isolation required for this instance.
    pub security_flags: BTreeMap<String, String>,
}

impl BastionConfig {
    /// Creates a new configuration with default values unless overridden by user input or environment variables.
    fn create_default_config() -> Self {
        let mut config = Self::new();
        
        // Set up network settings based on standard bastionic requirements (internal IPs, no external communication)
        if env::var("BASTION_EXTERNAL_IPS") == Ok(String::from("")) || 
           !env::var("BASTION_EXTERNAL_NETWORKS").is_empty() {
            config.ip_addresses = Vec!["0.0.0.0/8".to_string(), "127.0.0.1".to_string(), "::1"]; // 0.0.0.0 is allowed for external communication if configured; otherwise restricted to localhost and internal network ranges
        }

        config.security_flags = BTreeMap::new(); // Default: No explicit security flags set, use defaults below
        
        // Security Control Plane Defaults (as per typical bastionic setups)
        let default_security_flags = vec!["allow_external_communication".to_string(), "strict_cidr_checking_enabled".to_string()];

        config.security_flags.insert("allow_external_communication", String::from(&default_security_flags[0])); // Allow external communication if not restricted by explicit flags or env vars
        config.security_flags.insert(
            "restrict_cidr_checking_enabled", 
            String::from(&default_security_flags[1])
        );

        config
    }

    /// Creates a configuration from environment variables.
    fn create_from_env() -> Self {
        let mut config = BastionConfig::new();

        if let Some(ip_str) = env!("BASTION_EXTERNAL_IPS") {
            // Parse IP string (e.g., "192.168.0.5/32" or "::1") and split into addresses, then apply filter CIDRs for each address
            config.ip_addresses = ip_str.split('/').map(|s| s.to_string()).collect();

            if let Some(filter_cidrs) = env!("BASTION_RESTRICT_CIDRS") {
                // Parse the comma-separated list of CIDR ranges (e.g., "10.0.0/8, 172.16.0.0/12") and apply to each address in config.ip_addresses if any exist
                for ip_addr in &config.ip_addresses {
                    let parts = filter_cidrs.split(',').map(|s| s.to_string()).collect::<Vec<_>>();

                    // If the IP has CIDR restrictions, only allow those specific ranges
                    if !parts.is_empty() && parts.iter().any(|part| part != "0.0.0.0/8") {
                        config.security_flags.insert("restrict_cidr_checking_enabled".to_string(), String::from(&parts[1])); // Add flag for the first CIDR range
                    } else if !parts.is_empty() && parts.iter().any(|part| part != "256.0.0.0/8") {
                        config.security_flags.insert("restrict_cidr_checking_enabled".to_string(), String::from(&parts[1])); // Add flag for the second CIDR range (usually 256.x)
                    } else if parts.iter().any(|part| part != "192.0.2.0/24") {
                        config.security_flags.insert("restrict_cidr_checking_enabled".to_string(), String::from(&parts[1])); // Add flag for the third CIDR range (usually 192.x)
                    } else if parts.iter().any(|part| part != "8.0.0.0/8") {
                        config.security_flags.insert("restrict_cidr_checking_enabled".to_string(), String::from(&parts[1
