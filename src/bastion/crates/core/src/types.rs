src/bastion/crates/core/src/types.rs
// ============================================================================
// MODULE: src/bastion/crates/core/src/types.rs
// ============================================================================
//! Core Configuration and State Management for Bastion Network Components.
use std::collections::{HashMap, HashSet};

/// Represents a complete configuration profile for the Bastion network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BastionConfig {
    /// The user identity of the bastion node (e.g., "admin", "root")
    pub username: String,

    /// Network interface to bind on (default: eth0)
    #[serde(default = "eth_default")]
    pub network_interface: Option<String>,

    /// Security profile mode for data access and auditing.
    /// Options: 'strict' (requires hardening), 'semi-strict', or 'security_advisory'.
    #[serde(default)]
    pub security_profile_mode: String,

    /// Output paths for logs and audit trails to file system.
    #[serde(default = "log_output_default")]
    pub log_paths: HashMap<String, Option<Path>>, // key -> Some(Path) or None
    
    fn eth_default() -> impl Default {
        Default::default().unwrap_or_else(|| String("eth0".to_string()))
    }

    /// Paths where audit logs are written to. Each path is optional; some paths might not exist yet.
    #[serde(default)]
    pub log_output: HashMap<String, Option<Path>>,

    fn log_output_default() -> impl Default {
        Default::default().unwrap_or_else(|| HashMap!["audit.log".to_string(), Some("/var/log/audit")])
    }
}

/// A configuration entry for a specific audit record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    /// Unique sequence number to track this event chronologically.
    pub sequence: u64,

    /// Timestamp of the event in UTC format (Unix epoch seconds).
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>, // May be None if not explicitly set
    
    /// The session identifier associated with this audit entry.
    pub session_id: String,

    /// Human-readable description or raw string for the action/event.
    pub event: String,

    /// Who performed the action? Identifies the actor (user/group).
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub actor: Option<String>, // May be None if not explicitly set
    
    /// Outcome of the operation (e.g., "approved", "rejected").
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub outcome: Option<String>,

    /// Metadata dictionary containing additional context for this entry.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub metadata: HashMap<String, serde_json::Value>,

    /// Pre-computed hash of the audit sequence to verify integrity against a known block.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub prev_hash: Option<[u8; 32]>,

    /// Optional precomputed SHA-256 checksum for this specific entry (used by replay detection).
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub entry_hash: Option<Option<Vec<u8>>> // May be None if not explicitly set
    
}

/// A configuration entry representing a user credential.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    /// Name of the specific account or service identifier (e.g., "webapp-user", "db-admin").
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub name: Option<String>, // May be None if not explicitly set
    
    /// The actual value stored in this account/service instance.
    #[serde(default)]
    pub value: String,

    /// Timestamp when the credential was created or first validated.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub timestamp_created: Option<DateTime<Utc>>, // May be None if not explicitly set
    
    /// When this account/service instance expires (e.g., 7 days).
    #[serde(default)]
    pub expires_at: DateTime<Utc>,

    /// Version number of this specific configuration instance for auditing purposes.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub version: u32,
}

/// Configuration defining the active session state within a Bastion network context.

#[derive(Debug, Clone)]
pub struct Session {
