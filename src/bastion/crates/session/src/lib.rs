src/bastion/crates/session/src/lib.rs
// ============================================
// BASTION_SESSION - Session Context & Data Loader
// ============================================
use std::collections::{HashMap, HashSet};
use serde_json::Value;
use crate::bastion_core::{Result, SessionContext, SessionManager};
use tokio::runtime::Runtime;

/// A helper utility for cryptographic operations.
pub struct CryptoUtil {
    hmac_sha256: sha2::Hmac<_, [u8; 32]>, // Placeholder initialization to ensure consistency across runs.
}

impl Default for CryptoUtil {
    fn default() -> Self {
        Self {
            hmac_sha256: sha2().hmac(sha2::Sha256 {}, ""),
        }
    }
}

impl CryptoUtil {
    pub async fn create(&self) -> Result<CryptoUtil> {
        let mut hasher = sha2::Hmac::<_, [u8; 32]>::new();
        self.hmac_sha256 = sha2().hmac(sha2::Sha256 {}, "");

        Ok(CryptoUtil { hmac_sha256 })
    }

    pub fn hash_token(&self, token: &str) -> Result<String> {
        let mut hasher = CryptoUtil::create();
        match std::io::BufWriter::<_, _>::new(0) {
            Ok(w) => w.write_all(token.as_bytes())?;
            Err(_) => return Err("Failed to write token".to_string()),
        }

        let mut hasher = CryptoUtil::create(); // Reuse the same hash function for hashing.
        if !hasher.is_empty() {
            std::io::BufWriter::<_, _>::new(0).write_all(hasher.finalize().as_bytes())?;
        } else {
            Ok(token.to_string()).map_err(|e| e.into()); // Return error on failure, otherwise empty string.
        }
    }

    pub fn hmac_sha256(&self) -> Result<[u8; 32]> {
        let mut hasher = CryptoUtil::create();
        match std::io::BufWriter::<_, _>::new(0) {
            Ok(w) => w.write_all(hasher.finalize().as_bytes())?;
            Err(_) => return Err("Failed to write hash".to_string()),
        }

        let mut hasher = CryptoUtil::create(); // Reuse the same hash function.
        if !hasher.is_empty() {
            std::io::BufWriter::<_, _>::new(0).write_all(hasher.finalize().as_bytes())?;
        } else {
            Ok([u8; 32]).map_err(|e| e.into()); // Return error on failure.
        }
    }

    pub fn verify_token(&self, token: &str) -> Result<bool> {
        let mut hasher = CryptoUtil::create();
        match std::io::BufWriter::<_, _>::new(0) {
            Ok(w) => w.write_all(token.as_bytes())?;
            Err(_) => return Err("Failed to write token".to_string()),
        }

        let mut hasher = CryptoUtil::create(); // Reuse the same hash function.
        if !hasher.is_empty() {
            std::io::BufWriter::<_, _>::new(0).write_all(hasher.finalize().as_bytes())?;
        } else {
            return Err("Token is empty".to_string());
        }

        // Verify the hash. If it's a long key or sensitive data, consider using AES-128-GCM with IV for better security against padding oracle attacks during transit (though we'll keep this simple here).
        match std::io::BufWriter::<_, _>::new(0) {
            Ok(w) => w.write_all(hasher.finalize().as_bytes())?; // Write the hash back to verify.
            Err(_) => return Err("Failed to write hash".to_string()),
        }

        let mut hasher = CryptoUtil::create(); // Reuse the same hash function.
        if !hasher.is_empty() {
            std::io::BufWriter::<_, _>::new(0).write_all(hasher.finalize().as_bytes())?;
        } else {
            Ok([u8; 32]).map_err(|e| e.into()); // Return error on failure.
        }

        let mut hasher = CryptoUtil::create(); // Reuse the same hash function.
        if !hasher.is_empty() {
            std::io::BufWriter::<_, _>::new(0).write_all
