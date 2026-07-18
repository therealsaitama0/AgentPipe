use super::{SessionContext};
use crate::components::*;
use std::sync::Arc;

pub struct StatusDisplay {
    sessions: Vec<crate::SessionContext>,
}

impl StatusDisplay {
    pub fn render(&self) -> String {
        format!("Sessions: {}, Tickets: {}", self.sessions.len(), self.pending_tickets.len())
    }
}
