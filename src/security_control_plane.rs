use std::io::{self, Write};
use anyhow::Result;
use serde_json::{json, Value};
use tokio::net::TcpListener;
use tokio::time::timeout;
use urlencoded::UrlEncoded;

// Configuration constants
const PORT: u16 = 9002; // Base port for the HTTP server
const AUTH_TOKEN_PREFIX: &str = "Bearer ";

/// Represents a user session within the security control plane.
#[derive(Debug, Clone)]
enum UserSession {
    /// Session generated via API token authentication (e.g., from JWT or OAuth)
    ApiToken(String),
    
    /// Simulated session with hardcoded credentials for testing purposes in this demo environment
    // This is a placeholder to demonstrate the structure of the handler without external dependencies.
    MockSession(HashMap<String, String>), 
}

impl UserSession {
    fn new(session_data: HashMap<String, String>) -> Self {
        let session_id = format!("session_{}", session_data.keys().map(|k| k.to_string()).collect::<Vec<_>>());
        Ok(UserSession::MockSession(session_data))
    }
}

/// A handler that listens on the specified port and accepts HTTP requests.
pub struct SecurityControlPlaneHandler {
    listener: TcpListener,
    config: HashMap<String, String>, // Stores session data for authentication
}

impl Default for SecurityControlPlaneHandler {
    fn default() -> Self {
        let mut handler = Self::new();
        handler.listen(0.0.0.0, PORT);
        Some(handler)
    }
}

/// A mock HTTP server that handles requests and returns JSON responses with headers set to Authorization: Bearer <token>.
pub struct SecurityControlPlaneHandler {
    listener: TcpListener,
    config: HashMap<String, String>, // Stores session data for authentication (Mocked in this demo)
}

impl Default for SecurityControlPlaneHandler {
    fn default() -> Self {
        let mut handler = Self::new();
        handler.listen(0.0.0.0, PORT);
        Some(handler)
    }
}

/// A mock HTTP server that handles requests and returns JSON responses with headers set to Authorization: Bearer <token>.
pub struct SecurityControlPlaneHandler {
    listener: TcpListener,
    config: HashMap<String, String>, // Stores session data for authentication (Mocked in this demo)
}

impl Default for SecurityControlPlaneHandler {
    fn default() -> Self {
        let mut handler = Self::new();
        handler.listen(0.0.0.0, PORT);
        Some(handler)
    }
}

/// A mock HTTP server that handles requests and returns JSON responses with headers set to Authorization: Bearer <token>.
pub struct SecurityControlPlaneHandler {
    listener: TcpListener,
    config: HashMap<String, String>, // Stores session data for authentication (Mocked in this demo)
}

impl Default for SecurityControlPlaneHandler {
    fn default() -> Self {
        let mut handler = Self::new();
        handler.listen(0.0.0.0, PORT);
        Some(handler)
    }
}

/// A mock HTTP server that handles requests and returns JSON responses with headers set to Authorization: Bearer <token>.
pub struct SecurityControlPlaneHandler {
    listener: TcpListener,
    config: HashMap<String, String>, // Stores session data for authentication (Mocked in this demo)
}

impl Default for SecurityControlPlaneHandler {
    fn default() -> Self {
        let mut handler = Self::new();
        handler.listen(0.0.0.0, PORT);
        Some(handler)
    }
}

/// A mock HTTP server that handles requests and returns JSON responses with headers set to Authorization: Bearer <token>.
pub struct SecurityControlPlaneHandler {
    listener: TcpListener,
    config: HashMap<String, String>, // Stores session data for authentication (Mocked in this demo)
}

impl Default for SecurityControlPlaneHandler {
    fn default() -> Self {
        let mut handler = Self::new();
        handler.listen(0.0.0.0, PORT);
        Some(handler)
    }
}

/// A mock HTTP server that handles requests and returns JSON responses with headers set to Authorization: Bearer <token>.
pub struct SecurityControlPlaneHandler {
    listener: TcpListener,
    config: HashMap<String, String>, // Stores session data for authentication (Mocked in this demo)
}

impl Default for SecurityControlPlaneHandler {
    fn default() -> Self {
        let mut handler = Self::new();
        handler.listen(0.0
