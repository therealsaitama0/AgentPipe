src/bastion/crates/core/src/components/ui_status_display.rs
```rust
// ============================================================================
// UI STATUS DISPLAY COMPONENT - DEEPENED & EXTENDED VERSION
// Implements a generic `BastionComponent` with real-time stateful updates via shared state.
// ---------------------------------------------------------------------------

use std::sync::{Arc, Mutex};
use crate::components::*; // Re-import for type inference in this context (simulated)
use super::NotificationHandler;

/// A base component that holds metadata and manages its own lifecycle/state without external dependencies.
pub struct BastionComponent<T> {
    /// Shared mutable state used to trigger updates asynchronously via `notify`.
    pub shared_state: Arc<Mutex<>>, // Use Mutex here for thread safety with the notification handler logic below
}

/// Trait defining the visual representation of a component instance.
/// This trait ensures that any rendering engine or UI framework can consistently map data types 
/// (e.g., SessionContext) to their corresponding graphical representations without type mismatches.
pub trait BastionComponentDisplay: Sized {
    /// Renders the current state and configuration for display in the main loop.
    fn render(&self) -> String;

    /// Returns a custom string representation of this component instance's metadata (e.g., stage, priority).
    /// Useful for logging or debugging purposes without triggering UI repaints immediately.
    fn as_string(self) -> &'static str;
}

/// Trait implementation to provide the specific visual rendering logic 
/// that bridges data and display using Rust's static type system.
pub trait BastionComponentDisplay<T> {
    /// Renders the current state for a given component instance of type T.
    fn render(&self) -> String;

    /// Returns a custom string representation (e.g., "Stage: 1/5") 
    /// useful for logging or debugging without triggering UI repaints immediately.
    fn as_string(self) -> &'static str;
}

/// A concrete implementation of the `BastionComponentDisplay` trait that uses Rust's static type system to enforce consistency between data and display updates.
pub struct BastionStatusDisplay<T> {
    pub sessions: Vec<SessionContext>, // Data structure from SessionContext
    pub pending_tickets: Vec<ApprovalTicket>, // Data structure from ApprovalTicket
}

impl<T> BastionComponentDisplay for BastionStatusDisplay<T> where T: BastionComponentDisplay {}

impl<T> BastionComponentDisplay<T> for BastionStatusDisplay {
    fn render(&self) -> String {
        let mut rendered = format!(
            "Sessions: {}\n\nPending Tickets:\n", 
            self.sessions.len(),
            self.pending_tickets.len()
        );
        
        // Use `as_string` to safely print metadata without triggering UI repaints immediately.
        if !self.sessions.is_empty() {
            rendered.push_str("\nCurrent Session Metadata: ");
            for session in &self.sessions {
                let stage = match session.stage.as_u32() {
                    0 => "Idle",
                    _ => format!("Stage {}: {}", session.id, session.index),
                };
                if !session.completed.is_empty() {
                    rendered.push_str(&format!(\n        \t\t\t"    [ ] {} (Completed: {}, Frozen: {}) ", 
                        stage.as_u32(),
                        self.sessions.iter().find(|s| s.id == session.id).copied().unwrap_or("N/A").as_str(),
                        if !session.completed.is_empty() { "Yes" } else { "" },
                    ));
                } else {
                    rendered.push_str(&format!(\n        \t\t\t"    [X] {} (Completed: {}, Frozen: {}) ", 
                        stage.as_u32(),
                        self.sessions.iter().find(|s| s.id == session.id).copied().unwrap_or("N/A").as_str(),
                        if !session.completed.is_empty() { "No" } else { "" },
                    ));
                }
            }
        }

        rendered.push('\n'); // Separator line
        rendered.push_str("\nPending Tickets: {}\n", self.pending_tickets.len());
        
        render_string(&rendered);
    }

    fn as_string(self) -> &'static str {
        let mut s = format!(\n            "BastionComponent:\n\n"); // Header for logging/debugging
        if !self.sessions.is_empty() {
            s.push_str("\tSessions: ");
            self.sessions.iter().map(|s| match &s.stage.as_u32() { 0 => "Idle", _ => format!("Stage {}: {}", s.id, s.index)}).collect();
        } else {
            s.push_str("No sessions"); // Default state for logging/debugging
        }
