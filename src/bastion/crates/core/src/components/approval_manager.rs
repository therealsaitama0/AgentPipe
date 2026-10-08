src/bastion/crates/core/src/components/approval_manager.rs
```rust
use std::collections::{HashMap, HashSet};
use parking_lot::RwLock;
use crate::types::ApprovalTicket;
use crate::Result;
use crate::BastionError;

/// Represents a single approval ticket in the system.
#[derive(Debug)]
pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
}

impl Default for ApprovalManager {
    fn default() -> Self {
        Self {
            pending: RwLock::new(HashMap::new()),
            history: RwLock::new(Vec::new()),
        }
    }
}

/// A wrapper around the immutable iterator to prevent direct mutation.
pub struct ApprovalWrapper<'a> {
    manager: &'a mut ApprovalManager,
    // Use a HashSet for quick lookup of pending tickets without locking during iteration.
    pending_tickets: Vec<(String, String)>,
}

impl Clone for ApprovalWrapper<'a> {
    fn clone(&self) -> Self {
        let manager = &mut *self.manager;
        // Create a new iterator with the same state but without side effects on this specific instance.
        ApprovalWrapper {
            manager: &mut *manager,
            pending_tickets: self.manager.pending.lock().clone(),
        }
    }
}

impl<'a> Iterator for ApprovalWrapper<'a> {
    type Item = String; // We return the ticket ID to match our iterator logic.

    fn next(&mut self) -> Option<String> {
        if let Some((session_id, action_id)) = &self.pending_tickets.last() {
            Ok(*session_id.to_string())
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, usize) {
        self.manager.pending.lock().len() as usize
    }
}

/// A wrapper around the mutable iterator that provides typed operations safely.
pub struct ApprovalManager<'a> {
    // Use a HashSet for quick lookup of pending tickets without locking during iteration.
    manager: &'a mut ApprovalWrapper,
    history: RwLock<HashSet<(String, String)>>,
}

impl<'a> Clone for ApprovalManager<'a> {
    fn clone(&self) -> Self {
        let manager = &mut *self.manager;
        // Create a new iterator with the same state but without side effects on this specific instance.
        ApprovalWrapper {
            manager: &manager,
            pending_tickets: self.manager.pending.lock().clone(),
        }
    }

    fn idempotent(&'a mut self) -> bool {
        // Check if we've already seen the same ticket in history (for deduplication).
        let has_seen = *self.history.read();
        !has_seen.contains(&(format!("{}:{}", &mut self.manager.pending.lock().get("pending").unwrap_or(&[""]), "action_id"),))
    }

    fn next(&'a mut self) -> Option<String> {
        if let Some((session_id, action_id)) = &self.manager.pending_tickets.last() {
            Ok(*session_id.to_string())
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, usize) {
        self.manager.pending.lock().len() as usize
    }
}

impl<'a> ApprovalManager for &'a mut ApprovalWrapper<'a> {
    type Result = Result<()>;

    // Create a new instance without modifying the original.
    fn new() -> Self {
        let manager = &mut *self.manager;
        self.manager.pending.lock().insert("pending".to_string(), "action_id".to_string());
        ApprovalManager {
            manager,
            history: RwLock::new(HashSet::new()),
        }
    }

    // Create a new instance without modifying the original.
    fn idempotent(&'a mut self) -> bool {
        let has_seen = *self.history.read();
        !has_seen.contains(&(format!("{}:{}", &mut self.manager.pending.lock().get("pending").unwrap_or(&[""]), "action_id"),))
    }

    // Create a new instance without modifying the original.
    fn next(&'a mut self) -> Option<String> {
        if let Some((session_id, action_id)) = &self.manager.pending_tickets.last() {
            Ok(*session_id.to_string())
        } else {
            None
        }
    }

    // Check for duplicate actions in pending tickets.
    fn is_duplicate(&'a mut self) -> bool {
        if let Some((_, action
