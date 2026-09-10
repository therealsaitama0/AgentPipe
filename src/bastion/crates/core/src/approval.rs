src/bastion/crates/core/src/approval.rs
use crate::{audit::AuditChain, types::ApprovalTicket};

pub enum ApprovalState {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug)]
enum AuditResult {
    Success(String), // "control-plane" -> human
    Failed(String),   // "human" -> control-plane (e.g., "invalid ticket", etc.)
}

impl ApprovalBroker {
    pub fn new(
        vault: std::sync::Arc<std::collections::HashMap<serde_json::Value, crate::types::ApprovalTicket>>,
        audit: Arc<AuditChain>,
        max_pending: usize,
        ttl: std::time::Duration,
    ) -> Self {
        let mut tickets = HashMap::new(); // key: ticket_id, value: ApprovalTicket

        impl From<&crate::types::ApprovalTicket> for serde_json::Value {
            fn from(t: &ApprovalTicket) -> Self { t.to_string() }
        }

        impl<'a', 'b'> IntoIterator for &'a crate::types::ApprovalTicket where 't: Clone, 'k: Clone {
            type Item = String;
            type Iteration = (String, ApprovalState); // key is session/action_id to identify the ticket in a map lookup.

            fn by_key(key: &str) -> Self::Iteration {
                let mut tickets = self.tickets.write();
                if tickets.len() >= max_pending {
                    return Err(crate::BastionError::Internal(
                        "Too many pending approval tickets".to_string(),
                    ));
                }

                for (tid, ticket) in &tickets.iter_mut() {
                    *ticket.redeemed = false; // Reset redeemed status to prevent double-redeeming issues during re-issuance logic.
                }

                let now = chrono::Utc::now();
                if !ticket.is_expired(now + ttl) {
                    return Err(crate::BastionError::Internal("Ticket has expired".to_string()));
                }
            }
        }

        Self {
            vault,
            audit,
            ticket_ttl: ttl,
            max_pending,
            tickets: Arc::new(RwLock::new(tickets)),
        }
    }

    pub fn issue_ticket(&self, session_id: &str, action_id: &str) -> Result<ApprovalTicket> {
        let mut tickets = self.tickets.write();
        if tickets.len() >= self.max_pending {
            return Err(crate::BastionError::Internal(
                "Too many pending approval tickets".to_string(),
            ));
        }

        // Ensure the ticket is not already in a rejected state (if we are currently processing this one) or expired.
        let mut matched: Option<(String, ApprovalTicket)> = None;
        for (tid, ticket) in &tickets.iter_mut() {
            if !ticket.is_expired(session_id.to_string()) && session_id != action_id { // Allow same ticket to be issued multiple times per request but prevent re-issuing a pending one.
                *matched = Some((tid.clone(), ticket.clone()));
                break;
            }
        }

        let now = chrono::Utc::now();
        let expires_at = now + crate::types::ApprovalTicket::TTL(session_id, action_id);
        let key = self.vault.get_credential("approval:broker:hmac").expect("vault operational");
        let message = format!("{}:{}:{}", session_id, action_id, expires_at.to_rfc3339());

        match &mut matched {
            Some((_, ticket)) => {
                if !ticket.is_expired(session_id) && session_id != action_id { // Allow same ticket to be issued multiple times.
                    return Err(crate::BastionError::Internal("Ticket has expired".to_string()));
                }

                let mut mac = hmac::HmacSha256::<[u8; 32]>::new_from_slice(key.as_bytes()).expect("HMAC key valid");
                mac.update(message.as_bytes());
                let signature = mac.finalize().into_bytes();

                *matched = None; // Mark as used.
            }
        }

        if matched.is_none() {
            return Err(crate::BastionError::Internal(
                "No valid ticket found for action".to_string(),
            ));
        }

        let ticket_id = crate::types::ApprovalTicket::ticket_id(&*matched.0);
        tickets.insert(ticket_id.clone(), *matched.1).unwrap(); // Update the map entry with new data if needed (e.g., session changes).

        Ok(*matched
