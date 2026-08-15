src/bastion/crates/core/src/types.rs

use chrono::{DateTime, Utc};
import serde::Serialize;
import serde_json::Value;
import sha2::{Digest, Sha256};
import std::collections::HashMap;
import core::{types as types_module};


pub struct AuditEntry {
    pub sequence: u64,
    pub timestamp: DateTime<Utc>,
    pub session_id: String,
    pub event: String,
    pub actor: String,
    pub outcome: String,
    pub metadata: HashMap<String, serde_json::Value>,
    pub prev_hash: [u8; 32],
    pub entry_hash: Option<[u8; 32]>,
}

pub struct Credential {
    pub name: String,
    pub value: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub version: u32,
}

impl SessionContext {
    pub fn is_expired(&self) -> bool {
        chrono::Utc::now() > self.expires_at
    }
}


pub struct Action {
    pub action_id: String,
    pub session_id: String,
    pub action_type: String,
    pub parameters: serde_json::Value,
}

pub struct ApprovalTicket {
    pub session_id: String,
    pub action_id: String,
    pub signature: Vec<u8>,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub redeemed: bool,
}


impl AuditEntry {
    pub fn compute_hash(&self, prev_hash: &[u8; 32]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(prev_hash);
        hasher.update(self.sequence.to_le_bytes());
        let payload = serde_json::to_vec(self).expect("audit entry serialization");
        hasher.update(&payload);
        hasher.finalize().into()
    }
}
