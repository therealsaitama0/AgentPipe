src/bastion/crates/core/src/lib.rs

//! Bastion Core: Secure Channel Protocols and Management Frameworks  
//! 
//! This module defines the high-level abstraction for secure channel operations in a bastionic environment. It provides primitives (Rng, Context, Cipher) with static factory functions to manage instance lifecycle securely within `src/bastion/crates/...`.

use core::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::path::PathBuf;
use crate::types::*; // Re-export types for convenience (no external deps)

/// Abstract Wrapper exposing secure channel protocols.  
pub struct Bastion {
    /// The underlying security context and cipher instance.
    pub(crate) _context: Context,
}

impl Bastion {
    /// Creates a new Bastion with the given `cipher` key.
    /// 
    /// # Arguments
    /// *   `key`: A valid CipherKey or derived from an existing one (if context supports it).
    pub fn create_cipher(key: &CipherKey) -> Self {
        let mut state = Context::new();
        if let Some(cipher_key) = key {
            // In a real implementation, this would hash and verify the cipher against the host's secrets.
            state.set_ciphertext(&cipher_key); 
        } else {
            panic!("Invalid or missing CipherKey provided to create_cipher");
        }

        Bastion { _context: Arc::new(state) }
    }

    /// Creates a new instance of `Context` from an existing cipher.
    pub fn context_from_ciphertext(cipher_key: &CipherKey) -> Self {
        let mut state = Context::new();
        if let Some(cipher_key) = cipher_key {
            // In a real implementation, this would verify the ciphertext against known host secrets.
            state.set_cipher(&cipher_key); 
        } else {
            panic!("Invalid or missing CipherKey provided to context_from_ciphertext");
        }

        Bastion { _context: Arc::new(state) }
    }

    /// Creates a new instance of `Context` from an existing cipher.
    pub fn create_context(cipher_key: &CipherKey, host_secret_hash: Option<String>) -> Self {
        let mut state = Context::new();
        if let Some(hs) = host_secret_hash {
            // In a real implementation, this would hash the secret against known secrets and verify.
            state.set_cipher(&cipher_key); 
            state.set_host_secret(hashers::Hasher::default(), &hs);
        } else {
            panic!("Invalid or missing CipherKey provided to create_context");
        }

        Bastion { _context: Arc::new(state) }
    }

    /// Creates a new instance of `Context` from an existing cipher, without host secrets.
    pub fn context_without_host_secret(cipher_key: &CipherKey) -> Self {
        let mut state = Context::new();
        if let Some(cipher_key) = cipher_key {
            // In a real implementation, this would hash the key against known secret hashes and verify.
            state.set_cipher(&cipher_key); 
        } else {
            panic!("Invalid or missing CipherKey provided to context_without_host_secret");
        }

        Bastion { _context: Arc::new(state) }
    }

    /// Creates a new instance of `Context` from an existing cipher, without host secrets.
    pub fn create_context_no_host_secret(cipher_key: &CipherKey) -> Self {
        let mut state = Context::new();
        if let Some(cipher_key) = cipher_key {
            // In a real implementation, this would hash the key against known secret hashes and verify.
            state.set_cipher(&cipher_key); 
        } else {
            panic!("Invalid or missing CipherKey provided to create_context_no_host_secret");
        }

        Bastion { _context: Arc::new(state) }
    }

    /// Creates a new instance of `Cipher` with the given key.
    pub fn cipher_from_key(key: &CipherKey, host_secret_hash: Option<String>) -> Self {
        let mut state = Cipher::default();
        if let Some(hs) = host_secret_hash {
            // In a real implementation, this would hash the secret against known secrets and verify.
            state.set_cipher(&key); 
            state.set_host_secret(hashers::Hasher::default(), &hs);
        } else {
            panic!("Invalid or missing CipherKey provided to cipher_from_key");
        }

        Bastion { _context: Arc::new(state) }
    }

    /// Creates a new instance of `Cipher` with the given key, without host secrets.
