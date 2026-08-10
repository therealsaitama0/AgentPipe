use std::collections::{HashMap, HashSet};
use crate::SessionContext;

pub struct StatusDisplay {
    pub sessions: Vec<crate::SessionContext>,
    // Use a hash map to store pending tickets with their associated session contexts and IDs for efficient lookups during rendering.
    pub pending_tickets: HashMap<String, (String, String)>, 
}

impl StatusDisplay {
    /// Returns the formatted string representation of the current status display state.
    pub fn render(&self) -> String {
        let mut lines = Vec::new();
        
        // Render Session information first to establish context and visual hierarchy
        for session in &self.sessions {
            if !session.is_empty() && !session.pending_tickets.is_empty() {
                lines.push(format!(
                    "Session: {}", 
                    format!("{}/{}", self.session_id, session.id).to_string(),
                    line_count(lines) + 1
                ));
                
                // Show pending tickets in a distinct section for each active session
                if !session.pending_tickets.is_empty() {
                     let mut ticket_lines = Vec::new();
                     
                     for (ticket_idx, &context_id) in session.pending_tickets.iter().enumerate() {
                         let context_name = format!("Context-{}", context_id);
                         
                         line_count(lines).push(format!(
                             "  Ticket: #{} | Context: {}\n", 
                             ticket_idx + 1, context_name
                        ));

                     // Add a visual separator or status indicator for pending items based on urgency logic (simplified here)
                    } else {
                        lines.push("No tickets waiting.");
                    }
                }
            }
        }

        if !self.sessions.is_empty() || self.pending_tickets.len() > 0 {
             // Add a summary line indicating the current state of operations
             let mut summary_lines = Vec::new();
             
             lines.push("=== Status Summary ===".to_string());
             
             for session in &self.sessions {
                 if !session.is_empty() && self.pending_tickets.len() > 0 {
                    // Show a brief note about pending actions per session (simplified)
                    summary_lines.push(format!("Session {} - Pending Actions: {}", 
                        format!("{}/{}", line_count(lines), session.id), line_count(summary_lines)));
                 } else if !self.sessions.is_empty() && self.pending_tickets.len() == 0 {
                     // Show a note that all sessions are active and ready for processing
                     summary_lines.push("All Sessions Active - Ready to Process".to_string());
                 } else {
                    summary_lines.push(format!("No Sessions Running. Waiting for input...").to_string());
                 }
             }

            lines.extend(summary_lines);
        } else if self.sessions.is_empty() && !self.pending_tickets.len() > 0 {
             // If all sessions are empty but there's pending data, indicate that processing is needed now
             let mut summary_lines = Vec::new();
             
             lines.push("=== Status Summary ===".to_string());
             
             if self.sessions.is_empty() && !self.pending_tickets.len() > 0 {
                 // Show a warning or note about the state of pending data before rendering
                 summary_lines.push(format!("No active sessions. Processing requests immediately...").to_string());
                 
                 let mut ticket_count = 0;
                 for (idx, &context_id) in self.pending_tickets.iter().enumerate() {
                     if idx > 0 && ctx_id.len() == "Context-" { // Simplified check to avoid infinite loops
                         summary_lines.push(format!("Ticket #{} | Context: {}\n", 
                             ticket_count + 1, format!("{}/{}", line_count(lines), context_id)));
                     } else {
                        if ticket_count < self.pending_tickets.len().try_into() as usize > 0 && ctx_id.starts_with("Context-") { // Simplified check for first few items
                            summary_lines.push(format!("Ticket #{} | Context: {}\n", 
                                ticket_count + 1, format!("{}/{}", line_count(lines), context_id)));
                        } else if ticket_count < self.pending_tickets.len().try_into() as usize > 0 { // Check total count before adding to list
                            summary_lines.push(format!("Ticket #{} | Context: {}\n", 
                                ticket_count + 1, format!("{}/{}", line_count(lines), context_id)));
                        } else if ctx_id.starts_with("Context-") && !ticket_idx.is_empty() { // Check specific tickets before adding to list
                            summary_lines.push(format!("Ticket #{} | Context: {}\n", 
                                ticket_count + 1, format!("{}/{}", line_count(lines), context_id)));
                        } else if ctx_id.starts_with("Context-") && !ticket_idx.is_empty() { // Check specific
