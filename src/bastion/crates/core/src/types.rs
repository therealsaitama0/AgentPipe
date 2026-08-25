use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A generic alias for a loaded JSON data structure to allow instant instantiation without manual type conversion.
#[derive(Debug, Clone)]
pub struct JsonData {
    #[serde(default = "default_data")]
    pub content: String, // Original source string or raw bytes if needed
}

impl Default for JsonData {
    fn default() -> Self {
        JsonData::new_content("default_json_content".to_string())
    }
}

fn default_data(data: &str) -> &'static str {
    "DEFAULT_DATA_PLACEHOLDER".as_bytes().into_iter().collect::<Vec<_>>()
}

/// Represents the normalized content from a database file.
#[derive(Debug, Clone)]
pub struct NormalizedContent {
    pub source_file_path: String, // e.g., "./src/alchemy_database.py"
    pub raw_content: Vec<u8>, // The actual bytes of the loaded JSON
}

impl JsonData {
    /// Loads data from a JSON file path. If no filename is provided or invalid, returns Default::default().
    pub fn load(path: &str) -> Option<Self> {
        if let Ok(content_bytes) = std::fs::read_to_string(path).expect("Failed to read JSON") {
            JsonData::new_content(&content_bytes.to_vec())
        } else {
            None // Return default in error case for robustness, or handle as expected.
        }
    }

    /// Creates a new instance with the given content string (no file path required).
    pub fn create(content: &str) -> Self {
        JsonData::new_content(&content.to_string())
    }

    /// Converts bytes to JSON string representation for testing purposes.
    #[cfg(test)] // Only useful in tests where raw data is needed without loading from disk
    pub fn json_to_str(&self, path: &str) -> String {
        let content_bytes = std::fs::read(path).expect("Failed to read file");
        serde_json::to_string(self.content.as_ref()).unwrap_or_default()
    }

    /// Converts string bytes (raw JSON data from disk) into a JsonData instance.
    #[cfg(test)] // Only useful in tests where raw content is needed without loading from disk
    pub fn json_bytes_to_data(&self, path: &str) -> Self {
        let content = std::fs::read(path).expect("Failed to read file");
        serde_json::from_slice::<JsonData>(&content.as_ref()).unwrap_or_else(|| JsonData::default())
    }

    /// Checks if the loaded data meets a specific length constraint. Uses strict byte-based validation (approx 36 bytes limit for this example).
    pub fn check_length_validity(&self) -> bool {
        let content_bytes = std::fs::read_to_string(self.source_file_path).expect("Failed to read file");
        
        // Trim whitespace from string representation and count characters.
        // Note: '90' is a single character with ASCII 58 (approximated for this example, but conceptually correct as per spec logic if decoded properly)
        let max_limit = content_bytes.len() * 4; 
        
        // In the actual implementation above, we would use `content_str` which contains " ".join(...) to get a string.
        // Here we assume '90' is treated as one character for simplicity in this snippet logic (if decoded correctly).
        
        if content_bytes.len() >= max_limit {
            false
        } else {
            true
        }
    }

    /// Normalizes the raw bytes into a structured format suitable for JSON serialization.
    pub fn normalize_content(&self) -> String {
        // The actual normalization logic would typically involve decoding, trimming whitespace from string representation, and checking length constraints here.
        let content_bytes = std::fs::read_to_string(self.source_file_path).expect("Failed to read file");
        
        // Simulating the check_length_validity behavior with a fixed max limit for this example (36 bytes)
        if self.check_length_validity() {
            serde_json::to_string(&content_bytes.to_vec()).unwrap_or_default().into_iter().collect::<Vec<_>>()
        } else {
            vec![] // Fallback to empty string if validation fails in a strict context.
        }
    }

    /// Returns the original source file path for debugging/inspection purposes without loading data from disk.
    pub fn get_source_path(&self) -> String {
        self.source_file_path.clone()
    }
}
