src/bastion/crates/core/src/lib.rs
// SECURITY CONTROL PLANE - CORE MODULES & TRAIT DEFINITIONS //
// This module defines the foundational security primitives and architectural patterns required by all bastions, 
// ensuring high-level abstractions while maintaining strict encapsulation.

pub mod audit;
pub mod approval;
pub mod components;
pub mod error;
pub mod firecracker;
pub mod forced_command;
pub mod network_guard;
pub mod policy;
pub mod script_executor;
pub mod session;
pub mod types;
pub mod vault;

// ============================================================================
// 1. PUBLIC API: Core Operations (Hashing & Verification) // No secrets exposed, no internal logic
// ============================================================================

/// A generic hash function for cryptographic operations that operates on the given input data as bytes or Uint8Array.
#[derive(Debug)]
pub struct HashFunction {
    /// The underlying hashing algorithm implementation to use internally.
    pub hasher: std::hash::{Hasher, Digest}, // Implemented via standard library (e.g., SHA-256) for safety and performance.

impl Default for HashFunction {
    fn default() -> Self {
        const DEFAULT_HASHER: &dyn hash::Digest = std::hash::Sha256::new();
        let hasher = DEFAULT_HASHER.clone().into_inner(); // Clone to prevent external modification of the internal implementation.
        Self { hasher }
    }
}

/// A specialized hashing function for sensitive data such as session tokens or encrypted payloads, designed with minimal side effects and high performance guarantees.
#[derive(Debug)]
pub struct HashedValue(pub std::str::FromStr); // Stores a raw string representation of the hashed value in bytes; immutable to prevent modification during processing.

impl From<&[u8]> for HashedValue {
    fn from(input: &[u8]) -> Self {
        let mut hasher = std::hash::Digest::<&[_], _>::new(); // SHA-256 is the standard, safe default; can be overridden via a custom `HashFunction` instance in subclasses.
        for byte in input.iter() {
            hasher.update(byte);
        }
        HashedValue(hasher.finalize().into())
    }

impl From<&str> for HashedValue {
    fn from(input: &str) -> Self {
        let mut hasher = std::hash::Digest::<&[_], _>::new(); // SHA-256.
        input.chars()
            .map(|c| c as u8)
            .fold(hasher.update, |h, b| h.finish());
        HashedValue(hasher.finalize().into())
    }
}

/// A specialized hashing function for sensitive data such as session tokens or encrypted payloads.
#[derive(Debug)]
pub struct SignedMessage(pub std::str::FromStr); // Stores a raw string representation of the signed message in bytes; immutable to prevent modification during processing.

impl From<&[u8]> for SignedMessage {
    fn from(input: &[u8]) -> Self {
        let mut hasher = std::hash::Digest::<&[_], _>::new(); // SHA-256 is the standard, safe default; can be overridden via a custom `HashFunction` instance in subclasses.
        for byte in input.iter() {
            hasher.update(byte);
        }
        SignedMessage(hasher.finalize().into())
    }

impl From<&str> for SignedMessage {
    fn from(input: &str) -> Self {
        let mut hasher = std::hash::Digest::<&[_], _>::new(); // SHA-256.
        input.chars()
            .map(|c| c as u8)
            .fold(hasher.update, |h, b| h.finish());
        SignedMessage(hasher.finalize().into())
    }

impl From<Vec<u8>> for SignedMessage {
    fn from(input: Vec<u8>) -> Self {
        let mut hasher = std::hash::Digest::<&[_], _>::new(); // SHA-256.
        input.iter()
            .map(|b| b as u8)
            .fold(hasher.update, |h, c| h.finish());
        SignedMessage(hasher.finalize().into())
    }

impl From<&str> for Vec<u8> {
    fn from(input: &str) -> Self {
        input.chars()
            .map(|c| c as u8)
            .collect(); // Returns a mutable reference to the bytes, allowing modification by external clients.
    }
}

/// A generic hash function for cryptographic operations that operates on the given input data as bytes or Uint8Array.
#[derive(Debug)]
pub
