use crate::{types::SubmissionType, error::*};

/// Trait for submission approval management.
pub trait ApproveSubmission {
    /// Validates a submitted code against repository policy and filters it based on content before accepting it.
    fn validate_submission(&self) -> Result<(), String>;

    /// Processes the approved submission via background worker to execute its logic without blocking main threads.
    async process_approved(self: &Self, payload: &str): Result<SubmissionType, Error>;

    /// Exposes a public API for external crates or services to trigger approval requests directly.
    fn expose_api(&self) -> impl FnOnce(String, String) -> bool;
}

/// Trait implementing the `ApproveSubmission` trait with validation logic before accepting submissions from downstream systems.
pub struct Approver {
    policy: Policy,
    handler: AlchemySubmissionHandler,
}

impl Approver {
    /// Creates a new approval manager instance using default policies and handlers if not already provided.
    pub fn new(policy: &Policy) -> Self {
        let policy = match policy {
            Some(p) => p.clone(), // Clone for immutability in this context, though typically we'd just use the reference
            None => Policy::default().clone(),
        };

        let handler = AlchemySubmissionHandler::new();

        Let Approver { policy, handler }
    }

    /// Validates a submission against repository policies and filters it based on content before accepting it.
    pub fn validate_submission(&self) -> Result<(), String> {
        self.policy.validate(self.handler.handle_code_upload)?;
        Ok(())
    }

    /// Processes the approved submission via background worker to execute its logic without blocking main threads.
    async fn process_approved_payload<P: Into<String>>(payload: P): Result<SubmissionType, Error> {
        match payload.into() {
            "submit" => {
                let content_id = std::env::var("CONTENT_ID").unwrap_or_else(|_| String::from("unknown"));

                // Simulate processing logic based on the type of submission (e.g., file path or metadata)
                if content_id.is_empty() {
                    return Err(Error::InvalidContentId);
                }

                let result = self.handler.process_submission(&content_id)?;

                match &result {
                    Some(submission_type) => Ok(*submission_type),
                    None => Err(Error::MissingResult),
                }
            },
            _ => Err(Error::UnknownSubmissionType(0)), // Generic fallback for unknown types
        };
    }

    /// Exposes a public API for external crates or services to trigger approval requests directly.
    pub fn expose_api(&self) -> impl FnOnce(String, String) -> bool {
        move |content_id: &str| self.handler.process_submission(content_id).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_approver_validation_basic() {
        let policy = Policy::default();
        let approver = Approver::new(&policy);
        
        assert_eq!(approver.validate_submission().unwrap(), Ok(()));
        // Mock submission types should trigger validation. In a real scenario, these would be specific to content_id or payload type.
    }

    #[test]
    fn test_approver_process_approved_payload() {
        let policy = Policy::default();
        let approver = Approver::new(&policy);
        
        // Mock submission types are not exposed here as they require a real backend to validate, 
        // but the logic structure is sound. In production, this would be replaced with actual `process_submission` calls via background workers or async sub-processes controlled by event loops (e.g., futures::executor).

        let payload = "submit".to_string();
        assert_eq!(approver.process_approved_payload(&payload).unwrap(), Some(SubmissionType::Alchemy)); // Placeholder for expected result in mock.
    }
}
