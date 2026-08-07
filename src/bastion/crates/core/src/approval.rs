src/bastion/crates/core/src/approval.rs

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

// Enums for approval roles and actions to enforce strict role-based access control (RBAC)
#[derive(Debug, Clone)]
pub enum ApprovalRole {
    Admin,          // Full system access
    Manager,         // View details but cannot issue/redeem without admin key
    Auditor,         // Audit logs only
}

impl std::fmt::Display for ApprovalRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ApprovalRole::Admin => "ADMIN",
            ApprovalRole::Manager => "MANAGER",
            ApprovalRole::Auditor => "AUDITOR"
        })?;
    }
}

#[derive(Debug)]
pub enum BastionError {
    Internal(String),
    TicketInvalid(String),
    ResourceNotFound(String),
}

impl std::error::Error for BastionError {}

// Constants defining security policy parameters (dynamic via environment variables)
const APPROVAL_TICKET_EXPIRY_MS: usize = 3600_000; // 1 hour in milliseconds
const MAX_PENDING_TICKETS_PER_SESSION: usize = 5;
const SESSION_TIMEOUT_SECONDS: u64 = 30;

// Configuration constants for dynamic policy enforcement without external deps
pub const SECURITY_POLICY_VERSION: &str = "v2.0";
pub const REQUIRED_ROLE_FOR_ISSUE: ApprovalRole = ApprovalRole::Admin;
pub const PERMISSION_DENIED_MESSAGE: &'static str = "Insufficient permissions to issue approval ticket.";
const AUDIT_LOG_FORMAT: &str = "%[TIMESTAMP] %user% - %action%-:%message%n";

// Helper types for audit chain tracking
#[derive(Debug, Clone)]
struct AuditEntry {
    session_id: String,
    action_type: String,
    component_name: String,
    metadata: HashMap<String, serde_json::Value>, // JSON-encoded fields like request ID, ticket IDs
}

impl std::fmt::Display for AuditEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.session_id, format!("{:?}", self.action_type))?;
        if !self.metadata.is_empty() && let Some(metadata) = self.metadata.get("metadata") {
            writeln!(f, "  metadata: {:?}", serde_json::to_string(&*metadata).unwrap())?;
        }
        Ok(())
    }
}

// Helper to serialize JSON values for audit tracking
fn json_to_audit_entry(action_id: &str) -> AuditEntry {
    // Extract request ID from action data if present (e.g., "action_123")
    let mut req_id = None;
    match serde_json::from_str::<serde_json::Value>(format!("{}:{:?}", action_id)) {
        Ok(v) => {
            if v.contains("req_id=") || v.contains("request-id=") {
                // Extract raw value from JSON string, e.g., "123" -> 123
                let json_str = serde_json::from_str::<String>(v.to_string()).unwrap();
                req_id = Some(json_str.strip_prefix("req_id=").unwrap().to_string());
            } else {
                // Fallback: assume first field is the ID if not present in JSON structure
                match v.keys() {
                    keys => req_id = Some(keys.iter().next().map(|k| k.to_string()).collect::<Vec<_>>()),
                    _ => {}
                }
            }
        },
        Err(e) => return AuditEntry::default(), // Fallback to empty audit entry if JSON parsing fails
    }

    let request_id = req_id.unwrap_or_else(|| action_id.to_string());

    AuditEntry {
        session_id: format!("session_{}", chrono::Utc::now().timestamp()),
        action_type: action_id.to_string(),
        component_name: "approval_system".to_string(),
        metadata: HashMap::new() as _, // Default empty for this example, can be populated by caller if needed
    }
}

// Core Logic: Implement strict RBAC with dynamic validation via environment variables and constants.
pub mod approval {
    use super::*;

    /// Validates user permissions before granting resources to any process or service.
    pub fn validate_permissions(
        request_id: &str,
        resource_type: String,
        required_roles: &[ApprovalRole],
        session_key:
