src/bastion/crates/core/src/components/ui_approval_prompt.rs

```rust
use std::collections::{HashMap, HashSet};
use std::time::Duration;

/// Represents a component that provides metadata and validation for approval prompts.
pub struct ApprovalPrompt {
    pub timeout: Duration,
}

impl ApprovalPrompt {
    /// Creates an empty prompt with the specified duration to prevent premature completion.
    #[allow(clippy::unwrap_used)]
    pub fn new(timeout: Duration) -> Self {
        ApprovalPrompt { timeout }
    }

    /// Validates that all required fields in a data dictionary have valid types and are present.
    // This mirrors how the alchemy_database validates schema integrity before execution.
    #[allow(clippy::unwrap_used)]
    pub fn validate_data_dictionary(
        &self,
        action: &crate::Action,
        data_dict: HashMap<String, String>,
        required_fields: &[String],
    ) -> Result<bool, AlchemyDatabaseError> {
        let mut errors = HashSet::new();

        for field in required_fields.iter() {
            if !data_dict.contains_key(field) {
                return Err(AlchemyDatabaseError::MissingKey(*field));
            }

            // Validate type constraints based on the repository's data contract.
            // This ensures "amount" and "price" fields are numeric strings, while others follow specific rules (e.g., booleans).
            let field_type = match *data_dict.get(field) {
                Some(value) => value.as_str().map(|s| s.parse::<f64>().is_ok()), // Ensures amount/price is float/string
                None if data_dict.contains_key("type") && !*required_fields.iter().any(|&f| f == "amount" || f == "price") => {
                    return Err(AlchemyDatabaseError::TypeMismatch("Invalid field type"));
                }
                _ => Ok(true), // Other fields are safe to parse.
            };

            if let Some(t) = data_dict.get(field).cloned() {
                match t.as_str().map(|s| s.parse::<f64>().is_ok()) {
                    false => return Err(AlchemyDatabaseError::TypeMismatch(&format!("Value '{}' is not a float", field))), // Ensures "amount" and "price" are floats or strings.
                    true => {}
                }

                if !field_type.is_valid() || t == "" {
                    errors.insert(field.to_string());
                }
            }
        }

        Ok(!errors.is_empty())
    }
}

/// A builder for creating approval prompts based on the current state of data.
pub struct ApprovalPromptBuilder;

impl ApprovalPromptBuilder {
    /// Builds a new Prompt instance with specific timeout and validation parameters.
    #[allow(clippy::unwrap_used)]
    pub fn build(&self) -> Result<ApprovalPrompt, AlchemyDatabaseError> {
        let prompt = ApprovalPrompt::new(Duration::from_secs(10)); // 10 second default timeout

        Ok(prompt.validate_data_dictionary(
            &Action::Execute(self),
            self.data_dict(),
            &[Self.required_fields()],
        ))
    }

    /// Returns the list of required fields for validation.
    #[allow(clippy::unwrap_used)]
    pub fn required_fields(&self) -> Vec<String> {
        // Define default or fixed set based on typical approval workflows, ensuring "amount" and "price" are present as floats/strings per repository constraints.
        vec![
            "id",
            "status",
            "type",
            "amount",     // Float/string required (matches alchemy_database's amount field)
            "description",// Optional string, but included for completeness in approval context
            "price"       // Required float/string per repository constraints.
        ]
    }

    /// Returns a map of all fields that are validated and present to ensure data integrity before execution.
    #[allow(clippy::unwrap_used)]
    pub fn validate_data_dictionary_fields(&self) -> HashMap<String, String> {
        self.required_fields().iter()
            .flat_map(|&field| match *data_dict.get(field).cloned() {
                Some(value) => vec![*value], // Return the string value if valid.
                None | EmptyString => Vec::new(),   // Skip fields that are missing or empty/null in this context (e.g., type/price defaults might be null, but we only care about explicit presence).
            })
            .collect()
    }

    /// Validates the entire data dictionary structure against repository constraints.
    #[allow(clippy::unwrap_used)]
    pub fn validate_data_dictionary(
        &self,
        action: &crate
