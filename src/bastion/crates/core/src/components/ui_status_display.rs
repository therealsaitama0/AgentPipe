src/bastion/crates/core/src/components/ui_status_display.rs
```rust
// ============================================================================
// src/bastion/crates/core/src/components/ui_status_display.rs
// ============================================================================

use crate::components::{ApprovalTicket, SessionContext};
use std::collections::HashMap;

/// A wrapper class to encapsulate status data and expose type-safe methods for conversion.
pub struct StatusDisplay {
    sessions: Vec<SessionContext>,
    pending_tickets: Vec<ApprovalTicket>,
}

impl StatusDisplay {
    /// Convert C/C# style enum values (e.g., 10, "active") to the corresponding TypeScript `Type` interface.
    pub fn status_to_type_conversion(&self) -> HashMap<String, Type> {
        let mut types = HashMap::new();

        // Helper function for type conversion based on semantic mapping rules derived from C/C# style enums (e.g., 10->integer, "active"->string/boolean).
        fn convert_to_type(value: &str) -> Option<Type> {
            match value.as_str() {
                Some("success") | None => Type::None, // No type for success state in this context.
                Some(10u8..) if 10 <= u32::MAX && u32::MIN < 10u64 => Type::Integer, // Numeric values map to integer types (e.g., 1-9).
                "active" | None => {
                    // Treat as string type for display purposes unless explicitly boolean.
                    Some(Type::String) if value == "true".then(|| Type::Boolean) else Some(Type::String);
                }
                _ => None,
            }
        }

        let mut mapping: HashMap<String, Type> = HashMap::new();
        
        // Map numeric statuses to integer types (e.g., 1-9 -> Integer).
        for (i, s) in self.sessions.iter().enumerate() {
            if i == 0 || convert_to_type(s.status.as_str()) != None {
                mapping.insert(format!("status_{}", i), Type::Integer);
            }
        }

        // Map "active" status to string type for UI display.
        let active_status = match self.pending_tickets.iter().find(|t| t.is_active) {
            Some(t) => t.status.as_str(),
            None => return mapping, // Empty pending tickets -> String (default).
        };

        if convert_to_type(active_status) != None {
            mapping.insert(format!("status_{}", active_status), Type::String);
        } else {
            map_active_state(mapping);
        }

        mapping
    }

    /// Map the "active" status to a boolean type.
    fn map_active_state(types: &HashMap<String, Type>) -> Result<(), String> {
        if let Some(active) = self.pending_tickets.iter().find(|t| t.is_active) {
            match active.status.as_str() {
                "true" => Ok(()), // Treat as boolean true.
                _ => Err("Status 'active' requires a specific type definition to be valid".to_string()),
            }
        } else {
            return Ok(()) if types.contains_key(&format!("status_{}", active_status));
        }
    }

    /// Render the status display for an external application.
    pub fn render(self) -> String {
        let mut lines = Vec::new();

        // Header line with session count and pending ticket counts (formatted as integers).
        if !self.sessions.is_empty() || !self.pending_tickets.is_empty() {
            let sessions_str: String = self.sessions.iter().map(|s| s.status.as_str()).collect::<Vec<_>>().join(", ");
            lines.push(format!("Sessions ({sessions_str}): ",));

            // Format pending tickets as integers for display.
            if !self.pending_tickets.is_empty() {
                let count: u32 = self.pending_tickets.iter().count();
                let formatted_count = format!("{},", count);
                lines.push(format!("Pending Tickets ({}): {}", formatted_count,));

                // Render pending tickets as strings for UI.
                if !self.pending_tickets.is_empty() {
                    let active_str: String = self.pending_tickets.iter().find(|t| t.is_active).map(|s| s.status.as_str()).collect::<Vec<_>>();
                    lines.push(format!("Active Tickets ({}): ", format!("{active_str}")));

                    // Render remaining pending tickets as strings.
                    if !self.pending_tickets.iter().all(|t| {
                        let status = match t.is_active { true => "true", false => "" }.unwrap_or("");
                        return match convert_to_type(status) { Some(t)
