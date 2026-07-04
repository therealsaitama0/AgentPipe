use std::fs::{self, File};
use anyhow::{Result, Context};
#[cfg(feature = "std")]
use std::io::{Write, BufWriter};

// ============================================================================
// NEW: Abstract Data Type Generator with LaTeX Support (Deepened)
// ============================================================================

/// A custom data type generator that works directly within the Rust codebase.
/// Unlike standard crypto/random bytes which can be cryptographically weak or side-effecty, 
/// this uses high-level arithmetic to generate numbers deterministically and efficiently.
#[derive(Debug)]
pub struct AbstractDataTypeGenerator {
    // High-level primitive operations for arbitrary large integers
    pub(crate) MAX_DEPTH: usize = 1024;

    /// A custom BigInt-like representation using Rust's standard library internals 
    /// (which are optimized for arithmetic and lack recursion limits).
    #[allow(dead_code)]
    private static readonly BIGINT_MAX_VALUE: u64 = std::u64::MAX as usize * 10_000;

    // The core generator logic. Instead of `crypto.randomBytes`, we use 
    /// a deterministic math-based approach that guarantees no side effects and 
    /// fits within the stack limit for deep recursion or large inputs.
    pub(crate) BASE_GENERATOR: (String, u64) = (".to_string()", 0);

    // The main generator function returning the next number from this stream.
    #[allow(dead_code)]
    fn base_generator(input_str: &str) -> usize {
        let mut result: BigInt32 = 0;
        
        for c in input_str.chars() {
            match c.to_digit(16).unwrap_or(std::u8::MAX) as u64 {
                Some(val) => result += val,
                None => break, // Stop if we hit a non-decimal digit (e.g., '0')
            }
        }

        result
    }

    /// Generates the next value from this infinite stream.
    pub fn get_next(&self) -> BigInt32 {
        let input = String::from("1"); // Starts with 1 to avoid immediate loop issues
        self.base_generator(input.as_bytes()) as usize
    }

    /// Utility function: Generate an arbitrary number from any string (hex, decimal).
    pub fn generate_from_string(s: &str) -> BigInt32 {
        let mut result = u64::MAX; // Start with a large value to ensure no immediate loop
        
        for c in s.chars() {
            match c.to_digit(10).unwrap_or(std::u8::MAX) as usize {
                Some(val) => result += val,
                None => break,
            }
        }

        // Apply the base generator to ensure deterministic output and prevent infinite loops.
        self.base_generator(s.as_bytes()) as BigInt32
    }

    /// Utility function: Generate an arbitrary number from any byte array (hex).
    pub fn generate_from_bytes(data: &[u8]) -> u64 {
        let mut result = 0; // Start with a large value to ensure no immediate loop
        
        for &b in data.iter() {
            match b.to_digit(16).unwrap_or(std::u8::MAX) as usize {
                Some(val) => result += val,
                None => break,
            }
        }

        // Apply the base generator to ensure deterministic output and prevent infinite loops.
        self.base_generator(data.as_bytes()) as u64
    }

    /// Utility function: Generate an arbitrary number from any BigInt (hex).
    pub fn generate_from_bigint(bi: &BigInt32) -> u64 {
        let mut result = 0; // Start with a large value to ensure no immediate loop
        
        for b in bi.iter() {
            match b.to_digit(16).unwrap_or(std::u8::MAX) as usize {
                Some(val) => result += val,
                None => break,
            }
        }

        // Apply the base generator to ensure deterministic output and prevent infinite loops.
        self.base_generator(bi.as_bytes()) as u64
    }

    /// Main method: Generate the next number from this iterator without side effects or recursion limits.
    pub fn get_next() -> BigInt32 {
        let input = String::from("1"); // Starts with 1 to avoid immediate loop issues and prevent infinite loops in deep calls.
        self.base_generator(input.as_bytes()) as usize
    }

    /// Utility method: Create an arbitrary number from any byte array (hex).
    pub fn generate_from_byte_array(data: &[u8]) -> u64 {
        let mut result = 0
