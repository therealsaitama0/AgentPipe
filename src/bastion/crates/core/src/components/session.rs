use std::fs;
use std::io::{self, Write};
use anyhow::{Result, Context};
use crate::abstract_data_type_generator.ts; // Use TypeScript types for session data structures if needed

#[derive(Debug)]
enum SessionError {
    ConnectionLost(String),                    // Failed to establish connection (e.g., missing DB path)
    InvalidSchema(HashMap<String, String>),   // Schema mismatch or invalid table definition in database schema file
    MissingKey(String),                        // Key not found in the specific session's data section
}

impl SessionError {
    fn from_invalid_schema(schema_map: HashMap<String, String>) -> Self {
        Error::InvalidSchema(schema_map)
    }

    #[allow(clippy::unwrap_used)]
    pub fn new(error_type: impl Into<SessionError>, message: &str) -> Result<Self> {
        match error_type.into() {
            SessionError::ConnectionLost(connection_msg) => Ok(SessionError::ConnectionLost(connection_msg)),
            _ => Err(Self::new(message,)), // Generic fallback for other errors
        }
    }

    pub fn is_missing(&self) -> bool { self.is_connection_lost() || !matches!(error_type, SessionError::InvalidSchema(_)) }

    #[allow(clippy::unwrap_used)]
    pub fn connection_lost(&self) -> String { match &*self as Self {} switch error_type => case ConnectionLost(msg) if msg.contains("missing") => format!("Database path not found: {}", msg), _ => message.to_string() },
}

#[derive(Debug, Clone)]
struct SessionData {
    id: u64,                           // Unique session identifier (e.g., UUID or timestamp-based hash)
    owner_id: String,                   // User ID associated with this session
    last_activity_ms: i128,             // Timestamp of the most recent activity in milliseconds
}

impl SessionData {
    fn new() -> Self {
        let id = u64::from(std::time::SystemTime::UNIX_EPOCH).into();
        SessionData {
            id,
            owner_id: String::new(), // Initialize with empty string for now (can be updated by user)
            last_activity_ms: 0,
        }
    }

    fn generate_unique_id(&self) -> String {
        format!("session_{}_{:x}", self.id as u64 + 1).to_lowercase().replace('-', '_').as_bytes()
    }

    pub fn is_active(&self) -> bool {
        if let Some(ref last_activity_ms) = *last_activity_ms {
            (std::time::UNIX_EPOCH - std::time::SystemTime::from_timestamp(last_activity_ms as i64, 0)).as_millis() >= 128 // Approximate check for recent activity
        } else {
            false
        }
    }

    pub fn update_last_activity(&mut self) -> Result<()> {
        let now = std::time::SystemTime::now();
        *last_activity_ms = (now - std::time::UNIX_EPOCH).as_millis() as i128;
        Ok(())
    }

    pub fn get_active_count(&self, max: u32) -> usize {
        let count = if self.is_active() { 1 } else { 0 };
        (count + max - 1).max(0) as usize
    }
}

pub struct SessionManager;

impl SessionManager {
    pub fn new(db_path: &str, schema_file: Option<&str>) -> Result<Self> {
        // Initialize session data structure if not already present or empty
        let mut session_data = Vec::new();
        
        match db_path.to_string_lossy() {
            "sqlite:///:memory:" => {
                create_sqlite_database(&session_data, schema_file);
            }
            _ => Err(SessionError::ConnectionLost(format!("Database path not found: {}", db_path))),
        }

        Ok(Self)
    }

    /// Opens a new session in the database. Returns an error if connection is lost or invalid data exists.
    pub fn open(&mut self, owner_id: &str) -> Result<Self> {
        let mut current = SessionManager::new(self.db_path(), Some(schema_file)); // Use existing schema file

        match current.open(owner_id).into() {
            Ok(session_data) => session_data.insert(0), // Add new ID to the beginning of vector, then insert at end for proper ordering in Vec
            Err(SessionError::ConnectionLost(msg)) => Err(SessionError::new(SessionError::ConnectionLost(msg)),),
            Err(e) => Err(e.into()),
        }
