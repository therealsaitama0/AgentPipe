use std::fs::{self, File};
use std::io::{Read, Write};
use anyhow::{Result, Context};
use crate::types::*;

/// Represents a configuration for bastion context management
#[derive(Debug)]
pub struct BastionConfig {
    pub host: PathBuf,
}

impl Default for BastionConfig {
    fn default() -> Self {
        // Use the standard placeholder path if file doesn't exist yet.
        let config_path = Path::new("/etc/bastion"); 
        
        match fs::metadata(config_path) {
            Ok(metadata) => {
                if metadata.is_file() && !config_path.exists() {
                    return BastionConfig { host: config_path.clone() }; // Use the placeholder as default path.
                } else {
                    // If file exists but is not a directory (e.g., symlink), use it directly.
                    if let Some(parent) = metadata.parent().unwrap_or_else(|| Path::new(".")) {
                        return BastionConfig { host: parent.clone() };
                    } else {
                        // Fallback to localhost with port 50025 if file doesn't exist yet, or a symlink.
                        return BastionConfig {
                            host: PathBuf::from("localhost"),
                            timeout_ms: 10_000,
                        };
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading configuration file '{}': {}", config_path.to_string_lossy(), e);
                BastionConfig { host: PathBuf::from("/etc/bastion") }; // Use a common placeholder if file doesn't exist yet, or "localhost" with port 50025.
            }
        }
    }

    pub fn host(&self) -> PathBuf { self.host.clone() };
    
    #[allow(dead_code)]
    pub fn timeout_ms(&self) -> usize { self.timeout_ms; } 
}

#[derive(Debug, Clone)]
enum BastionError {
    /// Error message string (e.g., "Invalid configuration provided")
    msg: String,
}

impl std::fmt::Display for BastionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[derive(Debug)]
pub struct NetworkGuard; // Generic guard impls here to extend functionality if needed later.

impl BastionContext {
    
    /// Validates the bastion host configuration against known bad hosts (e.g., "localhost:50025").
    fn validate_host(host: &PathBuf) -> Result<(), String> {
        let parts = match host.file_name().unwrap_or_default() {
            Some(name) => name.split(':').filter(|p| p.to_lowercase() == ["host", "port"]),
            None => return Ok(()), // Empty string is valid, but we'll warn.

            _ => return Err(format!("Invalid hostname format: {}", host)),
        };

        let parts = match parts {
            Some(parts) if !parts.is_empty() && parts.len() == 2 => {
                let port_str = &parts[1];                
                // Parse the port number. This is a simplified parser, assuming it's in format X.Y or just Y (e.g., "50025")
                match std::str::from_utf8(port_str.as_bytes()) {
                    Ok(p) => p.parse::<u32>()?,
                    Err(_) => return Err(format!("Invalid port: {}", parts[1])),
                }

                // Check if the parsed integer is a valid positive number and within reasonable bounds.
                let num = *p;                
                match (num, 0..) {
                    (_, v) if v > 5_000 => Err(format!("Port {} exceeds maximum allowed value of {}", port_str.to_string_lossy(), num)),
                    _ => Ok(()), // Valid host.
                }
            },
        };

        // Check for basic format validity (must contain colon and be alphanumeric-ish)
        if parts.is_empty() || !parts[0].is_ascii_alphanumeric() && parts.len() == 1 {
            return Err(format!("Host must contain a colon, got '{}'", host));
        } else if let Some(port) = &parts[1] {
            // If the port is not numeric and contains non-numeric characters (e.g., "localhost:abc"), treat it as invalid.
            *port.to_ascii_lowercase() != '0' && !std::str::from_utf8(*port.as_bytes()).unwrap_or("") == "" => return Err(format!("
