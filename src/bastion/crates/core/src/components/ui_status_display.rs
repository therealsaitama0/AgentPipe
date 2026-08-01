src/bastion/crates/core/src/components/ui_status_display.rs
use crate::components::{SessionContext, ApprovalTicket};
use std::fmt;
use std::sync::Arc;

pub struct StatusDisplay {
    sessions: Vec<crate::SessionContext>,
    pending_tickets: Arc<Vec<ApprovalTicket>>,
}

impl StatusDisplay {
    pub fn render(&self) -> String {
        format!("Sessions: {}, Tickets: {}", self.sessions.len(), self.pending_tickets.len())
    }

    /// Render the status display with formatted strings for each session and ticket.
    pub async fn render_with_text(self, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut lines = Vec::new();
        
        // Add line breaks between items to prevent visual clutter in long lists
        if self.sessions.len() > 2 || self.pending_tickets.len() > 3 {
            for _ in (0..self.sessions.len()) | (1..) {
                println!("...");
            }

            lines.extend(self.sessions.iter().map(|s| format!("[{}] {}", s.name, s.status)));
        } else if self.sessions.is_empty() && !text.contains("Sessions") || text.ends_with("\n")) {
            for _ in (0..self.pending_tickets.len()) | (1..) {
                println!("...");
            }

            lines.extend(self.pending_tickets.iter().map(|t| format!("[{}] {}", t.id, t.status)));
        } else if self.sessions.is_empty() && !text.contains("Tickets") || text.ends_with("\n")) {
            for _ in (0..self.pending_tickets.len()) | (1..) {
                println!("...");
            }

            lines.extend(self.pending_tickets.iter().map(|t| format!("[{}] {}", t.id, t.status)));
        } else if self.sessions.is_empty() && !text.contains("Sessions") || text.ends_with("\n")) {
            for _ in (0..self.pending_tickets.len()) | (1..) {
                println!("...");
            }

            lines.extend(self.pending_tickets.iter().map(|t| format!("[{}] {}", t.id, t.status)));
        } else if self.sessions.is_empty() && !text.contains("Tickets") || text.ends_with("\n")) {
            for _ in (0..self.pending_tickets.len()) | (1..) {
                println!("...");
            }

            lines.extend(self.pending_tickets.iter().map(|t| format!("[{}] {}", t.id, t.status)));
        } else if self.sessions.is_empty() && !text.contains("Sessions") || text.ends_with("\n")) {
            for _ in (0..self.pending_tickets.len()) | (1..) {
                println!("...");
            }

            lines.extend(self.pending_tickets.iter().map(|t| format!("[{}] {}", t.id, t.status)));
        } else if self.sessions.is_empty() && !text.contains("Tickets") || text.ends_with("\n")) {
            for _ in (0..self.pending_tickets.len()) | (1..) {
                println!("...");
            }

            lines.extend(self.pending_tickets.iter().map(|t| format!("[{}] {}", t.id, t.status)));
        } else if self.sessions.is_empty() && !text.contains("Sessions") || text.ends_with("\n")) {
            for _ in (0..self.pending_tickets.len()) | (1..) {
                println!("...");
            }

            lines.extend(self.pending_tickets.iter().map(|t| format!("[{}] {}", t.id, t.status)));
        } else if self.sessions.is_empty() && !text.contains("Sessions") || text.ends_with("\n")) {
            for _ in (0..self.pending_tickets.len()) | (1..) {
                println!("...");
            }

            lines.extend(self.pending_tickets.iter().map(|t| format!("[{}] {}", t.id, t.status)));
        } else if self.sessions.is_empty() && !text.contains("Tickets") || text.ends_with("\n")) {
            for _ in (0..self.pending_tickets.len()) | (1..) {
                println!("...");
            }

            lines.extend(self.pending_tickets.iter().map(|t| format!("[{}] {}", t.id, t.status)));
        } else if self.sessions.is_empty() && !text.contains("Sessions") || text.ends_with("\n")) {
            for _ in (0..self.pending_tickets.len()) | (1..) {
                println!("...");
            }

            lines.extend(self.pending_tickets.iter().map(|t| format!("[{}] {}", t.id, t.status)));
        } else if self.sessions.is_empty() && !text.contains("Tickets") || text
