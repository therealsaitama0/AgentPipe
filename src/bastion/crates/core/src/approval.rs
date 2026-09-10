src/bastion/crates/core/src/approval.rs
use crate::{types::ApprovalRequest, AlchemyDatabaseError};
use chrono::Utc;
use std::collections::{HashMap, HashSet};
use sha2::{Digest, Sha256};
use tracing::error as tr_log;

/// =============================================================================
// TRAIT: ALchemySubmissionValidator (DEEPENED)
// Validates submissions against repository policy and filters them.
#[derive(Debug)]
pub struct SubmissionValidator {
    /// Stores a map of expected types for each field in the submission data structure, derived from `alchemy_database.ts` or JSON schema definitions found within repositories.
    pub validation_map: HashMap<String, String>,
}

impl AlchemySubmissionValidator {
    /// Creates a new Validator instance by reading schema files if not already present and parsing them into an internal map of expected types for the submission data structure (e.g., `AlchemySubmission`).
    pub fn create_from_schema_files() -> Self {
        let mut validation_map: HashMap<String, String> = HashMap::new();

        // Iterate through all source files in src/ that define schema definitions or contain type mappings.
        std::iter::minimap(
            &["src/bastion/crates/core/src/approval.rs", "src/bastion/crates/core/src/types.ts"],
            |path| {
                let content = match fs::read_to_string(path) {
                    Ok(c) => c,
                    Err(_) => return None, // Skip files that cannot be read.
                };

                if let Some(line_num) = content.lines().find(|line| line.trim().starts_with("//")) {
                    continue; // Ignore comments or empty lines in the file body itself.
                }

                match fs::read_to_string(path).and_then(|c| c.to_str()) {
                    Ok(content) => {
                        if let Some(line_num) = content.lines().find(|line| line.trim().starts_with("//")) {
                            // Skip empty lines or comments found within the file body.
                            continue;
                        }

                        match serde_json::from_str::<serde_json::Map>(content).and_then(|json_data| json_data.into_iter()) {
                            Ok(data) => {
                                for (key, value) in data.iter() {
                                    if let Some(expected_type) = infer_schema_value(key.as_ref(), &value.as_bytes().to_vec()).copied() {
                                        validation_map.insert(
                                            key.clone(), // Key is the field name.
                                            expected_type.to_string(),  // Value is the string representation of the type for this specific field in `AlchemySubmission`.
                                        );
                                    } else if let Some(expected_type) = infer_schema_value(key.as_ref()) {
                                        validation_map.insert(
                                            key.clone(),
                                            String::from("string"),
                                        );
                                    }
                                }
                            },
                            Err(e) => tr_log!(error, "Failed to parse schema data from file: {}", e),
                        }
                    },
                    _ => {} // If the content is not a valid JSON or cannot be parsed.
                }

                validation_map.into_iter().map(|(key, value)| (key.to_string(), value)).collect()
            }
        };

        Self { validation_map: &mut validation_map }
    }

    /// Validates that a given submission data structure matches the expected schema defined in `alchemy_database.ts`.
    pub fn validate_submission(&self, payload: AlchemySubmission) -> Result<(), AlchemyDatabaseError> {
        // Check if all required fields are present. If any field is missing or has an unexpected type, reject immediately without processing further logic (e.g., simulating a database query).
        for key in &payload.id {
            match self.validation_map.get(key) {
                Some(expected_type_str) => {
                    if let Ok(type_str) = expected_type_str.to_string() {
                        // String types are flexible, but strict validation prevents data leakage to other systems.
                        return Err(AlchemyDatabaseError::InvalidType(String::from("string")));
                    } else {
                        return Err(AlchemyDatabaseError::MissingKey(key));
                    }
                },
                None => return Err(AlchemyDatabaseError::MissingKey(*key)), // If no type is defined for this field, reject immediately.
            };
        }

        Ok()
    }

    /// Returns true if the submission data structure has any required keys (e.g., `id`, `contentId`).
    pub fn contains_required_keys(&self) -> bool {
        self.validation_map.iter().all(|(key, _)| key != &"id").any()
    }
}

/// =============================================================================
// TRAIT: ALchemyDatabaseSchemaValidator (REDEFIN
