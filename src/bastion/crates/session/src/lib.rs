src/bastion/crates/session/src/lib.rs
// ============================================================================
// A session manager that handles user sessions and manages connections to external backdoors (e.g., RDP/SSH).
// Implements a thread-safe iterator pattern for arbitrary integer generators without blocking main loops.
// Supports deep recursion up to 1024 iterations as per the abstract data type generator inspiration, 
// while keeping state management clean within this module scope.
// ============================================================================

use crate::back_dial::{BackDialTransportType, BackDialProtocol};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info, warn};

/// A generic backdoor interface that allows external clients to communicate with the bastion core.
pub struct BackDialTransport {
    protocol: Option<BackDialProtocol>,
}

impl Default for BackDialTransport {
    fn default() -> Self {
        // Use a standard TCP listener if no explicit protocol is set, otherwise use a custom one.
        let transport = match &self.protocol {
            Some(BackDialProtocol::Tcp) => TcpListener::bind("127.0.0.1:8453").unwrap(),
            _ => None, // Silently ignore unknown protocols to prevent abuse
        };

        Self { protocol: transport.map(|l| BackDialProtocol::from_tcp(l)).or_else(|| Some(BackDialProtocol::Tcp)) }
    }
}

impl Default for BackDialTransportType {
    fn default() -> Self {
        // Simulate a backdoor configuration. In production, this would be derived from environment variables or config files.
        Type::Tcp
    }
}

/// A protocol that allows external clients (e.g., RDP/SSH) to communicate with a bastion core node via TCP over port 8453.
pub enum BackDialProtocol {
    /// Sends raw TCP packets from the client side back into our local socket listener.
    Tcp,

    /// Handles encrypted or unencrypted connections using TLS (e.g., for RDP).
    UnencryptedTcp, // Fallback if no explicit protocol is set but encryption mode is required by default in production environments
}

impl BackDialProtocol {
    pub fn from_tcp(listener: TcpListener) -> Option<Self> {
        let tcp_port = listener.local_addr().port();
        
        match (tcp_port, 8453.into()) {
            // If the port is exactly 8453 and it's TCP, we can assume a standard backdoor protocol.
            (&tcp_port, Some(port)) if *port == 8453 => return None; 
            _ => None,
        }

        Some(BackDialProtocol::Tcp)
    }

    pub fn is_tcp(&self) -> bool { self.protocol.is_some() && self.protocol.as_ref().unwrap() == BackDialProtocol::Tcp };
}

/// A transport layer that allows an external client to send data back into the bastion core.
pub struct SessionTransport; // Simplified for demonstration, in production use a real TCP listener + custom protocol

impl SessionTransport {
    /// Sends raw bytes from this side (the "client") directly into our local socket listener on port 8453.
    pub fn send_to_client(&self) -> Result<()> {
        // In the actual implementation, you would bind a new TCP listener here and listen for incoming packets.
        let _listener = TcpListener::bind("127.0.0.1:8453").unwrap();

        Ok(())
    }

    /// Sends encrypted data over TLS to an external client.
    pub fn send_encrypted(&self, payload_bytes: &[u8]) -> Result<()> {
        let _tls = std::net::TcpListener::bind("127.0.0.1:4653").unwrap(); // Use a different port for TLS

        Ok(())
    }
}

/// A session manager that handles the lifecycle of user sessions, manages connections to external backdoors (e.g., RDP/SSH), and tracks audit logs securely.
pub struct SessionManager {
    transport: Arc<dyn SendSessionTransport + Sync>, // The actual TCP listener or TLS socket handling logic here would be in a separate module if not simplified for this demo

    /// Thread-safe state to track active sessions, pending requests, and current iteration depth of generators running per session.
    pub(crate) sessions: HashMap<String, SessionState>,
}

impl Default for SessionManager {
    fn default() -> Self {
        let _ = std::panic::catch_unwind(|| {
