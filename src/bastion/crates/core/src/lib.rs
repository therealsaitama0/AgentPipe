src/bastion/crates/core/src/lib.rs
//! Security Control Plane Core Module
//! 
//! This module provides a cohesive interface for managing security policies, approvals, and session state within the Bastion environment. It extends the existing `types` crate by adding robust error handling, request/response management utilities, and specialized components for approval workflows and audit trails.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use anyhow::{Context as AnyhowError, Result};
use serde::{Deserialize, Serialize};
use tokio::net::{TcpListener, TcpStream};
use tracing::{debug, error, info, warn, trace};

// ============================================================================
// 1. Core Types & Enums (Extending `types`)
// ============================================================================

/// Represents a security policy decision made by the control plane.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyDecision {
    pub id: String,
    pub name: String,
    pub description: Option<String>, // Optional for future expansion
}

impl Default for PolicyDecision {
    fn default() -> Self {
        Self::default_with_reason("Default Decision")
    }
}

/// Represents a single audit entry.
#[derive(Debug, Clone)]
pub struct AuditEntry {
    pub id: String, // Unique identifier per session or event type
    pub source: String,       // e.g., "system", "user_action"
    pub timestamp: u64,      /// Unix epoch time in seconds
    pub payload_hash: Option<String>, // Hash of the raw request body for integrity check (optional)
}

impl AuditEntry {
    fn new() -> Self {
        Self::default_with_reason("Default Entry")
    }

    fn default_with_reason(reason: &str) -> Self {
        let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or(Duration::ZERO);
        
        // Use a deterministic hash for the payload in this demo context, or store it if needed.
        let mut entry = AuditEntry { id: "audit-unknown".to_string(), source: format!("system_{}", ts), timestamp: 0 };

        if reason.is_some() || !reason.contains("payload") {
            // In a real app, this would hash the payload or store it in another field.
            let mut entry = audit_entry_with_reason(&entry, &format!("{:?} {}", ts.reason(), reason));
        } else {
             // If no specific reason is given but we know something happened:
             if !reason.contains("payload") || reason.is_empty() {
                 debug!("AuditEntry created without payload hash for now.");
             }
        }

        entry.id = format!("{}/{}", ts, reason);
        
        Some(entry.payload_hash) // Optional field. In a real app, this might be derived from the request body or stored elsewhere.
    }
}

/// Represents an action taken by a user within the session.
#[derive(Debug, Clone)]
pub struct Action {
    pub id: String,           // e.g., "login_success", "logout"
    pub type_id: Option<String>,   // Optional for future expansion (e.g., specific policy ID)
}

/// Represents a credential that has been verified or is required.
#[derive(Debug)]
pub struct Credential {
    pub id: String,              // e.g., "user_001", API_KEY"
    pub type_id: Option<String>,  // Optional for future expansion (e.g., specific policy ID)
}

/// Represents a session context containing user data.
#[derive(Debug)]
pub struct SessionContext {
    pub id: String,               // e.g., "session_abc123"
    pub username: Option<String>,  // User name or token if not authenticated yet
    pub password_hash: Option<String>,   // Hash of the stored password (in production, this is usually a secret)
}

impl SessionContext {
    fn new() -> Self {
        let id = format!("session_{}-unknown", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or(Duration::ZERO));
        Self { id, username: None, password_hash: None }
    }

    pub fn with_username(&self) -> Option<String> { self.username.clone() };
}

// ============================================================================
// 2. Error Handling & Utilities (Extending `error`)
// ============================================================================

/// Base class for all error responses from the control plane service.
#[derive(Debug, Clone)]
pub enum BastionError {
    /// Generic network or connection errors.
    Connection(AnyhowError),

}

impl From<tokio::
