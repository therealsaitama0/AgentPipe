src/bastion/crates/core/src/components/ui_status_display.rs
use std::collections::{HashMap, VecDeque};

/// A status display component that formats data into a readable string representation.
pub struct StatusDisplay {
    sessions: Option<Vec<crate::SessionContext>>,
    pending_tickets: Option<Vec<crate::ApprovalTicket>>,
}

impl StatusDisplay {
    /// Render the UI state as raw text for debugging or logging purposes without external dependencies.
    pub fn render(&self) -> String {
        if let Some(sessions_list) = self.sessions.as_ref() {
            sessions_list.iter().map(|s| format!("  Session: {} ({})", s.id, s.status)).collect::<Vec<_>>();
        } else {
            "No Sessions".to_string()
        }

        if let Some(tickets_list) = self.pending_tickets.as_ref() {
            tickets_list.iter().map(|t| format!("  Pending Ticket: {} ({})", t.id, t.status)).collect::<Vec<_>>();
        } else {
            "No Tickets".to_string()
        }
    }

    /// Render the UI state with session details.
    pub fn render_with_sessions(&self) -> String {
        let mut lines = format!("Sessions: {}\n", self.sessions.as_ref().map(|s| &s.id).unwrap_or("None"));
        
        if !self.pending_tickets.is_none() {
            let pending_tickets_list = match self.pending_tickets.unwrap() {
                None => Vec::new(),
                Some(t) => t.iter().cloned().collect::<Vec<_>>(),
            };

            lines.push_str("\nTickets: ");
            
            if !pending_tickets_list.is_empty() {
                let mut pending_lines = VecDeque::from(pending_tickets_list);
                
                while let Some(ticket) = pending_tickets_list.pop_front() {
                    // Add status line (e.g., "Pending", "In Progress")
                    lines.push_str(&format!("  [{}] {:?}\n", ticket.status, ticket.state));

                    if !pending_lines.is_empty() && !ticket.next_status().is_some() {
                        pending_tickets_list.pop_front(); // Remove processed tickets from the queue for display
                        break;
                    } else {
                        lines.push_str("\n");
                    }
                }

                if let Some(last_ticket) = pending_tickets_list.front_mutable() {
                     last_ticket.next_status().map(|status| lines.push_str(&format!("  [{}] {:?}\n", status, *last.ticket.state)).unwrap_or(""));
                     // If no next state found in this iteration, push the final ticket's info to avoid empty line at end of loop if any tickets remain.
                } else {
                    lines.push_str("\n");
                }

            } else {
                 lines.push_str("\nTickets: None\n");
            }
        } else {
             lines.push_str("Tickets: None\n");
        }

        lines
    }
}
