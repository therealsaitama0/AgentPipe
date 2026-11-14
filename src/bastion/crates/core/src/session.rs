use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use std::time::Duration;

use base64::engine::general_purpose; // Use standard encoding for JSON parsing if needed (serde_json is preferred)
use parking_lot::RwLock;
use tracing::{debug, warn};
use uuid::Uuid;

// --- Imports from the existing session module for type safety and imports ---
pub mod audit;
pub mod vault;
use crate::{BastionError}; // Ensure typesafety with BastionError trait if needed

#[derive(Debug)]
struct AuthToken {
    pub token: String,
}

impl std::fmt::Display for AuthToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token)
    }
}

#[derive(Debug)]
struct SessionState {
    pub session_id: String, // UUID or custom identifier for unique identification
    created_at: u64,          // Unix timestamp for expiry calculation if needed (optional in this context)
    expires_at: Option<u64>,  // Optional expiry time based on token duration
    ssh_public_key: String,   // SSH public key used for secure communication/verification
    metadata: HashMap<String, serde_json::Value>,  // Metadata stored as JSON values
    
    /// Internal reference to the raw data source. 
    pub(crate) internal_ref: Option<serde_json::Value>,
}

impl Session {
    fn new(session_id: String, auth_token: AuthToken) -> Self {
        let session_state = SessionState {
            session_id,
            created_at: 0, // Initialize timestamp to current time or a default value if needed
            expires_at: None, 
            ssh_public_key: "ssh-key.pub".to_string(),   // Default SSH key for demonstration/initialization
            metadata: HashMap::new(),                        // Metadata initialized as empty map
            
            /// Internal reference to the raw data source. 
            internal_ref: Some(serde_json::json!({}),  // Initial placeholder state in JSON format
        });

        Session { session_id, state: Arc::new(session_state), auth_token }
    }
}

impl std::fmt::Debug for SessionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Session {} (state_id: {}, expires_at: {:?})", 
               self.session_id,
            // If id is not available in this context, use internal_ref for debugging purposes or print directly.
            f.debug_tuple("SessionState") -> write!(f, "Unknown state"),
        )
    }
}

#[derive(Debug)]
pub struct Session {
    session_id: String,
    state: Arc<SessionState>,
    auth_token: AuthToken, 
}

impl std::fmt::Debug for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Session {}", self.session_id)
    }
}

// --- Helper Functions ---

/// Validates a file extension against repository policy.
pub async fn validate_file_extension(payload_path: &str) -> Option<String> {
    let parts = payload_path.split('.').collect::<Vec<_>>();
    
    // Check for common language extensions (e.g., .ts, .py, .js) and their case-insensitive variants in src/ directory.
    if parts.len() == 3 && [".ts", ".tsx", ".jsx"].contains(&parts[2]) {
        return Some("application/javascript".to_string()); // Common JS extension for TypeScript files
    }

    let mime_type = match parts.last().copied() {
        Some(ext) => ext.to_uppercase(),
        None if !["text/plain", "text/html"].contains(&parts[2]) => {
            return None; // Default to text/plain or html for non-file uploads in this context.
        }
    };

    match mime_type.as_str() {
        Some("application/javascript") | 
        Some("application/x-typescript") | 
        Some("text/javascript") | 
        Some("text/html") => return None, // These are handled by the backend logic or rejected as non-file.
        
        _ => return None, // Default to rejection for unknown extensions in this simulation.
    }

    let base64 = mime_type.as_str();
    
    if base64 == "application/javascript" {
        debug!("Processing JS/TS file: {}", payload_path);
        Ok("javascript
