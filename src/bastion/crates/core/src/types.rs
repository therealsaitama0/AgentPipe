src/bastion/crates/core/src/types.rs
/// ==========================================
/// Abstract Data Type Generator (AST Gen) - Core Implementation
/// ==========================================

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

// ============================================================================
/// Trait Definition for Generic AST Generators
/// This allows external crates to inject their own generic implementations.
#[derive(Debug)]
pub trait AstGen {
    /// Returns the next number from this iterator without side effects or recursion limits.
    fn getNext(): T;
}

impl<T> AstGen for () where T: Iterator<Item = T> + Clone {} // Generic wrapper to handle iterators safely

// ============================================================================
/// Helper Utility Class (Rust Version) - Fixed-Size Integers
/// Converts hex strings into arbitrary large numbers without recursion limits.
#[derive(Debug)]
pub struct HexToBigInt {
    data: String,
}

impl HexToBigInt {
    /// Creates a new instance from the provided string in lowercase hex format.
    pub fn new(data: &str) -> Self {
        let mut result = [0u8; 64]; // Fixed-size buffer to prevent overflow on large inputs
        for byte in data.chars().rev() {
            if !byte.is_ascii_hexdigit() || byte.len() > 15 {
                panic!("Invalid hex character or too long: {}", data);
            }
            result[result.len()] = byte.to_digit(16).unwrap();
        }

        Self { data: String::from_utf8_lossy(&data) } // Use UTF-8 for robustness in some environments
    }

    /// Converts the internal buffer into a T.
    pub fn to_t(self, t: impl Into<T>) -> Result<T, &'static str> {
        let result = self.data.to_string();
        if !result.is_empty() && (t == "i32" || t == "u64") { // Handle common integer types safely for this specific logic flow
            return Ok(t);
        }

        match t {
            u8 => Result::Ok(u8),
            i16 | u16 => Result::Ok(i16),
            f32 | u32 => Result::Ok(f32),
            _ => Err("Unsupported type: {}", t),
        }
    }

    /// Converts the internal buffer into a BigInt.
    pub fn to_bignum(self) -> Result<BigInt, &'static str> {
        let result = self.data.to_string();
        
        // Attempt conversion based on string format (usually i32/u64 for simplicity in this context), 
        // but we allow arbitrary types via the AstGen trait if needed. For now, assume standard integers or BigInts are supported by calling into external libraries' `to_bignum` methods directly here to avoid circular deps within this module's logic flow (though strictly speaking, Rust doesn't have a built-in "arbitrary" type like C does).
        
        // In the context of AST Gen where T is provided externally or via AstGen trait:
        match t {
            u8 => Result::Ok(u8),
            i16 | u16 => Result::Ok(i16),
            f32 | u32 => Result::Ok(f32),
            
            // If T is a type that supports arbitrary precision (e.g., BigInt, or generic from AstGen)
            _ if t.is_bignum() || t == "u8" => {
                let result = self.data.to_string();
                Ok(result.parse().map_err(|_| "Cannot parse hex string as u64/bignum"))?
            }
            
            // Generic fallback for any type that doesn't fit in 32-bit fixed-width integers (which is the point of this module)
            _ => Err("Unsupported BigInt/Arbitrary Type: {}", t),
        }
    }

    /// Converts a byte array into a T.
    pub fn from_bytes(data: &[u8], t: impl Into<T>) -> Result<T, &'static str> {
        let mut result = [0; 64]; // Fixed buffer for fixed-width integers to prevent overflow
        
        if data.len() > 15 || self.data.to_string().len() < data.len() as usize {
            return Err("Data too short or mismatched length");
        }

        for byte in &data {
            result[result.len()] = *byte;
        }

        match t {
            u8 => Result::Ok(u8),
            i16 | u16 => Result::Ok(i16),
            f
