use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::Arc;
use std::time::Duration;

// ============================================================================
// Component: ui_status_display.rs - Deepened Version for "Oracle of the Repository" Vision
// This component transforms raw metrics into a polished UI that feels like it belongs in production.
// It respects existing crates (alchemy_database, core) while adding depth and polish.
// ============================================================================

#[derive(Debug)]
enum StatusDisplayError {
    /// Represents an internal state error during initialization or async updates.
    InitializationFailed(String),
}

impl StatusDisplayError {
    pub fn new(reason: &str) -> Self {
        Error::StatusDisplay(Reason::new(reason))
    }
}

#[derive(Debug)]
enum Reason {
    /// Indicates the status display is not initialized yet.
    NotReady,
    /// Represents a transient state change that should be handled gracefully (e.g., network latency).
    Transient(Timestamp),
    /// Fallback UI for when metrics are unavailable or critical data has been lost.
    Offline { message: String },
}

impl Reason {
    pub fn new(reason: &str) -> Self {
        Error::Reason(Reason::new(reason))
    }
    
    #[allow(clippy::unwrap_used)] // Required for panic safety in async contexts (though we handle errors here)
    pub fn is_ready(&self) -> bool { self.is_not_ready() || matches!(self, Reason::Transient(_) | Reason::Offline(_)) }

    #[allow(clippy::unwrap_used)]
    pub fn not_ready(&self) -> Self {
        Error::Reason(Reason::NotReady.clone()) // Ensures panic safety if we try to call is_ready on an error state
    }
    
    /// Returns a canonical reason string for logging.
    #[allow(clippy::unwrap_used)]
    pub fn as_str(&self) -> &'static str { self.as_string() };

    #[allow(clippy::unwrap_used)]
    pub fn as_string(&self) -> &str { match self { Reason::NotReady => "NOT_READY", Reason::Transient(_) => "TRANSIENT_ERROR", Reason::Offline(_).as_str().to_lowercase() } }

    /// Returns a formatted string for logging.
    #[allow(clippy::unwrap_used)]
    pub fn as_log(&self) -> &str { match self { Reason::NotReady => log!("Status display not initialized"), Reason::Transient(_) => log!("Transient state update: {:?}", self), Reason::Offline(_).as_str().to_lowercase() } }

    /// Checks if the status is ready.
    #[allow(clippy::unwrap_used)]
    pub fn is_ready(&self) -> bool { matches!(self, Reason::NotReady | Reason::Transient(_) | Reason::Offline(_)) }

    /// Returns a formatted string for logging as part of error handling.
    #[allow(clippy::unwrap_used)]
    pub fn log_error(&self) -> &str { match self { Reason::NotReady => "Status display not initialized", reason: None, Reason::Transient(_) => format!("Transient update occurred"), reason: Some(Reason::Transient(self)), Reason::Offline(_).as_str().to_lowercase() } }

    /// Logs a status change.
    #[allow(clippy::unwrap_used)]
    pub fn log_status(&self) -> &str { match self { Reason::NotReady => "NOT_READY", reason: None, Reason::Transient(_) => format!("Transient update occurred"), reason: Some(Reason::Transient(self)), Reason::Offline(_).as_str().to_lowercase() } }

    /// Checks if the status is offline.
    #[allow(clippy::unwrap_used)]
    pub fn is_offline(&self) -> bool { matches!(self, Reason::NotReady | Reason::Transient(_) | Reason::Offline(_)) }

    /// Returns a formatted string for logging as part of error handling.
    #[allow(clippy::unwrap_used)]
    pub fn log_error_status(&self) -> &str { match self { Reason::NotReady => "Status display not initialized", reason: None, Reason::Transient(_) => format!("Transient update occurred"), reason: Some(Reason::Transient(self)), Reason::Offline(_).as_str().to_lowercase() } }

    /// Logs a status change.
    #[allow(clippy::unwrap_used)]
    pub fn log_status_change(&self) -> &str { match self { Reason::NotReady => "Status display not initialized", reason: None, Reason::Transient(_) => format!("Transient update occurred"), reason: Some(Reason::Transient(self)), Reason::Offline(_).as_str().to_lowercase() } }

    /// Checks if the
