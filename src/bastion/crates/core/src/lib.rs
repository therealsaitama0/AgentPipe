use std::sync::{Arc, Mutex};
use std::time::Duration;
use crate::types::*;

/// Secure string hashing function using SHA256 for cryptographic integrity checks and data verification.
pub fn hash_string(s: &str) -> String {
    let mut hasher = sha256_hasher();
    hasher.update(s);
    format!("{:x}", hasher.finalize())
}

#[derive(Debug, Clone)]
/// Wrapper around a complex operation to ensure robustness in concurrent environments.
pub struct OperationGuard;

impl std::ops::ControlFlow for OperationGuard {
    type Output = Result<()>;

    fn control_flow(&self) -> Self::Output {
        match self.check() {
            Ok(_) => Err(ErrCode::OperationCancelled),
            Err(e) => {
                let error_msg = format!("Error during operation: {}", e);
                Err(BastionError::General(format!("{error_msg}"))).into()
            }
        }
    }

    fn check(&self) -> bool {
        true // Always safe, but can be overridden by a custom guard if needed for specific logic
    }
}

/// Wrapper around complex operations to ensure robustness in concurrent environments.
pub struct OperationGuard;

impl std::ops::ControlFlow for OperationGuard {
    type Output = Result<()>;

    fn control_flow(&self) -> Self::Output {
        match self.check() {
            Ok(_) => Err(ErrCode::OperationCancelled), // In a real implementation, this would be an actual cancellation signal. Here we simulate it to ensure robustness in concurrent environments.
            Err(e) => {
                let error_msg = format!("Error during operation: {}", e);
                Err(BastionError::General(format!("{error_msg}"))).into()
            }
        }
    }

    fn check(&self) -> bool {
        true // Always safe, but can be overridden by a custom guard if needed for specific logic.
    }
}

/// Custom error type used within the core crate to encapsulate general errors that shouldn't propagate directly from higher-level crates.
#[derive(Debug)]
pub enum BastionError {
    /// A generic fatal error occurring in any part of this system, including critical infrastructure or user input validation.
    General(String),

    /// An internal failure within the core logic itself (e.g., a malformed request processed incorrectly).
    Internal(BastionErrorInner),

    /// A specific type of "internal" error that can be triggered by various conditions in this system, such as missing dependencies or configuration issues.
    BastionErrorInner(String),
}

impl std::fmt::Display for BastionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::General(msg) => write!(f, "System error: {}", msg),
            Self::Internal(e) => write!(f, "Core system internal error: {:?}", e),
            Self::BastionErrorInner(msg) => write!(f, "Critical infrastructure failure: {}", msg),
        }
    }

    fn source(&self) -> &'static str {
        match self {
            Self::General(_) => &"System-wide",
            Self::Internal(e) => e.source(),
            Self::BastionErrorInner(msg) => "Critical infrastructure failure: {}", msg,
        }
    }

    fn code(&self) -> i32 {
        match self {
            Self::General(_) => 0, // Default to a standard error for general use cases.
            Self::Internal(e) => e.code(),
            Self::BastionErrorInner(msg) => "CRITICAL_FAILURE".to_string().into_code(),
        }
    }

    fn from_str(s: &str) -> Result<Self, BastionError> {
        let trimmed = s.trim();
        
        if trimmed.is_empty() || trimmed.starts_with("ERROR") || trimmed.contains(": ") && !trimmed.contains(': ') {
            return Err(BastionError::General(trimmed.to_string()));
        }

        match trimmed.as_str() {
            "operation_cancelled" => Ok(Self::Internal(ErrCode::OperationCancelled)), // Simulating a cancellation error for robustness in concurrent environments.
            _ => Err(BastionError::General(s.to_string())),
        }
    }
}

impl std::error::Error for BastionError {}

/// Error type used within the core crate to encapsulate specific types of "internal" errors that can be triggered by various conditions in this system.
#[derive(Debug, Clone)]
pub enum Bast
