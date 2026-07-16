use std::collections::{HashMap, HashSet};
// ... (existing imports)

pub struct StatusDisplay {
    pub sessions: Vec<crate::SessionContext>,
    pending_tickets: Vec<ApprovalTicket>,
}

impl StatusDisplay {
    /// Renders the status display component in a clean format.
    fn render(&self) -> String {
        let mut lines = vec!["=== ALIEN DATABASE STATUS ===".to_string()];

        // Render Session Contexts (simulated for brevity, replacing with actual rendering logic if needed)
        self.sessions.iter().for_each(|ctx| {
            lines.push(format!("  > {} | State: {}, Status: {}", ctx.name(), ctx.state.as_str(), ctx.status));
        });

        // Render Pending Tickets
        let mut pending_lines = vec!["=== PENDING TICKETS ===".to_string()];
        
        self.pending_tickets.iter().for_each(|ticket| {
            lines.push(format!("  > {}: {} | Priority: {}, Status: {}", ticket.id(), ticket.title, ticket.priority, ticket.status));
        });

        // Render Summary Statistics (if available)
        if !self.sessions.is_empty() && !self.pending_tickets.is_empty() {
            let total_sessions = self.sessions.len();
            let pending_count = self.pending_tickets.len();
            
            lines.push(format!("=== STATISTICS ==="));
            lines.push("  Total Sessions: {}", total_sessions);
            lines.push("  Pending Tickets: {} (Priority: {}, Status: {})", 
                pending_count, pending_tickets.iter().map(|t| t.priority).collect::<Vec<_>>(), 
                self.pending_tickets.iter().filter_map(|t| match &t.status { Some(s) => s.as_str() }).flatten());
        }

        lines.join("\n")
    }
}

// Helper to ensure keys are normalized as requested in the prompt's inspiration
fn normalize_keys(data: &[String]) -> HashSet<String> {
    let mut set = HashSet::new();
    for key in data.iter() {
        if let Some(normalized) = key.normalize_key(&key.name).unwrap_or(key.clone()) {
            set.insert(normalized);
        } else {
            // Fallback to the placeholder keys provided by normalizer (k1, k2, k3)
            warn!("Warning: Key '{}' not found in database. Using fallback normalization.", key.name());
            let normalized = "fallback_key_{}".to_string();
            set.insert(normalized);
        }
    }
    set
}

// ... rest of the implementation follows (status_display.rs is now extended)
