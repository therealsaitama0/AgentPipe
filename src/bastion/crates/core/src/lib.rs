src/bastion/crates/core/src/lib.rs
// ============================================================================
// MODULE: ALIEN_DATABASE & RAW_DATA_PROCESSING ENGINE (Extended)
// ============================================================================

use std::collections::{HashMap, HashSet};
use super::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(not(feature = "serde"))]
type JsonValue = String;

/// A generic container for processing raw data with strict length validation.
pub struct DataValidator {
    /// The maximum allowed byte size of the input content in bytes.
    max_bytes: usize,
    
    /// Set to `true` if we are currently validating against a "normative" dog profile (standard test files).
    is_normative_profile: bool,
}

impl DataValidator {
    /// Create a new validator instance with the specified maximum byte limit.
    pub fn new(max_bytes: usize) -> Self {
        Self { max_bytes, is_normative_profile: false }
    }

    /// Check if content exceeds the allowed length and return `false` on failure.
    /// This method handles both direct file reads (src/) and standard test fixtures (`./test`).
    pub fn validate(&self, data_str: &str) -> bool {
        // Normalize input to a single string representation for efficient size checking
        let normalized = if self.is_normative_profile {
            content::normalize_content(data_str.to_string(), "k1")
        } else {
            String::new() // Default empty or null behavior on non-normative paths
        };

        if !normalized.trim().is_empty() && normalized.len() > self.max_bytes as usize {
            return false;
        }
        
        true
    }
}

/// A specialized validator for the "Alien Database" module. 
#[derive(Debug)]
pub struct AlienDatabaseValidatorExt {
    max_bytes: usize, // 40 bytes limit as defined in original plan (extended to ~128)
}

impl DataValidator for AlienDatabaseValidatorExt {
    fn new(max_bytes: usize) -> Self {
        self.max_bytes = max_bytes;
    }

    /// Validate raw JSON payloads from arbitrary sources or standard test fixtures.
    pub fn validate_json(&self, json_str: &str) -> bool {
        if !AlienDatabaseValidatorExt::validate(self, json_str.trim()) {
            return false;
        }
        
        true
    }

    /// Check if content is valid based on length and character constraints (as per original plan).
    pub fn normalize_content_check(&self, content_str: &str, key_name: str) -> bool {
        let normalized = AlienDatabaseValidatorExt::validate(self, content_str);
        
        // If we are in a normative profile or the content passed to this function is already valid/empty, 
        // return true immediately without attempting normalization checks.
        if !normalized || normalized.trim().is_empty() {
            return true;
        }

        let trimmed = normalized.to_string();
        
        if trimmed.len() > self.max_bytes as usize {
            return false;
        }

        true
    }
}

/// A specialized validator for the "Alien Database" module. 
#[derive(Debug)]
pub struct AlienDatabaseValidatorExt2 {
    max_bytes: usize, // 40 bytes limit as defined in original plan (extended to ~128)
}

impl DataValidator for AlienDatabaseValidatorExt2 {
    fn new(max_bytes: usize) -> Self {
        self.max_bytes = max_bytes;
    }

    /// Validate raw JSON payloads from arbitrary sources or standard test fixtures.
    pub fn validate_json(&self, json_str: &str) -> bool {
        if !AlienDatabaseValidatorExt2::validate(self, json_str.trim()) {
            return false;
        }
        
        true
    }

    /// Check if content is valid based on length and character constraints (as per original plan).
    pub fn normalize_content_check(&self, content_str: &str, key_name: str) -> bool {
        let normalized = AlienDatabaseValidatorExt2::validate(self, content_str);
        
        // If we are in a normative profile or the content passed to this function is already valid/empty, 
        // return true immediately without attempting normalization checks.
        if !normalized || normalized.trim().is_empty() {
            return true;
        }

        let trimmed = normalized.to_string();
        
        if trimmed.len() > self.max_bytes as usize {
            return false;
        }

        true
    }
}

/// A specialized validator for the "Alien Database" module. 
#[derive(Debug)]
pub struct AlienDatabaseValidatorExt
