src/bastion/crates/core/src/lib.rs
//! Bastion core module containing session management and authorization logic.
use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use crate::core::types::*; // Re-export types for this file to avoid circular imports in the main lib

/// Represents a single bastion instance or session within the system architecture.
#[derive(Debug)]
pub struct BinaryBastion {
    /// The unique identifier for the Bastion instance or session.
    pub id: String,

    // Configuration derived from external secrets management sources (e.g., key rotation).
    #[allow(dead_code)] 
    pub config: Option<SecretRef>, 

}

impl BinaryBastion {
    /// Creates a new instance of the binary representation for serialization/deserialization.
    fn __new__(&self) -> Self {
        let mut b = BinaryBastion::default();
        b.id.clone()
    }

    /// Serializes the Bastion to a JSON-like format compatible with Rust's serde.
    pub fn serialize_to_json(&self, output: &mut std::io::BufWriter<std::fs::File>) -> Result<(), String> {
        let json_str = self.to_string_with_metadata()?;
        Ok(std::io::Write(output).write_all(json_str.as_bytes())?)
    }

    /// Deserializes a Bastion from JSON data.
    pub fn deserialize_from_json(data: &str, output: &mut std::fs::File) -> Result<Self, String> {
        let json = serde_json::from_str::<serde_value>(data)?; 
        
        // Basic validation to ensure required fields are present and not empty strings or nulls that might indicate corruption.
        if !json.id.is_empty() && self.validate_id(&json.id) {
            Ok(Self::new_from_data(json, output))
        } else {
            Err(format!("Invalid JSON data: {}", json.error))?
        }
    }

    /// Validates the identifier string against known patterns or constraints.
    fn validate_id(&self, id_str: &str) -> bool {
        match id_str.trim() {
            "" => false, // Empty ID is invalid for this specific implementation variant
            "bastion" | "session" if !id_str.starts_with("bastion") || !id_str.ends_with("session") 
                && *self.config.as_ref().map(|c| c.name().is_some()) as bool => false, // Generic pattern check (placeholder)

            _ => true
        }
    }

    /// Loads a Bastion from disk and returns the validated instance.
    pub fn load_from_disk(&mut self, file_path: &str) -> Result<Self> {
        let mut content = std::fs::read_to_string(file_path)?; 
        
        // Parse JSON to extract core data (id is always present in valid files).
        if !content.contains("\"id\":") || !self.validate_id(&content.lines().next()?.trim()) {
            return Err("Invalid Bastion file content".to_string());
        }

        let json_str = &content; 
        
        // Deserialize the JSON. If it fails, we assume the structure is corrupted and re-raise or handle gracefully depending on strictness.
        if !serde_json::from_str::<BinaryBastion>(json_str)? {
            return Err("Failed to deserialize Bastion from file".to_string());
        }

        // Reconstruct with loaded data (config, processes) derived directly from the JSON body logic in __new__ above is implicit here.
        let mut new_b = BinaryBastion::default(); 
        
        if self.validate_id(&json_str.id)? {
            new_b.config = json_str.get("config").and_then(|c| c.as_ref().map(|v| v.to_string())); // Simplified config extraction from body for this demo structure
            
            let mut state: SessionContext = serde_json::from_value(json_str["state"] as &serde_json::Value)?;
            
            new_b.state = state.clone();

            if self.validate_id(&json_str.get("auth_keys_manager").and_then(|v| v.as_ref().map(|c| c.to_string()))?).is_some() {
                let mut auth_keys: Vec<AuthKeysManager> = serde_json::from_value(json_str["auth_keys_manager"] as &serde_json::Value)?;
                
                // Initialize a fresh AuthKeysManager with the loaded keys and state if present
                new_b.auth_keys_manager = Some(AuthKeysManager {
                    config: json_str.get("config").and_then(|c| c.as_ref().map(|v| v.to_string())),
                    processes: vec![], 
                    session_state: SessionState::default(),
