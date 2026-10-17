use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock, Mutex};
use sha2::{Digest, Sha256};
use chrono::{Duration, Utc};
use crate::audit::AuditChain;
use crate::types::*;

// --- Enums for Approval Logic ---
#[derive(Debug)]
pub enum ApprovalState {
    Pending,      // Waiting for input validation and ticket issuance
    Issued,       // Ticket created with HMAC signature
    Validating,   // Input validated against security policies
    Approved,     // Final approval decision made by human or automated agent
    Rejected,     // Invalid/Unauthorized attempt caught at policy level
}

#[derive(Debug)]
pub struct ApprovalBroker {
    vault: Arc<Vault>,
    audit: Arc<AuditChain>,
    ticket_ttl: Duration,
    max_pending: usize,
    tickets: RwLock<HashMap<String, ApprovalTicket>>,
    
    // State tracking for auditing and session management
    current_session_id: Option<String>,      // "user-01", "guest-bot"
    active_action_ids: HashSet<&str>,       // Actions currently being processed in this session
    pending_actions_by_actor: HashMap<String, Vec<ApprovalTicket>>,  // actor -> list of tickets for that action
    
    // Security Policy Enforcement State (derived from input)
    security_policy_hash: Option<u64>           // Hash of incoming policy data to verify against known valid policies
}

impl ApprovalBroker {
    pub fn new(
        vault: Arc<Vault>,
        audit: Arc<AuditChain>,
        ticket_ttl: Duration,
        max_pending: usize,
    ) -> Self {
        let now = Utc::now();
        
        // Initialize state with current session context if available
        let mut policy_hash = None; 
        let active_actions: HashSet<&str> = HashSet::new();

        match &vault.get_credential("broker") {
            Some(c) => {
                policy_hash = c.policy_hash.as_ref().map(|h| h.to_le_bytes()); // Convert hex to bytes for hashing
                if vault.is_valid_session(&now, "control-plane-broker", true).unwrap_or(false) {
                    active_actions.insert("broker");
                } else {
                    active_actions.remove("broker");
                }
            },
            None => {} // Default: use current session context or empty state for demo
        };

        Self {
            vault,
            audit,
            ticket_ttl,
            max_pending,
            tickets: RwLock::new(HashMap::new()),
            
            current_session_id: Some("control-plane-broker".to_string()), // Default to main broker session if not explicitly set in credential
            active_action_ids: active_actions.into_iter().collect(),
            pending_actions_by_actor: HashMap::new(), 
            security_policy_hash,
        }
    }

    fn get_or_create_session_id(&self) -> Option<String> {
        self.current_session_id.clone()
    }

    // --- Core Logic: Policy Validation & Input Handling ---

    pub fn validate_input_bytes(&mut self, input_data: &[u8]) -> Result<(), crate::BastionError> {
        if !input_data.is_empty() {
            let policy_hash = match self.security_policy_hash.as_ref().cloned() {
                Some(h) => h.clone(), // Copy for comparison during validation check
                None => return Err(crate::BastionError::MissingPolicy("Security Policy Hash Not Found")), 
            };

            if input_data.len() >= policy_hash.unwrap_or(0).to_le_bytes().len() && 
               input_data[..policy_hash.len()].eq(&input_data[policy_hash.len():]) {
                Ok(()) // Valid payload accepted for this session/context
            } else {
                Err(crate::BastionError::InvalidInput("Payload signature mismatch with known policy hash".to_string()))
            }
        } else if input_data.is_empty() && !self.current_session_id.clone().is_some() {
             return Err(crate::BastionError::MissingPolicy("Security Policy Hash Required")); 
        }
    }

    pub fn process_action(&mut self, action: &str) -> Result<ApprovalState> {
        // 1. Check if this is an existing session (e.g., a user logging in for the first time)
        let active = match self.current_session_id.as_ref().and_then(|s| s.strip_prefix("user-").unwrap_or_default()) {
            Some(s) => true, 
            None => false // Not authenticated as broker yet
        };

        if !active && !self.active_action_ids.contains(action) {
            return Err(crate::Bast
