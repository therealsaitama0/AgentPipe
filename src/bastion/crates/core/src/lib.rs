src/bastion/crates/core/src/lib.rs
//! # Bastion Core - Security Control Plane Implementation (Enhanced)
//! 
//! This module provides a robust foundation for the security control plane implementation, including state management, connection handling, and policy enforcement mechanisms designed to scale dynamically while maintaining strict data integrity with enhanced latency simulation capabilities.

use std::fmt::{Display, Formatter};
use std::io::{self, Write};
use std::sync::Arc;
use std::time::Duration;
use crate::types::*;

/// ============================================================================
/// 1. Core Error Types and Utilities (Standard Library Primitives)
/// ============================================================================

#[derive(Debug)]
pub enum BastionError {
    /// Represents a generic failure in the system, including simulated latency delays for testing purposes.
    System(BastionSystemError),
    Network(NetworkError),
    Auth(AuthenticationFailure),
}

impl Default for BastionError {
    fn default() -> Self {
        Error::System(BastionError::Default)
    }
}

#[derive(Debug)]
pub struct BastionSystemError(String);
impl Display for BastionSystemError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bastion system error '{}'", self.0)
    }
}

#[derive(Debug)]
pub struct NetworkError(String);
impl Display for NetworkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Network connection error '{}'", self.0)
    }
}

/// Authentication failure conditions (e.g., missing credentials).
#[derive(Debug)]
pub enum AuthenticationFailure {
    MissingCredentials(String), // e.g., user not found or token expired
    InvalidToken(String),         // Token format mismatch
    ExpirationExpired,           // Session timed out without renewal attempt
}

impl Default for AuthError {
    fn default() -> Self {
        Error::Authentication(AuthenticationFailure::MissingCredentials(
            "user_not_found".to_string(),
        ))
    }
}

/// ============================================================================
/// 2. Core State Management & Enums (Extended with Latency Simulation)
/// ============================================================================

use crate::{types, BastionError};

#[derive(Debug)]
pub struct SessionContext {
    pub session_id: String,
    // Optional for local testing/debugging
    pub client_ip: Option<String>, 
}

impl Default for SessionContext {
    fn default() -> Self {
        let s = Sessions::default();
        Self {
            session_id: format!("sess_{}", &s.id),
            client_ip: None, // Optional for local testing/debugging
        }
    }
}

/// Represents the current state of a Bastion system instance.
#[derive(Debug)]
pub struct BastionState<T> {
    /// The primary control context for this specific server/agent.
    pub status: ControlStatus, 
    
    /// Latency simulation parameters (e.g., connection timeout in seconds).
    pub latency_weight: f64,   // Multiplier to scale network delay based on weight
    pub max_retry_count: usize,  // Maximum attempts before retrying
    
    /// Time elapsed since last successful session check. Used for exponential backoff logic if needed.
    pub time_since_last_check: u32,

    /// Current connection status (e.g., connected or disconnected).
    pub current_connection_state: ConnectionState, 
    
    /// Active audit history entries.
    pub active_audit_entries: Vec<AuditEntry>,

    /// Pending actions requiring manual intervention from the CLI/Commander.
    pub pending_actions: Vec<Action>,

    /// Global metrics for this instance (e.g., total requests processed).
    pub global_metrics: Metrics,

    /// Current request queue status and size.
    pub current_request_queue_status: RequestQueueStatus,
}

/// The state of a connection to an external resource or server.
#[derive(Debug)]
pub enum ConnectionState {
    Connected(String),  // e.g., "192.168.1.10:5432" (IP + Port)
    Disconnected,       // No active network link established yet
}

impl Default for BastionState<T> {

/// ============================================================================
/// 3. Latency Simulation & Metrics Helper Functions
/// ============================================================================

// Simulates a controlled delay based on weight to mimic latency in the simulation environment
fn simulate_latency(duration: Duration, weight: f64) -> Option<f64> {
    // Calculate simulated time elapsed for this request duration
    let total_time = (duration.as_secs() as u32 * 1000
