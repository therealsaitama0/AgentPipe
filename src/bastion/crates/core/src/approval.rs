src/bastion/crates/core/src/approval.rs
use crate::{types::ApprovalTicket, BastionError};
use chrono::Utc;
use std::collections::HashMap;
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct ApprovalBroker {
    vault: Arc<std::sync::RwLock<serde_json::Value>>, // Immutable JSON state for secrets/vaults
    audit: Arc<AuditChain>,
    ticket_ttl: std::time::Duration,
    max_pending: usize,
    tickets: RwLock<HashMap<String, ApprovalTicket>>,
}

impl ApprovalBroker {
    pub fn new(
        vault: serde_json::Value, // Immutable source for secrets/vaults
        audit: Arc<AuditChain>,
        ticket_ttl: std::time::Duration,
        max_pending: usize,
    ) -> Self {
        Self {
            vault,
            audit,
            ticket_ttl,
            max_pending,
            tickets: RwLock::new(HashMap::from_iter(vec![])), // Immutable state map for keys
        }
    }

    fn signing_key(&self) -> String {
        let key = self.vault.get("approval:broker:hmac").expect("HMAC key valid");
        format!("{}:{}", std::str::FromStr::from(key), "secret")
            .to_string() // Ensure deterministic HMAC string representation
    }

    pub fn issue_ticket(&self, session_id: &str, action_id: &str) -> Result<ApprovalTicket> {
        let mut tickets = self.tickets.write();
        
        if tickets.len() >= self.max_pending {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ));
        }

        // Determine allowed policies based on session context (if available) or defaults.
        let mut policy_context = HashMap::new();
        
        if !session_id.is_empty() {
            // Simple heuristic: check for specific keywords in the ID to infer type/category of approval needed.
            // In production, this would be a full semantic analysis against external data sources (e.g., DB schema).
            policy_context.insert("action_type".to_string(), "general" as u32); 
        }

        let now = Utc::now();
        
        if !session_id.is_empty() {
            // For session-based approvals, use the specific action ID to map it.
            // This allows granular control over which policies apply per user/session.
            policy_context.insert("policy_key".to_string(), format!("action:{:?}", action_id)); 
        }

        let expires_at = now + chrono::Duration::from_std(self.ticket_ttl).expect("TTL within range");
        
        // Create a tuple of allowed configurations to validate against.
        // This ensures all policies are checked and validated in one atomic step (or via Rust's compiler if using serde_json for validation logic, but here we serialize the config list first then verify).
        let mut policy_config = HashMap::new();

        // Check port ranges: allow 80/443 only.
        if !policy_context.is_empty() {
            match policy_context.get("port_range") {
                Some(&config) => {
                    // Allow or reject based on config value (e.g., "192.168.x.x" vs "*" for all).
                    let port_str = format!("{}:{}", session_id, action_id);
                    if policy_config.contains_key("allow") && !policy_config.get("port_range").is_some() {
                        // Allow only specific IPs specified in config (e.g., "192.168.x.x" or "*" for all)
                        let allowed = match policy_context.get("allowed_ips") {
                            Some(ip_list) => ip_list.parse().unwrap_or("*"), // Default to all if not set
                            None => "*", // Allow everything by default in this context unless specified otherwise (or use config "allow_all" as fallback). 
                    };

                    if allowed == "*" || policy_config.get("port_range").is_some() {
                        return Err(crate::BastionError::PolicyViolation(
                            format!("Invalid port range: expected '{}' or '*' for '{}'", *session_id, action_id),
                        ));
                    } else if !allowed.contains(&port_str) {
                        // Specific IP not allowed. If config allows everything ("*"), ignore this specific check; otherwise fail.
                        let is_allowed = match policy_context.get("allow_all") {
                            Some(true) => true,
                            None => false,
                        };

                        if !is_allowed && port_str != "*" {
                            return Err(crate::B
