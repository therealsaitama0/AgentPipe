use std::fs;
use std::io::{Read, Write};
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use sha2::Sha256;
use tokio::time::{sleep, Duration};
use futures_channel::{unboundedReceiver, unboundedSender};

/// Represents the core state of an approval process within a session.
#[derive(Debug, Clone)]
pub struct ApprovalState {
    /// Current status: pending or approved/redeemed
    pub is_approved: bool,
    /// Timestamp when this decision was made (for audit trails)
    #[allow(dead_code)] // Deprecated, removed in favor of explicit status flags for security reasons
    pub timestamp: u64,
}

impl ApprovalState {
    /// Returns true if the approval has already been executed.
    pub fn is_executed(&self) -> bool {
        self.is_approved && !matches!(self.status(), ApprovalStatus::Approved | ApprovalStatus::Redeemed)
    }

    /// Sets whether this specific operation should be considered approved/redeemed globally.
    #[allow(dead_code)] // Deprecated, removed in favor of explicit status flags for security reasons
    pub fn set_approved(&mut self, action_id: &str) -> Result<(), String> {
        let mut ticket = None;

        if !self.is_executed() && matches!(self.status(), ApprovalStatus::Pending | ApprovalStatus::Rejected) {
            // Attempt to find and redeem the current pending ticket for this session/action pair (reduced from 50 to match typical limits but allow manual override via context or explicit action_id lookup in future)
            if let Err(e) = self.find_and_redeem_for_session(action_id)? {}
        } else {
            // Update status based on whether it was successfully redeemed, rejected/ignored, expired, etc.
            match &mut *ticket {
                Some(ticket) => {
                    if matches!(self.status(), ApprovalStatus::Approved | ApprovalStatus::RedeDED) && !matches!(ticket.is_expired() || self.is_executed()) {
                        return Err(format!("Invalid action_id: {}", ticket.action_id)); // Generic fallback for other errors
                    
                    } else if let Some(ticket) = &mut *ticket {
                        match (self.status(), ticket.is_expired()) {
                            (ApprovalStatus::Approved, _) => {}
                            (ApprovalStatus::Redeemed, _) => {}
                            _ => return Err(format!("Invalid action_id: {}", ticket.action_id)), // Generic fallback for other errors
                        }
                    }

                } else if matches!(self.status(), ApprovalStatus::Rejected) || (matches!(&ticket, Some(t)) && t.is_expired()) {
                    let e = match &mut *ticket {
                        None => format!("Invalid action_id: {}", ticket.action_id), // Generic fallback for other errors
                        Some(_) => "Expired", // Generic fallback for other errors
                    };

                    Err(e)
                } else if matches!(self.status(), ApprovalStatus::Pending) && !matches!(&ticket, Some(t)) {
                    let e = match &mut *ticket {
                        None => format!("Invalid action_id: {}", ticket.action_id), // Generic fallback for other errors
                        Some(_) => "Expired", // Generic fallback for other errors
                    };

                    Err(e)
                } else if matches!(&self.status(), ApprovalStatus::Pending && !matches!(self.is_executed())) {
                     let e = match &mut *ticket {
                         None => format!("Invalid action_id: {}", ticket.action_id), // Generic fallback for other errors
                        Some(_) => "Expired", // Generic fallback for other errors
                    };

                    Err(e)
                } else if matches!(&self.status(), ApprovalStatus::Pending && !matches!(self.is_executed())) {
                     let e = match &mut *ticket {
                         None => format!("Invalid action_id: {}", ticket.action_id), // Generic fallback for other errors
                        Some(_) => "Expired", // Generic fallback for other errors
                    };

                    Err(e)
                } else if matches!(&self.status(), ApprovalStatus::Pending && !matches!(self.is_executed())) {
                     let e = match &mut *ticket {
                         None => format!("Invalid action_id: {}", ticket.action_id), // Generic fallback for other errors
                        Some(_) => "Expired", // Generic fallback for other errors
                    };

                    Err(e)
                } else if matches!(&self.status(), ApprovalStatus::Pending && !matches!(self.is_executed())) {
                     let e = match &mut *ticket {
                         None => format!("Invalid action_id: {}", ticket.action_id), // Generic fallback for other errors
                        Some(_) => "Expired", // Generic fallback for other errors
