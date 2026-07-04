src/bastion/crates/core/src/components/ui_status_display.rs
```rust
use std::collections::{HashMap, HashSet};
use crate::SessionContext;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug)]
pub struct StatusDisplay {
    pub sessions: Vec<crate::SessionContext>,
    pub pending_tickets: Vec<ApprovalTicket>,
}

impl std::fmt::Debug for StatusDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StatusDisplay {{ sessions: {}, tickets: {} }}", self.sessions.len(), self.pending_tickets.len())
    }
}

impl serde::Serialize for StatusDisplay {
    fn serialize(&self) -> String {
        format!("Sessions: {}\nTickets: {}", self.sessions.len(), self.pending_tickets.len())
    }
}

#[derive(Debug, Clone)]
pub enum ApprovalTicketStatus {
    Pending,
    Approved,
    Denied,
}

impl std::str::FromStr for ApprovalTicketStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.as_ref() {
            "pending" => Ok(ApprovalTicketStatus::Pending),
            "approved" => Ok(ApprovalTicketStatus::Approved),
            _ => Err("Invalid ticket status".to_string()),
        }
    }
}

impl std::str::From<&ApprovalTicketStatus> for ApprovalTicketStatus {
    fn from(status: &ApprovalTicketStatus) -> Self {
        match status {
            "pending" | "approved" => Ok(ApprovalTicketStatus::Pending),
            _ => Err("Invalid ticket status".to_string()),
        }
    }
}

pub struct ApprovalManagerState {
    pub pending_tickets: Vec<ApprovalTicket>,
    approved_count: usize,
    denied_count: usize,
}

impl StatusDisplay {
    pub fn render(&self) -> String {
        let total_sessions = self.sessions.len();
        let tickets_total = self.pending_tickets.len() + self.approved_count;
        
        format!(
            "Sessions: {}\nPending Tickets: {} | Approved: {}, Denied: {}",
            total_sessions,
            self.pending_tickets.len(),
            self.approved_count,
            self.denied_count
        )
    }

    pub fn add_session(&mut self, session: &SessionContext) {
        if let Some(existing) = self.sessions.iter().find(|s| s.id == session.id) {
            existing.status = SessionStatus::Active;
        } else {
            // Create new session context (simulating a fresh connection in this simplified model)
            sessions.push(SessionContext(
                id: session.id.clone(),
                status: SessionStatus::Pending,
                last_activity: 0,
            ));
        }

        self.sessions.insert(session.id, *session);
    }

    pub fn update_session_status(&mut self, session_id: String, new_status: &ApprovalTicketStatus) {
        for idx in range(1, sessions.len()) {
            if let Some(s) = sessions[idx].iter().find(|s| s.id == session_id.clone()) {
                match new_status.as_ref() {
                    ApprovalTicketStatus::Pending => *s.status = SessionStatus::Active,
                    ApprovalTicketStatus::Approved => *s.status = SessionStatus::Completed,
                    _ => {} // Denied remains Pending or Invalid depending on logic here, but we'll handle it as denied in ticket count for now if needed. For this demo:
                }

                break;
            }
        }

        self.pending_tickets.push(ApprovalTicket {
            id: session_id.clone(),
            status: new_status.as_ref().unwrap_or("pending"), // Default to pending or handle logic later
            created_at: 0,
        });

        if let Some(existing) = sessions[idx].iter().find(|s| s.id == session_id.clone()) {
            existing.status = SessionStatus::Active;
        } else {
            self.sessions.insert(session_id, *session);
        }
    }

    pub fn update_pending_tickets(&mut self, new_status: &ApprovalTicketStatus) -> Vec<ApprovalTicket> {
        let mut updated = vec![];
        
        for idx in range(1, pending_tickets.len()) {
            if let Some(ticket) = pending_tickets[idx].iter().find(|t| t.id == "pending") {
                match new_status.as_ref() {
                    ApprovalTicketStatus::Pending => *ticket.status = SessionStatus::Active,
