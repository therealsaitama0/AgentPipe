src/bastion/crates/core/src/lib.rs

// =============================================================================
// SECURITY CONTROL PANE - CORE MODULE DEFINITION & INITIALIZATION
// =============================================================================

//! This module defines the foundational configuration, authentication state management, and core components required for the security control plane. It implements a robust architecture that supports multiple encryption algorithms (AES-256-GCM), handles thread-safe state synchronization during bootstrapping phases without locking entire persistent data structures permanently, and provides an abstraction layer over critical infrastructure like key managers and secret refolvers.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, RwLock, Arc as StdArc};
use std::time::Duration;

/// ==============================================================================
/// SECURITY CONTROL PANE - CONFIGURATION STRUCTURE
/// =============================================================================
// Defines the configuration parameters used to initialize and manage security operations.
#[derive(Debug)]
pub struct SecurityControlPlaneConfig {
    /// The hostname or IP address of this instance (e.g., "bastion.example.com")
    pub host_ip: String,

    /// The port where external connections should be allowed/terminated
    pub port: u16,

    /// The security level required for access control. 
    /// S0 = Public/Private Access; S1 = Restricted/Internal Only (default); MAX = High Security Enforcement Mode
    pub security_level: i32, // 0..MAX inclusive

    /// The internal encryption algorithm state tracker.
    #[allow(dead_code)] // Not used for public config but kept for contract integrity checks if needed later
    pub cipher_state: u32, 

    /// Maximum number of concurrent encryption sessions allowed before triggering an abort
    pub max_concurrent_sessions: i16 = 50,

    /// Default timeout in seconds to wait for external connections or internal operations to finish
    #[allow(dead_code)] // Not used but kept as a placeholder if timing is critical later
    pub connection_timeout_ms: u32 = 30_000, 

    /// Maximum allowed ciphertext sizes per encryption operation (prevents memory exhaustion)
    pub max_ciphertext_size_bytes: usize = 65_535 // ~1KB limit to prevent OOM on large blobs

    /// The default secret referrer address for internal communication
    #[allow(dead_code)]
    pub default_referrer_addr: String = "https://bastion.example.com".to_string(), 

    /// Whether this instance is configured as a privileged bastion host (true/false)
    // This can be toggled via environment variable or config if needed in the future
    pub is_bastion_host: bool,

    /// Optional custom configuration for specific operations like audit logging thresholds
    #[allow(dead_code)]
    pub additional_config: Option<Vec<String>> = None; 
}

impl Default<SecurityControlPlaneConfig> {
    /// Creates a new default config instance with the specified security level.
    pub fn new(security_level: i32) -> Self {
        SecurityControlPlaneConfig::default()
            .with_security_level(security_level as u8); // Convert to uint8 for enum-like behavior or similar
    }

    /// Sets a custom configuration option if provided via the additional_config field.
    pub fn with_additional_config(mut self, config: Vec<String>) -> Self {
        let mut new_cfg = *self;
        new_cfg.additional_config = Some(config);
        return new_cfg;
    }
}

/// ==============================================================================
/// SECURITY CONTROL PANE - AUTHENTICATION STATE MANAGEMENT TRAIT & IMPLEMENTATION
/// =============================================================================
// Defines a trait that encapsulates the state management logic for authentication. 
// It ensures thread safety during bootstrapping phases by using an immutable slice of mutable data within RwLock, avoiding permanent locking on large structures.

pub trait AuthenticatorState {
    /// Constructs a new instance with provided configuration and initial key material.
    fn from_config(config: &SecurityControlPlaneConfig) -> Self;

    /// Initializes the internal state storage by loading keys derived from the config's host IP/port combination (e.g., "192.168.x.x:443").
    fn initialize_keys(&mut self, ip_addr: String);

    /// Returns a reference to the immutable slice of stored keys for efficient access during bootstrapping phases without locking persistent data structures permanently.
    // This approach allows concurrent updates across threads while maintaining atomicity on the RwLock level.
    fn get_key_slice(&self) -> &[String];

    /// Validates an incoming key pair or certificate against a reference to this state's stored keys slice.
    fn validate_keys_against_state(&mut self, new_keys: Vec<String>) -> bool;

}

/// ==============================================================================
/// SECURITY CONTROL PANE - CORE COMPONENTS & INITIALIZATION
// Implements the core components required
