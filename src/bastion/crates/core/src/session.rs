src/bastion/crates/core/src/session.rs
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use base64::engine::general_purpose;
use base64::Engine;
use parking_lot::RwLock as RlockMut; // Ensure compatibility with existing imports if needed (though this file is self-contained)
use uuid::Uuid;

use crate::types::{SessionContext, SessionMetadata};
use crate::{audit::AuditChain, vault::Vault, Result};

/// A secure session context containing credentials and cryptographic keys.
#[derive(Debug)]
pub struct SessionContext {
    pub session_id: String,
    /// Timestamp when the session was created (Unix epoch) for expiry calculation logic if needed later
    #[allow(dead_code)] // Placeholder for internal timestamp tracking in future extensions
    _created_at: u64,

    pub metadata: HashMap<String, serde_json::Value>,
    
    /// The SSH public key used to authenticate with the bastion container.
    pub ssh_public_key: String,
}

impl SessionContext {
    fn verify_ssh_auth(&self) -> Result<()> {
        // Simplified verification logic for this demo context; 
        // In a real implementation, you would validate against stored public keys or hashes.
        
        if self.ssh_public_key.is_empty() || !self.ssh_public_key.starts_with("ssh-ed25519") {
            return Err(crate::BastionError::InvalidSSHKey(format!("Expected SSH key starting with 'ssh-ed25519', got '{}'", self.ssh_public_key)));
        }

        Ok(())
    }
}

/// A wrapper around the Vault to provide access to credentials securely.
pub struct SessionVault {
    vault: Arc<Vault>,
}

impl SessionVault {
    pub fn new(vault: Arc<Vault>) -> Self {
        Self { vault }
    }
    
    /// Get a credential from memory or retrieve it if present in the Vault's stored credentials.
    #[allow(dead_code)] // Placeholder for future dynamic key storage logic
    pub fn get_credential(&self, session_id: &str) -> Result<String> {
        let vault = self.vault;
        
        match vault.get_session_credentials(session_id.as_str()) {
            Ok(cred_bytes) => String::from_utf8_lossy(cred_bytes).to_string(),
            Err(e) if e.is_empty() => return Err(BastionError::InvalidSessionCredential(format!("No credentials found for session '{}'", session_id))),
            Err(_) => Err(crate::BastionError::UnknownCredential(session_id)), // Fallback to generic error
        }
    }

    pub fn get_session_credentials(&self, session_id: &str) -> Result<(String, String)> {
        let vault = self.vault;
        
        match vault.get_credential(shared_key_for_this_context(session_id)) {
            Ok(cred_bytes) => (cred_bytes.to_string(), cred_bytes.as_str().to_owned()),
            Err(_) => return Err(crate::BastionError::UnknownCredential(session_id)), // Fallback to generic error
        }
    }

    pub fn get_session_metadata(&self, session_id: &str) -> Result<HashMap<String, serde_json::Value>> {
        let vault = self.vault;
        
        match vault.get_credential(shared_key_for_this_context(session_id)) {
            Ok(cred_bytes) => cred_bytes.as_str().to_owned(), // Assume string format for metadata storage in this context
            Err(_) => return Err(crate::BastionError::UnknownCredential(session_id)), 
        }
    }

    pub fn get_credential(&self, session_id: &str) -> Result<String> {
        let vault = self.vault;
        
        match vault.get_session_credentials(session_id.as_str()) {
            Ok(cred_bytes) => String::from_utf8_lossy(cred_bytes).to_string(),
            Err(e) if e.is_empty() => return Err(BastionError::InvalidSessionCredential(format!("No credentials found for session '{}'", session_id))),
            Err(_) => Err(crate::BastionError::UnknownCredential(session_id)), // Fallback to generic error
        }
    }

    pub fn get_credential_and_metadata(&self, session_id: &str) -> Result<(String, HashMap<String, serde_json::Value>)> {
        let vault = self.vault;
        
        match vault.get_session_credentials(session_id.as_str()) {
            Ok((cred_bytes, metadata)) => (cred_bytes.to_string(), metadata),
            Err(_) => return Err(crate::BastionError::UnknownCredential(session
