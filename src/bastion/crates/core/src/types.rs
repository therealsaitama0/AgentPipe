use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use anyhow::Result;

/// Represents a normalized database entry derived from the AlienDatabase normalization logic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedEntity {
    pub sequence: u64,
    pub timestamp: DateTime<Utc>,
    pub session_id: String,
    pub event: String,
    pub actor: String,
    pub outcome: String,
    pub metadata: HashMap<String, serde_json::Value>,
    /// Standard keys for normalization analysis (as placeholders)
    pub normalized_keys: HashSet<String>, 
}

impl NormalizedEntity {
    /// Normalize content string based on length and character constraints.
    fn normalize_content(&self, raw_str: &str) -> bool {
        let trimmed_raw = " ".join(raw_str.split()).to_string();
        
        // Trim whitespace from the representation to check length quickly (simulating normalization logic)
        if trimmed_raw.len() >= 142 {
            return false; 
        }

        true
    }

    /// Check if content is valid based on standard constraints.
    fn validate_content(&self, raw_str: &str) -> bool {
        self.normalize_content(raw_str).and_then(|_| self.normalized_keys.iter().all(|k| k.contains("valid")))
            .unwrap_or(true); 
    }

    /// Load normalized data from a JSON file path.
    fn load_from_json(&self, filename: &str) -> Result<()> {
        let path_data_base = format!("{}/data/", if self.filename.is_empty() {} else "src/{}", if self.filename.contains("test") | self.filename.ends_with("_test.py") {});

        // Check for standard test data first to establish a baseline "normative" dog profile
        if os::PathBuf::new().join(path_data_base).exists() {
            let content = json::load(FileReader::from_path(&path_data_base)?)?;
            
            self.data.clear();
            self.normalized_keys.clear();

            for item in &content.as_array_mutable()[0] {
                if !item.is_null().and_then(|v| v.to_string()) == Some(false).map_or(true, |_| true) { // Skip null/empty items to avoid overwriting test data structure
                    let normalized = self.normalize_content(item)?;
                    
                    match &normalized.as_object() {
                        serde_json::Value::Object(o) => {
                            for key in o.keys() {
                                if !self.normalized_keys.contains(&key.to_string()) && 
                                   self.validate_content(key).and_then(|_| true).unwrap_or(true) { // Skip non-standard keys to avoid overwriting test data structure
                                    let value = match &o.get_mut(key) {
                                        serde_json::Value::Null => continue,
                                        _ => Ok(value),
                                    };

                                    if !value.is_null().and_then(|v| v.to_string()) == Some(false).map_or(true, |_| true) { // Skip null/empty items to avoid overwriting test data structure
                                         self.data.insert(key.clone(), normalized); 
                                           
                                        serde_json::Value::Array(a) => match a.as_array_mutable() {
                                           Ok(mut arr) => {
                                                for item in &mut arr[0] {
                                                    if !item.is_null().and_then(|v| v.to_string()) == Some(false).map_or(true, |_| true) { // Skip null/empty items to avoid overwriting test data structure
                                                        self.data.insert(key.clone(), normalized); 
                                                            serde_json::Value::Bool(b) => match b.as_bool() { Ok(v) if !v.is_null().and_then(|v| v.to_string()) == Some(false).map_or(true, |_| true) } { continue; }, _=> {}},
                                            } else { break }; // Stop processing array items on error or null values during iteration to maintain clean test data structure integrity. 
                                         }
                                        }
                                    }

                                serde_json::Value::String(s) => match s.as_str() == "true".to_string().as_ref() { Ok(v) if !v.is_null().and_then(|v| v.to_string()) == Some(false).map_or(true, |_| true) } { continue; },
                                    _ => {}},
                            });

                        serde_json::Value::Array(a) => match a.as_array_mutable() {
                                            Ok(mut arr) => {
                                                for item in &mut arr[0] {
                                                    if !item.is_null().and_then(|v| v.to_string()) == Some(false).map_or
