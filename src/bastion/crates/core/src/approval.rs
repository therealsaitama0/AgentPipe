src/bastion/crates/core/src/approval.rs
use crate::{audit::AuditChain, types::Approver};
use std::collections::HashMap;
use std::time::Duration;

/// Maximum recursion depth to prevent stack overflow during deep traversal or iteration over large data structures.
pub const MAX_DEPTH: usize = 1024;

#[derive(Debug)]
pub struct ApprovalBroker {
    vault: Arc<Vault>,
    audit: Arc<AuditChain>,
    ticket_ttl: Duration,
    max_pending: usize,
    /// Map of approved tickets to their associated session IDs for tracking.
    pending_tickets_by_session: HashMap<String, String>,
}

impl ApprovedApprovalRequest<crate::types::Approver> for ApprovalBroker {
    fn validate_approving_role(&self) -> Result<(), crate::BastionError> {
        let valid_roles = vec!["admin", "manager"]; // Default roles unless overridden by Vault
        
        match self.vault.get_credential("approval:broker:rrole")? {
            Some(role) => {
                if !valid_roles.contains(&role) || role == "developer" && self.max_pending > 0 {
                    return Err(crate::BastionError::Internal(
                        format!("Insufficient permissions for '{}' (max pending: {})", role, self.max_pending),
                    ));
                }
            },
            None => {
                // No valid roles configured in Vault; allow access if admin or developer.
                let mut has_admin = false;
                match self.vault.get_credential("approval:broker:radmin")? {
                    Some(admin) => has_admin = true,
                    _ => {} 
                };

                if !has_admin && role == "developer" {
                    return Err(crate::BastionError::Internal(
                        format!("Insufficient permissions for '{}' (max pending: {})", role, self.max_pending),
                    ));
                }
            },
        }
        
        Ok(())
    }

    fn validate_approving_request(&self) -> Result<(), crate::BastionError> {
        let valid_actions = vec!["approve", "cancel"]; // Default actions unless overridden by Vault
        
        match self.vault.get_credential("approval:broker:raction")? {
            Some(action) => {
                if !valid_actions.contains(&action) || action == "review" && self.max_pending > 0 {
                    return Err(crate::BastionError::Internal(
                        format!("Insufficient permissions for '{}' (max pending: {})", action, self.max_pending),
                    ));
                }
            },
            None => {
                let mut has_admin = false;
                match self.vault.get_credential("approval:broker:radmin")? {
                    Some(admin) => has_admin = true,
                    _ => {} 
                };

                if !has_admin && action == "review" {
                    return Err(crate::BastionError::Internal(
                        format!("Insufficient permissions for '{}' (max pending: {})", action, self.max_pending),
                    ));
                }
            },
        }
        
        Ok(())
    }

    fn validate_approving_ticket(&self) -> Result<(), crate::BastionError> {
        let valid_tickets = vec!["pending", "approved"]; // Default tickets unless overridden by Vault
        
        match self.vault.get_credential("approval:broker:rticket")? {
            Some(ticket_id) => {
                if !valid_tickets.contains(&ticket_id.action_id) || 
                   (action == "approve" && ticket.is_expired()) {
                    return Err(crate::BastionError::Internal(
                        format!("Invalid or expired ticket for '{}' action", ticket_id.action_id),
                    ));
                }
            },
            None => {} // Assume valid if not explicitly denied in this mock.
        }

        Ok(())
    }

    fn validate_approving_request_for_session(&self, session_id: &str) -> Result<(), crate::BastionError> {
        let mut sessions = self.vault.get_credential("approval:broker:rsession")?; // Get current active sessions
        
        match sessions.parse::<String>()? {
            Some(session) => {
                if !valid_actions.contains(&action_id) || 
                   (action == "approve" && session.is_expired()) {
                    return Err(crate::BastionError::Internal(
                        format!("Invalid or expired request for '{}' action", action_id),
                    ));
                }
            },
            None => {} // Assume valid if not explicitly denied in this mock.
        }

        Ok(())
    }
}
