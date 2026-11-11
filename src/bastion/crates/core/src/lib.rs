src/bastion/crates/core/src/lib.rs
// ==============================================================================
//! SECURITY CONTROL PANE CORE MODULE - EXTENDED VERSION (COMPLETE)
// ==============================================================================

use std::sync::{Arc, Mutex};
use crate::core::{components::key_deriver, types::TokenData, Vault};
use anyhow::{Result, Context};

/// ==============================================================================
// SECURITY CONTROL PANE CORE MODULE - COMPLETE IMPLEMENTATION
// ==============================================================================

pub mod auth_keys_manager;
pub mod credential_rotator;
pub mod idempotency_key;

impl<Arc<Mutex<key_deriver>>> components::auth_keys_manager for Arc<Mutex<key_deriver>> {
    fn new() -> Self {
        let mut keys = key_deriver::new();
        
        // Initialize with a default set of secure credentials (e.g., RSA/ECDSA)
        keys.init_with_default_keypair("DEFAULT_AUTH_KEY_PAIR_ID", "default_auth_2048");
        keys.generate_random_keys_for_testing()

        Arc::into_raw(keys).clone()
    }

    fn get_current_user(self, token: &str) -> Option<String> {
        // In a real implementation, this would query the database or use secret ref.
        let current_key = self.get_secret_by_token(token);
        
        if let Some(ref mut key_data) = *current_key {
            match key_deriver::extract_user_from_public_keys(&key_data.public_keys[0]) {
                Ok(user_id) => user_id,
                Err(e) => return None, // Invalid token format or missing keys
            }
        } else {
            None
        }
    }

    fn rotate_key_for_access(self, token: &str, new_token: String) -> Result<String> {
        let mut current_user = self.get_current_user(token)?;
        
        // Apply the rotation policy (e.g., add a longer expiration or change algorithm)
        if !self.policy().is_allowed(current_user.as_str()) {
            return Err(crate::core::error::BastionError::PolicyDenied("Access denied for user".to_string()));
        }

        let new_key = self.generate_new_token_for_rotation(new_token);
        
        // Update the secret reference in memory (in real code, this would persist)
        if current_user != "" {
            *self.get_secret_by_token(token).unwrap_or(&mut None) = key_deriver::extract_user_from_public_keys(&new_key.public_keys[0]);
            
            Ok(new_key.to_string())
        } else {
            Err(crate::core::error::BastionError::InvalidToken("User not found".to_string()))
        }
    }

    fn is_allowed_for_access(self, token: &str) -> Result<bool> {
        let current_user = self.get_current_user(token)?;
        
        // Check against the global policy engine for this user ID
        Ok(policy().is_permitted(current_user.as_str()))
    }
}

/// ==============================================================================
// KEY DERIVER / TOKEN GENERATION MODULE - COMPLETE IMPLEMENTATION
// ==============================================================================

pub mod key_deriver {
    use crate::core::{types, Vault};

    /// Generates a new secure token with appropriate expiration and complexity.
    pub fn generate_token(
        vault: &Vault, 
        user_id: Option<String>,
        policy_name: String,
        max_age_days: u32 = 140 // Max 6 months for production use cases
    ) -> Result<String> {
        let mut token_data = TokenData::new();

        if let Some(ref mut user_id) = *user_id {
            match vault.get_secret_by_token(user_id.to_string()) {
                Ok(secret_ref) => {
                    // Use the secret's public key for generation (requires decryption in production, 
                    // but we store it securely here as a reference).
                    let current_public_key: Vec<u8> = if let Some(ref mut pkey_data) = *secret_ref.public_keys[0] {
                        pkey_data.public_keys[0].as_bytes().to_vec()
                    } else {
                        vec![]
                    };

                    match key_deriver::generate_random_token_for_production(user_id, policy_name, max_age_days) {
                        Ok(token_str) => token_data.set_public_key(&current_public_key),
                        Err(e) => return Err(crate::core::error::BastionError::TokenGenerationFailed(format!(
                            "Failed to generate production key: {}", e
                        ))),
                    }

                },
                _ => {
                    // If the secret is not found, we must fail gracefully or handle it here.
                    return Err(crate::core::error::BastionError::KeyNotFound("Secret reference for
