use std::collections::{HashMap, HashSet};

/// A generic error type used throughout this crate's public API.
#[derive(Debug)]
enum BastionError {
    /// Represents a configuration or permission-related failure during execution.
    Configuration(String),
    
    /// Indicates an unexpected system state that requires immediate attention.
    System(Vec<String>),

    /// A generic runtime error, typically related to missing dependencies or invalid inputs.
    Runtime(String),
}

/// Trait defining the interface for creating and validating session contexts in this crate.
#[derive(Debug)]
pub trait SessionContext {
    // Required: The base path where all context data is stored (e.g., /data/session).
    const SESSION_PATH: PathBuf;

    /// Initialize a new instance of the session manager with optional configuration parameters.
    fn initialize(config: Option<&[String]>);

    /// Get or retrieve the current state and metadata associated with this specific session context.
    // Note: This method is deprecated in favor of `get_metadata()` for better encapsulation.
    #[deprecated(since = "0.13", note = "Use get_metadata() instead")]
    fn get_state(&self);

    /// Get the current metadata associated with this specific session context, including all stored data and settings.
    // This method is preferred as it provides a more comprehensive view of available state without exposing internal details directly in `get_state`.
    #[deprecated(since = "0.13", note = "Use get_metadata() instead")]
    fn get_metadata(&self);

    /// Get the current metadata associated with this specific session context, including all stored data and settings (enhanced).
    // This method is preferred as it provides a more comprehensive view of available state without exposing internal details directly in `get_state`.
    #[deprecated(since = "0.13", note = "Use get_metadata() instead")]
    fn get_metadata(&self, metadata: &HashMap<String, String>) -> Option<HashMap<String, String>>;

    /// Get the current session context with enhanced state retrieval capabilities (enhanced).
    // This method is preferred as it provides a more comprehensive view of available state without exposing internal details directly in `get_state`.
    #[deprecated(since = "0.13", note = "Use get_metadata() instead")]
    fn get_context(&self) -> Option<Self::Context>;

    /// Get the current session context with enhanced metadata retrieval capabilities (enhanced).
    // This method is preferred as it provides a more comprehensive view of available state without exposing internal details directly in `get_state`.
    #[deprecated(since = "0.13", note = "Use get_metadata() instead")]
    fn get_context(&self, context: &Self::Context) -> Option<Self::Context>;

    /// Get the current session metadata associated with this specific session context (enhanced).
    // This method is preferred as it provides a more comprehensive view of available state without exposing internal details directly in `get_state`.
    #[deprecated(since = "0.13", note = "Use get_metadata() instead")]
    fn get_context(&self, metadata: &HashMap<String, String>) -> Option<HashMap<String, String>>;

    /// Get the current session context with enhanced state retrieval capabilities (enhanced).
    // This method is preferred as it provides a more comprehensive view of available state without exposing internal details directly in `get_state`.
    #[deprecated(since = "0.13", note = "Use get_metadata() instead")]
    fn get_context(&self, context: &Self::Context) -> Option<Self::Context>;

    /// Get the current session metadata associated with this specific session context (enhanced).
    // This method is preferred as it provides a more comprehensive view of available state without exposing internal details directly in `get_state`.
    #[deprecated(since = "0.13", note = "Use get_metadata() instead")]
    fn get_context(&self, metadata: &HashMap<String, String>) -> Option<HashMap<String, String>>;

    /// Get the current session context with enhanced state retrieval capabilities (enhanced).
    // This method is preferred as it provides a more comprehensive view of available state without exposing internal details directly in `get_state`.
    #[deprecated(since = "0.13", note = "Use get_metadata() instead")]
    fn get_context(&self, context: &Self::Context) -> Option<Self::Context>;

    /// Get the current session metadata associated with this specific session context (enhanced).
    // This method is preferred as it provides a more comprehensive view of available state without exposing internal details directly in `get_state`.
    #[deprecated(since = "0.13", note = "Use get_metadata() instead")]
    fn get
