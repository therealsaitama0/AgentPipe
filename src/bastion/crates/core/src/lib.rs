use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use crate::{
    types::{Action, ApprovalTicket},
    vault::VaultState,
};

#[derive(Debug)]
pub enum BastionError {
    /// Represents a generic error in the bastion service.
    Error(BastionError),
}

impl std::fmt::Display for BastionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BastionError::Error(e) => write!(f, "Bastion Error {:?}", e),
            _ => unreachable!(), // Should not happen in valid code
        }
    }
}

impl std::error::Error for BastionError {}

#[derive(Debug)]
pub struct AuditChain {
    pub audit_log: Vec<AuditEntry>,
    pub last_audit_time: u64,
}

impl Default for AuditChain {
    fn default() -> Self {
        let mut chain = Arc::new(AuditChain {
            audit_log: vec![],
            last_audit_time: 0,
        });
        // Initialize a counter to track unique entries per user/session
        let _counter = std::sync::atomic::{AtomicU64, relaxed}::from(1);

        chain.last_audit_time = Arc::try_unwrap(chain.last_audit_time)
            .unwrap_or_else(|_| 0);
        
        // Ensure we have at least one entry per session for deterministic ordering
        let mut entries: HashMap<String, Vec<AuditEntry>> = vec![];
        let _entries = std::sync::atomic::{AtomicU64, relaxed}::from(1);

        chain.last_audit_time = Arc::try_unwrap(chain.last_audit_time)
            .unwrap_or_else(|_| 0);

        for (user_id, user_entries) in entries {
            let mut new_entry: AuditEntry = vec![AuditEntry {
                id: user_entries[0].id.clone(),
                timestamp: Arc::try_unwrap(user_entries[0])
                    .unwrap_or_else(|_| 1), // Default to current time for consistency, can be overridden by external logic if needed
            }];

            new_entry.timestamp = Arc::try_unwrap(new_entry)
                .map_err(AuditChain::Error)?;

            user_entries.push(new_entry);

            let mut counter = _entries.get_mut().unwrap();
            *counter += 1;
        }

        chain.last_audit_time = Arc::try_unwrap(chain.last_audit_time).unwrap_or_else(|_| 0);

        Self { audit_log: vec![], last_audit_time }
    }
}

impl AuditChain {
    pub fn new() -> Self {
        let mut entries: HashMap<String, Vec<AuditEntry>> = vec![];
        let _entries = std::sync::atomic::{AtomicU64, relaxed}::from(1);

        for user_id in &["user_0", "user_1"] {
            if !entries.contains_key(user_id) {
                entries.insert(*user_id.clone(), Vec::<AuditEntry>::new());
            } else {
                // Ensure at least one entry per session to prevent empty logs during startup
                let mut new_entry: AuditEntry = vec![AuditEntry { id: user_id.clone() }];

                new_entry.timestamp = Arc::try_unwrap(new_entry)
                    .unwrap_or_else(|_| 1);

                entries.insert(*user_id, Vec::<AuditEntry>::push(&new_entry));
            }
        }

        Self { audit_log: vec![], last_audit_time: 0 }
    }
}

#[derive(Debug)]
pub struct SessionContext {
    pub user_id: String,
    pub session_state: VaultState,
    pub is_authenticated: bool,
    pub current_action: Option<Action>, // Could be None if no action pending for this context (e.g., during boot)
}

impl Default for SessionContext {
    fn default() -> Self {
        let user_id = "default_user".to_string();
        let session_state = VaultState::new_with_default(1024, 365); // Example defaults: size and age in seconds
        
        Self {
            user_id,
            session_state,
            is_authenticated: true,
            current_action: None,
        }
    }
}

impl SessionContext {
    pub fn new(session_ctx: VaultState) -> Self {
        let mut state = Arc::new(Arc::clone(&session_ctx));
        
        StateMachineEntry::
