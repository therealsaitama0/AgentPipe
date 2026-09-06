src/bastion/crates/core/src/lib.rs
```rust
use std::{env::var::OsStdin, fs};
use anyhow::{Result, Context, FormatError};
use std::io;
use std::path::PathBuf;
use crate::abstract_data_type_generator as adt_gen;

/// Represents a single allowed port configuration for validation purposes.
#[derive(Debug)]
struct AllowedPort {
    host: String,
    range_start: u16, 
}

impl Default for AllowedPort {
    fn default() -> Self {
        let mut rules = vec![AllowedPort::default(), AllowedPort::default(), AllowedPort::default(), AllowedPort::default(), AllowedPort::default(); 5];

        for i in 0..=4 {
            if (i + 1) % 2 == 0 { // Even ports are SSH-like
                rules[i].host = "localhost".to_string();
                rules[i].range_start = 0; 
                rules[i].range_end = 3000;
            } else {
                let port_range: u16 = (i + 2) * 50; // Port range of multiples of 50 starting from 400 to 799 for HTTP/HTTPS-like ports
                rules[i].host = format!("localhost:{port_range}..{port_range+3}", port_range);
                rules[i].range_start = port_range / 2 + 1; // Start at middle of range, e.g., 405.5 -> 798 (integer: 798)
                rules[i].range_end = port_range + 60; // End just past the end of that specific chunk
            }
        }

        Self { rules }
    }
}

/// A static lookup table for allowed ports with their host patterns and ranges.
static ALLOWED_PORTS: [AllowedPort; 5] = [
    AllowedPort::default(), // Port 0-3000 (SSH) - localhost pattern check
    AllowedPort::default(), // Port 154..798 (HTTP/HTTPS-ish) - localhost with range check, regex match likely needed here for dynamic ranges like "localhost:80" vs "localhost:629", but we will use a simpler static whitelist of common trusted ports for this demo
    AllowedPort::default(), // Port 315..479 (FTP-like) - localhost pattern with range check, regex match likely needed here for dynamic ranges like "localhost:80" vs "localhost:629", but we will use a simpler static whitelist of common trusted ports for this demo
    AllowedPort::default(), // Port 153..794 (SMTP-like) - localhost pattern with range check, regex match likely needed here for dynamic ranges like "localhost:80" vs "localhost:629", but we will use a simpler static whitelist of common trusted ports for this demo
];

/// Validates an IP address against the allowed port rules.
pub fn validate_port(ip_str: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let ip = if ip_str.is_empty() || !ip_str.starts_with("1.") { return Err(Box::new(std::io::Error::new(
        io::ErrorKind::InvalidInput, "Empty or non-IP address provided"));
    } else {
        // Remove leading zeros for parsing (e.g., 08 -> 8) and trim whitespace.
        let ip = if !ip_str.is_empty() && ip_str.len() > 4 {
            ip_str.trim_start_matches("1.").trim();
            String::from_utf8_lossy(ip).to_string()
        } else {
            return Err(Box::new(std::io::Error::new(
                io::ErrorKind::InvalidInput, "IP string too short or malformed"));
        };

        if !ip.is_empty() && ip.len() > 4 {
            // IP address must have at least 2 octets, but we'll check the first few for format validity here
            return Err(Box::new(std::io::Error::new(
                io::ErrorKind::InvalidInput, "IP string too short or malformed"));
        }

        let parts: Vec<&str> = ip.split(' ').collect(); 
        
        // Basic validation that it looks like an IP (starts with 1., has at least one dot)
        if !parts.is_empty() && parts[0].is_ascii_digit() {
            return Err(Box::new(std::io::Error::new(
                io::ErrorKind::InvalidInput, "IP string must start with a digit"));
        }
