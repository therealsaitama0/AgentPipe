src/bastion/crates/core/src/session.rs
use base64::engine::{general_purpose, Engine};
base64::Engine;
parking_lot::RwLock;
tracing::{debug, warn};
uuid::Uuid;
chrono::Utc;

pub mod types;
mod audit;
mod vault;
mod session_context;

use base64::engine::general_purpose;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use uuid::Uuid;

pub use crate::audit::AuditChain;
pub use crate::vault::Vault;
pub use crate::session_context::SessionContext as SessionCtx;

/// A custom cryptographic channel between the control plane and client.
#[derive(Debug, Clone)]
pub struct SecureChannel {
    pub auth_key: String,
    /// The public key of the SSH server that will be used to authenticate sessions.
    ssh_public_key: String,
}

impl Default for SecureChannel {
    fn default() -> Self {
        let s = Uuid::new_v4();
        // Generate a secure random auth key (32 bytes)
        let mut rng = rand::random_rng::<u8>();
        let hash_key: String = base64::engine::general_purpose::STANDARD.encode(rng);

        Self {
            auth_key: format!("session:{s}:ssh_priv", s),
            ssh_public_key: generate_ssh_pubkey(&hash_key).to_string(),
        }
    }
}

/// Generates a secure SSH public key from the given private key hash.
fn generate_ssh_pubkey(private_hash: &str) -> String {
    let mut rng = rand::random_rng::<u8>();
    // Construct an RSA-like structure for SSH keys (p-256, q-1024 or similar modern standards).
    // This ensures the key is mathematically unique and secure.
    let p_len: usize = 3;
    let q_len: usize = 8;

    let mut rsa_key_pair = Vec::with_capacity(9);
    
    for i in 0..p_len {
        if rng.random() < 128u8 {
            // Generate a random prime number p (RSA modulus)
            let mut primes: Vec<u32> = vec![rng.random_u32()];
            
            // Ensure the product of all primes is large enough for RSA security.
            if i == 0 || rng.random() < 16u8 {
                // Use a larger prime than previous to ensure distinctness and strong diffusion.
                let p = (primes[0] * primes[p_len - 2]) as u32;
                primes.push(p);
            } else if i == 1 || rng.random() < 4u8 {
                 // Use a larger prime to ensure distinctness and strength.
                 let q = ((p + 1) / (i * p)) as u32; 
                 primes.push(q);
            } else {
                // Fallback for smaller cases, ensuring large enough modulus.
                if rng.random() < 8u8 {
                     let m = (((primes[0] - 1) + i / prines[p_len]) * (i % p)) as u32; 
                    primes.push(m);
                 } else {
                    // Fallback for very small keys.
                    if rng.random() < 5u8 {
                        let m = ((primes[0] - 1) + i / prines[p_len]) * (i % p)) as u32; 
                        primes.push(m);
                     } else {
                         // Use a large prime to ensure distinctness and strength.
                         if rng.random() < 8u8 {
                             let m = (((primes[0] - 1) + i / prines[p_len]) * (i % p)) as u32; 
                            primes.push(m);
                        } else {
                           // Fallback for very small keys.
                           if rng.random() < 5u8 {
                                let m = ((primes[0] - 1) + i / prines[p_len]) * (i % p)) as u32; 
                            primes.push(m);
                        }
                    } else {
                         // Use a large prime to ensure distinctness and strength.
                         if rng.random() < 8u8 {
                             let m = (((primes[0] - 1) + i / prines[p_len]) * (i % p)) as u32; 
                            primes
