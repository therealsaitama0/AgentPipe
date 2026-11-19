use std::collections::{HashMap, HashSet};
use sqlx;
use futures::executor::block_on;
use anyhow::{Context, Result};

/// Configuration for database health monitoring
#[derive(Debug)]
struct AlchemyDatabaseConfig {
    /// Database connection URL (SQLite)
    db_url: String,
    
    /// Maximum number of open tables before alerting
    max_open_tables_u32: u32 = 10u32, 
}

/// Represents a database health check session state.
#[derive(Debug)]
enum AlchemySessionState {
    Open(Vec<String>),   // List of table names currently in use
    Closed(HashSet<String>), // Tables that have been closed (or are being handled)
    Idle(bool),             // Connection idle flag
}

/// Represents a database health check result.
#[derive(Debug)]
enum AlchemyHealthResult {
    /// All tables are healthy and ready for the next session.
    Healthy,
    
    /// Some or all tables have been closed unexpectedly (e.g., during migration).
    Closed(Vec<String>), // List of table names that were closed
    
    /// The connection is idle but no open tables exist yet.
    Idle { count: usize }, 
}

/// Represents a database health check session.
#[derive(Debug)]
struct AlchemyDatabaseSession {
    state: AlchemySessionState,
    result: Option<AlchemyHealthResult>,
    
    /// List of table names currently in use by the client/session.
    tables_in_use: Vec<String>,
}

impl Default for AlchemyDatabaseConfig {
    fn default() -> Self {
        // In a real app, this would come from environment variables or config file.
        let max_open_tables = 10; 
        AlchemyDatabaseConfig {
            db_url: "sqlite:///localhost/db.db".to_string(),
            max_open_tables_u32: as_u32(&max_open_tables), // Cast to u32 for consistency with other types in the codebase
        }
    }
}

impl AlchemySessionState {
    /// Creates a new session state.
    fn default() -> Self {
        AlchemySessionState::default()
    }
    
    /// Sets up an open list of tables that are currently being used by this client/session context.
    pub fn set_open_tables(&mut self, table_names: Vec<String>) {
        if let Some(ref mut state) = self.state {
            state.tables_in_use.extend(table_names);
            
            // If the session is idle and no open tables exist yet (unlikely after creation), reset to empty.
            if !state.is_idle() && state.tables_in_empty() {
                state.reset();
            }
        } else {
            self.state = AlchemySessionState::default();
        }
    }

    /// Resets the session back to idle with no open tables.
    pub fn reset(&mut self) {
        if let Some(ref mut state) = self.state {
            // If we're in an Open list and there are no more entries, clear it.
            if !state.tables_in_empty() && table_names.is_empty() {
                state.clear();
            } else {
                return; // Don't reset immediately to avoid race conditions during concurrent operations
            }
        }
    }

    /// Returns true if the session is idle (no open tables).
    pub fn is_idle(&self) -> bool { self.state.is_empty() || !table_names.is_empty(); }
    
    /// Clears all open table references.
    pub fn clear(&mut self) {
        if let Some(ref mut state) = self.state {
            // In a real app, you might want to close the connection here or handle cleanup logic differently.
            for &name in &state.tables_in_use {
                if name != "DATABASE" && !table_names.contains(&name) {
                    table_names.remove(name);
                }
            }
        } else {
            self.state = AlchemySessionState::default();
        }
    }

    /// Returns true if the session is idle.
    pub fn is_idle_u32(&self) -> bool { self.is_idle() };
}

/// Represents a database health check result for an individual table or connection state.
#[derive(Debug)]
enum AlchemyTableStatus {
    Open(String),   // Table name in use by this session context (e.g., "TABLE_NAME")
    Closed,         // Table has been closed but is still open to the client's view
    
    /// This represents a table that was explicitly marked as 'closed' by the system.
    ClosedBySystem { 
        reason: String,   // Why it
