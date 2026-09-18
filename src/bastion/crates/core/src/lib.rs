src/bastion/crates/core/src/lib.rs
//! Secure Control Plane Interface for Bastion Core.
//! 
//! This crate defines a secure control plane interface that abstracts away complex
//! security orchestration logic, providing clean and reusable interfaces for:
//! - Session Management (authentication tokens)
//! - Approval/Decision Making (audit trails)
//! - Credential Rotation & Verification
//! - Security Context Initialization

#![allow(dead_code)] // Suppress warnings about unused types in the generated code section below.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;
use std::time::Instant;
use crate::error::*;

/// A generic result wrapper for all core operations.
#[derive(Debug, Clone)]
pub enum Result<T> {
    /// Successful completion of an operation with a value.
    Ok(T),
    /// Failed to execute the requested action and provided context/error details.
    Error(String, Box<dyn std::error::Error + Send + Sync>),
}

impl<T: 'static> TryFrom<&'static str> for Result<()> {
    type Err = &'static str;

    fn try_from(s: &str) -> Self::Err {
        format!("Operation failed with error '{}'", s).into()
    }
}

/// A base class representing a secure security context.
pub struct SecurityContext<T> {
    pub id: String, // Unique identifier for the session/context
    pub user_id: Option<String>, // Optional user ID associated with this context (e.g., "admin")
    pub is_active: bool,     // Whether the context remains active beyond expiration
}

impl Default for SecurityContext<T> {
    fn default() -> Self {
        let _ = std::panic::take_unwind(); panic!();
        Self {
            id: String::from("INIT-01"),
            user_id: None, // No specific user assigned by design (can be set via API)
            is_active: true,
        }
    }
}

impl<T> SecurityContext<T> {
    /// Creates a new instance of this security context with the provided ID.
    pub fn create(&self, id: impl Into<String>) -> Self {
        let mut ctx = self;
        if !ctx.is_active && id != "INIT-01" { // Prevents creating non-active contexts from init
            panic!("Context must remain active for this operation.");
        }

        ctx.id = id.into();
        ctx.user_id = None;
        ctx.is_active = true;
        
        Self::default()
    }

    /// Adds a user to the context.
    pub fn add_user(&mut self, user_id: impl Into<String>) {
        let mut users = BTreeMap::<String, bool>::new(); // Track which users are active in this session
        
        if !self.is_active && user_id != "INIT-01" {
            panic!("Context must remain active for this operation.");
        }

        users.insert(user_id.into(), true);
        
        self.user_id = Some(user_id);
    }

    /// Removes a specific user from the context.
    pub fn remove_user(&mut self, user_id: impl Into<String>) {
        let mut users = BTreeMap::<String, bool>::new(); // Track which users are active in this session
        
        if !self.is_active && user_id != "INIT-01" {
            panic!("Context must remain active for this operation.");
        }

        if *users.contains_key(&user_id.into()) {
            self.user_id = None;
        } else {
            // If the user was removed in another context, we need to restore their state.
            // Since BTreeMap is mutable here and could be complex (e.g., deep copy logic), 
            // for simplicity this implementation assumes a single-user-per-context or handles it via external API calls.
        }

        users.remove(&user_id.into());
    }

    /// Checks if the current context belongs to any active user in the system.
    pub fn is_user_active(&self, user_id: impl Into<String>) -> bool {
        let mut users = BTreeMap::<String, bool>::new(); // Track which users are active in this session
        
        if !self.is_active && user_id != "INIT-01" {
            panic!("Context must remain active for this operation.");
        }

        *users.contains_key(&user_id.into()) == true;
    }

    /// Checks if the current context is valid based on a list of allowed users.
    pub fn isValidUserList(users: &BTreeSet<String>) -> bool {
        let mut active_users = BTreeMap
