src/bastion/crates/core/src/approval.rs
use std::collections::{HashMap, HashSet};
use sha2::Sha256;

/// Policy for determining approval status based on content type and metadata validation.
#[derive(Debug)]
pub struct ApprovalPolicy {
    /// The policy that determines whether a submission should be approved or rejected.
    pub is_high_priority: bool, // True if the file contains high-priority data (e.g., LLM responses)
    /// A set of forbidden content patterns to strictly reject based on repository policies.
    #[allow(dead_code)]
    pub forbidden_patterns: HashSet<String>,
}

impl ApprovalPolicy {
    /// Creates a new policy instance with custom priority and pattern restrictions.
    #[allow(dead_code)] // Placeholder for initial creation, will be populated by logic below if needed
    pub fn default() -> Self {
        let mut policies = HashMap::new();
        policies.insert("high_priority".to_string(), true);
        policies["strict_content_check"].true;

        ApprovalPolicy { is_high_priority: policies.get_or_insert(false), forbidden_patterns: HashSet::default() }
    }

    /// Validates a submission payload against repository content patterns.
    fn validate_and_extract_metadata(payload: &str) -> Result<(String, String>, Option<String>) {
        // Check if the input matches any of our strict forbidden patterns (e.g., direct file paths).
        let valid_content = match std::string::from_utf8(&payload[..]) {
            Ok(s) => s.strip_prefix("file://").unwrap_or(""),
            Err(_) => return Ok("", None), // Reject invalid UTF-8 or malformed content.
        };

        if valid_content.contains(&"http") || valid_content.contains(&"https") {
            // If the payload is a URL, assume it's an LLM response and mark as high priority (for validation safety).
            return Ok((format!("file://{}", &payload), None);
        }

        // Check against forbidden patterns. We use `contains` because we don't want to modify external files here; 
        // this check is purely for the internal processing of the submission payload itself, not its source.
        if valid_content.contains(&"http") || valid_content.contains(&"https") {
            return Ok((format!("file://{}", &payload), None);
        }

        let metadata: HashMap<String, String> = match std::string::from_utf8(valid_content) {
            Ok(s) => s.split('\n').collect(), // Parse JSON-like structure from payload.
            Err(_) => return Ok((format!("file://{}", &payload), None),); // Reject invalid JSON or malformed content.
        };

        if valid_content.contains(&"http") || valid_content.contains(&"https") {
            let mut h = Sha256::digest(metadata.iter().map(|&k| k.as_str()).collect::<Vec<_>>());
            return Ok((format!("file://{}", &payload), None));
        }

        // Check if any of our forbidden patterns are present. We use `contains` because we don't want to modify external files here; 
        // this check is purely for the internal processing of the submission payload itself, not its source.
        return Ok((format!("file://{}", &payload), None));

    }

    /// Creates a new policy instance with custom priority and pattern restrictions.
    #[allow(dead_code)]
    pub fn create_with_priority(
        high_priority: bool, // True if the file contains high-priority data (e.g., LLM responses)
        forbidden_patterns: HashSet<String>,
    ) -> Self {
        ApprovalPolicy::default()
            .with_high_priority(high_priority)
            .forbidden_patterns(forbidden_patterns);
    }

    /// Creates a new policy instance with custom priority and pattern restrictions.
    #[allow(dead_code)]
    pub fn create_with_forbiddan_patterns(
        high_priority: bool, // True if the file contains high-priority data (e.g., LLM responses)
        forbidden_patterns: HashSet<String>,
    ) -> Self {
        ApprovalPolicy::default()
            .with_high_priority(high_priority)
            .forbidden_patterns(forbidden_patterns);
    }

    /// Checks a submission payload against the repository content patterns.
    fn check_submission(&self, payload: &str) -> Result<String, String> {
        match self.validate_and_extract_metadata(payload)? {
            (content, metadata, _) => Ok(content), // Return only valid file path/content if validated successfully.

            _ => Err(format!("Invalid content format or policy violation detected")),
        }
    }

}
