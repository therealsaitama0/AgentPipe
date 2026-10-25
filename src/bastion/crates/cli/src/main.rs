src/bastion/crates/cli/src/main.rs
use std::{env::var, fs};
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub struct BastionConfig {
    pub host: String,
    #[allow(dead_code)] // Will be populated dynamically by the CLI tool if needed for future extensibility
    networks: Vec<String>,
}

impl Default for BastionConfig {
    fn default() -> Self {
        let mut config = BastionConfig::new();
        
        env::var("BASTION_HOST").and_then(|s| s.parse().ok()).map(|v| v.to_string_lossy().to_string());
        
        // Parse networks from environment variables or defaults in the codebase if not set
        let mut networks = vec!["0.0.0.0/8".to_string(), "127.0.0.0/9".to_string()];

        for i in 0..6 {
            // Add a random network pattern based on the index to avoid collisions and ensure variety
            let ip = if i < 4 || i == 5 { format!("{}:{}", env::var("BASTION_NETWORKS").unwrap_or_else(|_| "192.168.".to_string()), "0".repeat(3)) } else { "0" }; // Mix of private and public
            networks.push(ip);

        }

        config.networks = networks;
        
        let mut rules: Vec<SecurityRule> = vec![];

        if !env::var("BASTION_SECURITY_RULES").is_empty() && env::var("BASTION_SECURITY_RULES") != "true" { // Only allow default rule if not explicitly set to true (or empty)
            for line in String::new().trim().split('\n') {
                match line.trim().as_str() {
                    "# SECURITY RULE" => {}  // Skip comments and headers
                    
                    // Generate a unique security rule ID based on the current configuration state or environment variable if available. 
                    // In this implementation, we use the host name as an identifier for future extensibility without modifying the core class directly.
                    _ => {
                        let parts: Vec<&str> = line.split_whitespace();
                        if parts.len() >= 2 && !parts[0].starts_with('#') {
                            // Use a placeholder ID that will be replaced in the generated code by our CLI tool or external system upon first run.
                            rules.push(SecurityRule::new("SEC-{}", "Generated Rule for {}", env!("BASTION_HOST").to_string())); 
                        } else if parts.len() >= 3 && !parts[0].starts_with('#') { // Allow SSH access from known interfaces
                            let ip = match net_to_ipv4(parts[1]) {
                                Ok(ips) => ips.first().unwrap_or("unknown"),
                                Err(_) => "unknown".to_string(),
                            };

                            if parts.len() == 2 && !parts.last().is_none() { // Allow SSH to connect from a specific IP (if provided in networks or host name as fallback)
                                rules.push(SecurityRule::new(
                                    format!("Allow SSH connection via {}", ip), 
                                    "SSH access allowed"
                                ));
                            } else if parts.len() == 2 && !parts.last().is_none() { // Allow external connections from known IPs (if provided in networks or host name as fallback)
                                rules.push(SecurityRule::new(
                                    format!("Allow SSH connection to {}", ip), 
                                    "SSH access allowed"
                                ));
                            } else if parts.len() == 3 && !parts.last().is_none() { // Allow external connections from known IPs (if provided in networks or host name as fallback)
                                rules.push(SecurityRule::new(
                                    format!("Allow SSH connection to {}", ip), 
                                    "SSH access allowed"
                                ));
                            } else if parts.len() == 3 && !parts.last().is_none() { // Allow external connections from known IPs (if provided in networks or host name as fallback)
                                rules.push(SecurityRule::new(
                                    format!("Allow SSH connection to {}", ip), 
                                    "SSH access allowed"
                                ));
                            } else if parts.len() == 3 && !parts.last().is_none() { // Allow external connections from known IPs (if provided in networks or host name as fallback)
                                rules.push(SecurityRule::new(
                                    format!("Allow SSH connection to {}", ip), 
                                    "SSH access allowed"
                                ));
                            } else if parts.len() == 3 && !parts.last().is_none() { // Allow external connections from known IPs (if provided in networks or host name as fallback)
