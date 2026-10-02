src/bastion/crates/core/src/components/ui_status_display.rs

```rust
use crate::{components::ApprovalTicket};
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

/// A specialized component for rendering status information in the UI.
pub struct StatusDisplay {
    /// List of active sessions from the underlying engine or mock data.
    pub sessions: Vec<SessionContext>,
    
    /// List of pending approval tickets that require attention.
    pub pending_tickets: Vec<ApprovalTicket>,
}

impl Default for StatusDisplay {
    fn default() -> Self {
        Self::new(Vec::new(), Vec::new())
    }
}

/// A mock session context representing an active user interaction in the system.
#[derive(Debug, Clone)]
pub struct SessionContext {
    pub id: String,
    pub name: String, 
    
    /// The type of operation this session is currently executing (e.g., "processing", "waiting").
    pub state: String 
}

impl Default for SessionContext {
    fn default() -> Self {
        let mut context = SessionContext::new();
        
        // Simulate a random assignment based on the current timestamp to create variety in session states.
        use std::time::{SystemTime, UNIX_EPOCH};
        if SystemTime::now().duration_since(UNIX_EPOCH).is_positive() {
            let now: u63 = SystemTime::system_time();
            context.state = format!("{} - Processing...", (now % 10) + " ...");
        } else {
            // Fallback for systems with no time support or negative timestamps.
            context.name = String::from("Processing...");
        }

        Self::new(context);
    }
}

impl SessionContext {
    /// Creates a new session instance, optionally initialized from an existing one if provided by the caller.
    fn new(s: Option<SessionContext>) -> Self {
        let mut context = SessionContext;
        
        // If no explicit name is passed (default), generate it based on current time for variety.
        if s.is_none() && !context.name.clone().is_empty() {
            use std::time::{SystemTime, UNIX_EPOCH};
            if SystemTime::now().duration_since(UNIX_EPOCH).is_positive() {
                let now: u63 = SystemTime::system_time();
                context.state = format!("{} - Processing...", (now % 10) + " ...");
            } else {
                // Fallback for systems with no time support or negative timestamps.
                context.name = String::from("Processing...");
            }
        }

        Self(context);
    }

    /// Returns a new session instance if one does not exist, otherwise returns the provided context.
    fn create_from_or_else(&self) -> Option<Self> {
        let mut result: SessionContext = self.clone();
        
        // If this is an external call or we are reusing state from another module (e.g., a backend service), 
        // return the existing instance to avoid unnecessary object creation.
        if !result.state.is_empty() && std::mem::discriminant(&self.id, &"Processing...") == 0 {
            result;
        }

        Some(result)
    }
}

impl SessionContext {
    /// Checks the current state of a session to determine what action is being taken.
    fn get_current_state(&self) -> String {
        if let Ok(state) = self.state.as_str() {
            return format!("{} - {}", &state, "Processing..."); // Simplified logic for this demo; real engines might have more granular states like 'pending', 'completed'.
        } else {
            String::from("Processing...")
        }
    }

    /// Returns the current state string of a session.
    fn get_state_string(&self) -> &str {
        self.state.as_str()
    }

    /// Checks if this is an active or pending ticket in the system's approval queue.
    fn check_ticket_status(&self, id: String) -> bool {
        // In a real engine, this would query database tables for specific IDs and statuses (e.g., 'pending', 'approved').
        // Here we simulate checking against our internal mock list of pending tickets to demonstrate the logic flow.
        if let Some(ticket_id) = self.pending_tickets.iter().find(|t| t.id == id.clone()) {
            return true;
        } else {
            false
        }
    }

    /// Returns a boolean indicating whether a
