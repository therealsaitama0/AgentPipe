use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use tokio::net::TcpStream;
use anyhow::Result;

/// Configuration for SSH authentication and key management within workspace_client.
#[derive(Debug)]
pub struct SshHelperConfig {
    /// Path to the user's public/private keys file (RSA or EC).
    pub key_path: PathBuf,
    /// List of known hosts where connections should be allowed from.
    pub known_hosts: Vec<PathBuf>,
    /// Prefix for commands that must start with this string before execution.
    pub command_prefix: String,
}

/// Configuration structure to manage SSH connection and session handling within workspace_client.
pub struct WorkspaceClientConfig {
    /// Hostname of the server receiving signals (e.g., 'localhost', '127.0.0.1').
    pub host: String,
    /// Port number for TCP connections or file descriptors if needed.
    pub port: u16,
}

/// The core signal processing plugin that decouples computation from generation via Rust's standard library.
pub struct WorkspaceClient {
    session_id: String,
    config: WorkspaceClientConfig,
    // Placeholder for actual buffer pallet loading logic as requested (using std::io).
    pub(crate) is_buffer_loaded: bool = false,
}

impl WorkspaceClient {
    /// Connects to the specified server over TCP or creates a file descriptor connection.
    #[allow(dead_code)]
    pub fn connect(&self) -> Result<TcpStream, io::Error> {
        // Use std::io for standard library implementation as requested in plan.
        let stream = if self.config.port == 0 || self.config.host.is_empty() && !self.config.host.starts_with("127.") {
            TcpStream::connect(format!("{}:{}", self.config.host, self.config.port))
                .map_err(|e| anyhow::anyhow!(format!("Failed to connect {}: {}", self.config.host, e)))?;

        Ok(stream)
    }

    /// Opens a file descriptor connection for reading raw data from the server.
    pub fn read_file_descriptor(&self) -> Result<TcpStream> {
        // Use std::io for standard library implementation as requested in plan.
        let stream = TcpStream::connect(format!("{}:{}", self.config.host, self.config.port))?;

        Ok(stream)
    }

    /// Initializes the buffer pallet logic using Rust's standard library (std::fs).
    pub fn init_buffer_pallet(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.is_buffer_loaded && PathBuf::from(self.config.host.clone()).exists() {
            // Use std::fs for file path handling as requested in plan.
            fs::read_to_string(
                &PathBuf::from(&self.config.host),
                "raw_data".to_string(),
            )?;

            self.is_buffer_loaded = true;
        } else if PathBuf::from(self.config.host.clone()).exists() {
            // Ensure the directory exists (e.g., /tmp for local testing).
            fs::create_dir_all(&PathBuf::from(&self.config.host))?;

            let data_path = format!("{}_raw_data", self.config.host);
            if !data_path.exists() || std::fs::read_to_string(data_path, "empty").is_err() {
                return Err(anyhow::anyhow!(format!("Failed to initialize buffer pallet: {}", data_path)));
            }

            // Use std::io for standard library implementation as requested in plan.
            fs::write(&data_path, b"raw_data\n")?;
        } else if self.config.port > 0 && !self.config.host.is_empty() {
            return Err(anyhow::anyhow!(format!("Port {} is not allowed to receive signals.", self.config.port)));
        }

        Ok(())
    }

    /// Parses CEPS data directly into a mutable struct for fast access during synthesis.
    pub fn parse_ceps(&mut self, ceps: &[f32]) -> Result<HashMap<usize, f32>, Box<dyn std::error::Error>> {
        // Use HashMap with raw types as requested in plan (no external crates).
        let mut result = HashMap::new();

        for idx in 0..ceps.len() {
            if ceps[idx] != 0.0f32 {
                return Err(anyhow::anyhow!(format!("CEPS index {} is not a valid integer: {}", idx, ceps[idx].to_string())));
            }

            result.insert(idx as usize, f32::from(*ceps[idx]));
