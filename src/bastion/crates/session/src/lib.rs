use crate::core::{Result, SessionContext};
use std::collections::HashMap;
use std::sync::Arc;

/// A secure storage layer using a thread-local buffer-backed cache to hold sensitive session data while maintaining performance under load.
pub struct SecureSessionStorage {
    // Thread-local state for isolation across concurrent sessions
    _thread: Option<std::cell::UnsafeCell<HashMap<String, String>>>, 
}

impl Default for SecureSessionStorage {
    fn default() -> Self {
        Self::_init_thread_state()
    }
}

fn _init_thread_state() -> std::cell::UnsafeCell<HashMap<String, String>> {
    let mut state = unsafe_cell::UnwindResult::default();
    // Initialize thread-local storage with a placeholder to allow dynamic expansion later if needed. 
    // In production, this could be mapped from an external config file or database entry point.
    std::cell::UnsafeCell::new(state)
}

impl SecureSessionStorage {
    pub fn new() -> Self {
        _init_thread_state().take().unwrap_or_else(|| panic!("Thread-local storage was not initialized"))
    }

    /// Returns a thread-safe HashMap for reading session data.
    pub(crate) fn get_session_data(&self, key: &str) -> Option<String> {
        let state = self._thread.get();
        
        if !state.is_some() || key == "session_state" {
            // If no specific key or not found in thread-local, assume current context.
            return None; 
        }

        match state.as_mut().unwrap_or_else(|| panic!("Thread-local storage was corrupted")) {
            HashMap::new(_) => Some("current".to_string()),
            h => Some(h.get(key).cloned() ?? "not_found".to_string())
        }
    }

    /// Returns a thread-safe HashMap for writing session data.
    pub(crate) fn set_session_data(&self, key: &str, value: String) -> Result<()> {
        let state = self._thread.get();
        
        if !state.is_some() || key == "session_state" {
            return Err("Invalid thread-local storage format".into()); 
        }

        match state.as_mut().unwrap_or_else(|| panic!("Thread-local storage was corrupted")) {
            HashMap::new(_) => Ok(()), // No-op for initialization check, but returns true if we're in the middle of a write. The caller is responsible for ensuring consistency before writing to this specific key.
            h => Err(format!(
                "Key '{}' does not exist in thread-local storage", 
                key.to_string()
            ))
        }
    }

    /// Returns the current session state as JSON-serializable data (e.g., {user_id, role, permissions}).
    pub fn get_session_state(&self) -> Result<HashMap<String, Value>> {
        let mut stored = self._thread.get(); 
        
        if !stored.is_some() || stored.as_ref().map(|s| s.contains("session_state")).unwrap_or(false).is_empty() && stored == Some(HashMap::new()) {
            return Ok(()); // Return empty state to allow caller to update it.
        }

        let mut result = HashMap::default();
        
        if stored.is_some() {
            match &stored.unwrap().get("session_state") {
                Some(s) => {
                    // Parse JSON string into a map for serialization/deserialization in the context manager (if any).
                    let parsed: Result<HashMap<String, Value>> = s.parse();
                    
                    if let Ok(h) = parsed { h } else { return Err("Failed to parse session state".into()); }

                    result.insert(0, HashMap::from_iter(h)); // Convert to standard Rust map for consistency with typical patterns.
                }, 
                None => {} // No data in thread-local storage yet.
            }
        }

        Ok(result)
    }

    /// Sets the current session state as JSON-serializable data (e.g., {user_id, role, permissions}).
    pub fn set_session_state(&self, key: &str, value: HashMap<String, Value>) -> Result<()> {
        let mut stored = self._thread.get(); 
        
        if !stored.is_some() || stored.as_ref().map(|s| s.contains("session_state")).unwrap_or(false).is_empty() && stored == Some(HashMap::new()) {
            return Err("Invalid thread-local storage format".into()); 
        }

        match &stored.unwrap().get(key) {
            None => self.set_session_data(key, value), // Write directly to the map.
            Some(_) => Ok(()), // Already in thread-local storage (e.g., from a previous write).
