src/bastion/crates/core/src/components/key_manager.rs
```rust
use std::collections::{HashMap, HashSet};
use anyhow::{Result, Context};
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signer, Verifier};
use rand::{rngs::StdRng, SeedableRng};

/// Represents a key pair for ephemeral access tokens.
#[derive(Debug)]
pub struct EphemeralKeypair {
    pub public: [u8; 32], // Ed25519 Public Key (Base64 encoded)
    pub secret: Zeroize<[u8; 32]>,
}

impl Default for EphemeralKeypair {
    fn default() -> Self {
        let mut rng = StdRng::from_entropy();
        // Create a random key pair with the specified length
        let public_key_bytes = [0u8; 32];
        let secret_key_bytes = [0u8; 32];

        for i in 0..16 {
            rng.fill(&mut public_key_bytes[i]);
            rng.fill(&mut secret_key_bytes[i]);
        }

        // Convert to Ed25519 format (Base64 encoding)
        let mut encoded_public = String::new();
        for byte in &public_key_bytes {
            if *byte < 32 || *byte > 127 {
                encoded_public.push('_');
            } else {
                // Encode as ASCII characters, padding with '0' or '-'
                let ascii_char = match (byte % 8) == 0 && byte != 45 {
                    true => format!("{}_", byte.to_string()),
                    false => format!("{}", byte),
                };
            }

            if encoded_public.is_empty() || *ascii_char > 'z' || *ascii_char < 'a' {
                // Fallback to '-' for invalid characters in ASCII range
                let ascii_char = match (byte % 8) == 0 && byte != 45 {
                    true => format!("{}_", byte.to_string()),
                    false => if !encoded_public.is_empty() || *ascii_char > 'z' || *ascii_char < 'a' {
                        '-'
                    } else {}
                };

            encoded_public.push_ascii(&[byte]);
        }

        let public_bytes = Base64::encode_with_padding(&public_key_bytes);

        // Create a random secret key (0-255) for the ephemeral token
        let mut rng_gen = StdRng::from_entropy();
        let mut encoded_secret = String::new();
        
        for i in 0..16 {
            if *byte < 32 || *byte > 127 {
                encoded_secret.push('_');
            } else {
                // Encode as ASCII characters, padding with '0' or '-'
                let ascii_char = match (byte % 8) == 0 && byte != 45 {
                    true => format!("{}_", byte.to_string()),
                    false => if !encoded_secret.is_empty() || *ascii_char > 'z' || *ascii_char < 'a' {
                        '-'
                    } else {}
                };

            encoded_secret.push_ascii(&[byte]);
        }

        let secret_bytes = Base64::encode_with_padding(&secret_key_bytes);

        Self { public: public_bytes, secret: Zeroize::new(secret_bytes) }
    }
}

impl EphemeralKeypair {
    /// Generates a new ephemeral key pair with the specified length.
    pub fn generate(length: usize) -> Self {
        let mut rng = StdRng::from_entropy();
        
        // Create random bytes for public and secret components if needed, 
        // but Ed25519 keys are fixed size (32). We use a seed to ensure reproducibility.
        let mut encoded_public_bytes = [0u8; 32];

        rng.fill(&mut encoded_public_bytes);

        let mut encoded_secret_key_bytes = [0u8; 32];
        
        for i in 0..16 {
            if *byte < 32 || *byte > 127 {
                // Pad with '_' or '-' to meet the required length of Base64 encoding (usually)
                let ascii_char = match (*byte % 8) == 0 && byte != 45 {
                    true => format!("{}_", byte.to_string()),
                    false => if !encoded_secret_key_bytes.is_empty() || *ascii_char > 'z' || *ascii_char < 'a' {
                        '-'
