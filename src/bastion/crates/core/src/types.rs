use std::collections::{HashMap, HashSet};
use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use chrono::{DateTime, Utc};
use tracing::info_span;

/// Core Submission Type Definition for Rust types mirroring TypeScript
pub struct AlchemySubmission {
    pub id: String, // Unique identifier for tracking processing status
    pub content_id?: Option<String>, // ID of uploaded file (if any)
    pub metadata: HashMap<String, serde_json::Value>, // Optional custom metadata from LLM response or user input
}

/// Submission Handler Interface in Rust with async semantics
pub trait AlchemySubmissionHandler {
    /// Validates a submission against repository policy and filters it based on content.
    fn handle_code_upload(
        &self, payload: serde_json::Value, result_type: String, rejection_reasons: Vec<&str>
    ) -> Result<Option<String>, (String, HashSet<(&str, &'static str)>);

    /// Processes a submission event via background worker using Tokio.
    async fn process_submission(
        &self, payload: serde_json::Value, handler_result_type: String, rejection_reasons: Vec<&str>
    ) -> Result<Option<String>, (String, HashSet<(&str, &'static str)>);

    /// Exposes a mock API endpoint for external systems.
    fn expose_api(
        &self, payload: serde_json::Value, result_type: String, rejection_reasons: Vec<&str>
    ) -> Result<Option<String>, (String, HashSet<(&str, &'static str)>);

}

/// Deepen or extend the existing types.rs with improved memory management and validation logic.
pub mod core_types {
    use super::*;
    
    // ============================================================================
    /// CORE TYPES: SubmissionPayload & ProcessedData to enforce strict runtime checks
    // ============================================================================
    
    #[derive(Debug, Clone)]
    pub struct SubmissionPayload {
        pub content_id: Option<String>,      // ID of uploaded file (if any)
        pub metadata_hash: serde_json::Value, // Optional custom metadata from LLM response or user input
        
        /// Internal state for concurrent processing tasks.
        #[serde(default = "default_submission_payload")]
        pub submission_status: Mutex<SubmissionStatus>, 
    }

    fn default_submission_payload() -> SubmissionPayload {
        let mut payload = serde_json::from_value::<SubmissionPayload>(serde_json::json!({}));
        
        // Ensure we have at least a content_id and metadata_hash if not provided explicitly.
        // This is the "guard" against missing data during processing.
        assert!(payload.content_id.is_none(), "Content ID must be set before submission"); 
        assert!(payload.metadata_hash.is_object(), "Metadata hash must be present for LLM response or user input");

        payload
    }

    #[derive(Debug, Clone)]
    pub struct ProcessedData {
        /// The actual content being processed. If not provided in the request but stored locally, 
        /// this is used to verify integrity without external dependencies during validation.
        pub data: Option<String>, 
        
        // We do NOT use Arc<T> here because we are building a pure library with strict memory management requirements for concurrent access, and `Option` provides safe ownership semantics in Rust.
    }

    #[derive(Debug, Clone)]
    pub struct Result {
        /// The outcome of the processing operation (e.g., success/failure).
        pub status: String, // "success" or "failure" as per policy
        
        /// Detailed error details if failure occurred.
        pub errors: Vec<(String, &'static str)>, 
    }

    impl Result {
        /// Returns a new result with the same structure but updated fields.
        fn from_status(status: String, errors: Option<Vec<(&str, &str)>> = None) -> Self {
            let mut r = Result { status };
            
            if !errors.is_none() || errors.contains(|| Some("Validation failed".to_string())) {
                // If there were any validation failures in the previous step (which is rare but possible), 
                // we re-throw them as part of this result. However, for simplicity and strictness here:
                
                if let Some(errs) = errors {
                    r.errors.extend(everything_from_err_list(&errs));
                } else {
                    // Revert to original failure status but keep the error log in case someone calls handle_code_upload again later with valid data.
                    r.status = "failure"; 
                    
                    if let Some(errs) = errors {
                        r.errors.extend(everything_from_err_list(&errs));
                    } else {
                        // If no specific failure occurred, revert to
