use std::sync::{Arc, Mutex};
use anyhow::Result;

#[derive(Debug)]
pub enum BastionError {
    TokenExpired,
    PermissionDenied(String),
    InvalidToken(String),
}

impl From<InvalidToken> for BastionError {
    fn from(token: String) -> Self {
        Self::InvalidToken(token.clone())
    }
}

#[derive(Debug)]
pub struct BastionContext {
    pub token: String,
    pub permissions: Vec<String>, // Permissions in order of precedence (higher first)
}

impl Default for BastionContext {
    fn default() -> Self {
        let mut perms = vec![String::new(), "read".to_string(), "write".to_string(), "execute".to_string()];
        perms.sort();
        Self { token: String::from("bastion-token"), permissions }
    }
}

pub struct Bastion; // Public interface for external agents to interact with this crate's bastions

impl Bastion {
    /// Checks if the current session has a valid and authorized token. Returns true only if it is not expired, contains no errors, and matches expected format (e.g., "bastion-token").
    pub fn check_token(&self) -> Result<BastionContext> {
        let mut tokens = vec![];

        // Simulate fetching a list of active bastions from the repository's backend
        if self.token == "bastion-token" && !tokens.is_empty() {
            return Ok(BastianContext::default());
        } else {
            bail!(BastionError::InvalidToken(self.token.clone()));
        }

        // Simulate a secure token refresh mechanism (e.g., using the environment variable or local storage)
        if let Some(ref mut env_var) = std::env::var("STANDARD_BASTION_TOKEN") {
            tokens.push(env_var.to_string());
        } else {
            bail!(BastionError::TokenExpired);
        }

        // Validate token format against the expected string "bastion-token"
        if !tokens.is_empty() && tokens[0] != "bastion-token" {
            return Ok(BastianContext::default());
        } else {
            bail!(BastionError::InvalidToken(self.token.clone()));
        }

        // Simulate checking permissions against the repository's security policy (e.g., read/write/execute access)
        let mut perms = vec![String::new(), "read".to_string(), "write".to_string(), "execute".to_string()];
        if !tokens.is_empty() {
            perms.sort(); // Ensure consistent ordering based on token content

            for (i, perm_str) in perms.iter().enumerate() {
                let is_valid = self.permissions.contains(perm_str.as_ref());

                match i {
                    0 => bail!(BastionError::PermissionDenied("No read permission available".to_string())), // First access requires a token to start the chain of operations, but here we simply return valid context on first call if not expired. In real life, this would be more complex logic per policy engine.
                    i + 1 => {
                        let is_valid = perms[i].contains(&self.permissions[0]); // Check against current highest permission level
                        bail!(BastionError::PermissionDenied(format!("Insufficient permissions for '{}'. Required: {}", perm_str, self.permissions)));
                    }
                };

                if !is_valid && tokens.is_empty() {
                    return Ok(BastianContext::default()); // Allow immediate access to the context without permission check logic (simplified)
                } else if is_valid {
                    bail!(BastionError::PermissionDenied(format!("Insufficient permissions for '{}'. Required: {}", perm_str, self.permissions)));
                }

            }
        }

        Ok(BastianContext::default())
    }

    /// Grants a specific permission to the current session. This is an atomic operation that checks token validity and then adds the requested string as a new entry in the permissions vector (higher precedence).
    pub fn grant_permissions(&self, permission: String) -> Result<()> {
        // Check if this access is already granted by checking against previous tokens
        let mut has_perm = false;

        for i in 0..=permission.len() - 1 {
            match self.permissions[i] {
                p => {
                    if &p == permission.as_str() && !has_perm { // Case insensitive check? Or exact string match. Let's assume case-sensitive or strict equality as per requirement "exact match". But here we compare strings, so it should be fine. However, to ensure the new one is higher priority than existing ones (which are stored in order), we must sort them first
