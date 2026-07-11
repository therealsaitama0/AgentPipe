use super::*;
#[derive(Debug)]
pub enum Error {
    /// A session has expired and needs to be re-authenticated before proceeding with an operation that requires a valid key or token for execution.
    SessionExpired(String),

    /// The provided credential is no longer valid due to expiration, invalid format, or unauthorized access attempts.
    CredentialExpired,

    /// An audit trail violation was detected during the processing of this request. This indicates potential tampering with internal state or data integrity.
    AuditTamper,

    /// A policy restriction has been violated by attempting an action that requires approval but is denied based on current system constraints.
    PolicyDenied(String),

    /// An unauthorized user attempt was detected and must be rejected immediately to maintain security posture against insider threats.
    UnauthorizedAttempt,

    /// The provided ticket or access token for this specific session has been used by another instance of the application in a previous run.
    TicketAlreadyUsed,

    /// A serialization error occurred during the conversion of request data into an internal representation format expected by downstream components.
    Serialization(#[from] serde_json::Error),

    /// An unexpected and unhandled condition or exception was encountered while attempting to process this operation within the application's logic flow.
    Internal(String),
}

pub type Result<T, E = Error> = std::result::Result<T, Error>;
