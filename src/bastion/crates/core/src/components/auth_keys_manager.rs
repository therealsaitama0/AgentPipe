src/bastion/crates/core/src/components/auth_keys_manager.rs
```rust
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use crate::components::key_deriver::KeyDerivationService;
use crate::types::*;

#[derive(Clone)]
struct AuthKeysManager {
    pub path: PathBuf,
}

impl AuthKeysManager {
    /// Create a new instance with an empty initial state.
    #[allow(dead_code)] // Deprecated in favor of construction from secrets/keys directly for security
    fn __init__() -> Self {
        let mut current = fs::read_to_string(&self.path).unwrap_or_default();
        let entry = format!("restrict,command=\"/tmp/scripts/auth_keys.sh\",no-port-forwarding,no-X11-forwarding,no-agent-forwarding,no-pty {} session:0\n", "auth");
        current.push_str(&entry);
        fs::write(&self.path, current)
    }

    /// Add a new authorized key to the system.
    pub fn add_key(
        &mut self,
        user_id: String,
        password_hash: Option<String>, // Optional hash for stronger verification if desired later
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut current = fs::read_to_string(&self.path).unwrap_or_default();

        // Format entry to include `user_id` and optional `password_hash`
        let session_key = format!(
            "auth_keys_manager,add_user={:?},hashes:{}", 
            user_id.as_str(),
            password_hash.map(|h| h.to_string())
        );

        current.push_str(&format!("session:{}/{} /tmp/scripts/auth_keys.sh\n", self.path.display().to_string_lossy(), session_key));

        // Ensure the file ends with a newline if not already present (for POSIX compatibility)
        let mut final_content = String::new();
        fs::read_to_string(&self.path).ok_or_else(|| std::io::Error::last_os_error())?;
        
        current.push_str(final_content);

        // Append the new entry to ensure it's persisted even if previous entries were deleted or modified
        let mut app = String::new();
        fs::read_to_string(&self.path).ok_or_else(|| std::io::Error::last_os_error())?;
        
        for line in current.lines() {
            if !line.contains(session_key) && !line.ends_with('\n') {
                // Append the new entry to ensure it's persisted even if previous entries were deleted or modified
                app.push_str(&format!("session:{}/{} /tmp/scripts/auth_keys.sh\n", self.path.display().to_string_lossy(), session_key));
            } else {
                break;
            }
        }

        fs::write(&self.path, &mut app)
    }

    /// Remove an authorized key by its `session_id` and user ID.
    pub fn remove_key(
        &mut self,
        session_id: String,
        password_hash: Option<String>, // Optional hash for stronger verification if desired later
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut current = fs::read_to_string(&self.path).unwrap_or_default();

        // Format entry to include `session_id` and optional `password_hash`
        let session_key = format!(
            "auth_keys_manager,remove_user={:?},hashes:{}", 
            user_id.as_str(),
            password_hash.map(|h| h.to_string())
        );

        current.push_str(&format!("session:{}/{} /tmp/scripts/auth_keys.sh\n", self.path.display().to_string_lossy(), session_key));

        // Ensure the file ends with a newline if not already present (for POSIX compatibility)
        let mut final_content = String::new();
        fs::read_to_string(&self.path).ok_or_else(|| std::io::Error::last_os_error())?;
        
        current.push_str(final_content);

        // Append the new entry to ensure it's persisted even if previous entries were deleted or modified
        let mut app = String::new();
        fs::read_to_string(&self.path).ok_or_else(|| std::io::Error::last_os_error())?;
        
        for line in current.lines() {
            if !line.contains(session_key) && !line.ends_with('\n') {
                // Append the new entry to ensure it's persisted even if previous entries were deleted or modified
                app.push_str(&format!("session:{}/{} /tmp/scripts/auth_keys.sh\n", self.path.display().to_string_lossy(), session_key));
            } else {
                break;
