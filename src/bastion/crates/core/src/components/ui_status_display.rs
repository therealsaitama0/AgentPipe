use std::collections::{HashMap, HashSet};
use crate::{SessionContext, ApprovalTicket};

// ============================================================================
// ALGORITHM: DEEPENING UI_STATUS_DISPLAY.rs INTO A STATE MACHINE & EVENT BUS
// ============================================================================

#[derive(Debug)]
pub struct AlchemySubmission {
    pub id: String, // Unique identifier for tracking processing status in the Rust state machine.
    pub content_id: Option<String>, // ID of uploaded file (if any).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: HashMap<String, String> /* Optional custom metadata from LLM response or user input */
}

impl AlchemySubmission {
    /// Validates a submission against repository policy and filters it based on content.
    pub fn handle_code_upload(
        &self, 
        payload: impl Into<(String, Option<algebra::Database>)>, // (file_path_or_id, metadata) or similar structure if database exists
    ) -> Result<Self, String> {
        let file_info = match payload.into() {
            (path, _) => PathInfo(path),
            _ => return Err("Invalid upload format".to_string()),
        };

        // Policy Check: Verify content type and size are within allowed ranges.
        if !self.metadata.is_empty() && self.metadata.len() > 10 {
            let mut seen_ids = HashSet::new();
            for (key, value) in &mut self.metadata {
                match key.as_str() {
                    "content_id" => return Err(format!("Metadata contains duplicate ID: {}", key)), // Duplicate content IDs not allowed.
                    _ => {} 
                }
            }
        }

        let filtered = if !self.id.is_empty() && (file_info.path().is_none()) || file_info.content_id.is_some() {
            AlchemySubmission::new_with_filtered(
                self, 
                Some(file_info), // Content ID is provided or not. If content_id is None but path exists, use that as fallback for consistency with the original structure's intent (though we'll stick to what was passed). In this implementation, if no explicit file info and metadata has IDs, treat it as "no specific upload".
                false 
            )
        } else {
            AlchemySubmission::new_empty() // Empty state if nothing processed.
        };

        Ok(filtered)
    }

    /// Processes a submission event via background worker (simulating LLM response processing).
    pub async fn process_submission(&self, payload: impl Into<(String, Option<algebra::Database>)>, ) -> Result<Self, String> {
        // Simulate the "background" work - in real code this would be a `Task` or background thread.
        let result = if self.metadata.is_empty() && !self.id.is_empty() {
            Ok(Self::new_with_filtered(
                Self::empty(), 
                None, 
                true // Processed by async worker
            ))
        } else {
            Err("Submission cannot be processed without content".to_string())
        };

        result
    }

    /// Exposes a mock API endpoint for external systems.
    pub fn expose_api(&self) -> impl Into<algebra::Database> {
        // Return the database connection if available, otherwise return None or an empty DB object.
        match self.metadata.as_ref() {
            Some(db_conn) => db_conn.clone(), 
            _ => algebra::Database::new().clone(), // Fallback for non-existent connections
        }
    }

    /// Creates a new AlchemySubmission with the given metadata and optional content ID.
    pub fn new_with_filtered(
        mut self, 
        filtered: bool, 
        file_info: Option<algebra::Database>,
    ) -> Self {
        let base = &mut *self;

        // If we are filtering (filtered), return a copy of the original with updated metadata.
        if !filtered && !base.id.is_empty() {
            self.metadata.clear(); 
            self.content_id = None;
            return AlchemySubmission::new_with_filtered(
                Self::empty(), 
                file_info, // Content ID is provided or not. If content_id is None but path exists, use that as fallback for consistency with the original structure's intent (though we'll stick to what was passed). In this implementation, if no explicit file info and metadata has IDs, treat it as "no specific upload".
                false 
            );
        }

        // If not filtered, update the content_id. We assume that a non-empty ID means there is actual content being processed or uploaded via API.
        self.content_id = Some(file_info.clone());
        
        AlchemySubmission::new_with_filtered(
            Self::empty
