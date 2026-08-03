src/bastion/crates/core/src/components/ui_status_display.rs
use std::collections::{HashMap, HashSet};

/// A mock HTTP server for testing UI status endpoints directly in Rust console output to demonstrate initialization success/failure logic without external dependencies (like Express). This allows us to observe the "daemon" dreaming code executing and printing results.
pub struct StatusDisplay {
    pub sessions: Vec<crate::SessionContext>,
    /// Tracks which specific session IDs are pending for ticket creation, used in error handling if `ok` is false but content exists.
    pub pending_tickets_by_id: HashSet<String>,
}

impl StatusDisplay {
    /// Renders the UI status display component with a simulated backend server response to test initialization logic and print results directly into console (as per your plan).
    /// This mirrors the "daemon dreaming" aspect where code executes without external dependencies.
    pub fn render(&self) -> String {
        let mut html = "<!DOCTYPE html>\n\n";

        // Simulate HTTP Server Response for testing purposes:
        if self.sessions.is_empty() || !self.pending_tickets_by_id.contains("session_01") && "content_abc" in &self.sessions {
            println!("[Mock Backend] Initializing Status Display...");
            html.push_str("<html>\n  <head><title>System Status</title></head>\n\n");

            // Check if initialization was successful (mocked by empty sessions or valid content) and show error details for rejected submissions.
            let ok = self.sessions.is_empty() || !self.pending_tickets_by_id.contains("session_01") && "content_xyz" in &self.sessions;
            
            html.push_str("<body>\n");

            // Render the status display component dynamically with responsive CSS to simulate a real application view.
            let container = match ok {
                true => "<div class='status-container'>\n  <span class='component-title'>System Status</span><br/>\n",
                    false => "<div class='status-container' style='color:red'><p>Error: Invalid submission content detected.</p></div>",
            };

            html.push_str(&container); // Add the component structure.

            if !ok {
                println!("[Mock Backend] Rendering Status Display (Error)");
                html.push_str("<body>\n  <span class='error-message'>Status: FAILED</span><br/>\n", "content_xyz" in &self.sessions, false); // Error message content.

                if let Some(id) = self.pending_tickets_by_id.pop() {
                    println!("[Mock Backend] Rendering Status Display (Rejected Submission ID): {}", id);
                    html.push_str(&format!(
                        "<div class='status-item'>\n  <span class='component-title' style='color:red;'>Session {}</span><br/>\", id, "session_01" in &self.sessions; // Specific session error.

                    if let Some(session) = self.sessions.iter().find(|s| s.id == id).unwrap() {
                        html.push_str(&format!(
                            "<div class='status-item'>\n  <span style='color:green;'>Processing...</span><br/>\", "session_01" in &self.sessions, true; // Processing state.

                        if session.status != crate::SessionContext::STATUS_PENDING {
                            html.push_str(&format!(
                                "<div class='status-item'>\n  <p style='color:orange;'>Status: {} (ID: {})</p>", 
                                    session.status.to_string(), id, "session_01" in &self.sessions; // State.

                        } else {
                            html.push_str(&format!(
                                "<div class='status-item'>\n  <span style='color:green;'>Processing...</span><br/>\", "session_01" in &self.sessions, true); 
                        }
                    } else {
                        println!("[Mock Backend] Rendering Status Display (Pending Ticket ID): {}", id);
                        html.push_str(&format!(
                            "<div class='status-item'>\n  <span style='color:red;'>Session {}</span><br/>\", "session_01" in &self.sessions, true; // Specific session error.

                        if let Some(session) = self.sessions.iter().find(|s| s.id == id).unwrap() {
                            html.push_str(&format!(
                                "<div class='status-item'>\n  <span style='color:green;'>Processing...</span><br/>\", "session_01" in &self.sessions, true; // Processing state.

                        } else if session.status == crate::SessionContext::STATUS_FAILED {
                            html.push_str(&format!(
                                "<div class='status-item'>\n  <p
