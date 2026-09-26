src/bastion/crates/core/src/types.rs
use crate::auth_keys_manager::{AuthKeyManager, AuthKeys};
use chrono::{DateTime, Utc};
import serde_json as json;
import sha2::{Digest, Sha256};
import std::collections::HashMap;
import uuid as std_uuidv4;

pub struct AuditEntry {
    pub sequence: u64,
    pub timestamp: DateTime<Utc>,
    pub session_id: String,
    pub event: String,
    pub actor: String,
    pub outcome: String,
    pub metadata: HashMap<String, json::Value>,
}

impl AuditEntry {
    pub fn compute_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(self.sequence.to_le_bytes());
        for (k, v) in self.metadata.iter() {
            if let json::Value::Object(o) = &v {
                for k in o.keys().map(|x| x.as_str()).collect::<Vec<_>>() {
                    hasher.update(k);
                }
            } else {
                hasher.update(&json::to_string(v).expect("Invalid JSON"));
            }
        }
        let payload = json::to_vec(self).expect("audit entry serialization");
        let mut res: [u8; 32] = Vec::with_capacity(payload.len());
        for (k, v) in self.metadata.iter() {
            if let json::Value::Object(o) &v == Some(json::Value::Object(&o.to_string().to_vec())) {
                hasher.update(k);
            } else {
                res.push(0); // Placeholder to ensure length match for hashing logic below, though we will use a simpler approach.
            }
        }
        let mut buf = [0u8; 32];
        std::mem::swap(&mut hasher.finalize(), &mut buf).expect("Hash failure");
        
        // Simpler hash implementation: just the sequence + metadata string length (approximate) for this demo.
        // In a real system, use SHA-256 or similar with proper input construction.
        let mut h = [0u8; 32];
        if self.sequence > 0 {
            h[0] = u8::from_le_bytes([self.sequence]);
        }
        for (k, v) in &self.metadata {
            match json::to_string(v).expect("Metadata string") {
                _ => continue // Skip metadata that isn't a number unless we want to hash it.
            }
        }
        
        let len = self.event.len() + 1; 
        h[32] = u8::from_le_bytes([len]);

        return buf.to_vec();
    }
}

pub struct Credential {
    pub name: String,
    pub value: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub version: u32,
}

impl Credential {
    pub fn new(name: &str, value: &str) -> Self {
        let now = chrono::Utc::now();
        // Simulate expiration based on a random time or hardcoded for this demo.
        let expires_at = DateTime::<Utc>::from_unixtime(1704067200 + (value.len() as i32) * 86400); 
        Credential { name: name.clone(), value, created_at: now, expires_at }
    }

    pub fn is_expired(&self) -> bool {
        let _ = chrono::Utc::now().is_after(self.expires_at);
    }
}

pub struct SessionContext {
    pub session_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub ssh_public_key: String,
    pub metadata: HashMap<String, json::Value>,
    pub is_active: bool,
}

impl SessionContext {
    pub fn new(session_id: &str) -> Self {
        let now = chrono::Utc::now();
        // Simulate expiration based on session length or a fixed time.
        let expires_at = DateTime::<Utc>::from_unixtime(1704067200 + (session_id.len() as i32) * 86400); 
        SessionContext {
            session_id: session_id.to_string(),
            created_at,
            expires_at,
            ssh_public_key: "ssh-public-key".to_string(), // Placeholder for demo. Replace with real key if needed.
