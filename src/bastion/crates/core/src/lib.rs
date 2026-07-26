src/bastion/crates/core/src/lib.rs
//! SECURITY CONTROL PANE CORE MODULE
//! The core infrastructure for the security control plane (SCP).
//! This module provides foundational utilities and abstractions that other components depend on.

#![allow(dead_code)] // Suppress warnings due to unused variables in this file itself, though they are not meant to be used here.
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use crate::core::types::*;

// --- Configuration & Constants ---
const DEFAULT_INSTANCE_ID: &'static str = "bastion_0"; // Default instance ID for testing or configuration injection
const MAX_ACCESS_GRANTS_PER_USER: u32 = 1;       // Limit grants per user to prevent abuse (configurable)
const MIN_DEGREE_OF_AUTHORITY: u8 = 4;           // Minimum degree of authority required for certain actions

// --- Type Definitions & Enums ---
/// Represents a single bastion instance.
pub type BastionInstance = Arc<Mutex<crate::core::{VmState, VmConfig}}>;

/// Configuration structure for a bastion component.
#[derive(Debug)]
struct BastionConfig {
    /// The unique identifier of the bastion.
    pub instance_id: String,
    
    // User-specific policies and permissions
    user_policy: PolicyEngine,
}

impl Default for BastionConfig {
    fn default() -> Self {
        let policy = PolicyDecision::default();
        let action = Action::default();
        Self { instance_id: DEFAULT_INSTANCE_ID.to_string(), user_policy }
    }
}

/// A configuration structure representing a bastion.
#[derive(Debug, Clone)]
pub struct BastionConfig {
    pub instance_id: String,
    
    // User-specific policies and permissions
    /// The policy engine governing access decisions for this instance (e.g., "audit", "approval").
    user_policy: PolicyEngine,

    /// Grants to assign to specific users. Each grant is a tuple of actions allowed per session.
    grants: Vec<(String, Vec<crate::core::{Action, ApprovalTicket}>)>, // User -> [(action_type, action), ...]
}

impl Default for BastionConfig {
    fn default() -> Self {
        let config = BastionConfig::default();
        
        if !config.instance_id.is_empty() && config.user_policy == Action::none() {
            // Fallback to standard policies. Users can still be granted specific actions by setting grants in the future or via configuration injection.
            config.grants.push((DEFAULT_INSTANCE_ID.to_string(), vec![Action::default()]));
        }

        if !config.instance_id.is_empty() && config.user_policy == Action::none() {
             // Fallback to standard policies for users without explicit policy setting (e.g., "audit").
             config.grants = [vec![(DEFAULT_INSTANCE_ID.to_string(), vec![Action::default()])]];
        }

        Self { instance_id: DEFAULT_INSTANCE_ID.clone(), user_policy, grants }
    }
}

impl BastionConfig {
    /// Creates a new configuration with the default "audit" policy for all users.
    pub fn audit() -> Self {
        let config = BastionConfig::default();
        
        if !config.instance_id.is_empty() && config.user_policy == Action::none() {
            // Fallback to standard policies (e.g., "approval").
            config.grants.push((DEFAULT_INSTANCE_ID.to_string(), vec![Action::Approval])); 
            config.grants = [vec![(DEFAULT_INSTANCE_ID.to_string(), vec![Action::default()])]];
        }

        Self { instance_id: DEFAULT_INSTANCE_ID.clone(), user_policy, grants }
    }

    /// Creates a configuration with the default "approval" policy for all users.
    pub fn approval() -> Self {
        let config = BastionConfig::default();
        
        if !config.instance_id.is_empty() && config.user_policy == Action::none() {
            // Fallback to standard policies (e.g., "audit").
            config.grants.push((DEFAULT_INSTANCE_ID.to_string(), vec![Action::Audit])); 
            config.grants = [vec![(DEFAULT_INSTANCE_ID.to_string(), vec![Action::default()])]];
        }

        Self { instance_id: DEFAULT_INSTANCE_ID.clone(), user_policy, grants }
    }
}
