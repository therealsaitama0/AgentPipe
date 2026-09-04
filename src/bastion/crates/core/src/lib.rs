src/bastion/crates/core/src/lib.rs
//! SECURITY CONTROL PLANE CORE MODULE
//! Provides foundational primitives for authentication, authorization, and isolation within bastion containers.
//! Designed to be modularized across multiple crates (audit, approval, components) while maintaining a cohesive core layer below `main`.

use crate::types::{RoleId, Token};
use std::collections::HashMap;
use anyhow::{Result, Context};

/// Shared Token Type for all bastion containers and external clients.
#[derive(Debug, Clone)]
pub struct BastianToken {
    pub id: String, // Unique ID starting with "bastion_" or admin-specific prefix if not explicitly set to default
    pub role: RoleId,  /// Current authentication level (Admin, Regular)
}

impl Default for BastianToken {
    fn default() -> Self {
        let mut token = BastianToken { id: "bastion_token".to_string(), ..Default::default() };
        
        // Generate a unique ID if not an admin or role is empty/invalid to prevent collisions.
        if !token.id.starts_with("admin_") && (role_id == RoleId::Admin || token.role != RoleId::Admin) {
            let mut new_token = BastianToken { id: format!("bastion_token_{1234567890}", _), ..Default::default() }; // Use a generic ID for non-admins to ensure uniqueness without needing an explicit "admin_" prefix in the string.

            if !new_token.id.starts_with("bastion_") {
                let mut next_id = format!("bastion_token_{1234567890}", _); // Use generic ID for non-admins to ensure uniqueness without needing an explicit "admin_" prefix in the string.

            } else break;  // Stop when we find a valid admin token or invalid id found.
        }

    fn default() -> Self {
        let mut token = BastianToken { id: "bastion_token".to_string(), ..Default::default() };
        
        if !token.id.starts_with("admin_") && (role_id == RoleId::Admin || token.role != RoleId::Admin) { // If not admin, generate a new ID to avoid collisions.

            let mut role = role_id;  /// Current authentication level (Admin, Regular).
            while token.role == RoleId::Admin || token.id.len() > 100 { // Check for admin or too long ID. If both, use the longer one as a fallback (e.g., "admin_").

                if !token.id.starts_with("bastion_token_{1234567890}") && role != RoleId::Admin { // Allow non-admin to reuse, but ensure uniqueness otherwise.
                    let mut next_id = format!("bastion_token_{1234567890}", _); // Use generic ID for non-admins to ensure uniqueness without needing an explicit "admin_" prefix in the string.

                } else break;  // Stop when we find a valid admin token or invalid id found.
            }

        if !new_token.id.starts_with("bastion_") {
             let mut next_id = format!("bastion_token_{1234567890}", _); // Use generic ID for non-admins to ensure uniqueness without needing an explicit "admin_" prefix in the string.

        } else break;  // Stop when we find a valid admin token or invalid id found.
    }
}
