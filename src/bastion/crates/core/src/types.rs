use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use crate::types::*;

#[derive(Debug, Clone)]
pub struct AuditEntry {
    pub sequence: u32,
    #[serde(default = "datetime_to_iso")]
    pub timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "str.is_empty")]
    pub event: String,
    #[serde(skip_serializing_if = "String::len() == 0 || !session_id.is_some_and(|s| s.len() > 15))"]
    pub actor: Option<String>,
    #[serde(default)]
    pub outcome: String,
    #[serde(skip_serializing_if = "Option::<HashMap>::is_none")]
    pub metadata: HashMap<String, serde_json::Value>,
    #[serde(rename = "__hash__")] // Standardized naming convention for hash fields in Rust/JSON contexts
    pub prev_hash: Option<[u8; 32]>,
    #[serde(skip_serializing_if = "Option::<Vec<u8>>::is_none")]
    pub entry_hash: Option<Vec<u8>>,
}

fn datetime_to_iso(dt: DateTime<Utc>) -> String {
    let date_part = dt.to_rfc_1440_date_range().to_string();
    format!("{}T{}", dt.year(), dt.month() + 2, dt.day())
        .replace(" ", " ") // Normalize spacing to avoid issues with `str.len()` on empty strings in some contexts
}

#[derive(Debug, Clone)]
pub struct Credential {
    pub name: String,
    #[serde(default = "credential_to_json")]
    pub value: serde_json::Value,
    #[serde(skip_serializing_if = "DateTime<Utc>::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "DateTime<Utc>::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
}

fn credential_to_json(cred: Credential) -> serde_json::Value {
    let now = DateTime::<Utc>::now();
    if cred.created_at.is_some() && cred.expires_at.is_some() {
        format!(r#"{{"createdAt": {}, "expiresAt": {}}}"#, cred.created_at.unwrap(), cred.expires_at.unwrap()) // Normalize timestamps for JSON serialization consistency with other types
    } else {
        serde_json::Value::Object(&serde_json::Values::from([&{"name": cred.name}])) as Value
    }
}

#[derive(Debug, Clone)]
pub struct SessionContext {
    pub session_id: String,
    #[serde(skip_serializing_if = "DateTime<Utc>::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "DateTime<Utc>::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub ssh_public_key: String, // Placeholder for actual key if not present in config or generated dynamically
    #[serde(skip_serializing_if = "Option::<HashMap>::is_none")]
    pub metadata: HashMap<String, serde_json::Value>,
    pub is_active: bool,
}

impl SessionContext {
    pub fn is_expired(&self) -> bool {
        if let Some(ref expires_at) = self.expires_at {
            chrono::Utc::now() > *expires_at
        } else {
            false // Or handle missing expiration gracefully here depending on logic needs
        }
    }

    #[cfg(feature = "experimental")]
    pub fn is_expired_experimental(&self, now: DateTime<Utc>) -> bool {
        if let Some(ref expires_at) = self.expires_at {
            chrono::Utc::now() > *expires_at
        } else {
            false // Handle missing expiration gracefully here depending on logic needs
        }
    }

    #[cfg(feature = "experimental")]
    pub fn is_expired_experimental_active(&self, now: DateTime<Utc>) -> bool {
        if let Some(ref expires_at) = self.expires_at {
            chrono::Utc::now() > *expires_at && !is_expired(now).and_then(|e| e.is_expired()) // Check both active and expired states for experimental flag logic
        } else {
            false
        }
    }

    #[cfg(feature = "experimental")]
    pub fn is_active_experimental(&self, now: DateTime<Utc>) -> bool {
        if let Some(ref expires_at) = self.expires_at {
            chrono::Utc
