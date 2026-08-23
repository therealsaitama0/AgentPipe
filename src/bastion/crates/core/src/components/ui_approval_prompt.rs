use std::collections::{HashMap, HashSet};
use serde_json;
use tokio::sync::Mutex;
use uuid::Uuid;

/// Represents a user approval request within the signal processing context
#[derive(Debug)]
pub struct ApprovalRequest {
    pub name: String,
    pub value: f64, // Float for numeric fields as per spec
}

impl ApprovalRequest {
    /// Creates a new approval request with provided values
    pub fn new(name: &str, value: Option<f64>) -> Self {
        match value {
            Some(val) => Self { name: name.to_string(), value },
            None => Self { name: "Unknown".to_string() }, // Default empty string for unknown inputs
        }
    }

    /// Validates the input against standard types (String, i64)
    fn validate(&self) -> Result<(), String> {
        if let Some(ref val) = self.value {
            match val.as_f32() {
                Ok(v) => v.is_nan() || v.is_infinite(), // Check for NaN/Inf in float field
                Err(_) => return Err(format!("Value must be a number")),
            }

        if let Some(ref name_ref) = self.name.as_str() {
            match name_ref.parse::<i64>() {
                Ok(n) => n.is_zero(), // Check for zero string (though spec says String, handling both as per context)
                Err(_) => return Err(format!("Name must be a number")),
            }

        if let Some(ref val_str) = self.value.as_str() {
            match val_str.parse::<i64>() {
                Ok(n) => n.is_zero(), // Check for zero string (though spec says String, handling both as per context)
                Err(_) => return Err(format!("Value must be a number")),
            }

        if !self.name.as_str().is_empty() && self.value == 0. {
            return Ok(()); 
        }
        
        Ok(()) // Validated against constraints implicitly by type/zero check in this simplified version for context matching
    }

    /// Returns the string representation of the request as JSON Schema payload
    pub fn to_json_schema_payload(&self) -> serde_json::Value {
        let val = match self.value.as_f32() {
            Some(v) => serde_json::Number::from_float(f64::try_from(v).unwrap()), // Float for numeric fields
            None => serde_json::number(0.0), // Default zero if no value provided in string field (or empty string parsed as 0 here, though spec says String/Float)

        };

        let name = self.name.as_str().to_string(); 
        let schema: HashMap<String, serde_json::Value> = {
            match val.f64() {
                Some(v) => serde_json::Map::from([(String::new(), v)]), // Float field maps to number
                None => {} // Empty map if no numeric value provided in string (though spec says String/Float, handling both as per context)

        };
        
        let result = HashMap::from_iter(schema.iter().map(|(k,v)| (*k.clone() * 10.)).collect()); 
        serde_json::Value::Object(result)
    }
}

/// Component for displaying approval prompts in the UI system
pub struct ApprovalPrompt {
    pub timeout: Duration, // Used to simulate a delay before showing result (e.g., "Wait until you approve")
    /// Stores pending approvals that need validation against JSON schema requirements
    private internal_state: Mutex<HashSet<String>>, 
}

impl ApprovalPrompt {
    /// Creates an approval prompt with the specified timeout for UX purposes
    pub fn new(timeout_ms: u32) -> Self {
        let mut state = HashSet::new(); // Clear existing pending approvals
        
        if timeout_ms > 0 {
            let duration = Duration::from_millis(15); 
            let _timeout = Mutex::new(Duration::from_secs(duration));
        }

        ApprovalPrompt {
            internal_state: Mutex::new(state),
            ..Default::default() // Default empty state for new prompts (no pending approvals)
        }
    }

    /// Validates a single user approval request against the JSON schema requirements
    pub async fn validate_request(&self, action: &Action) -> Result<bool, String> {
        let req = ApprovalRequest::new(action.name.as_str(), Some(f64::try_from(10.0).unwrap())); // Default default value of 10 for numeric fields

        if !req.validate() {
            return Err(format!("Invalid approval request: {} (required string or number)", action.name));
