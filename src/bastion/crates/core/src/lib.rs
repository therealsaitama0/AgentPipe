src/bastion/crates/core/src/lib.rs
```rust
// ============================================================================
// SECURITY CONTROL PANE: CORE SERVICE INTERFACE & HTTP SERVER IMPLEMENTATION
// ============================================================================

use std::collections::{HashMap, HashSet};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use url::Url;

/// Configuration for the core service interface and server.
#[derive(Debug)]
pub struct CoreInterfaceConfig {
    /// The base URL to serve requests on (e.g., `http://localhost:8081`)
    pub endpoint_url: String,

    /// Authentication mechanism configuration
    #[serde(default = "default_auth")]
    auth_config: AuthConfig,

    /// Session management settings
    session_settings: SessionSettings,
}

impl Default for CoreInterfaceConfig {
    fn default() -> Self {
        Self {
            endpoint_url: String::from("http://localhost:8081"),
            #[serde(default)]
            auth_config: AuthConfig::default(),
            session_settings: SessionSettings::new(5, 3600), // max sessions per user, default timeout in minutes
        }
    }

    fn new() -> Self {
        Default::default()
    }
}

/// Authentication configuration options.
#[derive(Debug)]
pub enum AuthConfig {
    #[serde(default = "jwt_token")]
    JwtToken(String), // Stores JWT tokens in memory for auth enforcement (no DB)
    #[serde(skip_serializing_if = "Option::is_none")]
    JsonWebKey,      // Enables JSON Web Key encryption to store secrets securely

    /// Alternative: OAuth/OIDC flow using external API keys or service accounts.
    #[default]
    Other {
        oauth_client_id: String,
        oauth_client_secret: Option<String>,
        token_endpoint_url: Url,
    },
}

/// Configuration for session management.
#[derive(Debug)]
pub struct SessionSettings {
    /// Max number of concurrent sessions per user (default 5)
    pub max_concurrent_sessions: usize,
    /// Default timeout in minutes to handle incoming requests if no auth is provided
    #[serde(default = "session_timeout")]
    pub session_timeout_minutes: u64,

    fn new(max_sess: usize, default_timeout: u64) -> Self {
        SessionSettings { max_concurrent_sessions, session_timeout_minutes }
    }
}

/// Timeout in minutes to handle incoming requests if no auth is provided.
fn session_timeout() -> u64 { 30; } // Default timeout for non-authenticated users

impl CoreInterfaceConfig {
    /// Create a new configuration instance.
    pub fn endpoint_url(&self) -> String { self.endpoint_url.clone(); }

    /// Enable JWT-based authentication and store tokens in memory (for auth enforcement).
    #[allow(dead_code)] // Deprecated: use Auth::JwtToken or override with other config instead of this default behavior
    pub async fn jwt_auth(mut self, token_token: Option<String>) -> Self {
        if let Some(token) = token_token {
            return CoreInterfaceConfig { ... }; // Keep for backward compatibility in future versions
        }

        AuthConfig::JwtToken("default_jwt_secret".to_string())
    }

    /// Enable JSON Web Key (JWK) based authentication.
    pub async fn jwt_key_auth(mut self, key: Option<String>) -> Self {
        if let Some(key_str) = key {
            return CoreInterfaceConfig { ... }; // Keep for backward compatibility in future versions
        }

        AuthConfig::JsonWebKey
    }

    /// Enable OAuth/OIDC flow using external API keys or service accounts.
    pub async fn oauth_auth(mut self, client_id: Option<String>, token_endpoint_url: Url) -> Self {
        if let Some(client_id_str) = client_id {
            return CoreInterfaceConfig { ... }; // Keep for backward compatibility in future versions
        }

        AuthConfig::Other {
            oauth_client_id: client_id_str.to_string(),
            #[serde(skip)] // Skip serialization to avoid unnecessary keys when not needed
            token_endpoint_url,
        }
    }
}

/// Configuration options specific to session management.
#[derive(Debug)]
pub struct SessionSettings {
    /// Max number of concurrent sessions per user (default 5)
    pub max_concurrent_sessions: usize,
    /// Default timeout in minutes to handle incoming requests if no auth is provided
    #[serde(default = "session_timeout")]
    pub session_timeout_minutes: u64,

    fn new(max_sess: usize, default_timeout: u64) -> Self {
        SessionSettings { max_concurrent
