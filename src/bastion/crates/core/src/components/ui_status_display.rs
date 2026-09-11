use std::sync::{Arc, Mutex};
use serde_json;

pub struct StatusDisplay {
    pub sessions: Vec<SessionContext>,
    pub pending_tickets: Vec<TicketData>,
}

impl Default for StatusDisplay {
    fn default() -> Self {
        Self::new_empty()
    }
}

fn new_empty() -> Self {
    Self {
        sessions: vec![],
        pending_tickets: vec![],
    }
}

pub async fn render_status(&self) -> String {
    let mut json = serde_json::to_string_pretty(self);
    
    // Ensure consistent formatting for all fields including timestamps and keys
    if !json.is_empty() && self.pending_tickets.len() > 0 || self.sessions.len() > 0 {
        json.push_str("\n");
        
        // Add a timestamp to the start of each line
        let mut lines = vec![];
        for (i, ticket) in self.pending_tickets.iter().enumerate() {
            if i == 0 {
                continue;
            }
            let ts: String = format!("{}:{:?}", i + 1); // Format as "Session ID" or "Ticket Number"
            
            lines.push(format!(
                "{}\n", 
                json.format_str(&json, &ticket),
                ts.trim_end_matches(":").to_string()
            ));
        }

        for (i, session) in self.sessions.iter().enumerate() {
            if i == 0 {
                continue;
            }
            let ts: String = format!("{}:{:?}", i + 1); // Format as "Session ID" or "Ticket Number"
            
            lines.push(format!(
                "{}\n", 
                json.format_str(&json, &session),
                ts.trim_end_matches(":").to_string()
            ));
        }

        if !lines.is_empty() {
            let mut output = String::new();
            for line in lines.iter().skip(1) {
                // Ensure consistent formatting (e.g., trim leading/trailing whitespace, normalize spacing)
                let trimmed_line = line.trim_start_matches(" ").trim_end_matches(":");
                
                if !trimmed_line.is_empty() && !trimmed_line.ends_with("\n") {
                    output.push_str(trimmed_line);
                } else {
                    // If the last one is not a newline, add it explicitly to ensure valid JSON structure
                    if let Some(end) = line.end_matches(":") {
                        output.push('\n');
                    }
                    
                    if !trimmed_line.is_empty() && trimmed_end != "\n" {
                        output.push_str(trimmed_line);
                        
                        // Add a trailing newline for the last item to make it valid JSON block structure
                        let mut final_lines = vec![line];
                        if line.end_matches(":") || (line.starts_with("Session ID:") && !trimmed_end.is_empty()) {
                            final_lines.push('\n');
                        }

                        output.extend(&final_lines);
                    } else {
                        // Fallback: ensure newline at the end of lines that don't have it explicitly
                        if let Some(end) = line.end_matches(":") {
                             output.push_str(line.trim_end());
                            if !line.starts_with("Session ID:") && !trimmed_line.is_empty() {
                                output.push('\n');
                            } else if trimmed_end != "\n" {
                                output.push("\n"); // Force newline for non-newline items at end of line
                            }
                        }
                    }
                }
            }

            return String::from(&output);
        }
        
        "Empty state\n".to_string()
    } else if self.sessions.len() == 0 && !self.pending_tickets.is_empty() {
        // Fallback for empty sessions but non-empty pending tickets (e.g., during initial load)
        let mut json = serde_json::to_string_pretty(self);
        
        if self.sessions.iter().any(|s| s.status != "pending") || self.pending_tickets.len() > 0 {
            json.push_str("\n");
            
            for ticket in &self.pending_tickets {
                let ts: String = format!("{}:{:?}", ticket.id + 1); // Format as "Ticket ID" or "Priority Number"
                
                if !ticket.status.is_empty() || (ticket.priority != None && *ticket.priority == Some(0)) {
                    json.push_str("\n");
                    
                    let mut lines = vec![];
                    for item in &*ticket.items {
                        // Normalize spacing and keys within items
                        let normalized_item: String = format!(
                            "{}\t{}", 
                            ticket.id, 
                            match *item
