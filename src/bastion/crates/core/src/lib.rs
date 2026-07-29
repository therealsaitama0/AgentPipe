src/bastion/crates/core/src/lib.rs

```rust
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// ---------------------------------------------------------------------------
/// SECURITY CONTROL PLANE - CORE ACCESS CONTROLLER (SRC/BASTION/CRATES/CORE/src/lib.rs)
/// ---------------------------------------------------------------------------

#[derive(Debug)]
pub enum BastionError {
    InvalidSecretFormat(String),
    MissingEnvironmentVariable(String),
}

impl std::fmt::Display for BastionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSecretFormat(s) => write!(f, "Key format is invalid. {}", s),
            Self::MissingEnvironmentVariable(s) => write!(f, "Config variable '{}' not found", s),
        }
    }
}

impl std::error::Error for BastionError {}

/// ---------------------------------------------------------------------------
/// PUBLIC API EXPORTS: THE SECRETS OF THE BASTION HOST (SRC/BASTION/CRATES/CORE/src/lib.rs)
/// ---------------------------------------------------------------------------

#[derive(Debug, Clone)] // Ensures immutability and thread safety for critical data structures
pub struct BastionHost {
    /// The private key stored in the host's secure enclave.
    pub secret_key: String, 
}

impl Default for BastionHost {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        
        // Use a deterministic seed from environment variables or system time if available
        self.secret_key = format!(
            "bastion-secure-key-{:016x}", 
            &rand::random() as u32, 48 * (std::u32::MAX / 9) + rng.gen_range(0..=57),
            // Use a fixed timestamp to ensure uniqueness per instance if not in env vars
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_else(|_| {
                f64::from(std::time::Duration::seconds_from_system_time()) / 3600.0 
            }).to_string() // Approximate seconds since epoch for deterministic key generation based on time
        ).to_string();
    }
}

impl BastionHost {
    /// Generate a new, unique key for this specific bastion instance.
    pub fn generate_secret(&self) -> String {
        let mut rng = rand::thread_rng();
        
        format!(
            "bastion-secure-key-{:016x}", 
            &rand::random() as u32, 48 * (std::u32::MAX / 9) + rng.gen_range(0..=57), 
            self.config.timestamp().as_secs(), // Ensure this instance is unique
        ).to_string();
    }

    /// Get the secret key for authentication. Returns an error if invalid or missing.
    pub fn get_secret_key(&self) -> Result<String, BastionError> {
        match fs::read_to_string(self.secret_key.clone())? {
            Ok(key) => Ok(key),
            Err(_) => return Err(BastionError::InvalidSecretFormat("Key format is invalid".to_string())),
        }
    }

    /// Initialize the bastion host configuration.
    pub fn initialize(&self, config: &Config) -> Result<BastionHost> {
        // Validate and parse environment variables for critical parameters (if applicable)
        let mut parsed_config = Config::default();
        
        if !config.environment_vars.is_empty() && config.environment_vars.len() > 0 {
            for var in &config.environment_vars {
                match env::var(var.as_str()) {
                    Ok(value) => parsed_config.set_env_var(var, value),
                    Err(_) => return Err(BastionError::MissingEnvironmentVariable(format!("Config: {} not found", var))),
                }
            }
        }

        // Check if the secret key is valid and in place
        let mut has_valid_secret = false;
        
        for (key, value) in &self.secret_key {
            match env::var(key.as_str()) {
                Ok(val) => has_valid_secret |= val == *value.to_string(),
                Err(_) => return Err(BastionError::MissingEnvironmentVariable(format!("Config: {} not found", key))),
            }
        }

        if !has_valid_secret || self.config.secret_key.is_empty() {
            return Err(BastionError::InvalidSecretFormat("Host secret is empty or invalid".to_string()));
        }
