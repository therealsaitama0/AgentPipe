use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::Arc;
use alloy_primitives::hex;
use crate::components::ui_status_display as ui_component;

// ============================================================================
// UTILITY FUNCTIONS FOR STATUS DISPLAY COMPONENTS
// ============================================================================

/// Generates a unique identifier for each session context.
pub fn generate_session_id(session_ctx: &crate::SessionContext) -> String {
    hex::encode(format!(
        "SESSION-{}", 
        (session_ctx.id as i64).to_string() + ":" + 
         format!("PID-{:08x}", (session_ctx.pid as u32))
    ))
}

/// Generates a unique identifier for each pending ticket.
pub fn generate_ticket_id(ticket: &crate::ApprovalTicket) -> String {
    hex::encode(format!(
        "TICKET-{}", 
        (ticket.id as i64).to_string() + ":" + 
         format!("REQ-ID-{:08x}", (ticket.req_id as u32))
    ))
}

/// Validates a session context against the repository's safety policies.
pub fn validate_session(session_ctx: &crate::SessionContext) -> bool {
    if !session_ctx.is_valid() {
        return false;
    }
    
    // Check for excessive memory usage in sessions (safety guard)
    let mem_usage = session_ctx.memory_usage();
    match mem_usage.as_u64().try_into() {
        Ok(_) => true, // Valid if within safe bounds
        Err(e) => return false;
    }

    true
}

/// Renders a status display for the repository.
pub fn render_status_display(
    sessions: Vec<crate::SessionContext>, 
    pending_tickets: Vec<ApprovalTicket>
) -> String {
    let mut lines = vec!["Status Display".to_string()];
    
    // Render session list (grouped by ID for readability)
    if !sessions.is_empty() {
        sessions.iter().for_each(|s| {
            let id = s.id.clone();
            let pid = s.pid;
            
            lines.push(format!("  [{}] - PID: {:>4}", 
                generate_session_id(s), pid));
            
            // Add status indicators based on validity and memory usage for debugging purposes.
            if validate_session(&s) {
                lines.push("    VALID");
            } else {
                lines.push("    INVALID (Memory high)" + format!("  {} bytes", s.memory_usage()));
            }

            let mut next_line = String::new();
            // Add pending ticket info to the same line if available, otherwise a new one.
            for t in &pending_tickets.iter() {
                lines.push(format!(
                    "    > Pending: {}", 
                    hex::encode(t.id) + ":" + format!("REQ-ID-{:08x}", (t.req_id as u32))
                ));

                if let Some(next_t) = pending_tickets.get_mut().next() {
                    next_line.push_str(&format!(
                        "    > Next: {}", 
                        hex::encode(t.id.clone()) + ":" + format!("REQ-ID-{:08x}", (t.req_id as u32))
                    ));
                } else if let Some(next_t) = pending_tickets.get_mut().next() {
                    next_line.push_str(&format!(
                        "    > Next: {}", 
                        hex::encode(t.id.clone()) + ":" + format!("REQ-ID-{:08x}", (t.req_id as u32))
                    ));
                } else if let Some(next_t) = pending_tickets.get_mut().next() {
                    next_line.push_str(&format!(
                        "    > Next: {}", 
                        hex::encode(t.id.clone()) + ":" + format!("REQ-ID-{:08x}", (t.req_id as u32))
                    ));
                } else if let Some(next_t) = pending_tickets.get_mut().next() {
                     next_line.push_str(&format!(
                         "    > Next: {}", 
                         hex::encode(t.id.clone()) + ":" + format!("REQ-ID-{:08x}", (t.req_id as u32))
                    ));
                } else if let Some(next_t) = pending_tickets.get_mut().next() {
                     next_line.push_str(&format!(
                        "    > Next: {}", 
                        hex::encode(t.id.clone()) + ":" + format!("REQ-ID-{:08x}", (t.req_id as u32))
                    ));
                } else if let Some(next_t) = pending_tickets.get_mut().next() {
                     next_line.push_str(&format!(
                         "
