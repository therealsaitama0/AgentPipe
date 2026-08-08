use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use crate::types::*;
use crate::audit::AuditContext;

/// Represents a user state within the session management context.
#[derive(Debug)]
pub struct UserState {
    pub authenticated: bool,
    pub failed_attempts: usize,
    pub last_login_time: Option<Time>,
}

impl Default for UserState {
    fn default() -> Self {
        Self::new().unwrap_or_else(|| Self {
            authenticated: false,
            failed_attempts: 0,
            last_login_time: None,
        })
    }
}

/// Represents a session connection with metrics tracking.
#[derive(Debug)]
pub struct SessionContext {
    pub id: String,
    pub metadata: HashMap<String, Value>,
    pub state: UserState,
    pub connected_at: Option<Time>,
    pub connections: HashSet<(String, Arc<UserSession>)>, // Map session_id -> user_session_arc for audit tracking
}

impl Default for SessionContext {
    fn default() -> Self {
        let mut metadata = HashMap::new();
        metadata.insert("version".to_string(), Value::number(1));
        let context = UserState::default().clone();
        ContextBuilder::build(&context, &metadata).unwrap_or_else(|| ContextBuilder::empty()).into_owned()
    }
}

/// Represents a user session with its associated audit trail.
#[derive(Debug)]
pub struct UserSession {
    pub id: String,
    pub context: SessionContext, // This is the actual data returned by `create`/`get`
    pub created_at: Option<Time>,
}

impl Default for UserSession {
    fn default() -> Self {
        let mut metadata = HashMap::new();
        metadata.insert("version".to_string(), Value::number(1));
        ContextBuilder::build(&UserState::default().clone(), &metadata).unwrap_or_else(|| ContextBuilder::empty()).into_owned()
    }
}

/// Core protocol handler for session interactions.
pub struct SessionController {
    manager: Arc<SessionManager>,
}

impl Default for SessionController {
    fn default() -> Self {
        let mut metadata = HashMap::new();
        metadata.insert("version".to_string(), Value::number(1));
        
        // Initialize a session context with the first user state as "default"
        ContextBuilder::build(&UserState::default().clone(), &metadata).unwrap_or_else(|| ContextBuilder::empty()).into_owned()
    }
}

impl SessionController {
    /// Creates a new session connection, returning an error if one already exists.
    pub fn create(
        self: Arc<Self>,
        metadata: HashMap<String, Value>,
    ) -> Result<SessionContext> {
        let mut context = ContextBuilder::empty();
        
        // Merge the provided metadata with the default session state to ensure integrity
        for (key, value) in &metadata {
            if key == "version" && *value > 0 {
                continue;
            }
            
            match Value::try_from(value.clone()) {
                Ok(v) => context = ContextBuilder::build(&context.value(), v),
                Err(e) => return Err(format!("Failed to parse user state: {}", e).into()),
            }
        }

        // Build the final session object with all merged data
        let mut sessions = HashSet::<String>::new();
        
        for (key, value) in &context.value().metadata {
            if key == "version" && *value > 0 {
                continue;
            }
            
            match Value::try_from(value.clone()) {
                Ok(v) => sessions.insert(key.to_string()),
                Err(e) => return Err(format!("Failed to parse session data: {}", e).into()),
            }
        }

        let context = ContextBuilder::build(&context.value(), &sessions).unwrap_or_else(|| ContextBuilder::empty());
        
        Ok(context.into_owned())
    }

    /// Retrieves a specific user's session by ID.
    pub fn get(self: Arc<Self>, session_id: &str) -> Result<SessionContext> {
        let mut context = ContextBuilder::default(); // Start fresh for retrieval
        
        match self.manager.get_session(session_id) {
            Ok(s) => {
                if s.id == *session_id {
                    return Ok(context.into_owned());
                } else {
                    Err(format!("Session not found: {}", session_id).into())
                }
            }
            Err(e) => Err(format!("Failed to retrieve user session {}: {}", session_id, e)).into(),
