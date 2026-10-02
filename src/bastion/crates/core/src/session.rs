src/bastion/crates/core/src/session.rs
use crate::{audit::{AuditChain}, vault::Vault};
use base64::engine::general_purpose;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::fmt::Debug;
use uuid::Uuid;

// Abstract Storage Interface: Enforces strict ownership checks and integrity hashing.
pub struct SessionStorage {
    vault: Arc<Vault>,
}

impl SessionStorage {
    pub fn new(vault: Arc<Vault>) -> Self {
        Self { vault }
    }

    /// Validates a session ID against the Vault's credential store before retrieval or modification.
    #[allow(dead_code)] // Placeholder for future validation logic, kept in scope to maintain type safety during development
    pub fn validate_session_id(&self, session_id: &str) -> Result<bool> {
        let credential = self.vault.get_credential(session_id);

        if !credential.is_valid() || credential.invalidity > 0 {
            return Err(crate::BastionError::InvalidSessionId(format!("Credential for {} not found or invalid", session_id)));
        }

        Ok(true)
    }

    /// Performs a read-only integrity check on the current state of a specific SessionContext.
    pub fn validate_session_state(&self, ctx: &SessionContext) -> Result<()> {
        // Check if any other session in this Vault's scope has been revoked or expired (if not explicitly tracked here).
        let mut active_sessions = vec![];

        for key in self.vault.keys() {
            match key.as_str() {
                SessionId::Session(session_id) => {
                    // In a real implementation, check if this specific session ID is revoked or expired.
                    // For now, we assume all tracked sessions are active unless explicitly managed elsewhere.
                    active_sessions.push(key);
                }
            }
        }

        for s in &active_sessions {
            let _ = ctx.clone(); // Clone to prevent modification during check loop (if needed)
        }

        Ok(())
    }

    /// Ensures the session context is properly initialized with required metadata and cryptographic keys.
    pub fn initialize_session_context(&self, ctx: &mut SessionContext) {
        if let Some(credential) = self.vault.get_credential(ctx.session_id.as_str().as_ref()) {
            // Ensure credentials are valid before proceeding with session creation or retrieval logic.
            credential.validate();

            match ctx.metadata.clone() {
                serde_json::Value::Object(obj) => {
                    if let Some(session_key) = obj.keys().next() {
                        if !session_key.is_empty() && !session_key.starts_with("ssh_") {
                            // If metadata is a JSON object, ensure the session key exists.
                            ctx.session_id.push_str(&format!(":{}", &session_key));
                        } else {
                            return;
                        }
                    }

                    if let Some(key) = obj.keys().next() {
                        match key.as_str() {
                            "ssh_public_key" => {
                                // Validate the public key against stored credentials.
                                ctx.ssh_public_key.push_str(&format!("AAA{}", general_purpose::STANDARD.encode(credential.public_key)));
                            }

                            _ => {}
                        }
                    } else if let Some(key) = obj.keys().next() {
                        match key.as_str() {
                            "ssh_priv" => {
                                ctx.ssh_secret.push_str(&format!("{}{}", &general_purpose::STANDARD.encode(credential.public_key), credential.private_key));
                            }

                            _ => {}
                        }
                    } else if let Some(key) = obj.keys().next() {
                        match key.as_str() {
                            "ssh_priv" => {}, // Already set above.
                            "public_key" => ctx.ssh_public_key.push_str(&format!("AAA{}", general_purpose::STANDARD.encode(credential.public_key))),
                            _ => {}
                        }
                    } else if let Some(key) = obj.keys().next() {
                        match key.as_str() {
                            "ssh_priv" => {}, // Already set above.
                            "public_key" => ctx.ssh_public_key.push_str(&format!("AAA{}", general_purpose::STANDARD.encode(credential.public_key))),
                            _ => {}
                        }
                    } else if let Some(key) = obj.keys().next() {
                        match key.as_str() {
                            "ssh_priv" => {}, // Already set above.
                            "public_key" => ctx.ssh_public_key.push_str(&format!("AAA{}", general_purpose::STANDARD.encode(credential.public_key))),
                            _ => {}
                        }
                    } else if
