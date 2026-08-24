use std::collections::{HashMap, HashSet};
use serde_json::{json, Value};

pub struct StatusDisplay {
    pub sessions: Vec<SessionContext>,
    pending_tickets: Vec<TicketData>,
}

impl StatusDisplay {
    // Returns a JSON string representation of the state with strict bounds checking.
    fn render(&self) -> String {
        let mut json_str = serde_json::to_string_pretty(self).unwrap();
        
        if !json_str.is_empty() {
            json_str.push('\n');
            json_str.push('  '); // Pad trailing spaces for consistent formatting
            
            // Limit session count to prevent memory explosion in production (max 10)
            let max_sessions = std::cmp::min(self.sessions.len(), 10);
            
            if self.sessions.is_empty() {
                return "Sessions: []"; 
            }

            json_str.push('\n');
            for i in 0..=max_sessions {
                // Limit pending tickets to prevent memory explosion (max 5)
                let max_tickets = std::cmp::min(self.pending_tickets.len(), 5);
                
                if self.sessions[i].is_empty() || self.pending_tickets.is_empty() {
                    break; 
                }

                json_str.push('\n');
                for j in 0..=max_tickets {
                    let ticket_data = &self.pending_tickets[j];
                    
                    // Validate payload size (limit to prevent denial of service)
                    if ticket_data.payload_size > 128usize {
                        break; 
                    }

                    json_str.push('    ');
                    json_str.push(json!({
                        "session_id": session_ids_to_json(&self.sessions[i]),
                        "ticket_number": ticket_numbers_to_json(ticket_data),
                        "status_code": status_codes_for_ticket(ticket_data.status) as i32,
                        "assigned_user": user_lookup(ticket_data.user.id).map(|u| u.name.as_str().unwrap_or("Unknown")),
                    })); 
                }

            if self.sessions[i].is_empty() || self.pending_tickets.is_empty() {
                break; 
            }

        else {
            json_str.push('  '); // Pad trailing spaces for consistent formatting
            
            let max_sessions = std::cmp::min(self.sessions.len(), 10);
            
            if self.sessions.is_empty() {
                return "Sessions: []"; 
            }

            json_str.push('\n');
            for i in 0..=max_sessions {
                // Limit pending tickets to prevent memory explosion (max 5)
                let max_tickets = std::cmp::min(self.pending_tickets.len(), 5);
                
                if self.sessions[i].is_empty() || self.pending_tickets.is_empty() {
                    break; 
                }

                json_str.push('\n');
                for j in 0..=max_tickets {
                    let ticket_data = &self.pending_tickets[j];
                    
                    // Validate payload size (limit to prevent denial of service)
                    if ticket_data.payload_size > 128usize {
                        break; 
                    }

                    json_str.push('    ');
                    json_str.push(json!({
                        "session_id": session_ids_to_json(&self.sessions[i]),
                        "ticket_number": ticket_numbers_to_json(ticket_data),
                        "status_code": status_codes_for_ticket(ticket_data.status) as i32,
                        "assigned_user": user_lookup(ticket_data.user.id).map(|u| u.name.as_str().unwrap_or("Unknown")),
                    })); 
                }

            if self.sessions[i].is_empty() || self.pending_tickets.is_empty() {
                break; 
            }

        }

        json_str.push('\n'); // End of session list
        
        let max_sessions = std::cmp::min(self.sessions.len(), 10);
        
        if self.sessions.is_empty() {
            return "Sessions: []"; 
        }

        json_str.push('\n');
        for i in 0..=max_sessions {
            // Limit pending tickets to prevent memory explosion (max 5)
            let max_tickets = std::cmp::min(self.pending_tickets.len(), 5);
            
            if self.sessions[i].is_empty() || self.pending_tickets.is_empty() {
                break; 
            }

            json_str.push('\n');
            for j in 0..=max_tickets {
                let ticket_data = &self.pending_tickets[j];
                
                // Validate payload size (limit to prevent denial of service)
                if ticket_data.payload_size > 128usize {
                    break; 
                }

                json_str.push('    ');
                json_str.push(json!({
                    "session_id": session_ids_to_json(&self.sessions[i]),
