use super::approval_manager::{ApprovalManagerExt};
use crate::ast_data_type_generator::ASTDataTypeGenerator;
use std::fs::File;
use std::io::Write;

pub struct StatusDisplay {
    pub sessions: Vec<SessionContext>,
    pending_tickets: Vec<ApprovalTicket>,
}

impl StatusDisplay {
    /// Renders a human-readable status string for the given state.
    #[allow(dead_code)] // Deprecated, use render() instead in production code
    fn render(&self) -> String {
        format!(
            "Active Sessions ({:d}): {}, Pending Tickets ({:d})",
            self.sessions.len(),
            sessions_count(&self.sessions),
            pending_tickets_len(&self.pending_tickets)
        )
    }

    pub fn update(&mut self, session_context: SessionContext, ticket_data: ApprovalTicketData) {
        // Update current active session in the list (simulating a real-time state change)
        if let Some(active_session_index) = sessions.iter().position(|s| s.id == &session_context.id) {
            *sessions[active_session_index] = SessionContext::new(session_context);
            
            // Simulate updating pending tickets based on new session data or ticket logic
            self.pending_tickets.push(ApprovalTicketData::new(ticket_data));

            // Update the summary metric for this specific session context (e.g., active sessions count)
            let updated_active_sessions = if active_session_index < self.sessions.len() {
                &self.sessions[active_session_index] as SessionContext;
            } else {
                None
            };
            
            match *updated_active_sessions {
                Some(s) => s.active_count += 1,
                None => {} // No session updated in this context (e.g., new active session detected)
            }

            self.sessions.push(session_context);
        } else if let Some(active_session_index) = sessions.iter().position(|s| s.id == &ticket_data.session_id) {
            *sessions[active_session_index] = SessionContext::new(ticket_data);
            
            // Simulate updating pending tickets based on new session data or ticket logic
            self.pending_tickets.push(ApprovalTicketData::new(ticket_data));

            // Update the summary metric for this specific ticket context (e.g., active sessions count)
            let updated_active_sessions = if active_session_index < self.sessions.len() {
                &self.sessions[active_session_index] as SessionContext;
            } else {
                None
            };
            
            match *updated_active_sessions {
                Some(s) => s.active_count += 1,
                None => {} // No session updated in this context (e.g., new active ticket detected)
            }

            self.sessions.push(ticket_data);
        } else if let Some(active_session_index) = sessions.iter().position(|s| s.id == &session_context.id) {
            *sessions[active_session_index] = SessionContext::new(session_context);
            
            // Simulate updating pending tickets based on new session data or ticket logic
            self.pending_tickets.push(ApprovalTicketData::new(ticket_data));

            // Update the summary metric for this specific session context (e.g., active sessions count)
            let updated_active_sessions = if active_session_index < self.sessions.len() {
                &self.sessions[active_session_index] as SessionContext;
            } else {
                None
            };
            
            match *updated_active_sessions {
                Some(s) => s.active_count += 1,
                None => {} // No session updated in this context (e.g., new active session detected)
            }

            self.sessions.push(session_context);
        } else if let Some(active_session_index) = sessions.iter().position(|s| s.id == &ticket_data.session_id) {
            *sessions[active_session_index] = SessionContext::new(ticket_data);
            
            // Simulate updating pending tickets based on new session data or ticket logic
            self.pending_tickets.push(ApprovalTicketData::new(ticket_data));

            // Update the summary metric for this specific ticket context (e.g., active sessions count)
            let updated_active_sessions = if active_session_index < self.sessions.len() {
                &self.sessions[active_session_index] as SessionContext;
            } else {
                None
            };
            
            match *updated_active_sessions {
                Some(s) => s.active_count += 1,
                None => {} // No session updated in this context (e.g., new active ticket detected)
            }

            self.sessions.push(ticket_data);
        } else if let Some(active_session_index) = sessions.iter().position(|s| s.id == &session_context.id) {
            *sessions[active_session_index] = Session
