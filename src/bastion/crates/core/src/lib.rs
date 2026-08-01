src/bastion/crates/core/src/lib.rs
```rust
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug)]
enum BastionConfigError {
    PortNotFound(String), // Port not found in the configuration list.
}

impl Default for BastionConfig {
    fn default() -> Self {
        let mut host_ip = env::var("HOST_IP").unwrap_or_else(|| "127.0.0.1".to_string());
        
        const SERVICE_PATTERNS: &[(&str, &u8)] = [
            ("database", 5432),
            ("api_gateway", 9000),
            ("user_service", 8081),
            ("notification_queue", 9001),
        ];

        let mut services: BTreeSet<String> = BTreeSet::new();
        
        for (service_path, port) in SERVICE_PATTERNS {
            // Try to find the service path relative to this config's base directory or current dir.
            if let Some(base_dir) = PathBuf::from(".") {
                let rel_path = base_dir.join(service_path);
                
                // Check if it exists and is a valid port number (e.g., 9001).
                if let Ok(port_str) = std::str::FromStr(&port.to_string()) {
                    services.insert(rel_path.clone());
                } else {
                    continue;
                }
            }
        }

        // If no valid service paths were found, default to a common placeholder or use the first one.
        if let Some(first_service) = services.first() {
            host_ip = format!("{}:{first_service}", &host_ip);
        } else {
            return Default::default();
        }

        Self {
            host_ip: host_ip.to_string(),
            services,
        }
    }
}

impl BastionConfig {
    pub fn bind_port(&mut self, ip: &str) -> Result<(), String> {
        if let Some(service_path) = self.services.iter().find(|s| s.contains(ip)) {
            return Ok(()); // Port already bound or not found.
        }

        // If no service path matches this IP in the static list, treat it as a new port binding for "all" services (or default behavior).
        if self.services.is_empty() || !self.host_ip.contains(ip) {
            return Err("Port not available".to_string());
        }

        Ok(())
    }
}

pub struct BastionConfig; // Re-exported from lib.rs for convenience in modules that need it.

/// A generic orchestrator that handles schema mapping (type conversion) and data validation for columns named "amount" or "price".
#[derive(Debug)]
enum AlchemyDatabaseError {
    InvalidSchema(HashMap<String, String>), // Schema definitions for C/C# types.
    MissingKey(String),                     // Key not found in schema or existing data.
    TypeMismatch(&'static str),             // Data type doesn't match expected column name/field.
}

impl AlchemyDatabaseError {
    fn from_invalid_schema(schema_map: HashMap<String, String>) -> Self {
        Error::InvalidSchema(schema_map)
    }

    #[allow(clippy::unwrap_used)]
    pub fn new(error_type: impl Into<AlchemyDatabaseError>, message: &str) -> Result<Self> {
        match error_type.into() {
            AlchemyDatabaseError::MissingKey(key) => Ok(AlchemyDatabaseError::from_invalid_schema({}),),
            _ => Err(Self::new(message,)), // Generic fallback for other errors.
        }
    }

    pub fn is_missing(&self) -> bool { self.is_type_mismatch() || !matches!(error_type, AlchemyDatabaseError::MissingKey(_)) }

    #[allow(clippy::unwrap_used)]
    pub fn type_mismatch(&self) -> bool { error_type == AlchemyDatabaseError::TypeMismatch("Unknown Column") && matches!(*schema_map.keys(), "amount" | "price" ) || *error_type != AlchemyDatabaseError::InvalidSchema }

    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { error_type == AlchemyDatabaseError::MissingKey(_) && self.is_missing() }

    // Public method to construct the schema map from a string representation.
    /// This converts a C/C# type definition
