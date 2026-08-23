src/bastion/crates/core/src/components/ui_status_display.rs
```rust
use std::collections::{HashMap, HashSet};
use anyhow::{Result, Context};

/// Represents a generic status display component that can render various data structures.
pub trait StatusDisplay {
    /// Renders the current state of this instance as formatted text or JSON.
    fn render(&self) -> String;
}

/// A custom error type for UI rendering failures (e.g., missing fields, invalid types).
#[derive(Debug)]
enum RenderError {
    MissingField(String),   // Field not found in the data structure being rendered
    InvalidType(String),    // Data value is of a different type than expected
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderError::MissingField(field) => write!(f, "Missing field '{}' in status display", field),
            RenderError::InvalidType(type_name) => write!(f, "Value of type '{}': expected string (or a specific JSON format)", type_name),
        }
    }
}

impl std::error::Error for RenderError {}

/// A generic component that implements the `StatusDisplay` trait.
pub struct StatusDisplayComponent {
    // Placeholder to store actual data if not overridden by implementation
    _data: HashMap<String, String>,
}

impl Default for StatusDisplayComponent {
    fn default() -> Self {
        Self::new_empty_data()
    }
}

/// Creates a new empty `StatusDisplayComponent` without any stored data.
fn create_status_display_component(data: &HashMap<String, String>) -> Result<Self> {
    let mut component = StatusDisplayComponent { _data };
    
    // If provided custom schema/structure is given (though we don't have one here), apply it.
    if let Some(schema) = data.get("schema") {
        log::info!("Applying generic UI structure: {}", String::from_utf8_lossy(&schema));
        
        component._data.clear(); // Clear existing empty state to start fresh
        
        for (key, value) in schema.iter() {
            if let Ok(v) = serde_json::Value::String(value.to_string()) {
                component._data.insert(key.clone(), v);
            } else {
                return Err(RenderError::InvalidType("schema".to_string())); // Fallback to generic error for unknown keys
            }
        }

        log::info!("UI structure applied successfully");
    }

    Ok(component)
}

impl StatusDisplayComponent {
    /// Renders the current state of this instance as formatted text or JSON.
    pub fn render(&self) -> String {
        self._data.clone()
            .iter()
            .map(|(key, val)| (format!("{}: {}", key, if let Some(v) = &*val { v } else {})))
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// Attempts to parse the rendered string into a generic schema structure.
    fn try_parse_schema(&self) -> Result<HashMap<String, String>> {
        // Attempt simple JSON parsing if present; otherwise fall back to text split
        let json = self.render();

        match serde_json::from_str::<HashMap<String, serde_json::Value>>(json.as_bytes()) {
            Ok(schema) => schema.to_vec(),
            Err(e) => return Err(RenderError::InvalidType("schema parsing".to_string())), // Generic fallback
            _ => return Err(RenderError::MissingField(format!("Failed to parse UI data: {}", e))),
        }
    }

    /// Renders the component with a default empty schema if no custom structure is provided.
    fn render_with_default_schema(&self) -> String {
        self.render() // Returns current state directly; optional schema handling happens via try_parse_schema or fallback logic below
    }

    pub async fn create_component<F>(&mut self, f: F) where
        F: FnOnce(&HashMap<String, serde_json::Value>) -> Result<HashMap<String, String>> {
        
        let data = f(self); // Call the function to get current state
        
        if let Ok(schema) = try_parse_schema(data.as_ref()) {
            self._data.clear(); // Reset empty state for fresh rendering
            
            for (key, value) in schema.iter() {
                match serde_json::Value::String(value.to_string()) {
                    Ok(v) => self._data.insert(key.clone(), v),
                    Err(e) => return Err(RenderError::InvalidType(format!("Unknown key '{}' in UI data: {}", key, e))), // Generic fallback error for unknown keys
                }
