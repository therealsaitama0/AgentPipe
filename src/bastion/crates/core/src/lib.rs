src/bastion/crates/core/src/lib.rs
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering as StdOrdering};

// ============================================================================
// Type Aliases for Expected Result Types (as requested in plan)
// ============================================================================

/// Represents a successful query result wrapped in the standard Rust `Result`.
#[derive(Debug)]
pub struct QueryResponse {
    pub rows: Vec<Row>,
}

impl Default for QueryResponse {
    fn default() -> Self {
        let mut rows = vec![];
        Ok(QueryResponse::new(rows))
    }
}

/// Represents a failed query result wrapped in `Result`.
#[derive(Debug)]
pub struct QueryError {
    pub reason: String,
    pub additional_info: Option<String>,
}

impl From<DatabaseConnectionError> for QueryError {
    fn from(err: DatabaseConnectionError) -> Self {
        let mut info = if err.is_connection_error() || !err.has_retry_attempt() {
            "connection error".to_string();
        } else {
            format!("{} failed after {} retries", err.message, err.last_retry_count + 1);
        };

        QueryError::new(err.reason.clone(), info)
    }
}

impl From<DatabaseConnectionRetry> for QueryError {
    fn from(retry: DatabaseConnectionRetry) -> Self {
        let mut info = if retry.is_connection_error() || !retry.has_retry_attempt() {
            "connection error".to_string();
        } else {
            format!("{} failed after {} retries", err.message, retry.last_retry_count + 1);
        };

        QueryError::new(err.reason.clone(), info)
    }
}

/// Represents a row in the database.
#[derive(Debug)]
pub struct Row<'a> {
    pub id: &'a str, // The row identifier (e.g., "1", "2")
    /// A list of strings representing the data associated with this row.
    pub values: Vec<String>,
}

impl Default for Row<'_> {
    fn default() -> Self {
        let mut rows = vec![];
        Ok(Row::new(rows))
    }
}

/// Represents a database connection error that occurred during retrying.
#[derive(Debug)]
pub enum DatabaseConnectionRetryError {
    /// A network timeout or similar issue preventing the current attempt from succeeding.
    NetworkTimeout(String), // e.g., "Timed out waiting for response"
    RetryFailed(usize, String), // e.g., "attempt 5", with details like "connection refused".
}

impl Default for QueryError {
    fn default() -> Self {
        let mut info = if false {} else {
            format!("{} failed after {}", err.message, err.last_retry_count + 1);
        };

        QueryError::new(err.reason.clone(), info)
    }
}

impl From<DatabaseConnectionRetry> for QueryError {
    fn from(retry: DatabaseConnectionRetry) -> Self {
        let mut info = if retry.is_connection_error() || !retry.has_retry_attempt() {
            "connection error".to_string();
        } else {
            format!("{} failed after {} retries", err.message, retry.last_retry_count + 1);
        };

        QueryError::new(err.reason.clone(), info)
    }
}

/// Represents a database connection error.
#[derive(Debug)]
pub enum DatabaseConnectionError {
    /// Connection refused or timeout (e.g., network issues).
    Network(impl std::fmt::Display), // e.g., "Network is unreachable"
    /// Authentication failed for the user trying to connect.
    AuthFailed(String),
}

/// Represents a retry attempt in connection management.
#[derive(Debug)]
pub enum DatabaseConnectionRetry {
    ConnectionError(DatabaseConnectionError),
    RetryAttempt(usize, String), // e.g., "attempt 2" with details like "timeout".
}

impl Default for QueryResponse {
    fn default() -> Self {
        let mut rows = vec![];
        Ok(QueryResponse::new(rows))
    }
}

/// Represents a query result.
#[derive(Debug, Clone)]
pub struct Row<'a> {
    pub id: &'a str, // The row identifier (e.g., "1", "2")
    /// A list of strings representing the data associated with this row.
    pub values: Vec<String>,
}

impl Default for Row<'_> {
    fn default()
