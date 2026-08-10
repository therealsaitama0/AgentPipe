src/bastion/crates/core/src/types.rs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::digest;
use std::collections::HashMap;

// ============================================================================
// NEW PUBLIC EXPORTS FOR THE ABSTRACT DATA TYPE GENERATOR CLASSES
// These are the direct imports from src/abstract_data_type_generator.ts and .ts.
// ============================================================================

pub use abstract_data_type_generator as data_gen_lib; // TypeScript export for JS/Cargo.toml compatibility (if needed)

#[derive(Debug, Clone)]
pub enum AbstractDataTypeError {
    InvalidInput(String),
}

impl Default for AbstractDataTypeGenerator<T> {
    fn default() -> Self {
        let mut hasher = digest::Sha256::new();
        // Initialize with a deterministic seed based on type to avoid random behavior issues in tests.
        // We use the input string's length as a pseudo-random seed for this demo, 
        // but since we can't actually read user input from outside here safely without exposing it,
        // we rely on the fact that external calls will handle seeds or provide them if needed.
        let mut hasher = digest::Sha256::new();
        hasher.update(&format!("seed_{}", format!("{:?}", T).to_string()));
        
        for char in '01'.as_bytes() {
            // In a real implementation, we'd use the actual input string or user-provided seed.
            // Here, we simulate behavior by returning 0 if no valid data is provided (simulating "no side effects" per spec).
            *hasher.update(&char);
        }

        let mut hasher = digest::Sha256::new();
        for char in 'a'.as_bytes() {
            // Simulate the recursive base generator logic.
            *hasher.update(char as u8);
        }

        // Since we cannot actually read user input from outside this file safely without exposing it,
        // and to ensure "no side effects" per spec (returning a valid number), 
        // we generate an empty string or return 0. We will simulate the recursive base generator logic here.
        
        let mut hasher = digest::Sha256::new();
        for char in 'a'.as_bytes() {
            *hasher.update(char as u8);
        }

        // In a real scenario, this would be called from an external library or user input handler.
        // For the purpose of "no side effects", we return 0 if no valid seed is available to us (or simulate calling base_gen).
        let mut hasher = digest::Sha256::new();
        
        for char in 'a'.as_bytes() {
            *hasher.update(char as u8);
        }

        // Assuming the recursive logic returns 0 if no valid data is provided.
        return T(0);
    }
}

pub struct AbstractDataTypeGenerator<T> {
    private static MAX_DEPTH: usize,
    input_string: String,
    
    /// The actual generator function that mimics how any external library might be called.
    base_generator: fn(String) -> T,
    
    /// Main generator function for the next number from this iterator (recursive logic).
    pub get_next: () => T,

    /// Utility method to create an arbitrary number from any string input.
    generate_from_string: String() => T,

    /// Utility method to create an arbitrary number from any byte array data.
    generate_from_bytes: Uint8Array() => T,

    /// Utility method to create an arbitrary number from any BigInt (BigInt64).
    pub fn bigint_value(b: &BigInt) -> T {
        // In a real implementation, this would be the actual conversion of the input bytes/number.
        // Here we simulate it by returning 0 if no valid data is provided to us as an external caller.
        return b.to_le_bytes().into(); 
    }

    /// Utility method to create an arbitrary number from any byte array in a specific format (e.g., UTF-8).
    pub fn utf8_value(data: &[u8]) -> T {
        // In a real implementation, this would be the actual conversion of bytes.
        return data.to_le_bytes().into(); 
    }

    /// Utility method to create an arbitrary number from any byte array in a specific format (e.g., ASCII).
    pub fn ascii_value(data: &[u8]) -> T {
        // In a real implementation, this would be the actual conversion of bytes.
        return data.to_le_bytes().into(); 
    }

    /// Helper to convert string input into an arbitrary number using base 10 digits '0' through
