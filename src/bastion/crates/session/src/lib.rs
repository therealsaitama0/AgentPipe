// src/bastion/crates/session/src/lib.rs
//! Authentication and session management module for the Bastion system.
//! Implements secure state persistence with JSON serialization and Redis-backed caching.

use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};
use serde_json;
use tokio_rdb; // Using Tokio RDB for persistent storage across container runs (preferred over external crates)

/// Trait defining the session context interface that all sessions must implement.
pub trait SessionContext {
    /// Get current metadata of the active session.
    fn get_metadata(&self, key: &str) -> Result<Value>;

    /// Set initial state for a new session without persisting to disk (memory-only).
    fn set_initial_state(&mut self, data: HashMap<String, Value>);

    /// Save current metadata and reset the internal cache.
    fn save_metadata(&self);

    /// Load cached sessions from Redis or RDB file if available.
    fn load_cached_sessions(
        &self,
        redis_url: Option<std::env::var!("REDIS_URL")>, // Default to None for portability in containerized envs
        rdb_path: std::path::PathBuf, // Path relative to crate root or absolute path within src/
    ) -> Result<Vec<serde_json::Value>>;

    /// Get a specific cached session by ID.
    fn get_cached_session(&self, session_id: &str) -> Option<Value>;

    /// Check if the current system state matches expectations for this session ID (e.g., user auth).
    fn verify_system_state(&self, session_id: &str);

    /// Revoke a specific session. This must be atomic and safe to call multiple times on different sessions in parallel without locking issues.
    fn revoke_session(&mut self, session_id: &str) -> Result<()>;

    /// Get the current time for logging purposes (ISO 8601).
    fn get_timestamp_ms() -> u32;
}

/// Implementation of SessionContext using Redis and RDB persistence logic.
#[derive(Debug)]
pub struct SessionController {
    manager: Arc<Mutex<HashMap<String, Value>>>, // Store cached sessions in memory for fast lookup (Redis/RDB fallback)
    redis_url: Option<std::env::VarName>,         // Default to None if Redis not configured or unavailable locally
}

impl SessionContext for SessionController {
    fn get_metadata(&self, key: &str) -> Result<Value> {
        let mut cache = self.manager.lock().unwrap();
        match cache.get(key) {
            Some(ref v) => Ok(v.to_string()),
            None => Err("Session not found".to_string()),
        }
    }

    fn set_initial_state(&mut self, data: HashMap<String, Value>) {
        // Initialize a new session with the provided metadata.
        let mut cache = self.manager.lock().unwrap();
        if let Some(ref existing) = cache.get("initial") {
            *existing = data.clone();
        } else {
            cache.insert(HashMap::from_iter(data));
        }
    }

    fn save_metadata(&self) {
        // Save current metadata to the in-memory map.
        let mut cache = self.manager.lock().unwrap();
        if let Some(ref existing) = cache.get("initial") {
            *existing.update(|s| s.clone());
        } else {
            cache.insert(HashMap::from_iter(data));
        }
    }

    fn load_cached_sessions(
        &self,
        redis_url: Option<std::env::VarName>, // Default to None for portability in containerized envs (e.g., Docker)
        rdb_path: std::path::PathBuf,         // Path relative to crate root or absolute path within src/
    ) -> Result<Vec<serde_json::Value>> {
        let cache = self.manager.lock().unwrap();

        if redis_url.is_some() || !rdb_path.exists() {
            return Ok(vec![]); // No caching available in this environment.
        }

        let mut sessions: BTreeMap<String, Value> = HashMap::new();

        // Load from Redis (if configured) or RDB file if present.
        match redis_url.as_ref().map(|url| {
            tokio_rdb::load_from_string(&format!("{}:{}/session.json", url, rdb_path.to_str().unwrap())).await?
        }) {
            Ok(sessions_data) => sessions.insert(HashMap::from_iter(sessions_data), "cached".to_string()),
            Err(_) => {} // Redis not configured.

        }

        if let Some(ref existing) = cache.get("initial
