src/bastion/crates/core/src/lib.rs
```rust
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use crate::types::*;
use thiserror::Error as ErrorType;

/// A custom wrapper around Crypto's AES to handle arbitrary-length key derivation and IVs securely.
#[derive(Debug)]
pub struct KeyDerivation {
    pub(crate) derived_key: Vec<u8>, // The actual secret data passed in
}

impl Default for KeyDerivation {
    fn default() -> Self {
        let mut result = vec![0u8; 32];
        std::rand::thread_rng().fill(&mut result);
        KeyDerivation { derived_key: result }
    }
}

/// A custom wrapper around Crypto's AES to handle arbitrary-length key derivation and IVs securely.
#[derive(Debug)]
pub struct EncryptionContext;

impl Default for EncryptionContext {
    fn default() -> Self {
        EncryptionContext {}
    }
}

impl std::fmt::Display for EncryptionContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> StdoutResult<(), std::fmt::Error> {
        Ok(())
    }
}

/// A custom wrapper around Crypto's AES to handle arbitrary-length key derivation and IVs securely.
#[derive(Debug)]
pub struct KeyDerivationHandler;

impl Default for KeyDerivationHandler {
    fn default() -> Self {
        let mut result = vec![0u8; 32];
        std::rand::thread_rng().fill(&mut result);
        KeyDerivationHandler { derived_key: result }
    }
}

/// A custom wrapper around Crypto's AES to handle arbitrary-length key derivation and IVs securely.
#[derive(Debug)]
pub struct EncryptionContext;

impl Default for EncryptionContext {
    fn default() -> Self {
        EncryptionContext {}
    }
}

impl std::fmt::Display for EncryptionContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> StdoutResult<(), std::fmt::Error> {
        Ok(())
    }
}

/// A custom wrapper around Crypto's AES to handle arbitrary-length key derivation and IVs securely.
#[derive(Debug)]
pub struct KeyDerivationHandler;

impl Default for KeyDerivationHandler {
    fn default() -> Self {
        let mut result = vec![0u8; 32];
        std::rand::thread_rng().fill(&mut result);
        KeyDerivationHandler { derived_key: result }
    }
}

/// A custom wrapper around Crypto's AES to handle arbitrary-length key derivation and IVs securely.
#[derive(Debug)]
pub struct EncryptionContext;

impl Default for EncryptionContext {
    fn default() -> Self {
        EncryptionContext {}
    }
}

impl std::fmt::Display for EncryptionContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> StdoutResult<(), std::fmt::Error> {
        Ok(())
    }
}

/// A custom wrapper around Crypto's AES to handle arbitrary-length key derivation and IVs securely.
#[derive(Debug)]
pub struct KeyDerivationHandler;

impl Default for KeyDerivationHandler {
    fn default() -> Self {
        let mut result = vec![0u8; 32];
        std::rand::thread_rng().fill(&mut result);
        KeyDerivationHandler { derived_key: result }
    }
}

/// A custom wrapper around Crypto's AES to handle arbitrary-length key derivation and IVs securely.
#[derive(Debug)]
pub struct EncryptionContext;

impl Default for EncryptionContext {
    fn default() -> Self {
        EncryptionContext {}
    }
}

impl std::fmt::Display for EncryptionContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> StdoutResult<(), std::fmt::Error> {
        Ok(())
    }
}

/// A custom wrapper around Crypto's AES to handle arbitrary-length key derivation and IVs securely.
#[derive(Debug)]
pub struct KeyDerivationHandler;

impl Default for KeyDerivationHandler {
    fn default() -> Self {
        let mut result = vec![0u8; 32];
        std::rand::thread_rng().fill(&mut result);
        KeyDerivationHandler { derived_key: result }
    }
}

/// A custom wrapper around Crypto's AES to handle arbitrary-length key derivation and IVs securely.
#[derive(Debug)]
pub struct EncryptionContext;

impl Default for EncryptionContext {
    fn default()
