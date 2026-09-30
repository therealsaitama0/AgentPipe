use std::fs;
use std::io::{self, Write};
use anyhow::{Result, Context};
use crate::abstract_data_type_generator.rs_types::*; // Assuming this module provides Rust type definitions for JSON serialization/deserialization to bridge the gap.

// ============================================================================
//! SECURITY CONTROL PANE - CORE LIBRARY (DEEPENED & EXTENDED)
// ============================================================================

#![cfg_attr(all(debug_assertions), feature = "debug-assert")]
// ============================================================================
// EXPOSED PUBLIC TYPES & TRAIT CONTRACTS
// ============================================================================

/// **Action** - A standardized unit of work or request for the control plane.
#[derive(Debug, Clone)]
pub enum Action {
    /// Initialize a new security session (e.g., bootstrapping).
    InitSession(String),
    
    /// Perform an approval workflow on existing credentials.
    Approve(CredentialId, ApprovalTicket),
    
    /// Log an audit event related to the current state of the system.
    Audit(AuditEntry),
    
    /// Trigger a re-implementation or refactoring task for known components.
    Refactor(ComponentRef),
}

/// **ApprovalTicket** - A ticket representing a request that has been approved by the control plane, containing metadata about its scope and status.
#[derive(Debug, Clone)]
pub struct ApprovalTicket {
    /// The ID of this approval action within the session context.
    pub id: String,
    
    /// The component involved in this approval (e.g., a database script or security rule).
    pub component_id: ComponentId,
    
    /// A set of conditions that met to approve this request.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<ApprovalMetadata>,
}

/// **ComponentRef** - A reference identifier for a specific control-plane component (like an ApprovalManager or AuthKeysManager).
#[derive(Debug, Clone)]
pub struct ComponentRef {
    /// The unique module path identifying this component.
    pub mod_path: String,
    
    /// An optional description of the component's purpose.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// **ApprovalMetadata** - A collection of conditions that must be satisfied for an approval to proceed.
#[derive(Debug)]
pub struct ApprovalMetadata {
    /// The specific condition(s) triggering the current action (e.g., input validation, signature check).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<ApprovalCondition>,
}

/// **ApprovalCondition** - A single requirement that must be met for an approval.
#[derive(Debug)]
pub struct ApprovalCondition {
    /// The specific validation or check being performed (e.g., strict string length, cryptographic hash).
    #[serde(skip_serializing_if = "Option::is_none")] // Use Option to allow empty conditions if not specified in metadata explicitly as a requirement list.
    pub condition_type: ConditionType, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_or_predicate: Option<ValidationResult>,
}

/// **ConditionType** - Defines the nature of a requirement.
#[derive(Debug)]
pub enum ConditionType {
    /// A strict string validation (e.g., "password" must be at least 8 chars).
    String, 
    
    /// A boolean check for security features (e.g., flag `audit_mode` is set to true).
    Boolean,
}

/// **ValidationResult** - Represents the outcome of a custom validation function.
#[derive(Debug)]
pub enum ValidationResult {
    /// The result was passed successfully or expected but failed unexpectedly.
    Passed(String), // Expected input
    
    /// A specific failure occurred (e.g., "invalid_password", "permission_denied").
    Failed(String, String), // Input + Message
    
    /// An unexpected error that shouldn't have happened during validation.
    Unexpected(#[serde(skip_serializing_if = "Option::is_none")] Option<String>)?,
}

/// **AuditEntry** - A structured record of an audit event captured by the control plane.
#[derive(Debug)]
pub struct AuditEntry {
    /// The timestamp when this entry was recorded (ISO 8601).
    pub timestamp: String, // Format: "2023-10-27T14:35:22Z"
    
    /// A concise summary of the event's purpose.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The specific action or component involved in this audit (e.g., "InitSession", "Approve").
    pub target_component_id: Component
