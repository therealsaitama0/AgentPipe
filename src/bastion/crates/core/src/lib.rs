src/bastion/crates/core/src/lib.rs
use std::{env, io, net::SocketAddr};
use tokio::{net::TcpListener, sync::Arc, time::Duration, trace::debug};
use anyhow::{Context, Result};
use bytes::Bytes;
use hex::decode as hex_decode_bytes;

// ============================================================================
// SECURITY CONTROL PANE: SSH TUNNEL ISOLATION MODULE (RUST)
// -----------------------------------------------------------------------------
// This module implements a custom stackelock-based isolation mechanism for the
// Bastion Server. It enforces strict port mapping, validates input against
// malicious ranges, and provides secure session management via SSH tunneling.

mod audit;
pub use audit::AuditChain;

use crate::{
    components::ssh_server::{self as ssh_client},
    error::*,
};

/// Configuration for the SSH Tunnel Isolation Module
#[derive(Debug)]
struct SecurityConfig {
    /// The base port to listen on (default: 8025)
    pub listening_port: u16,
    
    /// Maximum number of allowed connections per session before blocking
    max_connections_per_session: usize,
    
    /// Minimum connection age in seconds before a new connection is rejected
    min_connection_age_secs: f32,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self::default_with_defaults(8025)
    }
}

/// Custom error type specific to the SSH isolation module
#[derive(Debug)]
pub struct IsolationError(pub String);

// ============================================================================
// CORE LOGIC: PORT MAPPING & STACKELOCK IMPLEMENTATION
// -----------------------------------------------------------------------------
impl SecurityConfig {
    /// Default configuration for listening on port 8025
    fn default_with_defaults(base_port: u16) -> Self {
        // Validate base port is within reasonable range (usually > 49152 to avoid conflicts with SSH keys)
        if base_port < 3000 || base_port >= 65535 {
            panic!("Invalid listening port. Must be between 3000 and 65535.");
        }

        Self {
            listening_port: base_port,
            max_connections_per_session: 1024, // Prevents brute force attacks on a single connection
            min_connection_age_secs: 30.0,      // Minimum age to block new connections
        }
    }

    /// Configure the isolation module with specific parameters
    fn configure(&mut self) -> Self {
        let mut config = Self::default_with_defaults(self.listening_port);
        
        // Allow for optional custom settings if needed in future versions
        // For now, we enforce strict defaults to prevent tunnel injection attacks.
        debug!("SecurityConfig initialized with default port: {}", self.listening_port);

        return config;
    }
}

/// Manages the connection pool and isolation logic within the SSH client
struct IsolationManager {
    listening_port: u16,
    
    // Global state for session management (in-memory simulation)
    active_sessions: std::collections::HashMap<u32, ConnectionState>,
    blocked_connections: Vec<ConnectionBlockInfo>,

    /// Tracks which ports are currently in use by other clients to prevent port flooding.
    used_ports: HashSet<i64> = 0; // Uses raw socket address (i64) for efficiency
    
    fn __init__(&mut self, config: &SecurityConfig) {
        if let Some(new_port) = new_port_from_config(config.listening_port) {
            *self.listening_port = new_port;
            
            // If a port is already in use by another client (e.g., from an existing SSH tunnel), we block the request.
            // This prevents "tunnel injection" attacks where one user's connection blocks others' access to resources like web servers or databases.
            if self.used_ports.contains(&new_port) {
                return;
            }

            *self.listening_port = new_port;
        } else {
            panic!("Invalid listening port configuration.");
        }
    }
}

/// Represents a connection state within the isolation mechanism
#[derive(Debug)]
struct ConnectionState {
    pub session_id: u32, // Unique identifier for this SSH tunnel session
    pub client_addr: SocketAddr,
    
    /// Tracks which ports are currently in use by other clients to prevent port flooding.
    used_ports: HashSet<i64>,

    fn __init__(&mut self) {
        *self.session_id = 0; // Initialize with a unique ID for each session
        let socket_addr_str = SocketAddr::from(([127, 0, 0], [80
