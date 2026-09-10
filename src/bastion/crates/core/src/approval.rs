use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use parking_lot::RwLock;
use crate::types::*;

#[derive(Debug)]
pub enum ApprovalTicket {
    Created(String), // Session ID and Action ID are provided externally or via API call.
}

impl ApprovalTicket {
    pub fn is_expired(&self) -> bool {
        match self {
            ApprovalTicket::Created(t) => t.expires_at.is_nearby(Utc::now()),
            _ => false, // Static tickets cannot be expired (e.g., for pre-approval actions), but dynamic ones can.
    }

    pub fn is_active(&self) -> bool {
        match self {
            ApprovalTicket::Created(t) => t.expires_at.is_nearby(Utc::now()),
            _ => false, // Static tickets cannot be expired (e.g., for pre-approval actions), but dynamic ones can.
    }

    pub fn is_valid(&self) -> bool {
        match self {
            ApprovalTicket::Created(t) => t.is_expired() || !t.expires_at.is_nearby(Utc::now()),
            _ => false, // Static tickets cannot be expired (e.g., for pre-approval actions), but dynamic ones can.
    }

    pub fn redeem(&self, action_id: &str) -> Result<(), String> {
        match self {
            ApprovalTicket::Created(t) if t.is_expired() => Err(format!("Action '{action_id}' is expired",)),
            _ => Ok(()), // Static tickets are not redeemed. Dynamic ones can be.
        }
    }

    pub fn get_ticket_metadata(&self, metadata: &HashMap<String, String>) -> Option<HashMap<String, String>> {
        match self {
            ApprovalTicket::Created(t) if t.is_expired() => None, // Static tickets cannot have dynamic metadata attached to them in this context.
            _ => Some(metadata.clone()),
        }
    }

    pub fn get_session_id(&self) -> Option<&str> {
        match self {
            ApprovalTicket::Created(t) if t.is_expired() => None, // Static tickets cannot have dynamic metadata attached to them in this context.
            _ => Some(t.session_id),
        }
    }

    pub fn get_action_id(&self) -> Option<&str> {
        match self {
            ApprovalTicket::Created(t) if t.is_expired() => None, // Static tickets cannot have dynamic metadata attached to them in this context.
            _ => Some(t.action_id),
        }
    }

    pub fn get_expires_at(&self) -> Option<DateTime<Utc>> {
        match self {
            ApprovalTicket::Created(t) if t.is_expired() => None, // Static tickets cannot have dynamic metadata attached to them in this context.
            _ => Some(t.expires_at),
        }
    }

    pub fn get_signature(&self) -> Option<Vec<u8>> {
        match self {
            ApprovalTicket::Created(_) => None, // Static tickets are not signed (e.g., for pre-approval actions). Dynamic ones can be.
            _ => Some(Sha256::new_from_slice(self.get_key().as_bytes()).finalize()),
        }
    }

    pub fn get_session_id(&self) -> Option<&str> {
        match self {
            ApprovalTicket::Created(t) if t.is_expired() => None, // Static tickets cannot have dynamic metadata attached to them in this context.
            _ => Some(t.session_id),
        }
    }

    pub fn get_action_id(&self) -> Option<&str> {
        match self {
            ApprovalTicket::Created(t) if t.is_expired() => None, // Static tickets cannot have dynamic metadata attached to them in this context.
            _ => Some(t.action_id),
        }
    }

    pub fn get_expires_at(&self) -> Option<DateTime<Utc>> {
        match self {
            ApprovalTicket::Created(_) => None, // Static tickets are not signed (e.g., for pre-approval actions). Dynamic ones can be.
            _ => Some(Sha256::new_from_slice(self.get_key().as_bytes()).finalize()),
        }
    }

    pub fn get_signature(&self) -> Option<Vec<u8>> {
        match self {
            ApprovalTicket::Created(_) => None, // Static tickets are not signed (e.g., for pre-approval actions). Dynamic ones can be.
            _ => Some(Sha256::new_from_slice(self.get_key().as_bytes()).finalize()),
        }
    }
