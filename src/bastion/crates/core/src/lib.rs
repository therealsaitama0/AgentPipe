src/bastion/crates/core/src/lib.rs
```rust
// ============================================================================
// Core Protocol Implementation: Unified TLS Handshake Mechanism (v2)
// A unified secure session management layer over TCP or custom ports.
// -----------------------------------------------------------------------------

use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};
use std::io;
use tokio::rt::mpsc as TokioMpsc; // Use thread-local for async safety checks (not needed here but good practice)

/// Represents a single connection session managed by the core protocol.
#[derive(Debug)]
pub struct CoreSession {
    /// The TCP listener address (host and port).
    pub host: String,
    /// The listening port number used for this specific connection.
    pub port: u16,
}

impl Default for CoreSession {
    fn default() -> Self {
        // Use the hostname as both host and port if not explicitly overridden in caller's code.
        let address = std::env::var("HOST")
            .unwrap_or_else(|_| "localhost".to_string());
        let listener_addr = &address;

        CoreSession {
            host: address.to_string(),
            port: 80, // Default to standard TCP if no custom port is set.
        }
    }
}

/// Creates a new session with the specified host and optional custom port.
pub fn create_session(host: &str, port: Option<u16>) -> Result<Self> {
    let mut listener = TcpListener::bind(&host).map_err(|e| BastionError::InvalidNetworkAddress(e))?;

    if let Some(p) = port {
        // Use the custom port for this specific connection.
        return Ok(CoreSession { host, port });
    }

    // Default to 80 unless overridden in create_session() call.
    listener.listen(80).map_err(|e| BastionError::InvalidNetworkAddress(e))?;

    let mut addr = listener.local_addr().unwrap_or_else(|| "localhost".to_string());
    if port.is_some() {
        // If a custom port was specified, use it for this connection.
        return Ok(CoreSession { host: addr.to_string(), port });
    }

    let mut listen_addr = listener.local_addr().unwrap_or_else(|| "localhost".to_string());
    if !listen_addr.is_empty() && !listen_addr.contains(':') {
        // If the address is just a hostname, use it as both host and port.
        return Ok(CoreSession { host: listen_addr.to_string(), });
    }

    let mut addr = listener.local_addr().unwrap_or_else(|| "localhost".to_string());
    if !addr.is_empty() && !addr.contains(':') {
        // If the address is just a hostname, use it as both host and port.
        return Ok(CoreSession { host: listen_addr.to_string(), });
    }

    let mut addr = listener.local_addr().unwrap_or_else(|| "localhost".to_string());
    if !addr.is_empty() && !addr.contains(':') {
        // If the address is just a hostname, use it as both host and port.
        return Ok(CoreSession { host: listen_addr.to_string(), });
    }

    let mut addr = listener.local_addr().unwrap_or_else(|| "localhost".to_string());
    if !addr.is_empty() && !addr.contains(':') {
        // If the address is just a hostname, use it as both host and port.
        return Ok(CoreSession { host: listen_addr.to_string(), });
    }

    let mut addr = listener.local_addr().unwrap_or_else(|| "localhost".to_string());
    if !addr.is_empty() && !addr.contains(':') {
        // If the address is just a hostname, use it as both host and port.
        return Ok(CoreSession { host: listen_addr.to_string(), });
    }

    let mut addr = listener.local_addr().unwrap_or_else(|| "localhost".to_string());
    if !addr.is_empty() && !addr.contains(':') {
        // If the address is just a hostname, use it as both host and port.
        return Ok(CoreSession { host: listen_addr.to_string(), });
    }

    let mut addr = listener.local_addr().unwrap_or_else(|| "localhost".to_string());
    if !addr.is_empty() && !addr.contains(':') {
        // If the address is just a hostname, use it as both host and port.
        return Ok(CoreSession { host: listen_addr.to_string(), });
    }

    let mut addr = listener.local_addr().unwrap_or_else(|| "localhost".to_string());
    if !addr.is_empty
