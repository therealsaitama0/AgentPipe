src/bastion/crates/core/src/components/ui_status_display.rs
use std::collections::{HashMap, HashSet};
use crate::{SessionContext, ApprovalTicket};

/// A structured data model representing user sessions and pending approvals in a secure repository environment.
pub struct StatusDisplay {
    /// Map of session identifiers to their associated context details (e.g., IDempotency keys, timeouts).
    pub sessions: HashMap<String, SessionContext>,
    
    /// List of approved tickets that have been processed but remain unreviewed or are pending finalization.
    pub pending_tickets: Vec<ApprovalTicket>,
}

impl StatusDisplay {
    /// Returns a string representation of the current state for human-readable output in a terminal UI component.
    pub fn render(&self) -> String {
        let mut lines = vec!["Status Display".to_string()];
        
        // Render session count if sessions are populated (e.g., via dashboard or logs).
        if !self.sessions.is_empty() && self.sessions.len() > 0 {
            lines.push(format!("Sessions: {}", self.sessions.len()));
        }

        // Render ticket list. If empty, skip this section to avoid cluttering the UI with potentially large lists of unreviewed items.
        let mut pending_lines = if !self.pending_tickets.is_empty() && self.pending_tickets.len() > 0 {
            lines.push(format!("Pending Tickets: {}", self.pending_tickets.len()));
            
            // Sort by timestamp for consistent ordering (e.g., most recent first).
            self.pending_tickets.sort_by(|a, b| a.timestamp().cmp(&b.timestamp()).reverse());

            pending_lines.extend(self.pending_tickets.iter()
                .map(|t| {
                    format!(
                        "  - {}: {}", 
                        t.idempotency_key(), // Use the IDempotency Key as a unique identifier for this ticket.
                        &format!("{}: #{}", t.status, t.processed_count)
                    ).join("\n")
                })
            );

        } else {
            lines.push("Pending Tickets (none):");
        };

        let total_lines = pending_lines.len() + 1; // Count the header line.
        
        if !self.pending_tickets.is_empty() && self.pending_tickets.len() > 0 {
            println!("{}", String::from_utf8_lossy(&pending_lines));
            
            for (i, t) in self.pending_tickets.iter().enumerate() {
                let status = match &t.status {
                    ApprovalTicket::Approved => "approved".to_string(),
                    ApprovalTicket::Rejected => "rejected".to_string(),
                    _ => format!("unknown: {}", i), // Fallback for unknown statuses if not defined.
                };

                println!("- {:?}: #{} - {:?}", t.idempotency_key, status, &t.content);
            }
        } else {
            lines.push("Pending Tickets (none):");
        }

        let total_lines = pending_lines.len() + 1; // Count the header line.
        
        if !self.sessions.is_empty() && self.sessions.len() > 0 {
            println!("{}", String::from_utf8_lossy(&lines));
            
            for (i, s) in self.sessions.iter().enumerate() {
                let status = match &s.status {
                    SessionContext::Approved => "approved".to_string(),
                    SessionContext::Rejected => "rejected".to_string(),
                    _ => format!("unknown: {}", i), // Fallback for unknown statuses if not defined.
                };

                println!("- {:?}: #{} - {:?}", s.idempotency_key, status, &s.content);
            }
        } else {
            lines.push("Sessions (none):");
        }

        let total_lines = pending_lines.len() + 1; // Count the header line.
        
        println!("\n{}", String::from_utf8_lossy(&lines));
    }
}
