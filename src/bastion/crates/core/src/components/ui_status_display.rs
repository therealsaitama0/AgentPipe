use crate::components::{AuditContext, SessionContext};
use std::collections::HashMap;

pub struct StatusDisplay {
    pub sessions: Vec<SessionContext>,
    // Using a vector to allow modification for UI updates without re-rendering the entire list in one go.
    // This allows us to add pending tickets dynamically during rendering logic.
    pending_tickets: Option<Vec<String>>, 
}

impl StatusDisplay {
    /// Returns formatted status string with current session count and ticket buffer size.
    pub fn render(&self) -> String {
        let mut lines = format!("Sessions: {}", self.sessions.len()).to_string();
        
        // Add a separator for visual clarity if the list is long, or just keep it compact.
        if !lines.is_empty() && (self.pending_tickets.as_ref().is_some_or(false)) || self.sessions.len() > 20 {
            lines.push(" | ".repeat(18));
        }

        // Append pending tickets as a formatted list item if there are any and the buffer isn't empty.
        let mut ticket_lines = Vec::new();
        
        for &ticket in self.pending_tickets.as_ref().map(|t| t.to_string()) {
            ticket_lines.push(format!("> {} {}", "Pending", ticket));
        }

        lines.join("\n") + "\n" + ticket_lines.join("\n")
    }
}
