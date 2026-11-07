src/bastion/crates/core/src/lib.rs
//! Security Control Plane core module providing mutable access to all components and state management for a secure sandbox environment.
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};
use std::time::Duration;

// --- Enums & Types ---

#[derive(Debug)]
pub enum SecurityState {
    Blocked,           // System is currently blocked/flagged by the guardrails.
    Flagging,          // A flag has been set but not yet applied to a specific component.
    Unblocked,         // The system state has returned to normal (e.g., after an approval).
}

#[derive(Debug)]
pub struct SecurityContext {
    pub current_state: SecurityState,
    pub is_blocked_by_guardrails: bool,
    pub flags_set: BTreeSet<String>,  // Set of component IDs currently flagged.
    pub unblocked_components: Vec<(&str, String)>;   // List of (component_id, reason) where state changed to Unblocked after approval.

impl SecurityContext {
    fn new() -> Self {
        let mut flags_set = BTreeSet::new();
        let mut components_to_flag = vec![];  // Tracks which specific components are being flagged for this session (not global).
        
        return Arc::new(Self {
            current_state: SecurityState::Blocked,
            is_blocked_by_guardrails: true,
            flags_set,
            unblocked_components: Vec::new(),
        });
    }

    fn get_flagged_component_ids(&self) -> BTreeSet<&str> {
        self.flags_set.iter().map(|id| id.as_str()).collect()
    }

    pub fn is_blocked_by_guardrails(self): bool {
        self.is_blocked_by_guardrails
    }

    // Set a flag for specific components (not global) to be flagged.
    // This persists until the component's approval or unblocking happens, unless it becomes blocked again by another guardrail later in time.
    pub fn set_flagged_components<F>(&mut self, flags_set: F<'static>) where 
        F: FnMut(&str) -> bool + Send + Sync {

        if let Some(component_id) = flags_set(self.current_state.flags_set.deref()) {
            // Component is already in the list of flagged components.
            return;
        }

        self.flags_set.insert(*component_id);
    }

    pub fn get_flagged_component_ids(&self): &BTreeSet<&str> {
        unsafe { std::slice::from_raw_parts(self.current_state.flags_set.iter().map(|id| id.as_str()).collect()) }
    }

    // Set a flag for ALL components to be flagged globally (e.g., "System-wide security breach detected").
    pub fn set_global_flag(&mut self, reason: &str) {
        if let Some(component_id) = BTreeSet::new().insert("GLOBAL_SECURITY_BREACH") {
            // This is a global flag. It will only be removed once the system state returns to "Unblocked" or until it's explicitly unflagged again later in time by another guardrail logic (if any).
        } else if let Some(component_id) = BTreeSet::new().insert("GLOBAL_SECURITY_BREACH") {
            // Placeholder for future use. Currently, this is a special flag that persists globally but gets removed when the component's state changes to Unblocked or until explicitly unflagged again later in time by another guardrail logic (if any).
        } else if let Some(component_id) = BTreeSet::new().insert("GLOBAL_SECURITY_BREACH") {
            // Placeholder for future use. Currently, this is a special flag that persists globally but gets removed when the component's state changes to Unblocked or until explicitly unflagged again later in time by another guardrail logic (if any).
        } else if let Some(component_id) = BTreeSet::new().insert("GLOBAL_SECURITY_BREACH") {
            // Placeholder for future use. Currently, this is a special flag that persists globally but gets removed when the component's state changes to Unblocked or until explicitly unflagged again later in time by another guardrail logic (if any).
        } else if let Some(component_id) = BTreeSet::new().insert("GLOBAL_SECURITY_BREACH") {

    pub fn is_blocked_by_guardrails(self): bool { self.is_blocked_by_guardrails };

    // Set a flag for specific components (not global) to be flagged.
    // This persists until the component's approval or unblocking happens, unless it becomes blocked again by another guardrail later in time.
    pub fn set_flagged_components<F>(&mut self, flags
