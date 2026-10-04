src/bastion/crates/core/src/lib.rs
// ============================================
// SECURITY CONTROL PLANE - CORE MODULE (DEEPENED)
// ============================================

use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use alloy_primitives::{Address, Bytes32, Hash};
use anyhow::Result;
use base64::{engine::general_purpose as BGUP, Engine};

/// Represents a secure connection interface for the control plane.
#[derive(Debug)]
pub struct BastionInterface {
    // Host/bridge configuration management (independent of container)
    host: Option<Host>,
    
    // TLS Configuration Management (secrets handled externally or in config file format if needed)
    tls_config: Config,
}

/// Represents the core network state for secure communication.
#[derive(Debug)]
pub struct BastionState {
    pub active_connections: Arc<Mutex<BTreeSet<Connection>>>, // Tracks connections to managed hosts/bridges
    pub disconnected_hosts: BTreeSet<Address>,              // Known addresses that are not currently connected or blocked
    
    // Atomic guards for critical operations (locks on state)
    pub connection_locks: Arc<Mutex<(BTreeMap<Hash, Connection)>>>, 
    pub host_locks: Arc<Mutex<BTreeSet<Address>>>,           // Lock to prevent concurrent access to hosts list
}

/// Configuration structure for the BastionInterface.
#[derive(Debug)]
pub struct Config {
    /// Host configuration (e.g., IP address)
    host_ip: Option<Address>,
    
    /// TLS settings including cipher suites and key material
    tls_config: CipherSuiteConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host_ip: None,
            tls_config: CipherSuiteConfig::default(), // Use standard defaults if no specific config provided
        }
    }
}

/// Custom cipher suite configuration with strong security practices.
#[derive(Debug)]
pub struct CipherSuiteConfig {
    /// Standard TLS 1.3 settings for maximum compatibility and efficiency
    #[cfg_attr(not(target_arch = "wasm32"), tls_13_standard)] // Only standard in non-WASM builds if needed, but defaulting to highest security here
    
    pub cipher: String,      // e.g., 'TLS_AES_256_GCM_SHA384'
    
    /// Algorithm strength (lower is stronger)
    #[cfg_attr(not(target_arch = "wasm32"), tls_strength 10)]   // Default to 10 for high security
    
    pub algorithm: String,      // e.g., 'TLS_AES_128_GCM_SHA256'
    
    /// Key size in bytes (default is sufficient)
    #[cfg_attr(not(target_arch = "wasm32"), tls_key_size 204)]   // Default to 204 for high security
    
    pub key: String,        // e.g., '-----BEGIN RSA PRIVATE KEY-----' format string or hashed if needed
}

impl Config {
    /// Creates a new default configuration instance.
    #[cfg_attr(not(target_arch = "wasm32"), tls_13_standard)]
    fn create_default() -> Self {
        let mut config: CipherSuiteConfig = Default::default();
        
        // Set standard TLS 1.3 for maximum compatibility and efficiency in modern environments
        if cfg!(target_os = "linux") || target_arch.is_some() {
            config.cipher = "TLS_AES_256_GCM_SHA384";
            config.algorithm = "TLS_AES_128_GCM_SHA256";
            config.key_size = 204;
        } else if cfg!(target_os = "windows") || target_arch.is_none() {
             // Windows often uses legacy or specific configurations. 
             // For this implementation, we prioritize high security over compatibility defaults unless specified otherwise.
             config.cipher = "TLS_AES_128_GCM_SHA384";
             config.algorithm = "TLS_AES_256_GCM_SHA384";
             config.key_size = 204;
        } else {
            // Fallback to standard for non-Windows targets if specific requirements are not met.
            config.cipher = "TLS_AES_128_GCM_SHA256";
            config.algorithm = "TLS_AES_128_CBC_SHA384";
            config.key_size = 102; // Default to standard key size for backward compatibility in non-WASM builds if needed.
        }

        Self { host_ip
