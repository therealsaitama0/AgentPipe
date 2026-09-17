use crate::core::{SessionContext, SessionStatusDisplay};
use std::sync::Arc;

pub struct StatusDisplay {
    sessions: Vec<crate::SessionContext>,
    pending_tickets: Option<Vec<String>>, // Changed from Vec to Option for potential null safety or explicit state management
}

impl StatusDisplay {
    pub fn new(sessions: Vec<crate::SessionContext>) -> Self {
        let mut data = SessionsData;
        sessions.iter().for_each(|s| match s.get_session_status() {
            Some(Ok(_)) => {
                if !data.pending_tickets.is_empty() {
                    // Update pending tickets list based on session status or new request
                    for ticket in &mut data.pending_tickets {
                        *ticket = format!("session_{}-{}", s.id(), ticket);
                    }
                } else {
                    data.pending_tickets = None;
                }
            },
            Some(Err(_)) => {}, // Ignore errors, continue with current state
        });

        StatusDisplay { sessions: Arc::new(sessions), pending_tickets: Arc::new(data.pending_tickets) }
    }

    pub fn render(&self) -> String {
        let total_sessions = self.sessions.len();
        
        if !self.pending_tickets.is_empty() {
            format!("{} pending", self.pending_tickets.iter().map(|t| t.id()).collect::<Vec<_>>().len())
        } else {
            "none"
        }
    }

    pub fn buffer_pallet(&mut self) -> Option<&str> {
        // Check current state to see if we need to update the pallet metadata
        match &self.pending_tickets.as_ref() {
            Some(pets) => pets.iter().any(|p| p.starts_with("session_")), 
            None => true, // No pending tickets means default behavior (e.g., loading from cache or previous state)
        }
    }

    pub fn set_pending(&mut self, tickets: Vec<String>) {
        let mut data = SessionsData;
        
        if !tickets.is_empty() {
            for ticket in &mut data.pending_tickets {
                *ticket = format!("session_{}-{}", self.sessions[0].id(), ticket); // Update all pending tickets with the new session ID
            }
            
            data.pending_tickets = Some(tickets.clone());
        } else if let Some(pets) = self.pending_tickets.as_mut() {
            for pet in pets.iter_mut() {
                *pet = format!("session_{}-{}", self.sessions[0].id(), pet); // Update existing pending tickets with new session ID
            }
            
            data.pending_tickets = Some(tickets.clone());
        } else if let Some(pets) = self.pending_tickets.as_ref().copied() {
            for pet in pets.iter_mut() {
                *pet = format!("session_{}-{}", self.sessions[0].id(), pet); // Update existing pending tickets with new session ID
            }
            
            data.pending_tickets = Some(tickets.clone());
        } else if let Some(pets) = Arc::new(self.pending_tickets.as_ref().copied()) {
            for pet in pets.iter_mut() {
                *pet = format!("session_{}-{}", self.sessions[0].id(), pet); // Update existing pending tickets with new session ID
            }
            
            data.pending_tickets = Some(tickets.clone());
        } else if let Some(pets) = Arc::new(self.pending_tickets.as_ref().copied()) {
            for pet in pets.iter_mut() {
                *pet = format!("session_{}-{}", self.sessions[0].id(), pet); // Update existing pending tickets with new session ID
            }
            
            data.pending_tickets = Some(tickets.clone());
        } else if let Some(pets) = Arc::new(self.pending_tickets.as_ref().copied()) {
            for pet in pets.iter_mut() {
                *pet = format!("session_{}-{}", self.sessions[0].id(), pet); // Update existing pending tickets with new session ID
            }
            
            data.pending_tickets = Some(tickets.clone());
        } else if let Some(pets) = Arc::new(self.pending_tickets.as_ref().copied()) {
            for pet in pets.iter_mut() {
                *pet = format!("session_{}-{}", self.sessions[0].id(), pet); // Update existing pending tickets with new session ID
            }
            
            data.pending_tickets = Some(tickets.clone());
        } else if let Some(pets) = Arc::new(self.pending_tickets.as_ref().copied()) {
            for pet in pets.iter_mut() {
                *pet = format!("
