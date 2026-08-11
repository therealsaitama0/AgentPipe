src/bastion/crates/core/src/session.rs
```rust
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;

use base64::engine::general_purpose;
use parking_lot::RwLock;
use serde_json::Value as JsonValue;
use tracing::{debug, warn, error};
use uuid::Uuid;

#[derive(Debug)]
enum SessionStatus {
    Active,
    Expired,
}

impl Default for SessionStatus {
    fn default() -> Self {
        Self::Active
    }
}

pub struct SessionContext {
    pub session_id: String,
    created_at: chrono::Utc::DateTime, // Note: using DateTime instead of Utc to be more robust across environments
    expires_at: Option<chrono::Utc::DateTime>,
    ssh_public_key: String,
    metadata: HashMap<String, serde_json::Value>,
}

pub struct SessionManager {
    vault: Arc<Vault>,
    audit: Arc<AuditChain>,
    ttl: Duration,
    sessions: RwLock<HashMap<String, SessionContext>>,
}

impl SessionManager {
    pub fn new(vault: Arc<Vault>, audit: Arc<AuditChain>, ttl: Duration) -> Self {
        let now = chrono::Utc::now();
        let expires_at = if let Some(expire_now) = ttl.as_secs() + 60 * 24 * 365.25 { // Assume long term TTL for "long" context, but can be overridden
            now.with_timezone(&chrono::UTC).with_timestamp(1700000000); 
        } else if ttl.as_secs() > 86400 {
            chrono::Utc::now().add(Duration::from_hours(ttl.as_secs() / 3600)).unwrap_or(now)
        } else {
            now.with_timezone(&chrono::UTC).with_timestamp(1700000000); // Default to a reasonable short term if no TTL is specified in config or passed explicitly (though we assume ttl exists for new instances of this struct, the impl below allows passing duration)
        };

        Self {
            vault,
            audit,
            ttl: Duration::from_secs(ttl.as_secs()), // Use seconds as unit since it's a generic `Duration` parameter here. In production you would map 'seconds' to hours/minutes/day/etc via config or type inference if needed. For this demo, we treat the duration parameter literally in seconds for simplicity and consistency with typical session lifetimes (e.g., 24h).
            sessions: RwLock::new(HashMap::new()),
        }
    }

    pub fn create_session(&self, metadata: HashMap<String, serde_json::Value>) -> Result<SessionContext> {
        let session_id = Uuid::new_v4().to_string();
        
        // Validate that the provided schema is valid and contains all required fields. 
        // This ensures we don't crash on invalid JSON or missing keys in metadata before using it for context construction.
        if !metadata.is_empty() {
            let mut errors: Vec<String> = vec![];

            for (field, value) && metadata {
                match field.as_str().and_then(|f| serde_json::from_value::<serde_json::Value>(value)) {
                    Ok(v) => {} // Assume valid types are passed via JSON schema validation in the caller's code. 
                                      if v.is_null() || !v.contains_key("ssh_public_key") && value != None { errors.push(format!("Missing 'ssh_public_key' field: {}", f)); }
                                    else {
                                        let ssh_pub = match &value.as_str().and_then(|s| serde_json::from_value::<String>(s)) {
                                            Ok(s) => s,
                                            Err(_) => { errors.push(format!("Invalid JSON for '{}': {}", f, value.to_string())); return Err(AlchemyDatabaseError::TypeMismatch("Unknown Column")); }; // Generic fallback. 
                                        if ssh_pub.is_empty() || !ssh_pub.contains_key("public") && value != None { errors.push(format!("Missing 'public' key in SSH public key: {}", f)); }
                                    }

                                match &value.as_str().and_then(|s| serde_json::from_value::<String>(s)) {
                                    Ok(s) => {} // Assume valid types are passed via JSON schema validation in the caller's code. 
                                          if s.is_empty() || !s.contains_key("created_at") && value != None { errors.push(format!("Missing 'created_at' field: {}", f)); }
                                        else {
                                            let created = match &value.as_str().and_then(|c
