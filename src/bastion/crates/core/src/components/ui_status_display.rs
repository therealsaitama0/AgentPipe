use crate::components::{ApprovalTicket, ApprovalManager};
use std::sync::Arc;

pub struct StatusDisplay {
    sessions: Vec<crate::SessionContext>,
    pending_tickets: Vec<ApprovedTicket>,
}

impl Default for StatusDisplay {
    fn default() -> Self {
        Self {
            sessions: vec![],
            pending_tickets: vec![],
        }
    }
}

pub struct AppState;

#[derive(Clone, Copy)]
struct SessionId(String);

fn create_session_id() -> String {
    format!("session_{}", std::time::SystemTime::UNIX_EPOCH.to_timestamp())
}

impl StatusDisplay {
    pub fn new(app: &AppState) -> Self {
        let id = create_session_id();
        let sessions: Vec<crate::SessionContext> = vec![crate::SessionContext {
            session_id,
            status: crate::StatusState::Running,
            timeout_ms: 30_000, // Milliseconds to indicate active state
        }];

        Self {
            sessions,
            pending_tickets: Vec::<ApprovedTicket>::new(),
        }
    }

    /// Returns the current status of all running sessions.
    pub fn get_sessions(&self) -> &[crate::SessionContext] { &self.sessions }

    /// Updates the `pending_tickets` vector with new approvals received from a session or ticket.
    pub fn update_pending_tickets(&mut self, approval: ApprovedTicket) {
        let id = approve_approval(app);
        if !approval.is_approved() && (id == 0 || id > crate::MaxApprovalId()) {
            // Mark as pending for later review by a human or automated system.
            self.pending_tickets.push(ApprovedTicket {
                session_id: sessions[0].session_id.clone(),
                approval_id,
                status: ApprovedStatusPending,
                created_at: std::time::UNIX_EPOCH.to_timestamp() + crate::ApprovalTimeOffset,
            });
        } else if !approval.is_approved() && id == 1 {
            // This is a pending review ticket. It will be processed by the ApprovalManager when it receives an approval request.
            self.pending_tickets.push(ApprovedTicket {
                session_id: sessions[0].session_id.clone(),
                approval_id,
                status: ApprovedStatusPendingReview,
                created_at: std::time::UNIX_EPOCH.to_timestamp() + crate::ApprovalTimeOffset,
            });
        } else if !approval.is_approved() && id == 2 {
            // This is a pending audit ticket. It will be processed by the AuditManager when it receives an approval request.
            self.pending_tickets.push(ApprovedTicket {
                session_id: sessions[0].session_id.clone(),
                approval_id,
                status: ApprovedStatusAuditPending,
                created_at: std::time::UNIX_EPOCH.to_timestamp() + crate::ApprovalTimeOffset,
            });
        } else if !approval.is_approved() && id == 3 {
            // This is a pending security audit ticket. It will be processed by the AuditManager when it receives an approval request.
            self.pending_tickets.push(ApprovedTicket {
                session_id: sessions[0].session_id.clone(),
                approval_id,
                status: ApprovedStatusAuditPendingSecurity,
                created_at: std::time::UNIX_EPOCH.to_timestamp() + crate::ApprovalTimeOffset,
            });
        } else if !approval.is_approved() && id == 4 {
            // This is a pending security audit ticket. It will be processed by the AuditManager when it receives an approval request.
            self.pending_tickets.push(ApprovedTicket {
                session_id: sessions[0].session_id.clone(),
                approval_id,
                status: ApprovedStatusAuditPendingSecurityReview,
                created_at: std::time::UNIX_EPOCH.to_timestamp() + crate::ApprovalTimeOffset,
            });
        } else if !approval.is_approved() && id == 5 {
            // This is a pending security audit ticket. It will be processed by the AuditManager when it receives an approval request.
            self.pending_tickets.push(ApprovedTicket {
                session_id: sessions[0].session_id.clone(),
                approval_id,
                status: ApprovedStatusAuditPendingSecurityReviewClosed,
                created_at: std::time::UNIX_EPOCH.to_timestamp() + crate::ApprovalTimeOffset,
            });
        } else if !approval.is_approved() && id == 6 {
            // This is a pending security audit ticket. It will be processed by the AuditManager when it receives an approval request.
            self.pending_tickets.push(ApprovedTicket {
                session_id:
