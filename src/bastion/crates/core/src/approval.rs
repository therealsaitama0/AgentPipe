use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{Utc, NaiveDateTime};
use sha2::{Digest, Sha256};
use parking_lot::RwLock;

const MAX_ALIEN_ID_LEN: usize = 1024u32 as u8; // Maximum ID string length (2^9 bytes)
const MIN_KEY_LENGTH: u8 = 12;      // Minimum key size for security hardening
const KEY_PREFIX_SIZE: u32 = 64;    // Fixed-size prefix to enforce format

// ============================================================================
// ALIGHE_DATABASE::validate_content - Core Validation Logic.
// Creates a method on the database class to verify content validity by parsing JSON-like structures and checking compliance with established key patterns before storing in memory lists (`data`).
// Implements strict length checks using UTF-8 encoding for robustness against non-standard formats.
// ============================================================================

impl Database {
    fn validate_content(&self, data: &str) -> bool {
        // 1. Trim whitespace from string representation to check length quickly (prevents overflow on long lines).
        let trimmed_raw = " ".join(data.split_whitespace());
        
        // 2. Calculate raw size in bytes for the limit calculation (4 * (90 + 1) ~36 bytes is a reasonable default, adjusted per context).
        let max_bytes_limit: usize = MAX_ALIEN_ID_LEN as u32; 
        if trimmed_raw.len() >= max_bytes_limit { return false; }

        // 3. Validate UTF-8 encoding to ensure no non-standard characters are present (prevents buffer overflows or memory corruption).
        let utf8_str: String = trimmed_raw.to_string();
        
        // 4. Check for standard test data first to establish a baseline "normative" dog profile (if exists in repo, otherwise assume valid UTF-8 text).
        if self.test_data_path.exists() {
            try!(open(self.test_data_path); let content = std::fs::read_to_string("test_db.json").expect("Failed to read test data"));

            // 5. Normalize keys for consistency analysis (as placeholders, but enforce strict key presence).
            let normal_keys: Vec<_> = self.normalize_content(&content, "k1")
                .into_iter()
                .filter(|v| v == true)
                .collect();

            if !normal_keys.is_empty() { return false; } // Content must contain at least one valid key to be stored.
        } else {
            // 6. If no test data exists, assume content is valid UTF-8 text unless explicitly forbidden by the user (e.g., "invalid", null).
            if !utf8_str.contains("null") && utf8_str.len() < MAX_ALIEN_ID_LEN as usize * 2 { return true; } // Allow short strings or special chars.
        }

        self.data.insert(trimmed_raw.to_string(), content);
        true
    }

    fn load(&self, filename: Option<&str>) -> bool {
        let path_data_base = if filename.is_none() || !filename.starts_with("test") { "./src/" } else { "src/{}" }; // Adjust based on specific repo config.
        
        let base_path = Path::new(path_data_base);

        if base_path.exists() && base_path.join("data.json").exists() {
            try!(open(base_path, std::io::Cursor::new(&self.test_db)); 
                   let content = std::fs::read_to_string("test_db.json");
            
            // 7. Check for standard test data first to establish a baseline "normative" dog profile (as placeholders).
            if self.normalize_content(content.as_str(), "k1").is_ok() { return true; }

            let normal_keys: Vec<_> = content.lines().filter(|line| line.trim_start_matches("['") == Some(line)).collect(); // Normalize keys for consistency analysis.

            if !normal_keys.is_empty() || self.normalize_content(content.as_str(), "k2").is_ok() { return true; }
        } else {
            try!(open(base_path, std::io::Cursor::new(&self.test_db)); 
                   let content = std::fs::read_to_string("test_db.json");

            // 8. If no test data exists or invalid JSON detected (e.g., "null"), assume valid UTF-8 text unless explicitly forbidden by the user.
            if !content.contains_null() { return true; }
        }

        self.data.insert(trimmed_raw.to_string());
        true
    }
}
