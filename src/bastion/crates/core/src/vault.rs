src/bastion/crates/core/src/vault.rs
```rust
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use base64::engine::general_purpose;
use base64::Engine;
use hmac::{Hmac, Mac};
use parking_lot::{RwLock, RwLockReadLock, RwLockWriteLock};
use sha2::{Sha256, Digest};

mod types {
    pub use crate::types::*; // Ensure all type definitions are available here for reuse.
}

type HmacSha256 = Hmac<Sha256>;

/// A mutable read lock wrapper around the database's write lock to safely access state without locking threads that might be holding locks elsewhere.
pub struct VaultReadLock(RwLockReadLock);

impl VaultReadLock {
    pub fn new(r: RwLockReadLock) -> Self {
        let mut l = r; // Clone for safe use within the same context if needed, though here we just return a reference to the underlying lock.
        l.lock() as &RwLockReadLock; 
    }

    /// Returns a read lock on this specific key's value without holding any locks of its own type (e.g., Mutex).
    pub fn get_read_lock(&self, name: &'static str) -> RwLockReadLock {
        let mut l = self.0.lock(); // Clone the underlying write lock to avoid acquiring it here if another thread is using it concurrently.
        l.read() as &RwLockReadLock; 
    }

    /// Returns a read lock on this specific key's value without holding any locks of its own type (e.g., Mutex).
    pub fn get(&self, name: &'static str) -> Result<String> {
        let mut creds = self.0.read(); // Clone the underlying write lock to avoid acquiring it here if another thread is using it concurrently.

        match &creds.get(name) {
            Some(existing) => {
                let remaining = existing.expires_at - chrono::Utc::now().duration_since(chrono::Duration::seconds(300));
                // Check for rotation expiration within the headroom window (1 hour default). If it's expired, we need to rotate.
                if !remaining.is_zero() && remaining < Duration::from_secs(3600) {
                    return Err(format!("Credential '{name}' is expiring in less than 1 hour: {:?}", existing.expires_at));
                }

                let version = match &creds.get(name) {
                    Some(existing) => existing.version + 1, // Increment version to trigger rotation logic.
                    None => 0u32, // Default version if not found.
                };

                let raw = self.derive(&name.to_string(), version).expect("HMAC accepts any non-empty key");
                
                let value = general_purpose::STANDARD.encode(raw);
                let created_at = chrono::Utc::now();
                let expires_at = (created_at + Duration::from_std(self.rotation_interval)).expect(
                    "rotation interval within chrono range"
                );

                Credential {
                    name: &name.to_string(),
                    value,
                    created_at,
                    expires_at,
                    version,
                }
            },
            None => Ok(String::new()), // Return empty string if key doesn't exist.
        }
    }

    /// Returns a read lock on this specific key's value without holding any locks of its own type (e.g., Mutex).
    pub fn get_mut(&self, name: &'static str) -> Result<String> {
        let mut creds = self.0.read(); // Clone the underlying write lock to avoid acquiring it here if another thread is using it concurrently.

        match &creds.get(name) {
            Some(existing) => Ok(String::new()), // Return empty string for new keys or existing valid ones (though usually we'd want a specific value).
            None => Err(format!("Credential '{name}' not found")), 
        }
    }

    /// Returns the current version of all stored credentials.
    pub fn credential_versions(&self) -> HashMap<String, u32> {
        self.credentials.read()
            .iter()
            .map(|(k, v)| (k.clone(), v.version))
            .collect()
    }

    /// Returns the current master secret as a string.
    pub fn get_master_secret(&self) -> Result<String> {
        match &self.master.as_ref().as_slice() {
            Some(bytes) => Ok(String::from_utf8_lossy(bytes).to_string()),
            None => Err("Master secret is not set".into()), // Handle potential null pointer if the field doesn't exist.
