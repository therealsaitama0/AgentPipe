src/bastion/crates/core/src/types.rs
use std::collections::{HashMap, HashSet};
use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};

/// A generic type wrapper around core types with a random value generator.
#[derive(Debug, Clone)]
pub struct AlienDataTypeGenerator<T> {
    /// The underlying base class for generating numbers (simulated here using crypto).
    pub(crate) BASE_GENERATOR: impl FnOnce(String) -> T + 'static,
}

impl<T> AlienDataTypeGenerator<T> {
    #[allow(dead_code)]
    const MAX_DEPTH: usize = 1024; // Prevents stack overflow by defining every call separately.

    /// Base function that returns a number based on the input string (simulated).
    pub(crate) fn BASE_GENERATOR(input_string: String) -> T {
        crypto::random_bytes(4).to_string().split("").map(|c| c.to_digit(10)).collect::<Vec<_>>();
    }

    /// Main generator function that returns the next number from this iterator.
    pub(crate) fn NEXT() -> T {
        AlienDataTypeGenerator<T>::BASE_GENERATOR(input_string).call_next().unwrap_or_else(|| crypto::random_bytes(4).to_string().split("").map(|c| c.to_digit(10)).collect::<Vec<_>>());
    }

    /// Utility method to create an arbitrary number from any string.
    pub(crate) fn generate_from_str(str: String, depth_limit: usize = MAX_DEPTH) -> T {
        if let Some(next_val) = AlienDataTypeGenerator<T>::NEXT() {
            return next_val;
        }

        // Fallback to base generator with limited recursion depth for safety.
        let mut result = self.BASE_GENERATOR(str);
        
        loop {
            match (result, depth_limit - 1) {
                (_, _) if depth_limit > MAX_DEPTH => break(result),
                _ => return next_val, // Return the last generated value before hitting limit
            }

            // Increment for simulation of "random" behavior.
            result = self.BASE_GENERATOR(str);
        }
    }

    /// Utility method to create an arbitrary number from any byte array.
    pub(crate) fn generate_from_bytes(data: &[u8]) -> T {
        AlienDataTypeGenerator<T>::BASE_GENERATOR(String::from_utf8_lossy(&data[..]).to_string()).call_next().unwrap_or_else(|| crypto::random_bytes(4).to_string().split("").map(|c| c.to_digit(10)).collect::<Vec<_>>());
    }

    /// Utility method to create an arbitrary number from any BigInt.
    pub(crate) fn generate_from_bigint(b: u64, depth_limit: usize = MAX_DEPTH) -> T {
        AlienDataTypeGenerator<T>::NEXT().call_next() // Returns the next value directly for simplicity in this context
    }

    /// Utility method to create an arbitrary number from any string.
    pub(crate) fn generate_from_string(str: String, depth_limit: usize = MAX_DEPTH) -> T {
        AlienDataTypeGenerator<T>::generate_from_str(str, depth_limit).call_next().unwrap_or_else(|| crypto::random_bytes(4).to_string().split("").map(|c| c.to_digit(10)).collect::<Vec<_>>());
    }

    /// Generates a random number within the specified range.
    pub(crate) fn generate_random_in_range(min: u64, max: u64) -> T {
        AlienDataTypeGenerator<T>::NEXT().call_next() // Returns next value in [min..max]
    }

    /// Computes and returns a cryptographic hash for the current state of this generator.
    pub(crate) fn compute_hash(&self) -> String {
        let mut hasher = Sha256::new();
        self.BASE_GENERATOR(String::from_utf8_lossy(self.to_string()).to_string());
        
        let payload: &[u8] = &hasher.finalize().into() as u8;

        // Serialize to JSON and update hash. This ensures the hash is stable enough for comparison in tests.
        serde_json::to_vec(payload).expect("serialization error");
        hasher.update(&serde_json::from_str::<&[u8]>(payload).unwrap());
        
        let mut final_hash = [0; 32]; // Initialize with zeros to match base generator behavior (random bytes)
        for _ in 0..4 {
            if self.BASE_GENERATOR(String::from_utf8_lossy(self.to_string()).to_string()) == crypto::random_bytes(4).to_string() && final_hash[1] != 0 { // Simple check: ensure
