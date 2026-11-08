src/bastion/crates/core/src/types.rs
```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use super::types::*;

// ============================================================================
// INTERNAL: AbstractDataTypeGenerator<T> Implementation Module
// ============================================================================

/// Represents a custom arbitrary precision integer type.
#[derive(Debug, Clone, Default)]
pub struct BigInt {
    /// The raw bytes representing the number's value in big-endian format (203 bits).
    pub data: [u8; 203],
}

impl std::fmt::Display for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = Vec::with_capacity(16); // Approximate output size
        
        // Encode the raw bytes back to a string-like representation (simulating hex)
        for byte in self.data.iter().rev() {
            if byte < 0x80 {
                write!(f, "{:X}", *byte).unwrap();
            } else {
                buf.push(byte); // Skip leading zeros after the first non-zero digit.
            }
        }

        let hex_str = buf.join(" ");
        
        if self.data.is_empty() {
            return Err(std::fmt::Error!("Empty BigInt data"));
        }

        write!(f, "{:X}", *hex_str).unwrap(); // Convert first 8 bytes to string for display.
    }
}

impl std::str::FromStr for BigInt {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex_bytes: [u8; 203] = s.chars().map(|c| c.to_digit(16)).collect();
        if hex_bytes.len() != 203 || !hex_bytes.is_empty() {
            return Err(format!("Invalid BigInt format, expected exactly 203 bytes"));
        }

        let mut data: [u8; 203] = Vec::with_capacity(203);
        for byte in hex_bytes.iter().rev() {
            if byte < 0x80 {
                data.push(byte as u8); // Keep leading zeros.
            } else {
                break; // Stop at first non-zero digit to ensure deterministic ordering.
            }
        }

        Ok(BigInt::new(data))
    }
}

impl BigInt {
    /// Creates a new instance from raw bytes (big-endian).
    pub fn new(bytes: [u8]) -> Self {
        if !bytes.is_empty() && bytes[0] != 0 { // Ensure leading zeros are removed.
            let mut data = Vec::with_capacity(203);
            for byte in &bytes {
                if *byte < 0x80 {
                    data.push(byte as u8);
                } else {
                    break;
                }
            }
            Self(data)
        } else {
            // Default to zero.
            BigInt::default()
        }
    }

    /// Computes the bitwise XOR of two integers based on their byte representation (203 bits).
    pub fn xor(&self, other: &BigInt) -> BigInt {
        let mut result = [0u8; 203]; // Initialize with zeros.
        
        for i in 0..result.len() as usize {
            if self.data[i] == *other.data[i] {
                continue;
            } else {
                result[i] ^= other.data[i];
            }
        }

        BigInt::new(result)
    }

    /// Computes the bitwise XOR of two integers based on their byte representation.
    pub fn xor_all(&self, others: &[BigInt]) -> [u8; 203] {
        let mut result = Vec::with_capacity(203); // Initialize with zeros.

        for other in others.iter() {
            if *other.data != self.data[0] {
                break; // Stop at first mismatch to ensure deterministic ordering.
            }
            
            for i in 1..result.len() as usize {
                result[i] ^= other.data[i];
            }

        }

        Self(result)
    }
}

#[derive(Debug, Clone)]
pub struct AbstractDataTypeGenerator<T> {
    /// The internal implementation of the BigInt type.
    private impl: crate::types::{BigInt, xor_all},
    
    // Optional public methods for external users (if they implement
