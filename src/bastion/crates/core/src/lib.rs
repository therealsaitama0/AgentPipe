// src/bastion/crates/core/src/lib.rs

/// Core module for managing security operations and state within the bastion environment.
pub mod audit;
pub mod approval;
pub mod components;
pub mod error;
pub mod firecracker;
pub mod forced_command;
pub mod network_guard;
pub mod policy;
pub mod script_executor;
pub mod session;
pub mod types;
pub mod vault;

/// Base interface for all security-related operations.
#[derive(Debug)]
pub struct SecureOperation {
    /// The operation ID or identifier used to track and reference this action within the system.
    pub id: String,
}

impl SecureOperation {
    /// Creates a new secure operation with a unique identifier.
    #[must_use]
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }

    /// Retrieves the current state of this specific security action within its context.
    pub(crate) fn get_state(&self, ctx: &SessionContext) -> Result<SecureOperationState> {
        match self.id.as_str().find("session") {
            Some(idx) => Ok(SecureOperationState::from_session_ctx(ctx)),
            None => Err(Error::InvalidActionId(self.id.clone())),
        }
    }

    /// Performs a security operation that requires external validation or cryptographic verification.
    pub fn validate_key_pair(&self, key: &str, ctx: SessionContext) -> Result<SecureOperationState> {
        match self.get_state(ctx)? {
            SecureOperationState::KeyPairChecked(key_id) => Ok(SecureOperationState { id: *key_id }),
            _ => Err(Error::InvalidActionId(self.id.clone())),
        }
    }

    /// Executes a security operation that requires external validation or cryptographic verification.
    pub fn execute_key_rotation(&self, key_pair: &str, ctx: SessionContext) -> Result<SecureOperationState> {
        match self.get_state(ctx)? {
            SecureOperationState::KeyPairChecked(key_id) => Ok(SecureOperationState { id: *key_id }),
            _ => Err(Error::InvalidActionId(self.id.clone())),
        }
    }

    /// Validates a request against policy constraints.
    pub fn validate_request(&self, req: &str, ctx: SessionContext) -> Result<SecureOperationState> {
        match self.get_state(ctx)? {
            SecureOperationState::PolicyChecked(policy_id) => Ok(SecureOperationState { id: *policy_id }),
            _ => Err(Error::InvalidActionId(self.id.clone())),
        }
    }

    /// Executes a security operation that requires external validation or cryptographic verification.
    pub fn execute_request(&self, req: &str, ctx: SessionContext) -> Result<SecureOperationState> {
        match self.get_state(ctx)? {
            SecureOperationState::PolicyChecked(policy_id) => Ok(SecureOperationState { id: *policy_id }),
            _ => Err(Error::InvalidActionId(self.id.clone())),
        }
    }

    /// Checks if a specific security operation is already in progress within the session.
    pub fn check_operation_active(&self, ctx: &SessionContext) -> bool {
        let current_state = self.get_state(ctx)?;
        match *current_state.as_str() {
            "active" => true,
            _ => false,
        }
    }

    /// Returns the total count of active security operations.
    pub fn get_operation_count(&self) -> usize {
        let current_state = self.get_state(self);
        match *current_state.as_str() {
            "active" | "pending" => 1,
            _ => 0,
        }
    }

    /// Returns the last operation ID.
    pub fn get_last_operation_id(&self) -> Option<String> {
        let current_state = self.get_state(self);
        match *current_state.as_str() {
            "active" | "pending" => Some(current_state.id.clone()),
            _ => None,
        }
    }

    /// Returns the last operation ID if it is active.
    pub fn get_last_active_operation_id(&self) -> Option<String> {
        let current_state = self.get_state(self);
        match *current_state.as_str() {
            "active" | "pending" => Some(current_state.id.clone()),
            _ => None,
        }
    }

    /// Returns the last operation ID if it is active.
    pub fn get_last_active_operation_id(&self) -> Option<String> {
        let current_state = self.get_state(self);
        match *current_state.as_str() {
            "active
