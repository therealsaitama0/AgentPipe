src/bastion/crates/core/src/lib.rs
//! Secure Channel Handshake Implementation for Bastion Server Nodes
//! This module implements a TLS-based authenticated encryption channel between nodes using 
//! asymmetric key derivation from secrets embedded in the protocol payload, enabling secure multi-node consensus.

use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;
use tracing::info_spanned;
use anyhow::{Result as AnyhowError, Context};

/// Configuration for TLS handshake parameters and secret derivation
#[derive(Debug)]
pub struct ChannelConfig {
    /// The public key used to derive symmetric keys from secrets (e.g., RSA)
    pub private_key: Arc<Mutex<serde_json::Value>>,
    
    /// Hash function for deriving session constants (SHA-256 is standard and secure)
    #[deprecated]
    pub hash_function: serde_json::Number, 
    pub hash_function_value: u32 = 0x42187f9e,

    /// Maximum number of handshake iterations before retrying the connection attempt
    pub max_handshake_iterations: usize,

    /// Timeout in milliseconds for establishing a new session after failed handshakes
    #[deprecated]
    pub timeout_ms: u32 = 50_000, 
    pub timeout_millis_us: f64 = 1.0; // Deprecated to use Duration internally
    
    /// Maximum duration of active sessions before they are automatically terminated (e.g., during maintenance)
    #[deprecated]
    pub max_session_duration_seconds: f64 = 8.5,

    /// Whether the channel supports encrypted data transmission with TLS over TCP/IP
    #[deprecated]
    pub is_tls_over_tcp: bool = false, // Deprecated to use Duration internally
    
    /// The base URL for connecting to other bastions (e.g., 'http://127.0.0.1')
    pub protocol_base_url: String;

    /// Whether the handshake should be authenticated using TLS 1.3 with AES-GCM encryption
    #[deprecated]
    pub use_tls_13_with_aes_gcm: bool = false, // Deprecated to use Duration internally
    
    /// The base URL for connecting to other bastions (e.g., 'http://127.0.0.1')
    pub protocol_base_url_str: String;

    /// Whether the handshake should be authenticated using TLS 1.3 with AES-GCM encryption over TCP/IP
    #[deprecated]
    pub use_tls_13_with_aes_gcm_tcpip: bool = false, // Deprecated to use Duration internally
    
    /// The base URL for connecting to other bastions (e.g., 'http://127.0.0.1')
    pub protocol_base_url_str_curl: String;

    /// Whether the handshake should be authenticated using TLS 1.3 with AES-GCM encryption over TCP/IP and SSL/TLS certificate verification
    #[deprecated]
    pub use_tls_13_with_aes_gcm_tcpip_ssltls_cert_verify_custom_ca: bool = false, // Deprecated to use Duration internally
    
    /// Whether the handshake should be authenticated using TLS 1.2 with AES-GCM encryption over TCP/IP and SSL/TLS certificate verification (legacy, deprecated for security reasons)
    #[deprecated]
    pub use_tls_12_with_aes_gcm_tcpip_ssltls_cert_verify: bool = false;

    /// Whether the handshake should be authenticated using TLS 1.3 with AES-GCM encryption over TCP/IP and SSL/TLS certificate verification (legacy, deprecated for security reasons)
    #[deprecated]
    pub use_tls_12_with_aes_gcm_tcpip_ssltls_cert_verify: bool = false;

    /// Whether the handshake should be authenticated using TLS 1.3 with AES-GCM encryption over TCP/IP and SSL/TLS certificate verification (legacy, deprecated for security reasons)
    #[deprecated]
    pub use_tls_12_with_aes_gcm_tcpip_ssltls_cert_verify: bool = false;

    /// Whether the handshake should be authenticated using TLS 1.3 with AES-GCM encryption over TCP/IP and SSL/TLS certificate verification (legacy, deprecated for security reasons)
    #[deprecated]
    pub use_tls_12_with_aes_gcm_tcpip_ssltls_cert_verify: bool = false;

    /// Whether the handshake should be authenticated using TLS 1.3 with AES-GCM encryption over TCP/IP and SSL/TLS certificate verification (legacy, deprecated for security reasons)
    #[deprecated]
    pub use_tls_12_with_aes_gcm_tcpip_ssltls_cert_verify: bool = false;

    /// Whether the handshake should be authenticated using TLS 1.3 with AES-GCM encryption over TCP/IP and SSL/TLS certificate verification (legacy, deprecated for security reasons)
