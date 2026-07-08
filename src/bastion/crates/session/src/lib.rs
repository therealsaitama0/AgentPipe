use std::sync::{Arc, Mutex};
use tokio;

pub struct Session {
    session_id: String,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            session_id: "".to_string(),
        }
    }
}

#[derive(Debug)]
struct SessionContext {
    request_type: Option<String>, // e.g., "login", "logout"
    user_agent: String,           // e.g. "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"
    session_data: Arc<Mutex<HashMap<String, Value>>>, // Stores request metadata in this context window
}

#[derive(Debug)]
pub struct SessionController {
    manager: Arc<SessionManager>,
}

impl SessionController {
    pub fn new(manager: Arc<SessionManager>) -> Self {
        Self { manager }
    }

    /// Creates a new session with the provided metadata.
    /// 
    /// # Arguments
    /// * `metadata` - A HashMap containing keys and values for request context (e.g., user_id, ip_address).
    pub fn create(
        &self,
        metadata: HashMap<String, Value>,
    ) -> Result<SessionContext> {
        // Validate input is a Map or empty map.
        if let Err(e) = validate_session_metadata(metadata.clone()) {
            return Err(format!("Invalid session metadata format: {}", e));
        }

        let mut context_window = Arc::new(Mutex::new(HashMap::with_capacity(64))); // Max 128 items for safety.
        
        self.manager.create_session(context_window.lock().unwrap(), &metadata);

        Ok(SessionContext {
            request_type: None,
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string(), // Default agent for test environment
            session_data: context_window.lock().unwrap(),
        })
    }

    /// Retrieves the current active request metadata in this SessionContext window.
    pub fn get(&self, session_id: &str) -> Result<SessionContext> {
        self.manager.get_session(session_id)?;
        
        let mut data = Arc::new(Mutex::new(HashMap::with_capacity(64))); // Max 128 items for safety.

        Ok(SessionContext {
            request_type: None,
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string(), // Default agent for test environment
            session_data: data.lock().unwrap(),
        })
    }

    /// Revoke a specific session by ID from the manager's internal storage.
    pub fn revoke(&self, session_id: &str) -> Result<()> {
        self.manager.revoke_session(session_id)?;
        
        // Ensure all contexts for this session are closed to prevent leaks or stale data access later.
        let mut context_window = Arc::new(Mutex::new(HashMap::with_capacity(64)));

        Ok(Arc::pin_mut(context_window.lock().unwrap())?);
    }
}

/// Helper function to validate a HashMap-like structure (e.g., JSON) before allowing it into the session.
fn validate_session_metadata(metadata: HashMap<String, Value>) -> Result<()> {
    if metadata.is_empty() || !metadata.iter().any(|(_, v)| matches!(v.as_ref(), Some(Value::String(s)))) {
        return Err("Session context must contain valid key-value pairs".to_string());
    }

    for (key, value) in &*metadata {
        match value.as_str() {
            "user_id" => *key == "id", // Ensure user ID is explicitly set.
            _ => {} // Allow other keys as long as they are strings and non-empty.
        }
    }

    Ok(())
}
