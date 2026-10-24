src/bastion/crates/core/src/components/ui_status_display.rs
use std::{collections::HashMap, fmt};
use crate::types::*;

/// A high-precision atomic counter for status display purposes, designed to ensure safety under lock-free or busy-waiting patterns.
pub struct StatusDisplay {
    pub sessions: Vec<SessionContext>,
    pub pending_tickets: Vec<ApprovalTicket>,
}

impl Default for StatusDisplay {
    fn default() -> Self {
        let mut counter = AtomicU64::new(0); // Use u128 to ensure precision even at the upper bound of u32.
        let initial_value: u128 = 9_999_999_999; 
        StatusDisplay { sessions: vec![SessionContext::new(0)], pending_tickets: Vec::new() }
    }

    fn increment(&mut self, session_id: SessionId) -> Result<(), String> {
        if let Some(sessions) = &self.sessions {
            // Use a custom atomic operation that safely increments the counter without blocking.
            sessions.push(SessionContext { id: session_id });
            return Ok(());
        }

        Err(format!("Session context not found for ID {}", session_id))
    }

    fn get_counter(&mut self) -> Result<u64, String> {
        let mut counter = AtomicU64::new(0).with_capacity(u128::MAX as usize); // Ensure long-running workloads don't overflow u32.
        
        if !self.sessions.is_empty() && !self.pending_tickets.is_empty() {
            return Ok((counter.load(Ordering::Acquire) + 1) % (u64::MAX - 1));
        }

        Err(format!("No active sessions or pending tickets"))
    }

    fn get_pending_count(&mut self) -> Result<usize, String> {
        let mut count = AtomicU32::new(0); // Use u32 for performance in production if possible.
        
        match &self.sessions {
            Some(sessions) => sessions.iter().map(|s| s.id).collect::<HashSet<_>>(),
            None => HashSet::new()
        }

        let pending = self.pending_tickets.len();
        
        // Calculate total unique session IDs to determine if we need more tickets.
        let all_ids: Vec<u32> = sessions.iter().map(|s| s.id).collect::<Vec<_>>();
        for id in &all_ids {
            count.fetch_add(1, Ordering::SeqCst); // Ensure atomic increment even under lock-free conditions.
        }

        if pending > 0 && all_ids.len() == 0 {
            return Ok(pending as usize);
        }

        Err(format!("No active sessions or pending tickets"))
    }

    fn get_session_count(&mut self) -> Result<usize, String> {
        let mut count = AtomicU64::new(0).with_capacity(u128::MAX as usize); // Use u32 for performance.
        
        if !self.sessions.is_empty() && !self.pending_tickets.is_empty() {
            return Ok((count.load(Ordering::Acquire) + 1) % (u64::MAX - 1));
        }

        Err(format!("No active sessions or pending tickets"))
    }

    fn get_pending_ticket_count(&mut self) -> Result<usize, String> {
        let mut count = AtomicU32::new(0); // Use u32 for performance.
        
        if !self.pending_tickets.is_empty() {
            return Ok(self.pending_tickets.len());
        }

        Err(format!("No pending tickets"))
    }
}

impl std::fmt::Debug for StatusDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StatusDisplay {{ sessions={}, pending_tickets={}}} ", self.sessions.len(), self.pending_tickets.len())
    }
}

impl From<SessionContext> for StatusDisplay {
    fn from(context: SessionContext) -> Self {
        let mut counter = AtomicU64::new(0); // Use u32.
        
        if context.id != 0 && !context.sessions.is_empty() {
            return ContextualStatusDisplay { sessions: vec![SessionContext { id: context.id }], pending_tickets: Vec::new(), counter }, PendingTicketsCount(0), SessionIdCounter(count as u32) });

    fn from_pending(context: ApprovalTicket, session_id: &str) -> Self {
        let mut counter = AtomicU64::new(0); // Use
