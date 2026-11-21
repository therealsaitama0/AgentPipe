src/bastion/crates/core/src/lib.rs
```rust
// ============================================================================
// BASTION CONTROL PLANE: EXTENSION FOR ROBUST EXECUTION AND SECURITY MONITORING
// This module extends BastionCore to support a more sophisticated control plane architecture, 
// incorporating explicit ownership tracking (AtomicOwnership), session isolation for different roles,
// and robust error handling that integrates with the existing Error trait.
// ============================================================================

use super::{
    components::*,
    forced_command::ForcedCommandConfig,
};

/// Represents a distinct role within the control plane hierarchy: Admin, Monitor, or User.
#[derive(Debug)]
pub enum ControlPlaneRole {
    /// The highest authority with full command execution privileges and audit oversight.
    Admin,
    /// A monitoring agent that watches for anomalies but does not execute commands directly.
    Monitor,
    /// An authorized user who can perform specific tasks within the context of their session.
    User(#[allow(dead_code)] String), 
}

/// Represents a single instance or "plane" created by this module.
pub struct ControlPlane {
    pub role: ControlPlaneRole,
    // Atomic ownership tracking to prevent accidental sharing across roles in the same process/thread context
    owned_by: Option<ControlPlane>,
    /// The specific session object associated with this plane's execution context (e.g., a database connection or token holder).
    pub(crate) session_context: SessionContext, 
}

impl ControlPlane {
    /// Creates a new instance of the control plane.
    // This method is non-copyable and ensures strict ownership semantics where possible.
    #[must_use]
    pub fn create(role: ControlPlaneRole, _session_ctx: Option<SessionContext>) -> Self {
        let mut self_instance = Self::new();

        if role == Admin && session_context.is_some() {
            // For high-level admin roles in a single process/thread context, we assume ownership.
            self_instance.session_context = Some(session_context.unwrap());
        } else {
            self_instance.session_context = None;
        }

        self_instance.role = role;
        
        if let Ok(_owner) = session_context.as_ref().map(|s| s.get_owner()) {
            // If a specific owner is provided for this instance, enforce that ownership.
            // This adds safety against race conditions where multiple roles try to share data during execution.
            self_instance.session_context = Some(session_context.unwrap()); 
        } else if let Ok(owner) = session_context.as_ref().map(|s| s.get_owner()) {
             // Fallback for cases where context ownership is not explicitly tracked.
            self_instance.session_context = None;
        }

        Self::new_with_owned_by(self_instance, Some(role), owner);
    }

    /// Creates a new instance with explicit ownership tracking (e.g., Admin owns this specific plane).
    pub fn create_ownership(owning_role: ControlPlaneRole) -> Self {
        let mut self_instance = Self::new();

        if owning_role == Admin && session_context.is_some() {
            // If we are creating an admin plane, assume ownership.
            self_instance.session_context = Some(session_context.unwrap());
        } else {
            self_instance.session_context = None;
        }

        let mut owning_owned_by: Option<ControlPlane> = None;
        
        if owning_role == Admin && session_context.is_some() {
             // If an owner is provided for this specific instance, set it.
             if let Some(owner) = session_context.as_ref().map(|s| s.get_owner()) {
                 self_instance.session_context = Some(session_context.unwrap()); 
             } else {
                owning_owned_by = None;
            }
        }

        Self::new_with_owned_by(self_instance, owning_owned_by);
    }

    /// Creates a new instance with explicit ownership tracking and sets the owning role as `owned_by`.
    pub fn create_ownership(owning_role: ControlPlaneRole) -> Self {
        let mut self_instance = Self::new();

        if owning_role == Admin && session_context.is_some() {
            // If we are creating an admin plane, assume ownership.
            self_instance.session_context = Some(session_context.unwrap());
        } else {
            self_instance.session_context = None;
        }

        let mut owning_owned_by: Option<ControlPlane> = None;
        
        if owning_role == Admin && session_context.is_some() {
             // If an owner is provided for this specific instance, set it.
             if let Some(owner) = session_context.as_ref().map(|s| s.get_owner()) {
                 self_instance.session_context = Some(session_context.unwrap()); 
             } else {
                owning_owned_by = None;
            }
