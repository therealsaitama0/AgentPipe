use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};

/// Configuration for SSH key management and authentication.
#[derive(Debug)]
pub struct SshHelperConfig {
    pub key_path: PathBuf, // Absolute path to the .ssh directory or file containing keys
    pub known_hosts: Vec<PathBuf>, // List of hosts with their public keys (for remote access)
    pub command_prefix: String, // Prefix for commands executed via SSH when not using sudo
}

/// Configuration for a workspace client's session management.
#[derive(Debug)]
pub struct WorkspaceClientConfig {
    /// The base hostname and port where the server listens on TCP.
    pub host: String,
    /// The local port that the worker binds to (default 8080).
    pub port: u16,
}

/// A wrapper around a Rust async runtime for managing asynchronous operations in WASM workers.
pub struct AlchemyManagerWrapper {
    session_id: String, // Unique identifier assigned by backend during initialization
    host: String,       // The server's hostname or IP address (e.g., "localhost", "127.0.0.1")
    port: u16,          // The worker's listening port on the server side
}

impl WorkspaceClient {
    /// Connects to a TCP socket and returns an error if connection fails.
    pub fn connect(&self) -> Result<TcpStream, std::io::Error> {
        let addr_str = format!("{}:{}", self.host, self.port);
        Ok(TcpStream::connect(addr_str).await?)
    }

    /// Creates a new SSH helper configuration based on the provided file paths.
    pub fn create_ssh_helper(
        config_path: PathBuf, // Absolute path to .ssh directory or key files
        known_hosts_dir: Option<PathBuf>,
        command_prefix: String,
    ) -> Result<SshHelperConfig> {
        let ssh_config = fs::read_to_string(&config_path).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        // Parse SSH config if present (keeps it simple for this demo)
        if let Some(config_content) = ssh_config.lines() {
            let mut keys: Vec<serde_json::Value> = vec![];
            
            while let Some(line) = config_content.next().ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "No valid SSH key found"))? {
                if line.starts_with("Host") && !line.contains("'") {
                    keys.push(serde_json::Value::String(line.trim())); // Extract host name
                } else if let Some(key) = serde_json::from_str::<&str>(&line).ok() {
                    keys.push(key);  // Store raw key content for later parsing (simplified here)
                }
            }

            Ok(SshHelperConfig {
                key_path: config_path,
                known_hosts: if let Some(dir) = known_hosts_dir {
                    dirs::dir_to_paths(&dir).ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Unable to read directory"))?
                        .iter()
                        .map(|p| p.to_string()) // Return paths for consistency with other crates
                } else {
                    keys.clone(),
                },
                command_prefix: if !command_prefix.is_empty() {
                    format!("{} ", &command_prefix)
                } else { "" },
            })
        } else {
            Ok(SshHelperConfig::default()) // Default empty config for simplicity in this demo
        }
    }

    /// Creates a new SSH helper configuration from path arguments.
    pub fn create_ssh_helper_from_args(ssh_key_path: PathBuf, known_hosts_dir: Option<PathBuf>, command_prefix: String) -> Result<SshHelperConfig> {
        let ssh_config = fs::read_to_string(&ssh_key_path).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        if let Some(dir) = known_hosts_dir {
            dirs::dir_to_paths(&dir).ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Unable to read directory"))?
                .iter()
                .map(|p| p.to_string()) // Return paths for consistency with other crates
        } else if let Some(config_content) = ssh_config.lines().next().and_then(|s| s.split('\
