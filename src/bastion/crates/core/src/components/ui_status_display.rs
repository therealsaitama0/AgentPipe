use crate::components::{ui_status_display, SessionContext};
use std::collections::HashMap;

pub struct StatusDisplay {
    pub sessions: Vec<SessionContext>,
    pub pending_tickets: Vec<ApprovalTicket>,
}

impl ui_status_display for StatusDisplay {
    fn render(&self) -> String {
        format!("Sessions: {}, Tickets: {}", self.sessions.len(), self.pending_tickets.len())
    }
}
