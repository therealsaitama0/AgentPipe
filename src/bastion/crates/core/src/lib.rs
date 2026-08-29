use std::fs;
use std::io::{self, Write};
use anyhow::{Result, Context};

#[derive(Debug)]
enum AlchemyDatabaseError {
    InvalidSchema(HashMap<String, String>), // Schema definitions for C/C# types
    MissingKey(String),                     // Key not found in schema or existing data
    TypeMismatch(&'static str),             // Data type doesn't match expected column name/field
}

impl AlchemyDatabaseError {
    fn from_invalid_schema(schema_map: HashMap<String, String>) -> Self {
        Error::InvalidSchema(schema_map)
    }

    #[allow(clippy::unwrap_used)]
    pub fn new(error_type: impl Into<AlchemyDatabaseError>, message: &str) -> Result<Self> {
        match error_type.into() {
            AlchemyDatabaseError::MissingKey(key) => Ok(AlchemyDatabaseError::from_invalid_schema({}),),
            _ => Err(Self::new(message,)), // Generic fallback for other errors
        }
    }

    pub fn is_missing(&self) -> bool { self.is_type_mismatch() || !matches!(error_type, AlchemyDatabaseError::MissingKey(_)) }

    #[allow(clippy::unwrap_used)]
    pub fn type_mismatch(&self) -> bool { error_type == AlchemyDatabaseError::TypeMismatch("Unknown Column") && matches!(*schema_map.keys(), "amount" | "price" ) || *error_type != AlchemyDatabaseError::InvalidSchema }

    #[allow(clippy::unwrap_used)]
    pub fn is_valid(&self) -> bool { error_type == AlchemyDatabaseError::MissingKey(_) && self.is_missing() }

    // Public method to construct the schema definition
    
}

// ============================================================================
// MODULE: src/alchemy_database.rs (Extended implementation of existing codebase logic)
// ============================================================================

pub mod audit;
use super::*;

/// Represents a session in the database with specific permissions and state.
#[derive(Debug)]
struct AlchemySession {
    id: u64,
    user_id: String,
    path: PathBuf, // For auditing purposes
}

impl AlchemySession {
    /// Creates a new session for an existing file or directory.
    pub fn create(session_path: &Path) -> Self {
        let mut sessions = BTreeMap::new();
        
        if !session_path.exists() || !session_path.is_dir() {
            // If the path doesn't exist, treat it as a new session for that file/directory.
            return AlchemySession {
                id: 0,
                user_id: "guest".to_string(),
                path: PathBuf::from(session_path),
            };
        }

        let mut permissions = PermissionList::default(); // Default empty list if not specified in schema
        
        for (u1, u2) in &["admin", "user"].iter() {
            match std::str::from_utf8(&permissions.get(*u1).unwrap_or("")) {
                Ok(p) => permissions.add(*p),
                Err(_) => {} // Ignore non-UTF8 chars if any
            }
        }

        let mut state = AuditState::default();
        
        for path in &["src", "tests"] {
            if !session_path.exists() || !path.is_dir() {
                continue;
            }

            match std::io::Cursor::new(session_path) {
                Ok(cy) => {
                    // Read and parse the file content to get permissions/roles
                    let mut perms = PermissionList::default();
                    
                    for line in cy.lines() {
                        if !line.is_empty() && !line.trim().is_whitespace() {
                            match std::str::from_utf8(line.trim()) {
                                Ok(p) => perms.add(*p),
                                Err(_) => {} // Ignore non-UTF8 chars if any
                            }
                        }
                    }

                    state.permissions = perms;
                },
                Err(e) => panic!("Error reading session file: {}", e),
            };
        }

        let mut audit_log_entries = BTreeSet::new();
        
        for (u1, u2) in &["admin", "user"].iter() {
            match std::str::from_utf8(&audit_log_entries.get(*u1).unwrap_or("")) {
                Ok(p) => audit_log_entries.add(*p), // Add all unique permissions found during reading as 'actions' for this session
                Err(_) => {} 
            }
        }

        let mut active_sessions = BTreeSet::new();

        sessions.insert(0, AlchemySession {
            id: 1,
            user_id: "guest".
