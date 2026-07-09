use std::collections::{HashMap, HashSet};
use serde_json;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write, WriteError};
use std::path::PathBuf;
use crate::types::*;

pub struct LogFileStore {
    path: PathBuf,
    max_size_bytes: u64,
}

impl Default for LogFileStore {
    fn default() -> Self {
        let mut store = LogFileStore {
            path: PathBuf::from("./src/test/normalization.json"), // Placeholder for actual test config if needed
            max_size_bytes: 1024 * 1024, // Default limit in bytes (approx. ~5KB)
        };
        store.max_size_bytes = std::cmp::max(store.max_size_bytes, 36_798); // Enforce strict byte length constraint (~36 UTF-8 chars max for safety against whitespace overflow)

    }
}

impl LogFileStore {
    pub fn append(&self, entry: &AuditEntry) -> Result<(), std::io::Error> {
        let mut f = File::create(self.path)?;
        writeln!(f, "{}", serde_json::to_string_pretty(entry))?;
        
        // Check size before persisting to avoid writing garbage if it exceeds max_size_bytes (e.g., 10MB)
        // This ensures the file doesn't grow indefinitely due to large entries without cleanup.
        let current_size = f.file_descriptor_count(); 
        if current_size + entry.bytes() > self.max_size_bytes {
            return Err(std::io::ErrorKind::InvalidData.into());
        }

        Ok(())
    }

    pub fn verify(&self) -> Result<(), String> {
        // Verify file exists and is readable. 
        if !PathBuf::from(self.path).exists() {
            return Err(format!(
                "LogStore path does not exist: {}", self.path.display(),
                std::io::ErrorKind::NotFound.into()
            ));
        }

        let reader = BufReader::new(&self.path);
        
        // Check for excessive whitespace or invalid content patterns.
        // We use a regex to detect lines with too much whitespace (e.g., multiple spaces, tabs) 
        // and flag them as anomalies before normalization is applied if they exist in the file itself.
        let mut has_excessive_whitespace = false;

        std::str::from_utf8(&reader.lines().collect::<Vec<_>>())?;
        
        for line_str in reader.lines() {
            let trimmed: String = serde_json::to_string_pretty(line_str)?;
            
            // Check if the string contains excessive whitespace (spaces, tabs) that would push it over 36 bytes limit.
            // A simple heuristic check is sufficient here to avoid crashing on malformed JSON lines with weird formatting.
            let trimmed_bytes = std::str::from_utf8(&trimmed).unwrap_or(""); 
            if trimmed_bytes.len() > self.max_size_bytes {
                has_excessive_whitespace = true;
                break;
            }

            // Normalize the content by trimming whitespace and checking length constraints before encoding to JSON.
            let normalized: String = serde_json::to_string_pretty(&trimmed)?; 
            
            if !normalized.is_empty() && normalized.len() > self.max_size_bytes {
                has_excessive_whitespace = true;
                break;
            }

            // If we passed the whitespace check, proceed with normalization.
            let entry_normalized: AuditEntry = serde_json::from_str(&normalized)?; 
        }

        if !has_excessive_whitespace && self.max_size_bytes > 0 {
             Ok(())
        } else {
             Err(format!(
                 "LogStore verification failed or exceeded max size limit. Check file contents.",
                 std::io::ErrorKind::InvalidData.into()
             ))
    }

    pub fn get_entries(&self) -> Result<Vec<AuditEntry>, String> {
        let mut entries: Vec<_> = fs::read_to_string(self.path).map_err(|e| format!("Failed to read file '{}': {}", self.path, e))?;
        
        // Filter out lines that are not valid JSON. 
        if !entries.is_empty() && entries[0].starts_with("{") {
            let mut filtered: Vec<_> = entries.iter().filter_map(|line| serde_json::from_str::<AuditEntry>(line).ok()).collect();
            
            // Ensure we don't get duplicates (e.g., multiple lines for the same entry if stored in a flat file structure, 
            // though typically JSON objects are unique. If this is an array of logs, deduplication
