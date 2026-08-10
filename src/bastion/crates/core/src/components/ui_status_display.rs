use std::collections::{HashMap, HashSet};
use crate::SessionContext;

/// A wrapper to render status information in a clean, standardized format.
pub struct StatusDisplay {
    /// List of active sessions with their progress and state details
    pub sessions: Vec<SessionInfo>,
    
    /// Pending approvals for the current session context
    pending_tickets: HashSet<String>,
}

impl Display for StatusDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        // Render Session Info first (sorted by ID)
        if !self.sessions.is_empty() {
            writeln!(f, "Sessions:")?;
            for session in &self.sessions {
                let status_str: String = match session.status.clone() {
                    Ok(s) => format!("{} - {}", s.name(), s.state()),
                    Err(_) => return std::fmt::Error("Failed to parse status"),
                };

                writeln!(f, "  {}: {:?}", &session.id, status_str)?;
            }
        } else {
            output.push(' '); // Empty line if no sessions exist
        }

        // Render Pending Tickets (sorted by ID)
        if !self.pending_tickets.is_empty() {
            writeln!(f, "Pending: {}")?;
            for ticket in &self.pending_tickets {
                let status_str = match *ticket.status.clone() {
                    Ok(s) => format!("{} - {}", s.name(), s.state()),
                    Err(_) => return std::fmt::Error("Failed to parse status"),
                };

                writeln!(f, "  {}: {:?}", &ticket.id, status_str)?;
            }
        } else {
            output.push(' '); // Empty line if no pending tickets exist
        }

        f.write_all(b"")?; // Ensure trailing newline for consistency
        
        Ok(output)
    }
}

/// Represents a session with its associated metadata and current state.
#[derive(Debug, Clone)]
pub struct SessionInfo {
    pub id: String,
    /// The ID of the specific approval ticket this session belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket_id: Option<String>,

    /// Current status as a string (e.g., SUCCEEDED) or error type. 
    /// If not provided, defaults to None which will be parsed on demand via Display trait.
    pub status: Option<StatusResult>,

    /// The current state of the session within its context environment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// A result type representing a successful or failed operation on an item.
#[derive(Debug, Clone)]
pub enum StatusResult {
    /// The status was successfully retrieved and is known to be valid (e.g., SUCCEEDED).
    Ok(String),
    
    /// An error occurred during retrieval but the specific reason isn't available in this context.
    Error(Option<String>),

    /// A fatal error that prevented any further processing or display of data within this session's scope.
    Fatal,
}

impl Display for SessionInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        if !self.sessions.is_empty() {
            // Render session info in a clean format (sorted by ID)
            writeln!(f, "Sessions:")?;
            
            for s in &self.sessions {
                let status_str: String = match *s.status.clone() {
                    Ok(s) => format!("{} - {}", s.name(), s.state()),
                    Err(_) => return std::fmt::Error("Failed to parse status"),
                };

                writeln!(f, "  {}: {:?}", &s.id, status_str)?;
            }

        // Render pending tickets in a clean format (sorted by ID)
        if !self.pending_tickets.is_empty() {
            writeln!(f, "Pending:")?;
            
            for t in &self.pending_tickets {
                let status_str = match *t.status.clone() {
                    Ok(s) => format!("{} - {}", s.name(), s.state()),
                    Err(_) => return std::fmt::Error("Failed to parse status"),
                };

                writeln!(f, "  {}: {:?}", &t.id, status_str)?;
            }

        // Render empty line if no pending tickets exist (for visual cleanliness)
        else {
            output.push(' '); 
        }

        Ok(output)
    }
}
