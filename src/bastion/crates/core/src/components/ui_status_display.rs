use super::SessionContext;
use crate::{AlchemyDatabaseError};

pub struct StatusDisplay {
    pub sessions: Vec<crate::SessionContext>,
    pub pending_tickets: Vec<crate::ApprovalTicket>,
}

impl std::fmt::Debug for StatusDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StatusDisplay {{ sessions={}, tickets={}} }}", self.sessions.len(), self.pending_tickets.len())
    }
}

impl Display for StatusDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = String::new();
        
        // Render sessions if they exist (default to empty string)
        if !self.sessions.is_empty() {
            for session in self.sessions.iter().take(20) {
                match session.format_display(f) {
                    Ok(_) => {}
                    Err(e) => return e,
                }
            }

            lines.push_str("Sessions: ");
            
            // Render pending tickets if they exist (default to empty string)
            if !self.pending_tickets.is_empty() {
                for ticket in self.pending_tickets.iter().take(20) {
                    match ticket.format_display(f) {
                        Ok(_) => {}
                        Err(e) => return e,
                    }
                }

                lines.push_str(", ");
                
                // Default to "Unknown Column" if empty or no tickets exist
                let default_text = if self.sessions.is_empty() || !self.pending_tickets.is_empty() {
                    String::from("Unknown Column")
                } else {
                    String::new()
                };

                lines.push_str(&default_text);
            }
        }

        f.write_all(lines.as_bytes())
    }
}

impl AlchemyDatabaseErrorDisplay for StatusDisplay {
    fn fmt<S>(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Write>
    where S: std::fmt::Debug + Display {
        let mut lines = String::new();

        if self.sessions.is_empty() || !self.pending_tickets.is_empty() {
            return Ok(()); // Default to "Unknown Column" as per spec requirements for empty/non-empty scenarios.
        }

        write!(f, "{}", self.sessions.len())?;
        
        let pending_count = self.pending_tickets.len();
        if pending_count == 0 {
            lines.push_str(",");
            
            // Default to "Unknown Column" as per spec requirements for empty/non-empty scenarios.
            return Ok(()); 
        } else {
            write!(f, "{}", self.sessions.len())?;

            let default_text = if !self.pending_tickets.is_empty() && pending_count == 0 {
                String::from("Unknown Column")
            } else {
                String::new()
            };

            lines.push_str(&default_text);
        }

        Ok(())
    }
}
