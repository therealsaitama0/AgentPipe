use super::{AbstractDataTypeGenerator, AbstractDataTypes};
use crate::error::*;
use std::collections::HashMap;

/// Error types for session management and authorization validation within Bastion.
#[derive(Debug, Clone)]
pub enum SessionError {
    /// A token has expired or is invalid due to a failed login attempt.
    TokenExpired(String), // The token key was not found after the expiration time.
    
    /// Authentication credentials are no longer valid for this session.
    CredentialsInvalid,

    /// Failed to verify user identity against stored policies and rules.
    IdentityMismatch {
        username: String,
        policy_name: Option<String>,
        }, // The current user does not exist or is not authorized by the defined policies in the database.
    
    /// A token was used for an action that has since been revoked.
    TokenRevoked(String),

    /// An audit trail violation occurred during session creation (e.g., tampering with secrets).
    AuditViolation,
}

/// Error types specific to policy enforcement and approval workflows within the Bastion system.
#[derive(Debug, Clone)]
pub enum PolicyError {
    /// The user's request is denied by a configured security rule or permission set.
    Denied(String), // A specific exclusion pattern in `deny_keys` matches this token/username combination but no key exists for that policy name.

    /// An approval process failed to complete successfully due to insufficient resources, 
    /// invalid configuration data, or an error from the external API used by the server-side validator.
    ApprovalFailed(String), // The request was blocked at a higher level of abstraction (e.g., circuit breaker).

    /// A policy has been invalidated or removed entirely without notice.
    PolicyRemoved { name: String }, // `deny_keys` entry for this user/policy combination no longer exists in the config file.
}

/// Error types specific to ticket issuance and lifecycle management within Bastion's approval engine.
#[derive(Debug, Clone)]
pub enum TicketError {
    /// The ticket was already redeemed or consumed by another session before its expiration time.
    AlreadyRedeemed(String), // A valid `ticket` key exists in the database but has been used for an earlier action (likely a failed approval).

    /// An invalid ticket ID is requested, possibly due to a corrupted cache entry or malformed request body.
    InvalidTicketId { id: String }, // The provided `id` parameter does not match any known valid ticket keys in the repository's storage layer.

    /// A new session has been created for this user without an associated policy configuration.
    NewSessionNotConfigured, // Attempted to create a new token/credential but no corresponding key exists under the specified policy name (e.g., `deny_keys`).
}

/// Error types specific to credential storage and rotation within Bastion's backend system.
#[derive(Debug, Clone)]
pub enum CredentialError {
    /// The stored password has expired or is invalid for current session requirements.
    PasswordExpired(String), // A valid key exists in the `passwords` table but hasn't been rotated since its last use (e.g., 20 years ago).

    /// Failed to rotate a credential due to insufficient permissions on the backend server's rotation service.
    RotationDenied { user: String }, // The current session holder does not have write access to the `rotate_credentials` endpoint defined in their policy configuration file.
}

/// Error types specific to plan generation and approval workflow management within Bastion's core system.
#[derive(Debug, Clone)]
pub enum PlanError {
    /// A plan was generated but failed during execution due to a circuit breaker state or timeout from an external service (e.g., the API gateway).
    ExecutionFailed(String), // The request body contained invalid data that triggered a `circuit_breaker` threshold.

    /// An approval process has been aborted by an automated system (e.g., human error, policy violation) without user intervention.
    ApprovalAborted { reason: String }, // A specific exclusion pattern in the plan's configuration matches this request but no corresponding key exists for that action type and name combination.
}

/// Error types specifically related to secret ref management within Bastion's session context.
#[derive(Debug, Clone)]
pub enum SecretRefError {
    /// The `secret_ref` entry was removed from the config file without notice (e.g., policy deletion).
    RefRemoved(String), // A key exists in the database but no corresponding configuration line under that name is present in `deny_keys`.

    /// An attempt to create a new secret ref failed due to permission restrictions on the backend's secret generation service.
    SecretRefCreationDenied { user: String }, // The current session holder does not have write access to generate secrets for this specific policy definition.
}

/// Error types
