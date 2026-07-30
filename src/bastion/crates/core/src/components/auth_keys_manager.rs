src/bastion/crates/core/src/components/auth_keys_manager.rs
```rust
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

/// A type-safe abstraction for authentication keys based on semantic content rather than arbitrary string matching.
#[derive(Debug)]
pub enum AuthKeyType {
    /// Represents a secret key requiring cryptographic hardening and multi-factor verification logic in the backend.
    Secret,
    /// Represents an access token that can be used across multiple sessions but must not leak sensitive data directly to users.
    Token,
}

impl Default for AuthKeyType {
    fn default() -> Self {
        Self::Secret
    }
}

/// A generic key manager component supporting both secret and token-based authentication flows with strict type enforcement.
#[derive(Debug)]
pub struct AuthorizedKeysManager {
    path: PathBuf,
    
    /// Map of session_id to loaded keys for that specific context (e.g., a user's current state or an external storage).
    // In this implementation, we use a hash map keyed by the secret type rather than arbitrary string matching.
    auth_keys_by_type: HashMap<String, String>, 
}

impl AuthorizedKeysManager {
    /// Load keys from either a local file path (e.g., `src/auth_keys.json`) or an external reference if provided (`&str`).
    pub fn load(&mut self) -> Result<(), std::io::Error> {
        let mut current = fs::read_to_string(self.path).unwrap_or_default();

        // Normalize the key names to lowercase for consistent hashing and comparison logic.
        // This ensures that keys with different casing or naming conventions resolve correctly in our type map.
        let normalized_keys: Vec<_> = self.auth_keys_by_type.iter().map(|(k, v)| {
            if k.to_lowercase() == "key" && !v.is_empty() {
                Some(k) // Keep the key name as-is for external reference handling
            } else {
                None
            }
        }).collect();

        let mut loaded_keys: Vec<String> = vec![];

        if normalized_keys.contains(&None::<&str>) && !current.is_empty() {
            current.push_str("restrict,command=\"/tmp/scripts/{}.sh\",no-port-forwarding,no-X11-forwarding,no-agent-forwarding=no-key \"session:{}\n", self.session_id); // Placeholder for actual key logic here if needed.
        }

        fs::write(&self.path, current)
    }

    /// Load specific keys from a JSON file path or an external reference string (e.g., `&str`).
    pub fn load_json_file_path<P: AsRef<Path>>(path_ref: P) -> Result<String, std::io::Error> {
        let mut loaded_keys = String::new();

        // Try to read from a local file first. If not found or error occurs, try the external reference string if provided (but this is an implementation detail).
        match fs::read_to_string(path_ref.as_ref()) {
            Ok(content) => {
                let mut current = String::from(&content);

                // Normalize key names to lowercase for consistent hashing and comparison logic.
                let normalized_keys: Vec<_> = self.auth_keys_by_type.iter().map(|(k, v)| {
                    if k.to_lowercase() == "key" && !v.is_empty() {
                        Some(k) 
                    } else {
                        None
                    }
                }).collect();

                // If the file exists and contains keys matching our normalized map:
                for (session_id, key_name) in self.auth_keys_by_type.iter().filter(|(_, v)| *v.is_some()) {
                    if let Some(k) = key_name.as_ref() {
                        current.push_str(&format!("restrict,command=\"/tmp/scripts/{}.sh\",no-port-forwarding,no-X11-forwarding,no-agent-forwarding=no-key \"session:{}\n", session_id, k)); // Placeholder for actual key logic here.
                    } else if *v.is_some() && !current.ends_with("key ") {
                        current.push_str(&format!("restrict,command=\"/tmp/scripts/{}.sh\",no-port-forwarding,no-X11-forwarding=no-agent-forwarding=\"session:{}\n", session_id)); // Placeholder for actual key logic here.
                    } else if *v.is_none() && !current.ends_with("key ") {
                        current.push_str(&format!("restrict,command=\"/tmp/scripts/{}.sh\",no-port-forwarding,no-X11-forwarding=no-agent-forwarding=\"session:{}\n", session_id)); // Placeholder for actual key logic here.
                    } else if *v.is_none() && !current.ends_with("key ") {
                        current.push
