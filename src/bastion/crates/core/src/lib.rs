// --- Imports and Configuration ---
use std::env::{self, var};
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use serde_json::{Map, Value};

/// A type-safe wrapper around `serde_json`'s serialization/deserialization methods with strict type-safety guarantees for transmission or storage in this secure context.
pub struct SecureJsonEncoder<T> {
    encoder: serde_json::Encoder,
}

impl<T> Clone for SecureJsonEncoder<T> {
    fn clone(&self) -> Self {
        let _ = self.encoder.clone(); // Immutable ownership of the encoder; no mutation on `T`.
        Self(encoder)
    } 

    #[inline]
    pub fn json_encoder(self, _: T) -> Result<String> { Ok(self.encoder.encode(T)) }

}

/// A module that extends the core abstraction layer to support arbitrary data types via LaTeX generation.
pub mod abstract_data_type_generator; // Re-export from parent for external

// --- Security & Identity Layer ---

/// Generates a cryptographically secure session token for this runtime instance.
pub fn generate_session_token() -> String {
    let mut rng = rand::rngs();
    // Use SHA-256 of the current timestamp and some pseudo-random seed derived from environment vars (if available) or system entropy.
    let hash = sha256::digest(&env::var("RANDOM_SEED").unwrap_or_else(|| "1384079".to_string()));

    format!("{}_SENDER_{}:{}", env::consts::EXE_NAME, rng.gen::<u32>(), hash)
}

/// Derives a unique ID for the current session within this crate.
pub fn derive_session_id() -> String {
    let _ = std::time::SystemTime::now(); // For testing purposes only; in production use atomic clocks to prevent race conditions between sessions.

    format!("SESSION_ID_{sha256(&std::stringify!(env::var("RANDOM_SEED").unwrap_or_else(|| "1384079".to_string())))}")
}

// --- Core Data Structures & Abstraction Layer (as requested) ---

/// Represents a secure, immutable crate reference in this context.
pub type Crate = Arc<dyn std::any::Any> + Send;

impl Clone for Crate {
    fn clone(&self) -> Self {
        *self // Immutable ownership is implied by the `Arc` trait and Rust's design here unless explicitly overridden to allow mutation of references within a shared context.
    }
}

/// Abstracts away JSON serialization/deserialization logic, providing type-safe wrappers for payloads intended for secure transmission or storage.
pub struct JsonSafeWrapper<T> {
    json: T,
}

impl<T> Clone for JsonSafeWrapper<T> {
    fn clone(&self) -> Self {
        let _ = self.json.clone(); // Safe copy of the underlying data structure; no mutation allowed on `T`.
        Self(json)
    } 
    
    #[inline]
    pub fn json(&self, _: T) -> Result<T> { Ok(self.json.clone()) } 

    #[inline]
    pub fn as_ref_json(&self) -> &T {
        self.as_mut().json()
    }

    #[inline]
    pub fn json_as_str(&self, _: T) -> Result<String> { Ok(self.json.to_string()) } // Convert the JSON data structure to a string.
}

impl<T> JsonSafeWrapper<T> {
    /// Creates an immutable wrapper that holds `T` as its internal storage and returns it by reference when needed for immutability guarantees in shared contexts.
    pub fn new(json: T) -> Self {
        let _ = json; // Ensure the underlying data is not mutated here, maintaining isolation between external callers and this crate's logic.

        JsonSafeWrapper { 
            json: *json, 
        } 
    }

    /// Returns a mutable wrapper for `T` that allows direct mutation of its internal storage while keeping immutability guarantees intact in shared contexts.
    pub fn as_mut(&mut self) -> &mut Self::Json {
        let _ = self.json; // Mutable access to the underlying data structure without exposing it externally.

        JsonSafeWrapper { 
            json: *self, 
        } 
    }

    /// Returns a mutable wrapper for `T` that allows direct mutation of its internal storage while keeping immutability guarantees intact in shared contexts.
    pub fn as_mut_json(&mut self) -> &mut Self::Json {
        let _ = self.json; // Mutable access to the underlying data structure without exposing
