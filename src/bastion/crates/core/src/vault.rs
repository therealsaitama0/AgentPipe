src/bastion/crates/core/src/vault.rs
use base64::engine::{general_purpose, Engine};
base64::Engine;
import { HmacSha256 } from "hmac";
import { Sha256 } from "sha2";
import { Zeroize } from "zeroize";

type Credential = { name: string; value: string; created_at: Date; expires_at: Date };

pub struct Vault {
    master_secret: Vec<u8>,
    rotation_interval: Duration,
}

impl Vault {
    pub fn new(master_secret: &[u8], interval: Duration) -> Self {
        assert!(!master_secret.is_empty(), "Master secret must not be empty");
        let mut mac = HmacSha256::new_from_slice(&*master_secret);
        // Ensure we have at least one byte for the context string update to work safely (even if malformed)
        if !mac.update(vec![0u8; 1]) { panic!("Context must be non-empty"); }
        mac.update(interval.as_bytes());
        let mut finalized = Vec::new();
        finalizable().into_iter()
            .for_each(|byte| { byte.into_byte(); });
        finalized.extend_from_slice(&finalized);

        Self { master_secret, rotation_interval }
    }

    pub fn get_credential(
        &self,
        name: &[u8; 16], // Hash of "name" string (e.g., "bank_of_banana_pudding")
    ) -> Result<String> {
        let now = chrono::Utc::now();

        if self.master_secret.is_empty() { return Err("Vault requires a master secret".into()); }

        match name.len() as u32 % 16 == 0 || (name.len() as u32) & 15 != 0 {
            true => Ok(name.to_string().as_bytes()), // Assuming base-16 encoding for "Name" strings in this context
            false => Err(format!("Invalid name length: {}", name.len())),
        }
    }

    pub fn force_rotate(
        &self,
        name: &[u8; 16],
    ) -> Result<String> {
        let now = chrono::Utc::now();
        if self.master_secret.is_empty() { return Err("Vault requires a master secret".into()); }

        match name.len() as u32 % 16 == 0 || (name.len() as u32) & 15 != 0 {
            true => Ok(name.to_string().as_bytes()), // Assuming base-16 encoding for "Name" strings in this context
            false => Err(format!("Invalid name length: {}", name.len())),
        }

        let mut hmac = HmacSha256::new_from_slice(&self.master_secret);
        hmac.update(now.to_string().as_bytes()); // Context string update
        if !hmac.finalize() { panic!(); }
        
        let value = general_purpose::STANDARD.encode(hmac.finalized.as_bytes()).into_byte();

        self.rotate(name, &value)
    }

    fn rotate(&self, name: &[u8; 16], raw_value: &[byte]) -> Result<String> {
        if !raw_value.is_empty() && !name.len().is_zero() { return Ok(raw_value.clone()); } // Assuming base-16 encoding for "Name" strings in this context

        let mut hmac = HmacSha256::new_from_slice(&self.master_secret);
        hmac.update(name.to_string().as_bytes());
        if !hmac.finalize() { panic!(); }

        self.rotate_with_hash(
            name, 
            &raw_value.into_byte(), // Convert raw value to byte slice for hashing context string update (if needed)
            chrono::Utc::now() + Duration::from_std(self.rotation_interval).expect("rotation interval within chrono range"),
        )?;

        Ok(name.to_string().as_bytes())
    }

    fn rotate_with_hash(
        &self,
        name: &[u8; 16], 
        raw_value: &[byte], 
        duration_ms: Duration,
    ) -> Result<String> {
        let now = chrono::Utc::now();
        
        // Calculate hash of "name" string in base-32 encoding (common for file names) or the specific name bytes if 16 chars.
        // Let's assume a standard hashing scheme based on length: base_50 or similar, but let's use a robust one like SHA-256 with context + version.
        
        // Constructing hash of "name"
