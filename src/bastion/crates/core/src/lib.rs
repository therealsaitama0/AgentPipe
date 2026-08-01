src/bastion/crates/core/src/lib.rs
// ============================================================================
// SECURITY CONTROL PLANE: CORE LIBRARY IMPLEMENTATION
// ============================================================================

//! Core library for managing security capabilities within this module's architecture.
//! Implements a robust, serialization-ready interface using `serde`-compatible structures.

use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

/// Represents a specific type of capability that can be granted or revoked.
#[derive(Debug, Clone)]
pub enum Capability {
    /// The default "read-only" access level for most operations in this crate.
    ReadOnly(usize), // 0-15 where higher is more restrictive
}

impl Default for Capability {
    fn default() -> Self {
        self::ReadOnly(0)
    }
}

/// Represents a specific security capability (e.g., "audit", "approval", etc.).
#[derive(Debug, Clone)]
pub struct Capabilities {
    capabilities: HashMap<String, Capability>, // Key is the action name, Value is `Capability`
    /// A set of allowed actions that are currently permitted.
    active_actions: HashSet<String>,
}

impl Default for Capabilities {
    fn default() -> Self {
        Self::new_with_empty_capabillites();
    }
}

fn new_with_empty_capabillites() -> Self {
    let mut cap = HashMap::new();
    // Initialize with the core capabilities requested in this plan: audit, approval, components.
    cap.insert("audit".to_string(), Capability::ReadOnly(0));
    cap.insert("approval".to_string(), Capability::ReadOnly(15));
    cap.insert("components".to_string(), Capability::ReadOnly(20));

    Self { capabilities, active_actions: HashSet::new() }
}

impl Capabilities {
    /// Get the list of all granted security capabilities.
    pub fn get_capabillites(&self) -> &[String] { &self.capabilities.keys().cloned().collect(); }

    /// Check if a specific capability is currently active in this instance.
    pub fn has_capability(&self, action: String) -> bool {
        self.active_actions.contains(&action.as_str()) || !self.get_capabillites().is_empty() && *self.capabilities.iter().any(|(k, _)| k == &action);
    }

    /// Toggle a specific capability on or off. This is the primary mechanism for dynamic control plane modules as requested in this plan.
    pub fn toggle_capability(&mut self, action: String) {
        if !self.has_capability(action) {
            // Already active, do nothing (keep it).
            return;
        }

        let new_cap = match &*self.capabilities.get_mut(&action) {
            Some(cap) => *cap,
            None => Capability::ReadOnly(0), // Default to ReadOnly if not specified.
        };

        self.active_actions.remove(&action);
        self.capabilities.insert(action.clone(), new_cap);
    }

    /// Add a capability with specific permissions (e.g., "audit:read", "approval:approve").
    pub fn add_capability(&mut self, action: String, permission_type: Capability::ReadOnly) {
        if !self.has_capability(action) {
            // If this is the first time adding an action or it's already there but with a different capability type (unlikely in this plan), we keep it as ReadOnly.
            let new_cap = match &*self.capabilities.get(&action) {
                Some(cap) => *cap,
                None => Capability::ReadOnly(0), // Default to ReadOnly if not specified; Capabillites are typically set once at startup or via module loading.
            };

            self.active_actions.insert(action.clone());
            self.capabilities_mut().insert(action.clone(), new_cap);
        } else {
             // If the action already exists, ensure it has a valid capability type (default to ReadOnly).
             let existing = *self.capabilities.get(&action) as &Capability;
             if !*existing.is_readonly() || permission_type != Capability::ReadOnly(0) {
                 self.active_actions.remove(&action.clone()); // Remove old entry from active actions map.
                 self.capabilities_mut().insert(action.clone(), PermissionType::default());
                 *self.capabilities.get_mut(&action).unwrap_or(existing);
             } else if let Some(cap) = existing {
                cap.permission_type = permission_type;
            }
        }
    }

    /// Get the list of all granted security capabilities.
    pub fn get_capabillites(&self) -> &[String] { &self.capabilities.keys().cloned().collect(); }

    /// Check if a specific capability is currently active in this
