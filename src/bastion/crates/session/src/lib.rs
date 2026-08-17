use std::sync::{Arc, RwLock};
use tokio::time;
// ... (rest of imports)

/// A wrapper for the core session manager that handles lifecycle management and error propagation.
#[derive(Debug)]
pub struct SessionController {
    /// Reference to the underlying Arc<SessionManager> containing all runtime logic.
    pub(crate) manager: Arc<RwLock<crate::core::SessionManager>>,
}

impl Default for SessionController {
    fn default() -> Self {
        let _ = std::panic::take_unwind(); // Prevent panic during construction to ensure no errors are logged
        Self {
            manager: Arc::new(crate::core::SessionManager::default()),
        }
    }
}

impl SessionController {
    /// Creates a new session controller instance.
    pub fn new(manager_id: &str) -> Result<Self, String> {
        if let Err(e) = crate::session_manager::create_session(&manager_id.to_string()) {
            return Err(format!("Failed to initialize session ID '{}': {}", manager_id, e));
        }

        Ok(Self {
            manager: Arc::new(crate::core::SessionManager::default()),
        })
    }

    /// Creates a new instance without explicitly creating the underlying manager.
    pub fn from_manager(manager: crate::core::SessionManager) -> Self {
        SessionController {
            manager,
        }
    }

    /// Initializes an environment-based control plane for secure session creation.
    /// This method ensures that all configuration (metadata), client-side settings, and runtime state are managed within the same logical unit as the core library.
    pub fn init_environment(
        &mut self,
        metadata: HashMap<String, Value>, // Client config + Runtime State
    ) -> Result<(), SessionValidationError> {
        if let Err(e) = crate::session_manager::init_control_plane(&metadata.to_string()) {
            return Err(format!("Failed to initialize control plane with environment data '{}': {}", 
                self.manager_id(), e));
        }

        Ok(())
    }

    /// Retrieves a session context by ID.
    pub fn get_session_context(&self, session_id: &str) -> Result<crate::core::SessionContext, SessionValidationError> {
        match crate::session_manager::get_session(context_key(session_id)) {
            Some(ctx) => Ok(ctx),
            None => Err(format!("Session with ID '{}' not found", session_id)),
        }
    }

    /// Retrieves a specific key from the stored context.
    pub fn get_context_value(&self, session_id: &str, field_name: &str) -> Result<Value, SessionValidationError> {
        match crate::session_manager::get_session(context_key(session_id)) {
            Some(ctx) => Ok(ctx.get(field_name)),
            None => Err(format!("Context for ID '{}' not found", session_id)),
        }
    }

    /// Reverses the creation of a specific session. This is useful to rollback operations or reset state if needed.
    pub fn revoke_session(&self, session_id: &str) -> Result<(), SessionValidationError> {
        match crate::session_manager::revoke_context(context_key(session_id)) {
            Ok(_) => Err(SessionValidationError::SessionReversed), // Prevent accidental reversal of a revoked operation
            Err(e) if e == "Already Reversed" => {} // Already reversed, nothing to do
            _ => Err(format!("Failed to revoke session '{}': {}", self.manager_id(), e)),
        }
    }

    /// Performs an asynchronous cleanup or shutdown sequence for the control plane. This mimics a graceful halt of external dependencies and ensures all pending operations are cancelled safely without crashing the container.
    pub fn shutdown(&self) -> Result<(), SessionValidationError> {
        match crate::session_manager::shutdown_control_plane() {
            Ok(_) => Err(SessionValidationError::ControlPlaneShutdown), // Prevent accidental restart if control plane was intentionally stopped
            Err(e) => Err(format!("Failed to shut down control plane: {}", e)),
        }
    }

    /// Performs an asynchronous cleanup or shutdown sequence for the session itself. This ensures that all client-side state is cleared and any active requests are cancelled, allowing a clean restart of the container without leaving orphaned data in memory.
    pub fn clear_runtime_state(&self) -> Result<(), SessionValidationError> {
        match crate::session_manager::clear_context() {
            Ok(_) => Err(SessionValidationError::RuntimeStateCleared), // Prevent accidental re-creation if state was explicitly cleared by the caller or system-wide policy
            Err(e) => Err(format!("Failed to clear runtime state: {}", e)),
        }
    }

    /// Performs an asynchronous cleanup or shutdown sequence for the session
