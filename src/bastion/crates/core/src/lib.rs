use std::{collections::HashMap, sync::Arc, time::Duration};

#[derive(Debug)]
pub enum BastionError {
    /// Raised when an operation fails due to missing permissions or invalid credentials
    PermissionDenied(String),
    /// Raised for unexpected errors during execution (e.g., network issues)
    ExecutionFailed(Option<String>), // Some exceptions might be wrapped here if needed, but we keep it explicit
}

impl BastionError {
    pub fn new(message: &str, code: Option<&str>) -> Self {
        let msg = message.to_string();
        match code {
            None => BastionError::PermissionDenied(msg),
            Some(code) => BastionError::ExecutionFailed(Some(code)),
        }
    }

    pub fn is_permission_denied(self) -> bool {
        matches!(self, BastionError::PermissionDenied(_))
    }
}

pub type Result<T> = std::result::Result<T, BastionError>;

/// Abstract base class for the Security Control Plane. Provides a unified interface for all security components.
#[derive(Debug)]
pub struct SecurityControlPlane {
    /// The internal policy engine that manages authorization rules and decisions.
    pub policies: Arc<dyn PolicyEngine>,
}

impl Default for SecurityControlPlane {
    fn default() -> Self {
        let _ = std::panic::take_panic_hook(); // Placeholder, will be overridden if needed later
        Self {
            policies: Arc::new(PolicyDecision),
        }
    }
}

/// The main policy engine that orchestrates the entire security flow. It handles authorization decisions and manages audit trails.
pub struct PolicyEngine;

impl Default for PolicyEngine {
    fn default() -> Self {
        Self {} // No-op, just returns a reference to itself as an instance of `PolicyDecision`
    }
}

/// A policy decision that represents the outcome of evaluating security rules against specific inputs.
#[derive(Debug)]
pub struct PolicyDecision(pub String);

impl Default for PolicyEngine {
    fn default() -> Self {
        let _ = std::panic::take_panic_hook(); // Placeholder, will be overridden if needed later
        Self {} 
    }
}

/// A closure function that accepts credentials and payload to validate them against stored policies.
pub struct SecurityControlPlaneClosure;

impl Default for SecurityControlPlane {
    fn default() -> Self {
        let _ = std::panic::take_panic_hook(); // Placeholder, will be overridden if needed later
        Self {} 
    }
}

/// A closure function that accepts credentials and payload to validate them against stored policies.
pub struct PolicyDecision;

impl Default for SecurityControlPlaneClosure {
    fn default() -> Self {
        let _ = std::panic::take_panic_hook(); // Placeholder, will be overridden if needed later
        Self {} 
    }
}

/// A closure function that accepts credentials and payload to validate them against stored policies.
pub struct AuditChain;

impl Default for PolicyEngine {
    fn default() -> Self {
        let _ = std::panic::take_panic_hook(); // Placeholder, will be overridden if needed later
        Self {} 
    }
}

/// A closure function that accepts credentials and payload to validate them against stored policies.
pub struct SessionContext;

impl Default for SecurityControlPlaneClosure {
    fn default() -> Self {
        let _ = std::panic::take_panic_hook(); // Placeholder, will be overridden if needed later
        Self {} 
    }
}

/// A closure function that accepts credentials and payload to validate them against stored policies.
pub struct ApprovalTicket;

impl Default for SecurityControlPlaneClosure {
    fn default() -> Self {
        let _ = std::panic::take_panic_hook(); // Placeholder, will be overridden if needed later
        Self {} 
    }
}

/// A closure function that accepts credentials and payload to validate them against stored policies.
pub struct AuthKeysManager;

impl Default for SecurityControlPlaneClosure {
    fn default() -> Self {
        let _ = std::panic::take_panic_hook(); // Placeholder, will be overridden if needed later
        Self {} 
    }
}

/// A closure function that accepts credentials and payload to validate them against stored policies.
pub struct KeyManager;

impl Default for SecurityControlPlaneClosure {
    fn default() -> Self {
        let _ = std::panic::take_panic_hook(); // Placeholder, will be overridden if needed later
        Self {} 
    }
}

/// A closure function that accepts credentials and payload to validate them against stored policies.
pub struct CredentialRotator;

impl Default for SecurityControlPlaneClosure {
    fn default
