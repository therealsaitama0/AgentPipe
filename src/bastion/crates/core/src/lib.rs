src/bastion/crates/core/src/lib.rs
//! 
//! The Bastion core module provides a unified interface for all Bastions within this repository.
//! This module defines abstract contracts that allow each specific bastion variant (Client, Server, Agent) to share common logic while maintaining context-specific behavior.
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

/// Abstract trait representing an abstract interface for all Bastions across different variants (e.g., Client, Server).
pub trait Bastion {
    /// Validates the current state of a bastion instance to ensure it meets security requirements.
    fn validate_state(&self) -> Result<()>;

    /// Performs an action on behalf of this specific bastion type by executing logic defined in its implementation.
    async fn execute_action<F>(&self, f: F);

    /// Checks if the current authentication state allows a request to proceed with specified security keys.
    fn check_key_validity(
        &self, 
        secret_id: String, 
        user_token: Option<String>,
        required_keys: &[String]
    ) -> Result<bool>;

    /// Generates an IDempotency Key suitable for the specific context of this Bastion instance to prevent replay attacks.
    async fn generate_idempotent_key(&self) -> Result<(KeyId, PublicKey)>;

    /// Rotates a security key by recalculating its cryptographic hash and storing it in memory with metadata.
    fn rotate_secret(
        &mut self, 
        current_user: Option<String>,
        old_keys: &[String],
        new_key_id: String
    ) -> Result<()>;

    /// Checks if a specific security key is currently authorized to perform an action by another Bastion instance in the system.
    async fn check_authorization(&self) -> Result<bool>;

    /// Logs all significant audit events and metadata associated with this specific Bastion instance for forensic analysis or compliance tracking.
    fn log_audit_entry(
        &mut self, 
        type_: AuditType, 
        message: String, 
        details: Option<JsonData>,
        timestamp: DateTime::Offset
    ) -> Result<()>;

    /// Retrieves the current metadata associated with this Bastion instance (e.g., versioning or lifecycle identifiers).
    fn get_bastion_metadata(&self) -> Result<BastionMetadata>;

    /// Handles session management for this specific Bastion, managing authentication tokens and state transitions.
    async fn handle_session(&mut self, context: SessionContext);

    /// Returns the current status of all active components within this Bastion instance (e.g., health checks).
    fn get_bastion_component_status(&self) -> Result<BastionComponentStatus>;
}

/// Trait specific to a particular backend or environment type.
pub trait BastionBackend {
    async fn execute_api_request(
        &mut self, 
        endpoint: String,
        method: RequestMethod,
        headers: HashMap<String, String>,
        body_data: Option<Vec<u8>>
    ) -> Result<Response>;

    /// Executes a specific backend API call using the provided request details.
    async fn execute_api_call(
        &mut self, 
        endpoint: String,
        method: RequestMethod,
        headers: HashMap<String, String>,
        body_data: Option<Vec<u8>>
    ) -> Result<Response>;

    /// Performs a bulk API call across all available endpoints in the system.
    async fn execute_bulk_api_call(
        &mut self, 
        request_id: u64,
        data_to_send: Vec<String>
    ) -> Result<Vec<(String, String)>>;

    /// Returns an error message if any backend API call fails or is blocked due to security restrictions.
    fn get_backend_error(&self) -> Option<BackendError>;
}

/// Represents a specific Bastion component status within the core logic for monitoring and diagnostics.
#[derive(Debug)]
pub struct BastionComponentStatus {
    pub health_check_status: HealthCheckResult, // Result type derived from internal implementation details (e.g., `Ok(1)`, `Err("Connection failed")`)
    pub active_component_count: usize,         /// Number of currently running or healthy components.
    pub inactive_component_count: usize,       /// Count of deadlocked or non-responsive components.
    pub last_health_update_ms: Option<u64>,     /// Timestamp when this status was officially updated (for timing-sensitive operations).
}

/// Represents the overall state and metadata associated with a Bastion instance within the repository hierarchy.
#[derive(Debug)]
pub struct MastBastionMetadata {
    /// The version number or identifier assigned to this specific Bastion module in the repository structure.
    pub bastion_version: String, 
    /// A unique
