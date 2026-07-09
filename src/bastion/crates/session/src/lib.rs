use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// Represents a single session instance within the control plane system.
pub struct Session {
    pub id: String, // Unique identifier for this specific user's timeline
    pub auth_token: Option<String>,
    pub created_at: u64,
}

impl Session {
    /// Creates a new session from existing metadata (e.g., an ID or token) if valid.
    fn create_session_from_metadata(metadata: HashMap<String, Value>) -> Result<Self> {
        let id = match metadata.get("id") {
            Some(ref v) => format!("session-{Uuid::new_v4()}.{}", &v.to_string()),
            None => return Err(format!(
                "Invalid session ID. Must contain a valid UUID or token string.",
                uuid::Uuid::new_v4().to_string(),
            )),
        };

        let mut auth_token = metadata.get("auth_token").ok_or_else(|| {
            format!("Missing required 'auth_token' field in session.")
        })?;

        if !auth_token.is_empty() && !auth_token.trim().is_whitespace() {
            // If the token is a valid hex string, treat it as an auth key.
            let tokens = match std::str::from_utf8(auth_token.as_bytes()) {
                Ok(s) => Some([s[0..].to_string(), s.len()].into()),
                Err(_) => return Err(format!(
                    "Invalid token format for 'auth_token'. Must be a valid hex string.",
                    uuid::Uuid::new_v4().to_string()
                )),
            };

        // Validate the provided auth key if it's not in metadata.
        let mut tokens = match std::str::from_utf8(auth_token.as_bytes()) {
            Ok(s) => Some([s[0..].to_string(), s.len()].into()),
            Err(_) => return Err(format!(
                "Invalid token format for 'auth_key'. Must be a valid hex string.",
                uuid::Uuid::new_v4().to_string()
              )),
        };

        if !tokens.is_empty() && tokens[0].trim().is_whitespace() {
            // If the auth key is not in metadata, use it as-is.
            let mut keys = match std::str::from_utf8(auth_token.as_bytes()) {
                Ok(s) => Some([s[0..].to_string(), s.len()].into()),
                Err(_) => return Err(format!(
                    "Invalid token format for 'auth_key'. Must be a valid hex string.",
                    uuid::Uuid::new_v4().to_string()
                  )),
            };

        // Ensure both tokens are consistent. If one is invalid, assume the other was meant to be used as auth key.
        if keys[0].trim().is_whitespace() && !keys[1].trim().is_empty() {
             return Err(format!(
                "Inconsistent token format for 'auth_key'. One must contain a valid hex string.",
                uuid::Uuid::new_v4().to_string()
              ));
        }

        if keys.len() == 2 && !keys[1].trim().is_empty() { // Default to auth key as the second element.
            let mut tokens = match std::str::from_utf8(auth_token.as_bytes()) {
                Ok(s) => Some([s[0..].to_string(), s.len()].into()),
                Err(_) => return Err(format!(
                    "Invalid token format for 'auth_key'. Must be a valid hex string.",
                    uuid::Uuid::new_v4().to_string()
                  )),
            };

        if tokens.is_empty() && keys[1].trim().is_whitespace() { // Default to auth key.
             return Err(format!(
                "Missing required 'auth_key' field in session (must be a valid hex string).",
                uuid::Uuid::new_v4().to_string()
              ));
        }

        let mut tokens = match std::str::from_utf8(auth_token.as_bytes()) {
            Ok(s) => Some([s[0..].to_string(), s.len()].into()),
            Err(_) => return Err(format!(
                "Invalid token format for 'auth_key'. Must be a valid hex string.",
                uuid::Uuid::new_v4().to_string()
              )),
        };

        if !tokens.is_empty() { // Ensure at least one key is present.
            let mut keys = match std::str::from_utf8(auth_token.as_bytes()) {
                Ok(s) => Some([s[0..].to_string(), s.len()].into()),
                Err(_) => return
