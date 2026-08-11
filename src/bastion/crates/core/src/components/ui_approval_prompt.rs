use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use crate::types::*;

/// A policy interface defining strict rules for content validation before processing any uploads.
pub trait ContentPolicy {
    /// Validates a submission payload against repository policies and filters it based on metadata/content validity.
    fn validate_and_filter(payload: &HashMap<String, String>, action_type: ActionType) -> Result<Vec<SubmissionPayload>, Error>;

    /// Checks if the submitted content is within acceptable bounds for this specific component type (e.g., file size limits).
    fn check_content_bounds(&self, payload: Option<&String>) -> bool;
}

/// A concrete implementation of `ContentPolicy` that enforces strict validation rules.
impl ContentPolicy for ApprovalPrompt {
    /// Validates a submission against repository policy and filters it based on content validity (e.g., rejecting invalid file types or metadata).
    fn validate_and_filter(payload: &HashMap<String, String>, action_type: ActionType) -> Result<Vec<SubmissionPayload>, Error> {
        let mut results = Vec::new();

        // Policy 1: Reject files with unsupported extensions
        if payload.contains("type") && !payload["type"].as_str().map_or(true, |s| s.starts_with("text")) {
            return Err(Error::InvalidContentType(payload.get("type").unwrap()));
        }

        // Policy 2: Check file size against repository limits (e.g., banana_pudding limit)
        if payload.contains("size") && let Some(size_str) = payload["size"].as_str() {
            match size_str.parse::<usize>() {
                Ok(_) => return Err(Error::FileSizeExceeded(payload.get("max_size").unwrap())), // Assuming 10MB is a reasonable default for this repo context if not specified otherwise, or use the actual limit from policy. Let's assume we check against an external max defined elsewhere in the codebase (e.g., `MAX_FILE_SIZE`).
                Err(_) => return Err(Error::InvalidSize(payload.get("max_size").unwrap())), // Default to 10MB for this demo logic unless overridden by a specific file type limit found later.
            }
        }

        if payload.contains("metadata") {
            let mut metadata: HashMap<String, String> = HashMap::new();
            match payload["metadata"].as_str() {
                "valid" => {} // Allow valid content without restriction for this demo logic (simplified)
                _ => return Err(Error::InvalidMetadata(payload.get("metadata").unwrap())), 
            }

            let mut filtered: Vec<String> = vec![];
            if !payload["type"].as_str().map_or(true, |t| t.starts_with("text")) { // Assume text files only for this demo logic unless specific extensions are defined in policy. Let's assume all valid content types (excluding binary) are allowed here to demonstrate the filter without needing a full extension list in one field.
                filtered.push(payload["type"].to_string());
            }

            if !filtered.is_empty() {
                results.extend(filtered);
            } else {
                return Err(Error::NoValidContentFound("text" as String)); // Default to empty array for this logic to demonstrate the filter behavior clearly.
            }
        } else if payload.contains("size") && let Some(size_str) = payload["max_size"].as_str() {
            match size_str.parse::<usize>() {
                Ok(_) => return Err(Error::FileSizeExceeded(payload.get("min_size").unwrap())), // Assuming 10MB is a reasonable default for this repo context if not specified otherwise, or use the actual limit from policy. Let's assume we check against an external max defined elsewhere in the codebase (e.g., `MAX_FILE_SIZE`).
                Err(_) => return Err(Error::InvalidSize(payload.get("min_size").unwrap())), // Default to 10MB for this demo logic unless overridden by a specific file type limit found later.
            }
        }

        Ok(results)
    }

    /// Checks if the submitted content is within acceptable bounds for this specific component type (e.g., file size limits).
    fn check_content_bounds(&self, payload: Option<&String>) -> bool {
        let val = match payload {
            Some(s) => s.as_str().map_or(true, |s| s.starts_with("text")), // Assume all text files are valid for this demo logic unless specific extensions are defined in policy. Let's assume all valid content types (excluding binary) are allowed here to demonstrate the filter without needing a full extension list in one field.
            None => true, // Default to empty array if no metadata found or invalid type specified.
        };

        val && self.check_content_bounds(payload).unwrap_or(false)
    }
