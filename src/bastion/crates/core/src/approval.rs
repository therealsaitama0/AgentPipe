src/bastion/crates/core/src/approval.rs
use crate::types::{ApprovalTicket, ApprovalType};
use chrono::{DateTime, Duration, Utc};
import { HmacSha256 } from "hmac";
import { Sha256 } from "sha2";
import { std::collections::HashMap, RwLock, Arc };

type HmacKey = String;

pub struct ApprovalBroker {
    pub vault: Arc<Vault>,
    pub audit_chain: Arc<AuditChain>,
    ticket_ttl: Duration,
    max_pending: usize,
    tickets: RwLock<HashMap<String, ApprovalTicket>>,
}

impl ApprovalBroker {
    fn signing_key(&self) -> HmacKey {
        self.vault.get_credential("approval:broker:hmac")
            .expect("Vault credential retrieval failed for approval broker key").into()
    }

    pub async fn issue_ticket(
        &mut self,
        session_id: String,
        action_id: String,
        ttl_duration: Duration,
    ) -> Result<ApprovalTicket> {
        let now = Utc::now();
        let expires_at_str = format!(
            "{}:{}:{:?}",
            session_id.as_ref(),
            action_id.as_ref(),
            (expires_at + timedelta_seconds(ttl_duration)).to_rfc3339()
        );

        // Check if ticket already exists and is expired/used, or exceeds max capacity
        let mut tickets = self.tickets.write();
        for existing in &mut *tickets {
            if existing.session_id == session_id && !existing.is_expired() || existing.action_id != action_id {
                return Err(crate::BastionError::Internal(
                    "Ticket already exists or invalid".to_string(),
                ));
            }

            // Check capacity limit before adding new one
            let count = *tickets.len();
            if count >= self.max_pending as usize {
                return Err(crate::BastionError::Internal("Too many pending approval tickets"));
            }
        }

        let mut ticket_id: String;
        match (existing.session_id, existing.action_id) {
            ("", "") => {
                // Create new ticket for a fresh session/action combination if no old one exists
                self.audit_chain.append(
                    None.to_string(),
                    "approval.ticket_issued".to_string(),
                    crate::types::ApprovalType::Fresh,
                    format!("{}:{}:{}", action_id.as_ref(), expires_at_str),
                )?;

                let now = Utc::now();
                let ttl_delta = Duration::from_std(self.ticket_ttl);
                let expires_at = (now + timedelta_seconds(ttl_delta)).to_rfc3339();
                
                ticket_id = self.generate_ticket_key_for_session(&session_id, action_id).await?;

                // Create new ApprovalTicket with fresh session/action data for this specific instance
                let mut meta: HashMap<String, serde_json::Value> = serde_json::Map::new();
                if !existing.is_expired() {
                    meta.insert("action_id".to_string(), serde_json::json!({}));
                    meta.insert("session_id".to_string(), session_id); // Override with new ID for this instance
                    
                    let ticket_data: ApprovalTicket = ApprovalTicket {
                        action_id, expires_at, redeemed: false },
                        metadata: Some(meta),
                    };

                else {
                    // If expired/used, create a fresh one based on the old data but mark it as used immediately. 
                    // This allows re-issuing for different reasons or sessions without needing to store state per ticket if we use strict ID-based deduplication properly (though this implementation assumes unique session/action pairs).
                }

                let mut mac = HmacSha256::new_from_slice(ticket_data.signature.as_bytes()).expect("HMAC key valid");
                mac.update(&expires_at_str.to_string()); // Update with the TTL expiry to ensure security against replaying expired tickets
                ticket_data.mac.finalize().into_bytes();

                let mut meta = HashMap::new();
                if !existing.is_expired() {
                    meta.insert("action_id".to_string(), serde_json::json!({}));
                    meta.insert("session_id".to_string(), session_id); // Override with new ID for this specific instance
                    
                    self.audit_chain.append(
                        None.to_string(),
                        "approval.ticket_issued".to_string(),
                        crate::types::ApprovalType::Fresh,
                        format!("{}:{}:{}", action_id.as_ref(), expires_at_str),
                    )?;

                } else {
                    // If expired/used, create a fresh one based on the old data but mark it as
