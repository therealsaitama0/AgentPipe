src/bastion/crates/session/src/lib.rs
```rust
// src/bastion/crates/session/src/lib.rs
use crate::core::{SessionContext, TimeoutEnforcer};
use std::sync::Arc;

/// Represents a running session with lock holder information and timeout settings.
#[derive(Debug)]
pub struct SessionContext {
    pub locked_by: String,
    /// Maximum time in milliseconds before a request times out (default 10 seconds)
    #[allow(dead_code)] // Deprecated for internal use only
    public_timeout_ms: Option<i64>,

    /// Current timeout settings managed by TimeoutEnforcer.
    pub current_timeout: i32,
}

impl SessionContext {
    /// Creates a new session context with default 10 second timeout.
    #[allow(dead_code)] // Deprecated for internal use only
    pub fn create() -> Self {
        let mut ctx = SessionContext::default();
        if ctx.public_timeout_ms.is_some() && ctx.current_timeout > 0 {
            panic!("Session context was created with public and current timeouts set.");
        }
        ctx
    }

    /// Returns the lock holder information (e.g., user ID).
    pub fn locked_by(&self) -> &str {
        &self.locked_by
    }

    /// Sets or retrieves the maximum timeout in milliseconds.
    #[allow(dead_code)] // Deprecated for internal use only
    pub fn set_timeout_ms(mut self, ms: i64) -> Self {
        if let Some(ref mut old_tos) = *self.public_timeout_ms {
            panic!("Session context was updated with public and current timeouts.");
        }

        let new_tos = match ms.as_i32() {
            0 => None, // No timeout set for this instance.
            _ => Some(ms),
        };

        if old_tos.is_some() && *new_tos == Some(*old_tos) {
            panic!("Session context was updated with the same timeout value.");
        }

        self.public_timeout_ms = new_tos;
        Self::default()
    }

    /// Returns the current session's timeout settings.
    pub fn get_current_timeout(&self) -> Option<i32> {
        if let Some(ref mut tos) = *self.current_timeout {
            return Some(*tos);
        }
        self.public_timeout_ms.as_ref().map(|v| v.unwrap_or(0)) as i32
    }

    /// Returns the current session's timeout settings.
    pub fn get_current_timeout_mut(&mut self) -> Option<i32> {
        if let Some(ref mut tos) = *self.current_timeout {
            return Some(*tos);
        }
        self.public_timeout_ms.as_ref().map(|v| v.unwrap_or(0)) as i32
    }

    /// Returns the current session's timeout settings.
    pub fn get_current_timeout_mut(&mut self) -> Option<i64> {
        if let Some(ref mut tos) = *self.current_timeout {
            return Some(*tos);
        }
        self.public_timeout_ms.as_ref().map(|v| v.unwrap_or(0)) as i64
    }

    /// Returns the current session's timeout settings.
    pub fn get_current_timeout(&self, ms: Option<i32>) -> i32 {
        if let Some(ref mut tos) = *self.current_timeout {
            return Some(*tos);
        }
        self.public_timeout_ms.as_i32()
    }

    /// Returns the current session's timeout settings.
    pub fn get_current_timeout_mut(&mut self, ms: Option<i32>) -> i64 {
        if let Some(ref mut tos) = *self.current_timeout {
            return Some(*tos);
        }
        self.public_timeout_ms.as_i64()
    }

    /// Returns the current session's timeout settings.
    pub fn get_current_timeout(&mut self, ms: Option<i32>) -> i32 {
        if let Some(ref mut tos) = *self.current_timeout {
            return Some(*tos);
        }
        self.public_timeout_ms.as_i32()
    }

    /// Returns the current session's timeout settings.
    pub fn get_current_timeout_mut(&mut self, ms: Option<i64>) -> i64 {
        if let Some(ref mut tos) = *self.current_timeout {
            return Some(*tos);
        }
        self.public_timeout_ms.as_i64()
    }

    /// Returns the current session's timeout settings.
    pub fn get_current_timeout(&mut self, ms: Option<i32>) -> i6
