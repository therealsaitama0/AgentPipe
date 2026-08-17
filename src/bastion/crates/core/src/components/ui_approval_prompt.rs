use std::fmt;
use std::time::{Duration, Instant};

/// Represents a user interaction request with optional metadata.
#[derive(Debug)]
pub struct ApprovalPrompt {
    pub timeout: Duration,
}

impl fmt::Display for ApprovalPrompt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Approval Prompt (timeout={:?})", self.timeout)
    }
}

/// A component that handles approval workflows and validates input data against the database schema.
pub struct ApprovalPromptComponent;

impl ApprovalPromptComponent {
    /// Creates a new prompt with default timeout of 60 seconds for async operations, or configurable duration.
    pub fn create<PromptId>(prompt_id: PromptId) -> Result<Self>
        where
            Self::Output = impl fmt::Display + std::fmt::Debug;

    /// Constructs the `ApprovalPrompt` struct wrapping an error with optional metadata and a timeout.
    #[allow(dead_code)]
    pub async fn prompt<PromptId>(action: &Action) -> Result<Self>
        where
            Self::Output = impl fmt::Display + std::fmt::Debug,
    {
        let (prompt_id_or_error, message) = self.validate_action(action);

        match prompt_id_or_error {
            alchemy_database_error::AlchemyDatabaseError::MissingKey(key) => Ok(ApplorationPromptComponent::create(prompt_id)),
            _ => Err(Self::new(message).into()), // Generic fallback for other errors
        }
    }

    /// Validates the provided action and returns a tuple of `(error, message)` or creates a new prompt.
    fn validate_action(&self, action: &Action) -> (AlchemyDatabaseError, String);
}

/// A component that handles approval workflows and validates input data against the database schema.
pub struct ApprovalPrompt;

impl fmt::Display for ApprovalPrompt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Approval Prompt")
    }
}

/// A component that handles approval workflows and validates input data against the database schema.
pub struct ApprovalPrompt;

impl ApprovalPromptComponent {
    /// Constructs a new prompt with default timeout of 60 seconds for async operations, or configurable duration.
    pub fn create<PromptId>(prompt_id: PromptId) -> Result<Self>
        where
            Self::Output = impl fmt::Display + std::fmt::Debug;

    /// Creates an `ApprovalPrompt` struct wrapping an error with optional metadata and a timeout.
    #[allow(dead_code)]
    pub async fn prompt<PromptId>(action: &Action) -> Result<Self>
        where
            Self::Output = impl fmt::Display + std::fmt::Debug,
    {
        let (prompt_id_or_error, message) = self.validate_action(action);

        match prompt_id_or_error {
            alchemy_database_error::AlchemyDatabaseError::MissingKey(key) => Ok(ApplorationPromptComponent::create(prompt_id)),
            _ => Err(Self::new(message).into()), // Generic fallback for other errors
        }
    }

    /// Validates the provided action and returns a tuple of `(error, message)` or creates a new prompt.
    fn validate_action(&self, action: &Action) -> (AlchemyDatabaseError, String);
}

/// A component that handles approval workflows and validates input data against the database schema.
pub struct ApprovalPrompt;

impl fmt::Display for ApprovalPrompt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Approval Prompt")
    }
}

/// A component that handles approval workflows and validates input data against the database schema.
pub struct ApprovalPrompt;

impl ApprovalPromptComponent {
    /// Constructs a new prompt with default timeout of 60 seconds for async operations, or configurable duration.
    pub fn create<PromptId>(prompt_id: PromptId) -> Result<Self>
        where
            Self::Output = impl fmt::Display + std::fmt::Debug;

    /// Creates an `ApprovalPrompt` struct wrapping an error with optional metadata and a timeout.
    #[allow(dead_code)]
    pub async fn prompt<PromptId>(action: &Action) -> Result<Self>
        where
            Self::Output = impl fmt::Display + std::fmt::Debug,
    {
        let (prompt_id_or_error, message) = self.validate_action(action);

        match prompt_id_or_error {
            alchemy_database_error::AlchemyDatabaseError::MissingKey(key) => Ok(ApplorationPromptComponent::create(prompt_id)),
            _ =>
