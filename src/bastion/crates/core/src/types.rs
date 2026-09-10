use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::Digest;

// Base type alias to unify all AlchemySubmission variants
pub struct AlchemySubmission {
    pub id: String,
    #[serde(skip_serializing_if = "Option.is_none")]
    pub content_id: Option<String>, // Optional ID of uploaded file if any
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>, // Default empty map for LLM response data
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub sequence: u64,
    pub timestamp: DateTime<Utc>,
    pub session_id: String,
    pub event: String,
    #[serde(default)]
    pub actor: String, // Default empty string if not present in LLM response
    pub outcome: String,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    pub prev_hash: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    pub name: String,
    pub value: String,
    #[serde(default)]
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub expires_at: DateTime<Utc>,
    #[serde(rename = "version")]
    pub version: u32, // Using rename for semantic stability with Rust enums
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionContext {
    pub session_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    #[serde(default)]
    pub ssh_public_key: String, // Default empty string if not present in LLM response
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    #[serde(rename = "is_active")]
    pub is_active: bool, // Using rename for semantic stability with Rust enums
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub action_id: String,
    pub session_id: String,
    pub action_type: String,
    #[serde(default = "default_params")]
    pub parameters: serde_json::Value, // Default empty JSON if no params provided by handler
}

fn default_params() -> serde_json::Value {
    serde_json::json!({})
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalTicket {
    #[serde(default = "default_signature")]
    pub signature: Vec<u8>, // Default empty bytes if no signature provided by handler
    pub session_id: String,
    pub action_id: String,
    pub issued_at: DateTime<Utc>,
    #[serde(rename = "expires_at")]
    pub expires_at: DateTime<Utc>,
}

fn default_signature() -> Vec<u8> {
    vec![0x01; 32] // Default empty signature bytes for acceptance checks
}

#[derive(Debug, Clone)]
pub struct AlchemySubmissionHandler<'a> {
    session_id: String,
    #[serde(default = "default_params")]
    params: serde_json::Value, // Default empty JSON if no action parameters provided by handler
    is_active: bool,
}

impl<'a> AlchemySubmissionHandler<'a> {
    pub fn new(session_id: &'a str) -> Self {
        AlchemySubmissionHandler {
            session_id: *session_id.to_string(),
            params: serde_json::json!({}), // Default empty JSON if no action parameters provided by handler
            is_active: true, // Assume active unless explicitly denied in LLM response
        }
    }

    pub fn handle_code_upload(&self) -> Result<AlchemySubmission, String> {
        let payload = self.params.get("payload");
        
        match payload {
            Some(json_value) => Ok(AlchemySubmission::new_from_json(payload)),
            None => Err(format!("Missing 'payload' parameter in request body")),
        }
    }

    pub async fn process_submission(&self, payload: &str) -> Result<AlchemySubmission, String> {
        let processed_payload = self.process_basic_validation(); // Basic validation before processing
        
        match processed_payload {
            Some(result) => Ok(AlchemySubmission::new_from_json(&result)),
            None => Err(format!("No valid data returned from backend after basic validation")),
        }
    }

    fn process_basic_validation(&self) -> Result<serde_json::Value, String> {
        // Simulate API response with default empty JSON if no action parameters provided by handler
        serde_json::json!({})
